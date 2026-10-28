"""[lane diskproof] law: `mirror.py drop` releases every disk image attached from or mounted below the mirror before it deletes the tree (usage: python3 -I test_mirror_drop.py <mirror.py>). macOS only (hdiutil); independent oracle: the text of `mount` and `hdiutil info`, not the plist the tool reads."""
import json, os, subprocess, sys, tempfile, time, unittest

TOOL = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else None
DARWIN = sys.platform == "darwin"


def attached_text(home):
    mounts = subprocess.run(["mount"], capture_output=True, text=True).stdout
    info = subprocess.run(["hdiutil", "info"], capture_output=True, text=True).stdout
    real = os.path.realpath(home)
    return [line for line in (mounts + info).splitlines() if (home in line or real in line)]


@unittest.skipUnless(DARWIN and TOOL, "needs macOS hdiutil and the tool path")
class MirrorDrop(unittest.TestCase):
    def setUp(self):
        self.home = tempfile.mkdtemp(prefix="dpd-", dir="/private/tmp")
        self.root = os.path.join(self.home, "x")
        os.makedirs(os.path.join(self.root, "mnt"))
        os.makedirs(os.path.join(self.home, ".meta"))
        with open(os.path.join(self.home, ".meta", "x.json"), "w") as handle:
            json.dump({"name": "x", "created": time.time(), "files": {}, "root": self.root}, handle)
        self.images = []

    def tearDown(self):
        import plistlib
        out = subprocess.run(["hdiutil", "info", "-plist"], capture_output=True).stdout
        for image in plistlib.loads(out).get("images", []) if out else []:
            places = [image.get("image-path") or ""] + [entity.get("mount-point") or "" for entity in image.get("system-entities", [])]
            if any(os.path.realpath(self.home) in os.path.realpath(place) or self.home in place for place in places if place):
                device = next((e["dev-entry"] for e in image.get("system-entities", []) if e.get("dev-entry")), None)
                for _ in range(5):
                    if device and subprocess.run(["hdiutil", "detach", "-force", device], capture_output=True).returncode != 0:
                        time.sleep(1)
                    else:
                        break
        subprocess.run(["rm", "-rf", self.home])

    def attach(self, name, mount):
        path = os.path.join(self.root, name)
        subprocess.run(["hdiutil", "create", "-size", "4m", "-fs", "APFS", "-volname", "dpd", path], check=True, capture_output=True)
        args = ["hdiutil", "attach", "-nobrowse", "-noverify"] + (["-mountpoint", mount] if mount else ["-nomount"]) + [path]
        subprocess.run(args, check=True, capture_output=True)
        self.images.append(path)

    def drop(self, *extra):
        env = dict(os.environ, SEMIO_MIRROR_HOME=self.home)
        return subprocess.run([sys.executable, "-I", TOOL, "drop", "x", "--sync", *extra], capture_output=True, text=True, env=env)

    def test_drop_detaches_mounted_and_unmounted_images_before_deleting(self):
        self.attach("a.dmg", os.path.join(self.root, "mnt"))
        self.attach("b.dmg", None)
        self.assertTrue(attached_text(self.home), "precondition: both images are attached from below the mirror")
        run = self.drop()
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertFalse(os.path.exists(self.root), "the mirror tree is gone")
        self.assertEqual(attached_text(self.home), [], "no image stays attached: " + run.stderr)

    def test_drop_without_images_is_unchanged(self):
        run = self.drop()
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertFalse(os.path.exists(self.root))


if __name__ == "__main__":
    sys.argv = sys.argv[:1] + sys.argv[2:]
    unittest.main()
