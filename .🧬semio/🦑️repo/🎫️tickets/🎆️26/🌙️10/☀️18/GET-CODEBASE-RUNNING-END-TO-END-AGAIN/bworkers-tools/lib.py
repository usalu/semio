# 🧰 Shared helpers of the bworkers staging tools: repository paths, anchored single replacements and content hashes.
import hashlib, os, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 8)))
if not os.path.isdir(os.path.join(ROOT, ".git")) and not os.path.exists(os.path.join(ROOT, "bunfig.toml")):
    ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(TOOLS, "staged")

FW = "\U0001F9F0️framework"
PRODUCTS = "\U0001F6CD️products"
REPO = "\U0001F991️repo"
MODULES = "\U0001F528️modules"
LIBRARY = "\U0001F4DA️library"
LIB = "/".join([FW, PRODUCTS, REPO, MODULES, LIBRARY])
BUILD = LIB + "/\U0001F3D7️build"
BAZEL = BUILD + "/\U0001F527️tools/\U0001F3D7️bazel"
WORKERS = BUILD + "/\U0001F477️workers"
PROTOCOL = WORKERS + "/\U0001F4E8️protocol"
SCRIPT = "\U0001F4DC️script.ts"
TS = "\U0001F7E6️.ts"
JSON_NAME = "\U0001F523️.json"
SCHEMA = "\U0001F9EC️schema"
TESTS = "\U0001F9EA️tests"
FIXTURES = "\U0001F9EB️fixtures"
BOOT = LIB + "/⚡️caching/\U0001F680️bootstrap"
NXTOOLS = BOOT + "/\U0001F6E0️tools"
NXWORKERS = NXTOOLS + "/\U0001F477️workers"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read(path: str) -> str:
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def staged(name: str) -> str:
    with open(os.path.join(STAGED, name), encoding="utf8") as handle:
        return handle.read()


def write_staged(name: str, text: str) -> None:
    os.makedirs(STAGED, exist_ok=True)
    with open(os.path.join(STAGED, name), "w", encoding="utf8") as handle:
        handle.write(text)


class Edit:
    def __init__(self, text: str, label: str):
        self.text, self.label = text, label

    def once(self, old: str, new: str) -> "Edit":
        parts = self.text.split(old)
        if len(parts) != 2:
            sys.exit(f"{self.label}: anchor count {len(parts) - 1}: {old[:90]!r}")
        self.text = new.join(parts)
        return self
