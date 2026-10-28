"""✂️ Builds (and rebuilds after a landing) the staged batch `scopedcut` from anchored edits on the CURRENT bytes of a root: `python3 -I build_scopedcut.py <root>`.

Writes `staged/scopedcut/{base,files}/<n>`, `manifest.json` ({rel:{file,base,new}}) and copies the mode-keeping `stage.py` from `C/xplat5-tools/stage.template.py`; every anchor must occur exactly once or the build stops naming it."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
OUT = os.path.join(HERE, "staged", "scopedcut")
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/"
K = G + "✂️cutoff/"

EDITS = {
    K + "🟨️.mjs": [(
        "/** ❓️ Asks every suspect question again:",
        "/** 🎯️ The suspects a verification scoped to the project names `names` asks again: the pseudo owners whose derived fact one of those projects holds. A question that only projects outside the scope hold cannot stale any of them; its stale support stays recorded, so the verification that selects one of its holders asks it then. */\n"
        "export function suspectsWithinV1(index,suspects,names){\n"
        " const view=viewOf(index),out=new Set();\n"
        " for(const name of suspects){const id=view.pseudoRow.get(name);if(id!==undefined&&[...(view.holders.get(id)??[])].some(holder=>names.has(holder)))out.add(name);}\n"
        " return out;\n"
        "}\n\n"
        "/** ❓️ Asks every suspect question again:",
    )],
    K + "🟨️.d.mts": [(
        "/** ❓️ Asks every suspect question again",
        "/** 🎯️ The suspects a verification scoped to `names` asks again: the pseudo owners whose derived fact one of those projects holds; the others stay stale for the verification that selects their holders. */\n"
        "export function suspectsWithinV1(index:FactIndexV1,suspects:Iterable<string>,names:ReadonlySet<string>):Set<string>;\n"
        "/** ❓️ Asks every suspect question again",
    )],
    G + "🟨️.mjs": [
        ("splitStaleConsumersV1,unsupportedDerivedV1,verifiableRowIdsV1}from\"./✂️cutoff/🟨️.mjs\";", "splitStaleConsumersV1,suspectsWithinV1,unsupportedDerivedV1,verifiableRowIdsV1}from\"./✂️cutoff/🟨️.mjs\";"),
        ("  const selection=selectFactClassesV1(index,only?scopeWithDerivedV1(index,new Set(skipEnvironment?only:[...only,P.envOwner])):undefined);",
         "  const scope=only?new Set(skipEnvironment?only:[...only,P.envOwner]):undefined,selection=selectFactClassesV1(index,scope?scopeWithDerivedV1(index,scope):undefined);"),
        ("const{direct,suspects}=splitStaleConsumersV1(index,selection.classes,stale),settled=await settleSuspectsV1({index,suspects,derive,signal,progress})",
         "const{direct,suspects:reached}=splitStaleConsumersV1(index,selection.classes,stale),suspects=scope?suspectsWithinV1(index,reached,scope):reached,settled=await settleSuspectsV1({index,suspects,derive,signal,progress})"),
    ],
    K + "🧬️schema/🔣️.json": [(
        '       "stale": {"type": "array", "items": {"type": "string"}, "description": "raw rows that no longer hold"},\n',
        '       "stale": {"type": "array", "items": {"type": "string"}, "description": "raw rows that no longer hold"},\n'
        '       "scope": {"type": "array", "items": {"type": "string"}, "description": "when present, the verification is scoped to these projects: only they can be dirty and only the questions one of them holds are asked; the others stay stale"},\n',
    )],
    K + "🧪️tests/🐍️oracle/🐍️.py": [
        ("A question whose support is stale but whose\nanswer is equal is held: its holders stay valid. Standard library only.",
         "A question whose support is stale but whose\nanswer is equal is held: its holders stay valid. With a `scope` (project names) only projects of the scope can be\ndirty and only questions a project of the scope holds are asked; the others stay stale. Standard library only."),
        ("    stale = set(world[\"stale\"])\n    direct = {name for name, row in world[\"owners\"].items() if stale.intersection(row[\"raw\"])}",
         "    stale = set(world[\"stale\"])\n    scope = set(world[\"scope\"]) if \"scope\" in world else set(world[\"owners\"])\n    direct = {name for name, row in world[\"owners\"].items() if name in scope and stale.intersection(row[\"raw\"])}"),
        ("        if not any(name for name, owner in world[\"owners\"].items() if question in owner[\"derived\"]):\n            continue",
         "        if not any(name in scope for name, owner in world[\"owners\"].items() if question in owner[\"derived\"]):\n            continue"),
        ("        if changed.intersection(row[\"derived\"]):\n            dirty.add(name)",
         "        if name in scope and changed.intersection(row[\"derived\"]):\n            dirty.add(name)"),
    ],
    K + "🧪️tests/🟦️.ts": [
        ("readonly expect:{dirty:string[];held:string[];changed:string[]}};", "readonly expect:{dirty:string[];held:string[];changed:string[]};readonly scope?:readonly string[]};"),
        ("scopeWithDerivedV1,settleSuspectsV1,splitStaleConsumersV1,unsupportedDerivedV1", "scopeWithDerivedV1,settleSuspectsV1,splitStaleConsumersV1,suspectsWithinV1,unsupportedDerivedV1"),
        (" const{index,questionOf}=indexOf(world),selection=selectFactClassesV1(index),stale=new Set(world.stale);\n const checked=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>!stale.has(row.path)));\n const{direct,suspects}=splitStaleConsumersV1(index,selection.classes,checked.stale);",
         " const{index,questionOf}=indexOf(world),scope=world.scope?new Set(world.scope):undefined,selection=selectFactClassesV1(index,scope?scopeWithDerivedV1(index,scope):undefined),stale=new Set(world.stale),inScope=(name:string)=>!scope||scope.has(name);\n const checked=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>!stale.has(row.path)));\n const{direct,suspects:reached}=splitStaleConsumersV1(index,selection.classes,checked.stale),suspects=scope?suspectsWithinV1(index,reached,scope):reached;"),
        (" const holders=holdersOfDerivedV1(index,settled.changed.keys()),dirty=new Set([...direct].filter(name=>!isDerivedOwnerV1(name)));\n for(const set of holders.values())for(const name of set)dirty.add(name);\n return{index,settled,",
         " const holders=holdersOfDerivedV1(index,settled.changed.keys()),dirty=new Set([...direct].filter(name=>!isDerivedOwnerV1(name)&&inScope(name)));\n for(const set of holders.values())for(const name of set)if(inScope(name))dirty.add(name);\n return{index,settled,suspects,"),
        ("test(\"a derived row is never checked as a raw fact",
         "test(\"a scoped verification asks only the questions a project of its scope holds, for every world under a seeded half scope, equal to the Python model\",async()=>{\n"
         " let deferred=0;\n"
         " for(const world of worlds){\n"
         "  const names=Object.keys(world.owners).sort(),scope=names.filter((_name,at)=>at%2===0),scoped={...world,scope},outcome=await decide(scoped),{questionOf}=indexOf(world);\n"
         "  expect({dirty:outcome.dirty,held:outcome.held,changed:outcome.changed},world.id).toEqual(model(scoped));\n"
         "  for(const name of outcome.suspects)expect(Object.entries(world.owners).some(([owner,row])=>scope.includes(owner)&&row.derived.includes(questionOf.get(name)!)),world.id).toBe(true);\n"
         "  deferred+=(await decide(world)).suspects.size-outcome.suspects.size;\n"
         " }\n"
         " expect(deferred,\"questions only projects outside the scope hold\").toBeGreaterThan(0);\n"
         "},300000);\n\n"
         "test(\"a derived row is never checked as a raw fact"),
    ],
    K + "🧪️tests/🏪️store/🟦️.ts": [(
        "test(\"a derived fact whose computation fails stales exactly its holders",
        "test(\"a verification scoped to some projects asks only the derived questions those projects hold; the others stay stale for the verification that selects their holders\",async()=>{\n"
        " const root=realpathSync(mkdtempSync(join(tmpdir(),\"cutoff-scope-\"))),storeRoot=join(root,\"store\");roots.push(root);write(root,fixture.files);\n"
        " const counters={computed:[] as string[],derived:0},parts=host(root,counters),store=createGenerationStoreV1({root:storeRoot,holds:parts.holds,derive:parts.derive,resources:{facts:\"facts\"}});\n"
        " await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:\"fresh\",membership:parts.membership,compute:parts.compute});\n"
        " writeFileSync(join(root,\"lib/c.ts\"),\"// shared by the closures of p1, p2 and p4\\n\");counters.derived=0;\n"
        " const scoped=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set([\"p2\"])});\n"
        " expect(counters.derived,\"only the question p2 holds\").toBe(1);expect(scoped.cutoff?.questions).toBe(1);expect(scoped.cutoff?.held).toBe(1);expect([...scoped.dirty]).toEqual([]);\n"
        " counters.derived=0;const whole=await store.plan({current:await store.read(),identity:IDENTITY,changes:[]});\n"
        " expect(counters.derived,\"the deferred question is still stale for a whole verification\").toBe(2);expect(whole.cutoff?.questions).toBe(2);\n"
        " writeFileSync(join(root,\"lib/c.ts\"),\"import ./d\\n\");counters.derived=0;\n"
        " const changed=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set([\"p2\"])});\n"
        " expect(counters.derived).toBe(1);expect([...changed.dirty]).toEqual([\"p2\"]);\n"
        " counters.derived=0;const holders=await store.plan({current:await store.read(),identity:IDENTITY,changes:[],only:new Set([\"p4\"])});\n"
        " expect(counters.derived).toBe(1);expect([...holders.dirty]).toEqual([\"p4\"]);\n"
        " counters.computed.length=0;await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:\"fresh\",only:new Set([\"p2\"]),membership:parts.membership,compute:parts.compute});\n"
        " expect([...new Set(counters.computed)].sort()).toEqual([\"p2\"]);\n"
        " counters.computed.length=0;await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,policy:\"fresh\",membership:parts.membership,compute:parts.compute});\n"
        " expect([...new Set(counters.computed)].sort(),\"the deferred holders are refreshed by the next whole verification\").toEqual([\"p1\",\"p4\"]);\n"
        " writeFileSync(join(root,\"lib/c.ts\"),fixture.files[\"lib/c.ts\"]);\n"
        "},60000);\n\n"
        "test(\"a derived fact whose computation fails stales exactly its holders",
    )],
}
WORLDS = [
    {"id": "scoped: a question only projects outside the scope hold is not asked", "owners": {"a": {"raw": [], "derived": ["q1"]}, "b": {"raw": [], "derived": ["q2"]}}, "derived": {"q1": {"answer": "d1", "support": ["s1"]}, "q2": {"answer": "e1", "support": ["s1"]}}, "stale": ["s1"], "answers": {"q1": "d1", "q2": "e2"}, "scope": ["a"], "expect": {"dirty": [], "held": ["q1"], "changed": []}},
    {"id": "scoped: a question held inside and outside the scope is asked and stales only the scoped holder", "owners": {"a": {"raw": [], "derived": ["q1"]}, "b": {"raw": ["s1"], "derived": ["q1"]}}, "derived": {"q1": {"answer": "d1", "support": ["s1"]}}, "stale": ["s1"], "answers": {"q1": "d2"}, "scope": ["a"], "expect": {"dirty": ["a"], "held": [], "changed": ["q1"]}},
]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main(root):
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(os.path.join(OUT, "base"))
    os.makedirs(os.path.join(OUT, "files"))
    manifest, problems = {}, []
    targets = dict(EDITS)
    targets[K + "🧪️tests/🔣️.json"] = "fixture"
    for number, (rel, edits) in enumerate(sorted(targets.items())):
        original = open(os.path.join(root, rel), "rb").read()
        text = original.decode("utf8")
        if edits == "fixture":
            data = json.loads(text)
            if any(world["id"].startswith("scoped:") for world in data["worlds"]):
                problems.append(rel + ": scoped worlds already present")
            data["worlds"][7:7] = WORLDS
            indent = None if not text.lstrip().startswith("{\n") else 1
            text = json.dumps(data, ensure_ascii=False, indent=indent, separators=(",", ":") if indent is None else None) + ("\n" if text.endswith("\n") else "")
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
    template = template.replace('"""🌌 Staged batch `universe` (lane rustedit): print\'s Bazel plan becomes reusable although a dependency reads a root-anchored pattern.', '"""✂️ Staged batch `scopedcut` (lane gatefast): a scoped graph verification asks only the derived questions its projects hold.')
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(template)
    shutil.copy(os.path.join(HERE, "verify_scopedcut.sh"), os.path.join(OUT, "verify.sh"))
    print(f"built {len(manifest)} files into {OUT}")


main(os.path.abspath(sys.argv[1]))
