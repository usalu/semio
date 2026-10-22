"""📝 Authoring aid: writes the generator-outputs corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
WS = "/ws"
REFUSED = "Generator output has no Build producer: "

c = {"version": 1, "workspaceRoot": WS}
c["inputs"] = [
    {"name": "patterns only", "contract": {"inputPatterns": ["a/*.json", "b/c.ts"]}, "expected": {"inputs": ["{workspaceRoot}/a/*.json", "{workspaceRoot}/b/c.ts"], "dependsOn": []}},
    {"name": "no patterns", "contract": {"inputPatterns": []}, "expected": {"inputs": [], "dependsOn": []}},
    {"name": "a discovery kind adds the fingerprint output and its producer", "contract": {"inputPatterns": ["x"], "inputDiscovery": {"kind": "registry-catalog"}}, "fingerprint": "registry-catalog"},
    {"name": "an unknown discovery kind is refused", "contract": {"inputPatterns": [], "inputDiscovery": {"kind": "nope"}}, "error": "Generator input discovery has no fingerprint owner: nope"},
    {"name": "an inherited object member is not a discovery kind", "contract": {"inputPatterns": [], "inputDiscovery": {"kind": "toString"}}, "error": "Generator input discovery has no fingerprint owner: toString"},
    {"name": "a constructor is not a discovery kind", "contract": {"inputPatterns": ["x"], "inputDiscovery": {"kind": "constructor"}}, "error": "Generator input discovery has no fingerprint owner: constructor"},
]
own = lambda root, target, outputs, ownership="owned": {"ownership": ownership, "ownerPath": root, "target": target, "outputRoots": outputs}
o = lambda path, producer=None: {"path": path, **({"producer": producer} if producer else {})}
c["owners"] = [
    {"name": "an owned contract maps its outputs to its own target", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [o("a/x"), o("a/y")])}, "expected": [["p:gen", ["{workspaceRoot}/a/x", "{workspaceRoot}/a/y"]]]},
    {"name": "a contract of another root maps nothing", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("s", "p:gen", [o("a/x")])}, "expected": []},
    {"name": "a contract that is not owned is skipped", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [o("a/x")], "external")}, "expected": []},
    {"name": "a contract of this root without outputs keeps its entry point", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [])}, "expected": [["p:gen", []]]},
    {"name": "an output with a producer of this root goes to that producer", "root": "r", "project": "p", "targets": ["gen", "build"], "contracts": {"c": own("s", "q:gen", [o("a/x", {"ownerPath": "r", "target": "p:build"})])}, "expected": [["p:build", ["{workspaceRoot}/a/x"]]]},
    {"name": "a producer of another root is left to its owner", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [o("a/x", {"ownerPath": "s", "target": "q:build"}), o("a/y")])}, "expected": [["p:gen", ["{workspaceRoot}/a/y"]]]},
    {"name": "two contracts for one target append in order and keep the first entry", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c1": own("r", "p:gen", [o("a/1")]), "c2": own("r", "p:gen", [o("a/2")])}, "expected": [["p:gen", ["{workspaceRoot}/a/1", "{workspaceRoot}/a/2"]]]},
    {"name": "a producer target missing from the project is refused", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [o("a/x", {"ownerPath": "r", "target": "p:nothing"})])}, "error": REFUSED + "a/x"},
    {"name": "a producer of another project is refused", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "p:gen", [o("a/z", {"ownerPath": "r", "target": "q:gen"})])}, "error": REFUSED + "a/z"},
    {"name": "a producer name that only shares the project prefix is refused", "root": "r", "project": "p", "targets": ["gen"], "contracts": {"c": own("r", "pp:gen", [o("a/w")])}, "error": REFUSED + "a/w"},
    {"name": "an own target missing from the project is refused", "root": "r", "project": "p", "targets": [], "contracts": {"c": own("r", "p:gen", [o("a/v")])}, "error": REFUSED + "a/v"},
]
rp = lambda output, root, expected: {"output": output, "root": root, "expected": expected}
c["paths"] = [
    rp("{projectRoot}/a/b", "pkg/x", "/ws/pkg/x/a/b"), rp("{workspaceRoot}/a", "pkg", "/ws/a"), rp("rel/path", "pkg", None), rp("{projectRoot}", "pkg", None), rp("{workspaceRoot}", "pkg", None),
    rp("/abs/path", "pkg", None), rp("{projectRoot}/../y", "pkg", "/ws/y"), rp("{projectRoot}/x", ".", "/ws/x"), rp("{workspaceRoot}/{projectRoot}/z", "pkg", "/ws/pkg/z"),
    rp("{projectRoot}/x/{projectRoot}", "pkg", "/ws/pkg/x/{projectRoot}"), rp("{projectRoot}/x", "a$&b", "/ws/a$&b/x"), rp("{projectRoot}/x", "a$'b", "/ws/a$'b/x"), rp("{projectRoot}/x", "a$$b", "/ws/a$$b/x"),
    rp("{workspaceRoot}/a/../../outside", "pkg", "/outside"), rp("{projectRoot}/é/📜️script.ts", "pkg", "/ws/pkg/é/📜️script.ts"), rp("{projectRoot}//a///b/", "pkg", "/ws/pkg/a/b"), rp("{Projectroot}/a", "pkg", None),
]
G = lambda *outs: {"outputs": list(outs)}
c["couplings"] = [
    {"name": "a build target is not coupled", "target": "build", "targets": {"generate": G("{projectRoot}/o")}, "root": "pkg", "expected": []},
    {"name": "a check target is coupled to the output of a generate sibling", "target": "check", "targets": {"generate": G("{projectRoot}/o")}, "root": "pkg", "expected": ["/ws/pkg/o"]},
    {"name": "a check-prefixed target is coupled", "target": "check-docs", "targets": {"generate-docs": G("{workspaceRoot}/d")}, "root": "pkg", "expected": ["/ws/d"]},
    {"name": "a name that only starts with check is not coupled", "target": "checks", "targets": {"generate": G("{projectRoot}/o")}, "root": "pkg", "expected": []},
    {"name": "recheck is not coupled", "target": "recheck", "targets": {"generate": G("{projectRoot}/o")}, "root": "pkg", "expected": []},
    {"name": "generator and regenerate are not generate siblings", "target": "check", "targets": {"generator": G("{projectRoot}/a"), "regenerate": G("{projectRoot}/b"), "generates": G("{projectRoot}/c")}, "root": "pkg", "expected": []},
    {"name": "siblings contribute in declaration order", "target": "check", "targets": {"generate-b": G("{projectRoot}/b"), "generate-a": G("{projectRoot}/a", "{workspaceRoot}/w")}, "root": "pkg", "expected": ["/ws/pkg/b", "/ws/pkg/a", "/ws/w"]},
    {"name": "outputs of an unrecognized shape and siblings without outputs are ignored", "target": "check", "targets": {"generate": G("rel/o", "{projectRoot}/ok"), "generate-x": {}}, "root": "pkg", "expected": ["/ws/pkg/ok"]},
    {"name": "a digest already authored for the same path is not repeated", "target": "check", "targets": {"generate": G("{projectRoot}/o", "{projectRoot}/p")}, "root": "pkg", "authored": ["/ws/pkg/o"], "expected": ["/ws/pkg/p"]},
    {"name": "the same output named twice is digested once", "target": "check", "targets": {"generate": G("{projectRoot}/o", "{projectRoot}/o"), "generate-b": G("{projectRoot}/o")}, "root": "pkg", "expected": ["/ws/pkg/o"]},
    {"name": "string and unrelated authored inputs do not count as digests", "target": "check", "targets": {"generate": G("{projectRoot}/o")}, "root": "pkg", "authoredOther": ["x", {"dependentTasksOutputFiles": "y"}, {"runtime": 5}], "expected": ["/ws/pkg/o"]},
    {"name": "the target itself is never its own sibling", "target": "check", "targets": {"check": G("{projectRoot}/o")}, "root": "pkg", "expected": []},
]
FILES = lambda **kw: kw
c["digests"] = [
    {"name": "a missing root digests like nothing", "path": "missing", "files": {}},
    {"name": "an empty directory digests like nothing", "path": "d", "files": {}, "directories": ["d"]},
    {"name": "one file", "path": "d", "files": {"d/a.txt": "hello"}},
    {"name": "a root that is itself a file", "path": "f.txt", "files": {"f.txt": "alone"}},
    {"name": "directory entries are visited in sorted order", "path": "d", "files": {"d/c": "2", "d/a": "1", "d/B": "0", "d/_": "u", "d/10": "x", "d/9": "y"}},
    {"name": "nested directories are walked depth first in name order", "path": "d", "files": {"d/z/1": "z1", "d/a/2": "a2", "d/a/1": "a1", "d/m": "m"}},
    {"name": "names with a slash joiner are separated from content by a NUL", "path": "d", "files": {"d/a": "b\u0000c", "d/ab": "c"}},
    {"name": "unicode names and content", "path": "d", "files": {"d/é.txt": "ünï", "d/日本.txt": "語", "d/emoji😀.txt": "🙂"}},
    {"name": "binary looking content", "path": "d", "files": {"d/x": "\u0001\u0002\u007f"}},
    {"name": "a symbolic link is not part of the digest", "path": "d", "files": {"d/a": "1"}, "links": {"d/l": "a", "d/dl": "../elsewhere"}},
    {"name": "an empty file still contributes its name", "path": "d", "files": {"d/empty": "", "d/other": "o"}},
    {"name": "a trailing difference changes the digest", "path": "d", "files": {"d/a": "hello "}},
]
import hashlib


def expected_digest(case):
    h = hashlib.sha256()
    path, files = case["path"], case["files"]
    if path in files:
        h.update(b"\0" + files[path].encode("utf-8"))
    else:
        names = sorted((tuple(name[len(path) + 1:].split("/")), name) for name in files if name.startswith(path + "/"))
        for parts, name in names:
            h.update(("/".join(parts)).encode("utf-8") + b"\0")
            h.update(files[name].encode("utf-8"))
    return h.hexdigest()


for case in c["digests"]:
    case["expected"] = expected_digest(case)
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print({k: len(v) for k, v in c.items() if isinstance(v, list)})
