#!/usr/bin/env python3
"""Generates the language-agnostic fixture of the generation laws from the independent model in oracle.py: scenario worlds, their edit steps and the expectations (graph, refreshed owners, publish decisions, closures).

usage: gen_fixture.py [--out DIR]   (default: staged/new/gen/)"""
import importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "staged", "new", "gen")
spec = importlib.util.spec_from_file_location("oracle", os.path.join(GEN, "oracle.py"))
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
out_dir = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else GEN


def project(name, package=None, uses=()):
    return json.dumps({"name": name, **({"package": package} if package else {}), "uses": list(uses)}, sort_keys=True)


def source(*imports, body=""):
    return "".join('import "%s";\n' % spec for spec in imports) + "export const v=%s;\n" % (body or "0")


def ring(count):
    files = {"shared/common.json": '{"v":1}', "shared/unused.json": '{"v":2}'}
    for i in range(count):
        name, nxt, after = "p%d" % i, "p%d" % ((i + 1) % count), "p%d" % ((i + 2) % count)
        files["pkg/%s/project.json" % name] = project(name, "@syn/%s" % name, ["shared/common.json"] if i % 3 == 0 else [])
        files["pkg/%s/src/index.ts" % name] = source("@syn/%s" % nxt, "../../%s/src/index" % after, body=str(i))
    return files


def write(path, content):
    return {"op": "write", "path": path, "content": content}


def step(label, *ops):
    return {"label": label, "ops": list(ops)}


scenarios = []
base = ring(6)
scenarios.append({"name": "ring-edits-env-and-structure", "initial": base, "steps": [
    step("cold"),
    step("warm no-op"),
    step("touch without change", write("pkg/p2/src/index.ts", base["pkg/p2/src/index.ts"])),
    step("edit body of p2", write("pkg/p2/src/index.ts", source("@syn/p3", "../../p4/src/index", body="42"))),
    step("p2 gains an import of p5", write("pkg/p2/src/index.ts", source("@syn/p3", "../../p4/src/index", "@syn/p5", body="42"))),
    step("shared file of p0 and p3", write("shared/common.json", '{"v":99}')),
    step("unrelated shared file", write("shared/unused.json", '{"v":3}')),
    step("new source in p1", write("pkg/p1/src/extra.ts", source("@syn/p0"))),
    step("delete that source", {"op": "delete", "path": "pkg/p1/src/extra.ts"}),
    step("p3 uses another shared file", write("pkg/p3/project.json", project("p3", "@syn/p3", ["shared/common.json", "shared/unused.json"]))),
    step("p4 package renamed", write("pkg/p4/project.json", project("p4", "@syn/p4b"))),
    step("new owner p6", write("pkg/p6/project.json", project("p6", "@syn/p6")), write("pkg/p6/src/index.ts", source("@syn/p0"))),
    step("owner p5 removed", {"op": "delete", "path": "pkg/p5"}),
    step("owner p6 moved", {"op": "rename", "from": "pkg/p6", "to": "pkg/p6x"}),
    step("two owners edited at once", write("pkg/p0/src/index.ts", source("@syn/p1", body="7")), write("pkg/p3/src/index.ts", source("@syn/p4b", body="8"))),
    step("final no-op"),
]})
nested = {"shared/common.json": '{"v":1}',
          "pkg/a/project.json": project("a", "@syn/a"), "pkg/a/src/index.ts": source("../../b/src/index"),
          "pkg/b/project.json": project("b", "@syn/b"), "pkg/b/src/index.ts": source("@syn/a"),
          "pkg/c/project.json": project("c", "@syn/c"), "pkg/c/src/index.ts": source("@syn/b")}
scenarios.append({"name": "nested-roots-and-renames", "initial": nested, "steps": [
    step("cold"),
    step("nested owner inside a", write("pkg/a/inner/project.json", project("inner", "@syn/inner")), write("pkg/a/inner/src/index.ts", source("@syn/c"))),
    step("a now imports the nested owner by path", write("pkg/a/src/index.ts", source("../../b/src/index", "../inner/src/index"))),
    step("project of c renamed in its config", write("pkg/c/project.json", project("c2", "@syn/c"))),
    step("inner removed", {"op": "delete", "path": "pkg/a/inner"}),
    step("config of b with a missing shared file", write("pkg/b/project.json", project("b", "@syn/b", ["shared/missing.json"]))),
    step("the missing shared file appears", write("shared/missing.json", "{}")),
    step("warm no-op"),
]})

for scenario in scenarios:
    for row, expected in zip(scenario["steps"], oracle.simulate(scenario["initial"], scenario["steps"])):
        row["expect"] = expected
    files = dict(scenario["initial"])
    for row in scenario["steps"]:
        oracle.apply_ops(files, row["ops"])
    state = oracle.model(files)
    scenario["closures"] = [{"roots": [name], "names": oracle.closure(files, [name]), "graph": oracle.graph(files, oracle.closure(files, [name]))} for name in sorted(state["nodes"])[:3]]

fixture = {"version": 1, "scenarios": scenarios}
with open(os.path.join(out_dir, "fixture.json"), "w", encoding="utf8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1, sort_keys=True)
    handle.write("\n")
print("scenarios:", len(scenarios), "steps:", sum(len(s["steps"]) for s in scenarios))
