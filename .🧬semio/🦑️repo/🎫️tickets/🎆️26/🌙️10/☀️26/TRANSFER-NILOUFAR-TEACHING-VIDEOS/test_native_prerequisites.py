"""🐧 Check the platform-neutral rendering prerequisite contract."""
import json
from pathlib import Path
import unittest

ROOT = next(p for p in Path(__file__).resolve().parents if (p / ".devcontainer/Dockerfile").is_file())
CASES = json.loads(Path(__file__).with_name("native-prerequisites.json").read_text(encoding="utf-8"))
BOOT = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh"


class NativePrerequisites(unittest.TestCase):
    def test_native_packages(self):
        shell = BOOT.read_text(encoding="utf-8")
        for manager, packages in CASES["packages"].items():
            line = next(line for line in shell.splitlines() if line.strip().startswith(manager + ") printf"))
            for package in packages:
                with self.subTest(manager=manager, package=package):
                    self.assertIn(package, line.split())
        self.assertTrue("brew install " + " ".join(CASES["homebrew"]) in shell, "Homebrew Cairo/Pango prerequisites")
        self.assertTrue("brew install --cask " + " ".join(CASES["homebrew_fonts"]) in shell, "Homebrew mathematical symbol font")

    def test_container_packages(self):
        docker = (ROOT / ".devcontainer/Dockerfile").read_text(encoding="utf-8")
        for package in CASES["docker"]:
            with self.subTest(package=package):
                self.assertTrue(package in docker.split(), package)


if __name__ == "__main__":
    unittest.main()
