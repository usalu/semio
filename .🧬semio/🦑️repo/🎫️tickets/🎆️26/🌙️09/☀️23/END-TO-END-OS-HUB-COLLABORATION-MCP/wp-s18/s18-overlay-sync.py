"""🪞️ S18 (copied from SH2): mirrors the working tree into the S18 scratch overlay (gitignored): tracked + untracked-unignored files and the gitignored
`🤖️generated/` dirs the kernel includes, copying only files whose size or mtime differ, and removing overlay files the tree deleted
since the previous mirror (listed in `overlay-files.txt`); paths in `overlay-pin-head.txt` (a peer's in-flight, uncompiled worktree edit)
take their HEAD content instead, paths in `overlay-drop.txt` (the same peer's untracked new files) are removed and the gitignored
generated single files `.gitignore` names explicitly (`overlay-extra-files.txt`, e.g. the ui token table) are mirrored as well."""
import os, shutil, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
DEST = os.path.join(ROOT, ".🧬semio/🌐hub/s14-s18-overlay")
HERE = os.path.dirname(os.path.abspath(__file__))
LISTED = os.path.join(HERE, "generated", "overlay-files.txt")
GENERATED = os.path.join(ROOT, ".tmp-ticket/wp-sh1/generated/ignored-generated.txt")
PINNED = os.path.join(HERE, "overlay-pin-head.txt")
DROPPED = os.path.join(HERE, "overlay-drop.txt")
EXTRA = os.path.join(ROOT, ".tmp-ticket/wp-sh2/overlay-extra-files.txt")


def copy(rel: str) -> int:
    src, dst = os.path.join(ROOT, rel), os.path.join(DEST, rel)
    try:
        st = os.lstat(src)
    except FileNotFoundError:
        return 0
    if os.path.isdir(src) and not os.path.islink(src):
        return 0
    try:
        dt = os.lstat(dst)
        if dt.st_size == st.st_size and int(dt.st_mtime) == int(st.st_mtime):
            return 0
    except FileNotFoundError:
        pass
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src):
        if os.path.lexists(dst):
            os.remove(dst)
        os.symlink(os.readlink(src), dst)
    else:
        shutil.copy2(src, dst)
    return 1


def generated_files() -> list[str]:
    files = []
    for line in open(GENERATED, encoding="utf-8"):
        folder = line.strip()
        if not folder or not os.path.isdir(os.path.join(ROOT, folder)):
            continue
        for base, _, names in os.walk(os.path.join(ROOT, folder)):
            files.extend(os.path.relpath(os.path.join(base, name), ROOT) for name in names)
    return files


def main() -> None:
    listed = subprocess.run(["git", "ls-files", "-z", "-co", "--exclude-standard"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8").split("\0")
    files = sorted({f for f in listed if f and not f.startswith(".🧬semio/🦑️repo/🎫️tickets/")} | set(generated_files()) | {line.strip() for line in open(EXTRA, encoding="utf-8") if line.strip()})
    previous = set(open(LISTED, encoding="utf-8").read().split("\n")) if os.path.exists(LISTED) else set()
    removed = 0
    for rel in sorted(previous - set(files)):
        if rel and os.path.lexists(os.path.join(DEST, rel)) and not os.path.lexists(os.path.join(ROOT, rel)):
            os.remove(os.path.join(DEST, rel))
            removed += 1
    with ThreadPoolExecutor(max_workers=8) as pool:
        copied = sum(pool.map(copy, files))
    with open(LISTED, "w", encoding="utf-8") as handle:
        handle.write("\n".join(files))
    pinned = 0
    if os.path.exists(PINNED):
        for rel in [line.strip() for line in open(PINNED, encoding="utf-8") if line.strip()]:
            staged = subprocess.run(["git", "show", f"HEAD:{rel}"], cwd=ROOT, capture_output=True, check=True).stdout
            with open(os.path.join(DEST, rel), "wb") as handle:
                handle.write(staged)
            pinned += 1
    dropped = 0
    if os.path.exists(DROPPED):
        for rel in [line.strip() for line in open(DROPPED, encoding="utf-8") if line.strip()]:
            target = os.path.join(DEST, rel)
            if os.path.isdir(target):
                shutil.rmtree(target)
                dropped += 1
            elif os.path.lexists(target):
                os.remove(target)
                dropped += 1
    print(f"files={len(files)} copied={copied} removed={removed} pinned-to-head={pinned} dropped={dropped}")


if __name__ == "__main__":
    sys.exit(main())
