"""🪆 Builds (and rebuilds after a landing) the staged batch `orchslice` from anchored edits on the CURRENT bytes of a root: `python3 -I build_orchslice.py <root>`.

A selected slice also admits the declared orchestration children of its active targets (`metadata.semio.execution` kind `orchestration`), so the generation an orchestrating build pins covers the closure of the nested runs it launches. Writes `staged/orchslice/{base,files}/<n>`, `manifest.json`, the mode-keeping `stage.py` and `verify.sh`; every anchor must occur exactly once."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
OUT = os.path.join(HERE, "staged", "orchslice")
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/"

EDITS = {
    D + "🟨️.mjs": [
        ("/** 🪶️ The part of a fragment that {@link sliceClosureV1} reads: the targets of its edges, per target the input references and `projects` rows of its inputs and its `dependsOn`, and the same of its named input groups.",
         "/** 🪆️ The tasks an orchestrating target launches as nested runs (`metadata.semio.execution` of kind `orchestration`): the nested runs read the generation the build pins, so a slice needs their closure too. */\n"
         "const orchestrationChildrenV1=operation=>{const execution=operation?.metadata?.semio?.execution;return execution?.kind===\"orchestration\"&&Array.isArray(execution.children)?execution.children.map(child=>({project:child.project,target:child.target})):[];};\n\n"
         "/** 🪶️ The part of a fragment that {@link sliceClosureV1} reads: the targets of its edges, per target the input references and `projects` rows of its inputs, its `dependsOn` and its declared orchestration children, and the same of its named input groups."),
        ("for(const[name,operation]of Object.entries(fragment.node?.targets??{}))targets[name]={inputs:reads(operation?.inputs),dependsOn:operation?.dependsOn};",
         "for(const[name,operation]of Object.entries(fragment.node?.targets??{})){const children=orchestrationChildrenV1(operation);targets[name]={inputs:reads(operation?.inputs),dependsOn:operation?.dependsOn,...(children.length?{metadata:{semio:{execution:{kind:\"orchestration\",children}}}}:{})};}"),
        ("and their `dependsOn` (`^target` over the dependency edges, `project:target`, `target`, objects with `projects` and `dependencies`, `*` for the active target only).",
         "their `dependsOn` (`^target` over the dependency edges, `project:target`, `target`, objects with `projects` and `dependencies`, `*` for the active target only) and their declared orchestration children (a nested run is served from the generation the outer build pins, so its tasks belong to the slice)."),
        ("    for(const value of projects)await task(value,dependency.target,dependency.dependencies===true);\n   }\n  }\n",
         "    for(const value of projects)await task(value,dependency.target,dependency.dependencies===true);\n   }\n  }\n  for(const child of orchestrationChildrenV1(operation))await task(child.project,child.target,false);\n"),
    ],
    D + "🟨️.d.mts": [
        ("/** 🪶️ The part of a fragment the slice closure reads (target edges; per target the projects of its inputs and its `dependsOn`). */\nexport function sliceViewV1(fragment:{edges:readonly {target:string}[];node?:{targets?:Record<string,{inputs?:readonly unknown[];dependsOn?:unknown}>;",
         "/** 🪶️ The part of a fragment the slice closure reads (target edges; per target the projects of its inputs, its `dependsOn` and its declared orchestration children). */\nexport function sliceViewV1(fragment:{edges:readonly {target:string}[];node?:{targets?:Record<string,{inputs?:readonly unknown[];dependsOn?:unknown;metadata?:unknown}>;"),
        ("dependencies?:boolean})[];dependsOn:unknown}>;namedInputs?:",
         "dependencies?:boolean})[];dependsOn:unknown;metadata?:{semio:{execution:{kind:\"orchestration\";children:{project:string;target:string}[]}}}}>;namedInputs?:"),
    ],
    D + "🧪️tests/🐍️.py": [
        ("                for value in projects:\n                    task(value, dep[\"target\"], dep.get(\"dependencies\") is True)\n",
         "                for value in projects:\n                    task(value, dep[\"target\"], dep.get(\"dependencies\") is True)\n"
         "        execution = ((operation.get(\"metadata\") or {}).get(\"semio\") or {}).get(\"execution\") or {}\n"
         "        if execution.get(\"kind\") == \"orchestration\":\n"
         "            for child in execution.get(\"children\", []):\n"
         "                task(child[\"project\"], child[\"target\"], False)\n"),
    ],
    D + "🧪️tests/🟦️.ts": [
        ("import {GENERATION_POLICY_V1,canonicalJsonV1} from \"../../🟨️.mjs\";",
         "import {GENERATION_POLICY_V1,canonicalJsonV1} from \"../../🟨️.mjs\";\nimport {encodeGenerationPinV1,generationPinOpenV1,readGenerationPinV1} from \"../../🧷️pin/🟨️.mjs\";"),
        ("test(\"the slice scenario corpus agrees with the python oracle\",",
         "test(\"an orchestrating build pins a generation that covers the nested run it launches: the outer slice admits the declared children, the nested slice is inside it\",async()=>{\n"
         " const fragments:Record<string,{root:string;edges:string[];targets:Record<string,unknown>}>={\n"
         "  dash:{root:\"pkg/dash\",edges:[\"lib\"],targets:{build:{executor:\"repo:script\",metadata:{semio:{execution:{version:1,kind:\"orchestration\",children:[{project:\"dash\",target:\"compile\"}]},cache:{class:\"driver-owned\"}}}},compile:{inputs:[\"default\",{input:\"scriptProvider:leases:native\",projects:[\"providers\"]}]}}},\n"
         "  providers:{root:\"lib/providers\",edges:[\"process\"],targets:{}},process:{root:\"process\",edges:[\"framework\"],targets:{}},framework:{root:\"framework\",edges:[\"value\"],targets:{}},value:{root:\"value\",edges:[],targets:{}},lib:{root:\"lib\",edges:[],targets:{}},course:{root:\"course\",edges:[\"framework\"],targets:{build:{}}},\n"
         " };\n"
         " const owners=Object.fromEntries(Object.entries(fragments).map(([name,row])=>[name,row.root])),fragment=async(name:string)=>sliceViewV1({edges:fragments[name]!.edges.map(target=>({target})),node:{targets:fragments[name]!.targets as never}});\n"
         " const closure=(target:string)=>sliceClosureV1({owners,fragment,selection:{kind:\"tasks\",tasks:[{project:\"dash\",target}]}});\n"
         " const outer=await closure(\"build\"),nested=await closure(\"compile\");\n"
         " expect([...outer].sort()).toEqual([\"dash\",\"framework\",\"lib\",\"process\",\"providers\",\"value\"]);\n"
         " expect([...nested].every(name=>outer.has(name))).toBe(true);\n"
         " const names=Object.keys(owners).sort(),pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:7,manifest:\"a\".repeat(64),owners:names,covered:outer,atMs:Date.now()}));\n"
         " expect(generationPinOpenV1({pin,owners:names,needed:nested})).toEqual([]);\n"
         " expect((await fragment(\"dash\")).node.targets.build).toEqual({inputs:[],dependsOn:undefined,metadata:{semio:{execution:{kind:\"orchestration\",children:[{project:\"dash\",target:\"compile\"}]}}}});\n"
         " fragments.dash!.targets.build={metadata:{semio:{execution:{version:1,kind:\"orchestration\",children:[{project:\"ghost\",target:\"compile\"}]}}}};\n"
         " await expect(closure(\"build\")).rejects.toMatchObject({code:\"GENERATION_UNKNOWN_OWNER\"});\n"
         "});\n\n"
         "test(\"the slice scenario corpus agrees with the python oracle\","),
    ],
}
CASES = [
    {"expected": {"admitted": ["dash", "framework", "lib", "process", "providers"]}, "owners": {
        "dash": {"edges": ["lib"], "root": "pkg/dash", "targets": {"build": {"metadata": {"semio": {"execution": {"version": 1, "kind": "orchestration", "children": [{"project": "dash", "target": "compile"}]}}}}, "compile": {"inputs": [{"input": "scheduling", "projects": ["providers"]}]}}},
        "providers": {"edges": ["process"], "root": "lib/providers", "targets": {}},
        "process": {"edges": ["framework"], "root": "process", "targets": {}},
        "framework": {"edges": [], "root": "framework", "targets": {}},
        "lib": {"edges": [], "root": "lib", "targets": {}},
        "course": {"edges": ["framework"], "root": "course", "targets": {"build": {}}},
    }, "roots": [], "selection": {"kind": "tasks", "tasks": [{"project": "dash", "target": "build"}]}},
    {"expected": {"error": "unknown"}, "owners": {
        "dash": {"edges": [], "root": "pkg/dash", "targets": {"build": {"metadata": {"semio": {"execution": {"version": 1, "kind": "orchestration", "children": [{"project": "ghost", "target": "compile"}]}}}}}},
    }, "roots": [], "selection": {"kind": "tasks", "tasks": [{"project": "dash", "target": "build"}]}},
]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main(root):
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(os.path.join(OUT, "base"))
    os.makedirs(os.path.join(OUT, "files"))
    manifest, problems = {}, []
    targets = dict(EDITS)
    targets[D + "🧪️tests/🔣️.json"] = "fixture"
    for number, (rel, edits) in enumerate(sorted(targets.items())):
        original = open(os.path.join(root, rel), "rb").read()
        text = original.decode("utf8")
        if edits == "fixture":
            data = json.loads(text)
            data["closures"].extend(CASES)
            text = json.dumps(data, ensure_ascii=False, indent=2) + ("\n" if text.endswith("\n") else "")
            if json.dumps(json.loads(original), ensure_ascii=False, indent=2) + ("\n" if original.endswith(b"\n") else "") != original.decode("utf8"):
                problems.append(rel + ": fixture formatting would change beyond the added cases")
        else:
            for old, new in edits:
                if text.count(old) != 1:
                    problems.append(f"{rel}: anchor x{text.count(old)} {old[:70]!r}")
                    continue
                text = text.replace(old, new)
        name = str(number)
        open(os.path.join(OUT, "base", name), "wb").write(original)
        open(os.path.join(OUT, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(original), "new": sha(text.encode("utf8"))}
    if problems:
        sys.exit("DEAD ANCHORS:\n" + "\n".join(problems))
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    template = template.replace('"""🌌 Staged batch `universe` (lane rustedit): print\'s Bazel plan becomes reusable although a dependency reads a root-anchored pattern.', '"""🪆 Staged batch `orchslice` (lane gatefast): a selected slice admits the declared orchestration children, so a pinned generation covers its nested runs.')
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(template)
    shutil.copy(os.path.join(HERE, "verify_orchslice.sh"), os.path.join(OUT, "verify.sh"))
    print(f"built {len(manifest)} files into {OUT}")


main(os.path.abspath(sys.argv[1]))
