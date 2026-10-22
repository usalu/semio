"""📝 Authoring aid: writes the Bun release corpus (corpus.json); expected digests are computed here once from the declared contents."""
import hashlib, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def content(spec):
    return (spec["repeat"] * spec["times"]).encode() if "repeat" in spec else spec["text"].encode()


def sha(spec):
    return hashlib.sha256(content(spec)).hexdigest()


MEMBERS = [
    {"name": "bun-darwin-aarch64/README.md", "method": "stored", "content": {"text": "readme of the pinned release\n"}},
    {"name": "bun-darwin-aarch64/bun", "method": "deflated", "content": {"repeat": "#!bun executable bytes 0123456789 ", "times": 400}},
    {"name": "bun-darwin-aarch64/zeros", "method": "stored", "content": {"repeat": "\u0000", "times": 64}},
]
MEMBER = "bun-darwin-aarch64/bun"
STORED = "bun-darwin-aarch64/zeros"


def ok(name, select, mutations=None):
    spec = next(m for m in MEMBERS if m["name"] == select)["content"]
    return {"name": name, "members": MEMBERS, "mutations": mutations or [], "select": select, "error": "", "sha256": sha(spec), "bytes": len(content(spec))}


def bad(name, select, error, mutations):
    return {"name": name, "members": MEMBERS, "mutations": mutations, "select": select, "error": error, "sha256": "", "bytes": 0}


def central(entry, field, value):
    return {"op": "central", "entry": entry, "field": field, "value": value}


c = {"schemaVersion": 1}
c["archives"] = [
    ok("a deflated member is inflated to its exact bytes", MEMBER),
    ok("a stored member is copied as it is", STORED),
    ok("a small stored text member", MEMBERS[0]["name"]),
    ok("a trailing blob within the comment window still finds the directory", MEMBER, [{"op": "trail", "bytes": 60000}]),
    bad("an unknown member is not found", "bun-darwin-aarch64/missing", "Bun archive has no selected executable", []),
    bad("a member name must match exactly", "bun-darwin-aarch64/BUN", "Bun archive has no selected executable", []),
    bad("a trailing blob beyond the window hides the directory", MEMBER, "Bun archive has no selected executable", [{"op": "trail", "bytes": 70000}]),
    bad("a truncated archive has no directory", MEMBER, "Bun archive has no selected executable", [{"op": "truncate", "bytes": 10}]),
    bad("an encrypted member is refused", MEMBER, "Invalid Bun ZIP executable", [central(MEMBER, "flags", 1)]),
    bad("a compression method other than stored or deflated is refused", MEMBER, "Invalid Bun ZIP executable", [central(MEMBER, "method", 9)]),
    bad("an expanded size beyond 256 MiB is refused", STORED, "Invalid Bun ZIP executable", [central(STORED, "expandedSize", 268435457)]),
    bad("a local header without its signature is refused", MEMBER, "Invalid Bun ZIP executable", [{"op": "local", "entry": MEMBER, "field": "signature", "value": 0}]),
    bad("a central entry without its signature is refused", MEMBER, "Invalid Bun ZIP directory", [central(MEMBERS[0]["name"], "signature", 0)]),
    bad("a stored member whose declared expanded size lies is refused", STORED, "Invalid Bun ZIP size", [central(STORED, "expandedSize", 65)]),
]
D = {"win32": "windows", "darwin": "darwin", "linux": "linux"}
c["distributions"] = [
    {"platform": "win32", "arch": "x64", "musl": False, "archive": "bun-windows-x64-baseline.zip"},
    {"platform": "win32", "arch": "arm64", "musl": False, "archive": "bun-windows-aarch64.zip"},
    {"platform": "darwin", "arch": "x64", "musl": False, "archive": "bun-darwin-x64-baseline.zip"},
    {"platform": "darwin", "arch": "arm64", "musl": False, "archive": "bun-darwin-aarch64.zip"},
    {"platform": "linux", "arch": "x64", "musl": False, "archive": "bun-linux-x64-baseline.zip"},
    {"platform": "linux", "arch": "arm64", "musl": False, "archive": "bun-linux-aarch64.zip"},
    {"platform": "linux", "arch": "x64", "musl": True, "archive": "bun-linux-x64-musl-baseline.zip"},
    {"platform": "linux", "arch": "arm64", "musl": True, "archive": "bun-linux-aarch64-musl.zip"},
]
c["unsupported"] = [
    {"name": "another version", "versionOffset": 1, "platform": "linux", "arch": "x64", "musl": False},
    {"name": "musl on macOS", "versionOffset": 0, "platform": "darwin", "arch": "arm64", "musl": True},
    {"name": "musl on Windows", "versionOffset": 0, "platform": "win32", "arch": "x64", "musl": True},
    {"name": "an unknown platform", "versionOffset": 0, "platform": "freebsd", "arch": "x64", "musl": False},
    {"name": "an unknown architecture", "versionOffset": 0, "platform": "linux", "arch": "ia32", "musl": False},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
