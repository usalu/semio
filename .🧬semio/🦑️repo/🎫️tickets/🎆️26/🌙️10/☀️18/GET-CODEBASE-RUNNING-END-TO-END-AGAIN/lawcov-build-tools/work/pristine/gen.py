"""📝 Authoring aid: writes the pristine restore corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
NESTED = "Registry package cannot own installed nested dependencies"
SHARED = "Exact shared binary registry bytes required"
UNKNOWN = "Unknown shared package inode refused"
SHAPE = "Registry package regular file shape required"
NAMESPACE = "Bounded Nx package namespace required"
CEILING = "Nx package path ceiling exceeded"


def a(path, text, mode=420):
    return {"path": path, "kind": "file", "text": text, "mode": mode}


def ad(path):
    return {"path": path, "kind": "directory", "text": "", "mode": 493}


def f(path, text, **k):
    return {"kind": "file", "path": path, "text": text, **k}


def d(path):
    return {"kind": "dir", "path": path}


def s(path, to="package.json"):
    return {"kind": "symlink", "path": path, "to": to}


def case(name, clone, archive, shared=None, error=None, generate=None):
    return {"name": name, "clone": clone, "archive": archive, "shared": shared or [], "error": error, "generate": generate}


BASE = [a("package.json", "{\"name\":\"nx\"}\n"), a("lib/index.js", "exports.a=1;\n"), a("bin/nx.js", "#!/usr/bin/env node\n", 493)]
SAME = [f("package.json", "{\"name\":\"nx\"}\n"), f("lib/index.js", "exports.a=1;\n"), f("bin/nx.js", "#!/usr/bin/env node\n")]
c = {"version": 1, "maximumEntries": 4096, "maximumPathBytes": 256, "recheckEvery": 64}
c["cases"] = [
    case("an unchanged clone is rewritten from the registry", SAME, BASE),
    case("an old version of a file is replaced", [f("package.json", "old\n"), f("lib/index.js", "old\n"), f("bin/nx.js", "old\n")], BASE),
    case("a file the registry package does not have is removed", SAME + [f("README.md", "x"), f("lib/.bun-tag", "tag")], BASE),
    case("a directory the registry package does not have is pruned with its files", SAME + [f("extra/a.txt", "a"), f("extra/deep/b.txt", "b"), d("empty")], BASE),
    case("a directory holding registry files is kept", SAME + [d("lib/empty")], BASE),
    case("the installed nested dependencies are left alone", SAME + [f("node_modules/dep/index.js", "dep"), f("node_modules/dep/package.json", "{}")], BASE),
    case("a shared binary with the registry bytes keeps its link", SAME + [f("bin/native.node", "BINARY", shared=True)], BASE + [a("bin/native.node", "BINARY", 493)], [{"path": "bin/native.node", "text": "BINARY"}]),
    case("a shared binary with other bytes than the registry is refused", SAME + [f("bin/native.node", "OTHER", shared=True)], BASE + [a("bin/native.node", "BINARY", 493)], [{"path": "bin/native.node", "text": "OTHER"}], SHARED),
    case("a shared file the registry package does not have is refused", SAME + [f("bin/stray.node", "S", shared=True)], BASE, [{"path": "bin/stray.node", "text": "S"}], SHARED),
    case("a shared file the receipt does not know is an unknown inode", SAME + [f("bin/native.node", "BINARY", extraLinks=1)], BASE + [a("bin/native.node", "BINARY", 493)], [], UNKNOWN),
    case("an unshared file with a second link is an unknown inode", [f("package.json", "{\"name\":\"nx\"}\n", extraLinks=1), f("lib/index.js", "exports.a=1;\n"), f("bin/nx.js", "x")], BASE, [], UNKNOWN),
    case("a link inside the package is refused", SAME + [s("lib/link.js")], BASE, [], SHAPE),
    case("a registry file missing from the clone is refused", [f("package.json", "{\"name\":\"nx\"}\n"), f("lib/index.js", "exports.a=1;\n")], BASE, [], "missing"),
    case("a registry package that owns nested dependencies is refused", SAME, BASE + [a("node_modules/dep/index.js", "x")], [], NESTED),
    case("a registry package that owns the nested directory is refused", SAME, BASE + [ad("node_modules")], [], NESTED),
    case("directory entries of the registry package are ignored", SAME, BASE + [ad("lib"), ad("bin")]),
    case("an empty registry file is written empty", SAME + [f("empty", "x")], BASE + [a("empty", "")]),
    case("an executable registry file is written executable and the others are not", [f("package.json", "{\"name\":\"nx\"}\n", exec=True), f("lib/index.js", "exports.a=1;\n"), f("bin/nx.js", "#!/usr/bin/env node\n")], BASE),
    case("more than 64 files cross the recheck window more than once", [f("many/f%03d.js" % i, "old") for i in range(150)] + SAME, BASE + [a("many/f%03d.js" % i, "new %d" % i) for i in range(150)]),
    case("a clone beyond the entry limit is refused", SAME, BASE, [], NAMESPACE, {"prefix": "gen/f", "count": 4100}),
    case("a path beyond the byte ceiling is refused", SAME + [f("/".join(["d" * 90] * 3) + "/x.js", "x")], BASE, [], CEILING),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(c["cases"]))
