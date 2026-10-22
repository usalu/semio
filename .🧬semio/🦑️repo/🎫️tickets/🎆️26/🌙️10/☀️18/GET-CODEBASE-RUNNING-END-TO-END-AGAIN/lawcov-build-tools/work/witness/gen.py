"""📝 Authoring aid: writes the witness digest corpus (corpus.json); expected digests are computed here once from the declared files."""
import hashlib, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def raw(content):
    return (content["repeat"] * content["times"]).encode() if "repeat" in content else content["text"].encode()


def digest(files):
    outer = hashlib.sha256()
    for path, content in sorted(((f["path"], raw(f["content"])) for f in files), key=lambda row: row[0]):
        outer.update(path.encode() + b"\0" + str(len(content)).encode() + b"\0" + hashlib.sha256(content).hexdigest().encode() + b"\n")
    return outer.hexdigest()


def f(path, text="x"):
    return {"path": path, "content": {"text": text}}


def case(name, files):
    return {"name": name, "files": files, "expected": {"version": 1, "files": len(files), "sha256": digest(files)}}


c = {"version": 1}
c["cases"] = [
    case("no files", []),
    case("one file", [f("a.ts", "export {}\n")]),
    case("the input order does not matter", [f("b.ts", "b"), f("a.ts", "a"), f("c.ts", "c")]),
    case("the same bytes under two paths differ from one path", [f("a.ts", "same"), f("b.ts", "same")]),
    case("an edit of one byte changes the digest", [f("a.ts", "abd")]),
    case("upper case sorts before lower case", [f("b.ts", "1"), f("C.ts", "2"), f("a.ts", "3"), f("D.ts", "4")]),
    case("paths with a directory depth sort as whole strings", [f("a/b.ts", "1"), f("a.ts", "2"), f("a-b.ts", "3"), f("a/a.ts", "4")]),
    case("spaces and non-ASCII names", [f("dir with space/é ü.ts", "1"), f("日本語.ts", "2")]),
    case("an empty file counts with its zero size", [f("empty", ""), f("full", "x")]),
    case("bytes that are NUL and newline", [f("bin", "\u0000\n\u0000\r\n")]),
    case("a one megabyte file", [{"path": "big.bin", "content": {"repeat": "0123456789abcdef", "times": 65536}}]),
    case("more files than the read concurrency of 64", [f("f%03d.ts" % i, "content %d" % i) for i in range(200)]),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
