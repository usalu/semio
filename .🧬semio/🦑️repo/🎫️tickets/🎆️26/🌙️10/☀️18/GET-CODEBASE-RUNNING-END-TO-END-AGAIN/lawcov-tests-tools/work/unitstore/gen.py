"""📝 Authoring aid: writes the unit cache store corpus (corpus.json): scenarios of publish, lookup, restore, mutate, prune and audit steps on a cache root and two lanes, plus tree digests and build runs."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
TREE = {"pkg/src/lib.rs": "fn a() {}\n", "pkg/src/util.rs": "fn u() {}\n"}
DEP = "{out}/libx.rlib: {lane}/pkg/src/lib.rs {lane}/pkg/src/util.rs\n\n{lane}/pkg/src/lib.rs:\n{lane}/pkg/src/util.rs:\n# env-dep:FOO=bar\n"
OUTPUTS = {"libx.rlib": "RLIB built in {lane}\n", "libx.rmeta": "META\n", "x.d": DEP}
LANES = ["laneA", "laneB"]


def publish(key, lane="A", outputs=None, executable=("libx.rlib",), stdout="compiling {lane}/pkg/src/lib.rs into {out}\n", stderr="warning at {lane}/pkg/src/util.rs\n", **kw):
    return {"op": "publish", "lane": lane, "key": key, "crate": "x", "outputs": outputs or OUTPUTS, "executable": list(executable), "stdout": stdout, "stderr": stderr, **kw}


def lookup(key, expect, lane="B", env=None, **kw):
    return {"op": "lookup", "lane": lane, "key": key, "env": env if env is not None else {"FOO": "bar"}, "expect": expect, **kw}


K1, K2, K3 = "1" * 64, "2" * 64, "ab" * 32
published = {"published": True, "crate": "x", "outputs": [{"name": "libx.rlib", "template": False, "executable": True}, {"name": "libx.rmeta", "template": False, "executable": False}, {"name": "x.d", "template": True, "executable": False}],
             "dependencies": ["@@semio-lane@@/pkg/src/lib.rs", "@@semio-lane@@/pkg/src/util.rs"], "environment": [{"name": "FOO", "value": "bar"}], "stdout": True, "stderr": True}
restored = {"files": {"libx.rlib": {"text": "RLIB built in {laneA}\n", "executable": True}, "libx.rmeta": {"text": "META\n", "executable": False},
                      "x.d": {"text": "{outB}/libx.rlib: {laneB}/pkg/src/lib.rs {laneB}/pkg/src/util.rs\n\n{laneB}/pkg/src/lib.rs:\n{laneB}/pkg/src/util.rs:\n# env-dep:FOO=bar\n", "executable": False}},
            "stdout": "compiling {laneB}/pkg/src/lib.rs into {outB}\n", "stderr": "warning at {laneB}/pkg/src/util.rs\n"}
scenarios = [
    {"name": "a compilation published in one lane is restored in another with its paths rewritten", "tree": TREE, "steps": [
        publish(K1, expect=published),
        lookup(K1, "hit"),
        {"op": "restore", "lane": "B", "key": K1, "env": {"FOO": "bar"}, "expect": restored},
        {"op": "audit", "expect": {"objects": 5, "entries": 1, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 1}},
    ]},
    {"name": "the lane that published also hits and a different key misses", "tree": TREE, "steps": [
        publish(K1, expect=published), lookup(K1, "hit", lane="A"), lookup(K2, "miss", lane="A"), lookup(K3, "miss", lane="B"),
    ]},
    {"name": "a changed dependency misses in its lane only and the original content hits again", "tree": TREE, "steps": [
        publish(K1, expect=published),
        {"op": "mutate", "lane": "B", "file": "pkg/src/util.rs", "content": "fn u() { changed }\n"},
        lookup(K1, "miss", lane="B"), lookup(K1, "hit", lane="A"),
        {"op": "mutate", "lane": "B", "file": "pkg/src/util.rs", "content": "fn u() {}\n"},
        lookup(K1, "hit", lane="B"),
    ]},
    {"name": "a missing dependency file is a miss", "tree": TREE, "steps": [
        publish(K1, expect=published), {"op": "remove", "lane": "B", "file": "pkg/src/lib.rs"}, lookup(K1, "miss", lane="B"), lookup(K1, "hit", lane="A"),
    ]},
    {"name": "an observed environment variable decides the hit", "tree": TREE, "steps": [
        publish(K1, expect=published),
        lookup(K1, "hit", env={"FOO": "bar"}), lookup(K1, "miss", env={"FOO": "baz"}), lookup(K1, "miss", env={}), lookup(K1, "miss", env={"FOO": ""}), lookup(K1, "hit", env={"FOO": "bar", "OTHER": "x"}),
    ]},
    {"name": "an environment variable observed as unset must stay unset", "tree": TREE, "steps": [
        publish(K1, outputs={"libx.rlib": "R\n", "x.d": "{out}/libx.rlib: {lane}/pkg/src/lib.rs\n# env-dep:GONE\n# env-dep:EMPTY=\n"}, executable=(), stdout="", stderr="", expect={"published": True, "crate": "x", "outputs": [{"name": "libx.rlib", "template": False, "executable": False}, {"name": "x.d", "template": True, "executable": False}], "dependencies": ["@@semio-lane@@/pkg/src/lib.rs"], "environment": [{"name": "GONE", "value": None}, {"name": "EMPTY", "value": ""}], "stdout": False, "stderr": False}),
        lookup(K1, "hit", lane="A", env={"EMPTY": ""}), lookup(K1, "miss", lane="A", env={"EMPTY": "", "GONE": ""}), lookup(K1, "miss", lane="A", env={}),
    ]},
    {"name": "an environment value that contains the lane is stored lane neutral", "tree": TREE, "steps": [
        publish(K1, outputs={"libx.rlib": "R\n", "x.d": "{out}/libx.rlib: {lane}/pkg/src/lib.rs\n# env-dep:ROOT={lane}/pkg\n"}, executable=(), stdout="", stderr="", expect={"published": True, "crate": "x", "outputs": [{"name": "libx.rlib", "template": False, "executable": False}, {"name": "x.d", "template": True, "executable": False}], "dependencies": ["@@semio-lane@@/pkg/src/lib.rs"], "environment": [{"name": "ROOT", "value": "@@semio-lane@@/pkg"}], "stdout": False, "stderr": False}),
        lookup(K1, "hit", lane="B", env={"ROOT": "{laneB}/pkg"}), lookup(K1, "miss", lane="B", env={"ROOT": "{laneA}/pkg"}),
    ]},
    {"name": "a removed object turns the entry into a miss and pruning drops the entry but not the other objects", "tree": TREE, "steps": [
        publish(K1, expect=published),
        {"op": "removeObject", "output": "libx.rmeta", "expect": 1},
        lookup(K1, "miss", lane="A"),
        {"op": "prune", "expect": {"entries": 1, "temporary": 0}},
        {"op": "audit", "expect": {"objects": 4, "entries": 0, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 0}},
    ]},
    {"name": "only the newest eight variants of a key are kept and the newest matching one is used", "tree": TREE, "steps": (
        [item for i in range(10) for item in ({"op": "mutate", "lane": "A", "file": "pkg/src/util.rs", "content": "fn v%d() {}\n" % i}, {"op": "sleep", "milliseconds": 3}, publish(K1, outputs={"libx.rlib": "R%d\n" % i, "x.d": "{out}/libx.rlib: {lane}/pkg/src/util.rs\n"}, executable=(), stdout="", stderr=""))]
        + [lookup(K1, "miss" if i < 2 else "hit", lane="A", env={}, content="fn v%d() {}\n" % i, file="pkg/src/util.rs") for i in range(10)]
        + [{"op": "audit", "expect": {"objects": 11, "entries": 8, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 8}}]
    )},
    {"name": "a compilation without a dependency file or with a missing one is not published", "tree": TREE, "steps": [
        publish(K1, outputs={"libx.rlib": "R\n"}, executable=(), stdout="", stderr="", expect={"published": False}),
        publish(K1, outputs={"libx.rlib": "R\n", "x.d": "{out}/libx.rlib: {lane}/pkg/src/lib.rs\n"}, executable=(), stdout="", stderr="", skipWrite=["x.d"], expect={"published": False}),
        {"op": "audit", "expect": {"objects": 0, "entries": 0, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 0}},
    ]},
    {"name": "two keys keep their own entries", "tree": TREE, "steps": [
        publish(K1, expect=published), publish(K2, expect=published),
        {"op": "audit", "expect": {"objects": 5, "entries": 2, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 1}},
        {"op": "removeObject", "output": "libx.rmeta", "expect": 1},
        lookup(K1, "miss", lane="A"), lookup(K2, "miss", lane="A"),
    ]},
    {"name": "pruning removes broken entries, empty entry directories and stale temporary files", "tree": TREE, "steps": [
        publish(K1, expect=published),
        {"op": "plant", "path": "e/zz/%s/000000000000001-bad.json" % ("z" * 64), "content": "not json"},
        {"op": "plant", "path": "e/yy/%s/000000000000002-empty.json" % ("y" * 64), "content": "{\"outputs\":[{\"sha256\":\"" + "0" * 64 + "\"}]}"},
        {"op": "plant", "path": "w/stale", "content": "x", "ageMilliseconds": 7200000},
        {"op": "plant", "path": "w/fresh", "content": "x", "ageMilliseconds": 1000},
        {"op": "prune", "expect": {"entries": 2, "temporary": 1}},
        {"op": "exists", "path": "e/zz", "expect": True}, {"op": "exists", "path": "w/stale", "expect": False}, {"op": "exists", "path": "w/fresh", "expect": True},
        {"op": "exists", "path": "e/zz/%s" % ("z" * 64), "expect": False},
        lookup(K1, "hit", lane="A"),
    ]},
    {"name": "objects are listed with size and use and removed by digest", "tree": TREE, "steps": [
        publish(K1, expect=published),
        {"op": "objects", "expect": {"count": 5}},
        {"op": "removeDigests", "digests": ["nothex", "../escape", "A" * 64, "0" * 64, "{object:libx.rmeta}"], "expect": 1},
        {"op": "objects", "expect": {"count": 4}},
        {"op": "removeDigests", "digests": ["{object:libx.rmeta}"], "expect": 0},
    ]},
]
tree_cases = [
    {"name": "files sorted by name inside each directory, depth first", "files": {"b": "2", "a/z": "3", "a/y": "4", "a.txt": "5", "C": "6"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 100}},
    {"name": "an empty tree digests like no rows", "files": {}, "directories": ["d"], "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 100}},
    {"name": "ignored directories and extensions are skipped at any depth", "files": {"src/a.rs": "1", "src/b.txt": "2", "target/x": "3", "src/target/y": "4", "node_modules/m": "5", "k.rs": "6"}, "options": {"ignoredDirectories": ["target", "node_modules"], "ignoredExtensions": [".rs"], "maximumFiles": 100}},
    {"name": "exactly the maximum number of files is digested", "files": {"a": "1", "b": "2", "c": "3"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 3}},
    {"name": "one file beyond the maximum has no digest", "files": {"a": "1", "b": "2", "c": "3", "d": "4"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 3}, "overflow": True},
    {"name": "ignored files do not count against the maximum", "files": {"a": "1", "x.rs": "2", "y.rs": "3", "b": "4"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [".rs"], "maximumFiles": 2}},
    {"name": "symbolic links are digested by their target text", "files": {"a": "1"}, "links": {"l": "a", "dir/l2": "../elsewhere"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 100}},
    {"name": "unicode names sort by code unit", "files": {"é": "1", "z": "2", "日": "3", "a": "4"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 100}},
    {"name": "content decides, not the time", "files": {"a": "1"}, "options": {"ignoredDirectories": [], "ignoredExtensions": [], "maximumFiles": 100}, "touch": True},
]

RTREE = {"pkg/build.rs": "fn main() {}\n", "pkg/data/in.txt": "input\n", "pkg/data/more.txt": "more\n", "pkg/src/lib.rs": "fn a() {}\n", "pkg/target/skip.bin": "x", "pkg/node_modules/m.js": "m"}


def run_publish(key, directives, env, out, lane="A", **kw):
    return {"op": "runPublish", "lane": lane, "key": key, "directives": directives, "env": env, "out": out, "stdout": "cargo:rerun-if-changed=data/in.txt in {lane}/pkg\n", "stderr": "", **kw}


def run_lookup(key, expect, lane="B", directivesEnv=None, env=None, **kw):
    return {"op": "runLookup", "lane": lane, "key": key, "env": env if env is not None else {"FOO": "1"}, "expect": expect, **kw}


OUT = {"gen.rs": {"text": "const P: &str = \"{lane}/pkg\";\n"}, "blob.bin": {"hex": "00ff10"}, "sub/x.txt": {"text": "plain\n"}, "tool.sh": {"text": "#!/bin/sh\n", "executable": True}}
DIR1 = {"files": ["data/in.txt"], "variables": ["FOO"]}
run_published = {"published": True, "facts": [{"path": "data/in.txt", "kind": "file"}, {"path": ".", "kind": "package"}], "environment": [{"name": "FOO", "value": "1"}],
                 "outputs": [{"path": "blob.bin", "template": False, "executable": False}, {"path": "gen.rs", "template": True, "executable": False}, {"path": "sub/x.txt", "template": False, "executable": False}, {"path": "tool.sh", "template": False, "executable": True}]}
run_restored = {"files": {"gen.rs": {"text": "const P: &str = \"{laneB}/pkg\";\n"}, "blob.bin": {"hex": "00ff10"}, "sub/x.txt": {"text": "plain\n"}, "tool.sh": {"text": "#!/bin/sh\n", "executable": True}}, "stdout": "cargo:rerun-if-changed=data/in.txt in {laneB}/pkg\n", "stderr": ""}
R1, R2, R3 = "3" * 64, "4" * 64, "5" * 64
runs = [
    {"name": "a build script run published in one lane is restored in another with its text rewritten and its binary untouched", "tree": RTREE, "steps": [
        run_publish(R1, DIR1, {"FOO": "1"}, OUT, expect=run_published),
        run_lookup(R1, "hit"), {"op": "runRestore", "lane": "B", "key": R1, "env": {"FOO": "1"}, "expect": run_restored},
        {"op": "audit", "expect": {"objects": 5, "entries": 0, "runs": 1, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 1}},
    ]},
    {"name": "a changed declared file or variable is a miss, a changed Rust source is not when inputs were declared and any changed package file is", "tree": RTREE, "steps": [
        run_publish(R1, DIR1, {"FOO": "1"}, OUT, expect=run_published),
        {"op": "mutate", "lane": "B", "file": "pkg/data/in.txt", "content": "other\n"}, run_lookup(R1, "miss"),
        {"op": "mutate", "lane": "B", "file": "pkg/data/in.txt", "content": "input\n"}, run_lookup(R1, "hit"),
        run_lookup(R1, "miss", env={"FOO": "2"}), run_lookup(R1, "miss", env={}), run_lookup(R1, "hit", env={"FOO": "1", "UNRELATED": "x"}),
        {"op": "mutate", "lane": "B", "file": "pkg/src/lib.rs", "content": "fn changed() {}\n"}, run_lookup(R1, "hit"),
        {"op": "mutate", "lane": "B", "file": "pkg/target/skip.bin", "content": "changed"}, run_lookup(R1, "hit"),
        {"op": "mutate", "lane": "B", "file": "pkg/node_modules/m.js", "content": "changed"}, run_lookup(R1, "hit"),
        {"op": "mutate", "lane": "B", "file": "pkg/data/more.txt", "content": "changed\n"}, run_lookup(R1, "miss"),
    ]},
    {"name": "without declared inputs every package file counts, Rust sources included", "tree": RTREE, "steps": [
        run_publish(R2, {"files": [], "variables": []}, {}, {"out.txt": {"text": "x\n"}}, expect={"published": True, "facts": [{"path": ".", "kind": "package"}], "environment": [], "outputs": [{"path": "out.txt", "template": False, "executable": False}]}),
        {"op": "mutate", "lane": "B", "file": "pkg/src/lib.rs", "content": "fn changed() {}\n"}, run_lookup(R2, "miss", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/src/lib.rs", "content": "fn a() {}\n"}, run_lookup(R2, "hit", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/build.rs", "content": "fn main() { changed }\n"}, run_lookup(R2, "miss", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/build.rs", "content": "fn main() {}\n"}, run_lookup(R2, "hit", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/data/in.txt", "content": "changed\n"}, run_lookup(R2, "miss", env={}),
    ]},
    {"name": "a declared directory is a tree fact", "tree": RTREE, "steps": [
        run_publish(R3, {"files": ["data"], "variables": []}, {}, {"o": {"text": "1"}}, expect={"published": True, "facts": [{"path": "data", "kind": "tree"}, {"path": ".", "kind": "package"}], "environment": [], "outputs": [{"path": "o", "template": False, "executable": False}]}),
        run_lookup(R3, "hit", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/data/more.txt", "content": "changed\n"}, run_lookup(R3, "miss", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/data/more.txt", "content": "more\n"}, run_lookup(R3, "hit", env={}),
        {"op": "mutate", "lane": "B", "file": "pkg/data/new.txt", "content": "new\n"}, run_lookup(R3, "miss", env={}),
    ]},
    {"name": "a run that cannot be replayed is not published", "tree": RTREE, "steps": [
        run_publish(R1, {"files": ["data/missing.txt"], "variables": []}, {}, {"o": {"text": "1"}}, expect={"published": False}),
        run_publish(R1, {"files": [], "variables": []}, {}, {"bin": {"hex": "00", "embeds": "{lane}"}}, expect={"published": False}),
        run_publish(R1, {"files": [], "variables": []}, {}, {"o": {"text": "1"}, "link": {"symlink": "o"}}, expect={"published": False}, onlyPosix=True),
        {"op": "audit", "expect": {"objects": 0, "entries": 0, "runs": 0, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 0}},
    ]},
    {"name": "a binary output that does not contain the lane is kept as it is and a text output that does is a template", "tree": RTREE, "steps": [
        run_publish(R1, {"files": [], "variables": []}, {}, {"b.bin": {"hex": "00010203"}, "t.txt": {"text": "plain\n"}, "u.txt": {"text": "in {lane}/pkg\n"}}, expect={"published": True, "facts": [{"path": ".", "kind": "package"}], "environment": [], "outputs": [{"path": "b.bin", "template": False, "executable": False}, {"path": "t.txt", "template": False, "executable": False}, {"path": "u.txt", "template": True, "executable": False}]}),
        {"op": "runRestore", "lane": "B", "key": R1, "env": {}, "expect": {"files": {"b.bin": {"hex": "00010203"}, "t.txt": {"text": "plain\n"}, "u.txt": {"text": "in {laneB}/pkg\n"}}, "stdout": "cargo:rerun-if-changed=data/in.txt in {laneB}/pkg\n", "stderr": ""}},
        {"op": "audit", "expect": {"objects": 4, "entries": 0, "runs": 1, "dangling": [], "leaks": [], "invalid": [], "maxVariants": 1}},
    ]},
]
json.dump({"version": 1, "lanes": LANES, "scenarios": scenarios, "runs": runs, "tree": tree_cases}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(scenarios), len(tree_cases))
