import os
import plistlib
import re
import signal
import stat
import subprocess
import sys
import tempfile
import time
from contextlib import contextmanager
from pathlib import Path


def native(tool, *args, timeout=30):
    began = time.monotonic()
    with subprocess.Popen(
        ["repo", "tools", "exec", "--light", tool, "--", *map(str, args)],
        start_new_session=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    ) as process:
        try:
            output, error = process.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGINT)
            try:
                process.communicate(timeout=12)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
            raise RuntimeError(f"{tool} {args[0]} exceeded {timeout}s") from None
    print(f"[DEBUG] {tool} {args[:2]} exit {process.returncode} in {time.monotonic()-began:.2f}s", flush=True)
    if process.returncode:
        raise RuntimeError(error.decode(errors="replace")[-2000:])
    return output


def identity(path):
    facts = path.lstat()
    return facts.st_dev, facts.st_ino


def native_path(path):
    if not isinstance(path, str) or not path.startswith("/"):
        return None
    return Path(os.path.normpath("/private" + path if path.startswith("/tmp/") else path))


def association(backing, mount, closing=False):
    images = plistlib.loads(native("hdiutil", "info", "-plist"))["images"]
    paths = (value for image in images
             for value in [image.get("image-path"), *(row.get("mount-point") for row in image["system-entities"])])
    if closing and any(path is not None and mount in path.parents for path in map(native_path, paths)):
        raise RuntimeError("Nested private association remains; preserve the outer mount")
    owned = [image for image in images if native_path(image.get("image-path")) == backing]
    occupants = [(image, entity) for image in images for entity in image["system-entities"]
                 if native_path(entity.get("mount-point")) == mount]
    if not owned and not occupants:
        return None
    if len(owned) != 1 or len(occupants) != 1 or occupants[0][0] is not owned[0]:
        raise RuntimeError("Unproved association; preserve the image and mount")
    device = occupants[0][1]["dev-entry"]
    if not re.fullmatch(r"/dev/disk\d+(?:s\d+)*", device):
        raise RuntimeError("Unproved mounted device; preserve the image and mount")
    return device


@contextmanager
def owned_volume(work, mount, outer=False):
    backing = work / ("outer.sparseimage" if outer else "fixed.dmg")
    bare = identity(mount)
    print(f"[DEBUG] private backing {backing}; mount {mount}", flush=True)
    backing_identity = None
    try:
        if outer:
            native("hdiutil", "create", "-type", "SPARSE", "-size", "512m", "-nospotlight",
                   "-fs", "Case-sensitive APFS", "-volname", "Semio lab", backing)
        else:
            seed = work / "seed"
            seed.mkdir()
            native("hdiutil", "create", "-size", "64m", "-format", "UDRW", "-nospotlight",
                   "-fs", "Case-sensitive APFS", "-volname", "semio", "-srcfolder", seed, backing)
        backing_identity = identity(backing)
        if not outer:
            native("hdiutil", "resize", "-size", "160m", backing)
        facts = backing.lstat()
        if ((facts.st_dev, facts.st_ino) != backing_identity or facts.st_uid != os.getuid()
                or not stat.S_ISREG(facts.st_mode) or facts.st_nlink != 1
                or facts.st_blocks * 512 > (512 if outer else 160) * 1024**2):
            raise RuntimeError("Private image identity or size changed")
        print(f"[DEBUG] allocated backing bytes {facts.st_blocks * 512}", flush=True)
        if association(backing, mount) or identity(mount) != bare or any(mount.iterdir()):
            raise RuntimeError("Unused owned mount required before attachment")
        if outer:
            native("hdiutil", "attach", "-plist", "-mount", "required", "-nobrowse",
                   "-owners", "on", "-mountpoint", mount, backing)
        else:
            native("diskutil", "image", "attach", "--nobrowse", "--mountOptions", "owners",
                   "--mountPoint", mount, backing, timeout=45)
        device = association(backing, mount)
        info = plistlib.loads(native("diskutil", "info", "-plist", mount))
        if (not device or info.get("DeviceNode") != device or identity(mount)[0] == bare[0]
                or info.get("FilesystemType") != "apfs" or info.get("GlobalPermissionsEnabled") is not True):
            raise RuntimeError("Canonical attach lacked independent mounted-device confirmation")
        print(f"[DEBUG] mounted {device}, filesystem={info.get('FilesystemType')}, "
              f"owners={info.get('GlobalPermissionsEnabled')}", flush=True)
        yield
    finally:
        device = association(backing, mount, closing=True)
        if device:
            if not backing_identity or identity(backing) != backing_identity or identity(mount)[0] == bare[0]:
                raise RuntimeError("Private attach custody changed; retain probe artifacts")
            native("hdiutil", "detach", mount)
            if association(backing, mount, closing=True):
                raise RuntimeError("Probe mount remains associated")
        if identity(mount) != bare or (backing.exists() and identity(backing) != backing_identity):
            raise RuntimeError("Private path identity changed; retain probe artifacts")
        mount.rmdir()
        backing.unlink(missing_ok=True)
        if not outer:
            seed.rmdir()
        work.rmdir()
        print("[DEBUG] private image and mount cleared; no surviving association", flush=True)


def main():
    if sys.platform != "darwin" or sys.argv[1:] not in ([], ["nested"]):
        raise SystemExit("Requires Darwin; optional mode: nested")
    ticket = Path(__file__).resolve().parent
    work = Path(tempfile.mkdtemp(prefix="attach-pulse-", dir=ticket / "🗑️generated"))
    mount = Path(tempfile.mkdtemp(prefix="lv-probe-", dir="/private/tmp"))
    nested = sys.argv[1:] == ["nested"]
    with owned_volume(work, mount, outer=nested):
        if nested:
            os.chmod(mount, 0o700)
            child = Path(tempfile.mkdtemp(prefix="native-", dir=mount))
            child_mount = child / "m"
            child_mount.mkdir(mode=0o700)
            with owned_volume(child, child_mount):
                pass


if __name__ == "__main__":
    main()
