"""📝 Authoring aid: writes the graph retry diagnostic corpus (corpus.json), the unit's case shape plus the digests the observer would currently see and an authored error message."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
A, B, C = "a" * 64, "b" * 64, "c" * 64
REFUSED = "Declared graph retry reason and content digests required"
BOUNDED = "Bounded observed graph fact required"


def stale(path, kind="file", fact=None):
    return {"kind": kind, "path": path, "fact": fact if fact is not None else {"sha256": A}}


def case(name, reason="observed-facts", before=A, after=B, rows=None, current=None, error=""):
    return {"name": name, "reason": reason, "before": before, "after": after, "stale": rows or [], "current": current or {}, "error": error}


c = {"version": 1, "root": "/repo"}
c["cases"] = [case("every reason is declared: " + r, r) for r in ["membership", "admission", "observed-facts", "producer-error", "owner-registry", "publication-facts"]]
c["cases"] += [
    case("an unknown reason is refused", "because", error=REFUSED),
    case("a digest in capitals is refused", before="A" * 64, error=REFUSED),
    case("a digest of 63 characters is refused", after="a" * 63, error=REFUSED),
    case("null digests are accepted", before=None, after=None),
    case("a nested file is projected with its current digest", rows=[stale("src/a/b.ts")], current={"src/a/b.ts": C}),
    case("a path spelled with a dot segment is normalized", rows=[stale("./src/../src/a.ts")], current={"src/a.ts": C}),
    case("an absolute path inside the root is made relative", rows=[stale("/repo/x.ts")], current={"x.ts": C}),
    case("an absolute path outside the root is redacted", rows=[stale("/other/x.ts")], current={"../other/x.ts": C}),
    case("a parent path is redacted and never read", rows=[stale("../x.ts")], current={"../x.ts": C}),
    case("a sibling directory with the root name as prefix is redacted", rows=[stale("/repository/x.ts")], current={"../repository/x.ts": C}),
    case("a digest that is not valid is not reported", rows=[stale("a.ts")], current={"a.ts": "nothex"}),
    case("an unreadable file has no current digest", rows=[stale("a.ts")], current={}),
    case("a fact without its own digest is keyed by the digest of the fact", rows=[stale("a.ts", "file", {"mtime": 5, "size": 9})], current={"a.ts": C}),
    case("a directory fact is keyed by the digest of the fact and never read", rows=[stale("src", "directory", {"entries": ["a", "b"]})], current={"src": C}),
    case("a very long kind is cut to 64 characters", rows=[stale("a.ts", "k" * 100, {"x": 1})]),
    case("only the first eight facts are kept", rows=[stale("f%d.ts" % i) for i in range(12)], current={"f%d.ts" % i: C for i in range(12)}),
    case("a path of 256 characters inside the root is kept", rows=[stale("p" * 256)], current={"p" * 256: C}),
    case("a path of 257 characters is redacted", rows=[stale("p" * 257)], current={"p" * 257: C}),
    case("a unicode path survives", rows=[stale("é/🧪.ts", "file", {"sha256": B})], current={"é/🧪.ts": C}),
    case("a fact above 64 KiB is refused", rows=[stale("a.ts", "file", {"blob": "x" * 70000})], error=BOUNDED),
    case("a fact of exactly 64 KiB is kept", rows=[stale("a.ts", "file", {"blob": "x" * (65536 - len('{"blob":""}'))})]),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
