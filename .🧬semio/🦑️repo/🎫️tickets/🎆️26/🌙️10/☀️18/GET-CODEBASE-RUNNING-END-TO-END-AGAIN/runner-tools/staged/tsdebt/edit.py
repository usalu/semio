"""✏️ tsdebt edits: exact, single-occurrence replacements on the staged copies (`python3 -I edit.py` from the batch folder; idempotent)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
EDITS = {
 "f01.ts": [("return operands.flatMap(operand=>{", "return operands.flatMap<Readonly<{operand:string;kind:\"bare\"|\"whole\"}>>(operand=>{")],
 "f02.ts": [("expect(found?.kind??\"path\",row.operand).toBe(row.kind);", "expect<string>(found?.kind??\"path\",row.operand).toBe(row.kind);")],
 "f03.ts": [("expect(item.expect.workers.map(", "expect<unknown>(item.expect.workers.map("), ("fullMegabytes:grown.fixedMegabytes+item.jobs*grown.perJobMegabytes}},JSON.stringify(item.lineage)", "fullMegabytes:grown.fixedMegabytes+item.jobs*grown.perJobMegabytes}} as unknown,JSON.stringify(item.lineage)")],
 "f04.ts": [("expect(readFileSync(join(repository,path),\"utf8\"),path).not.toMatch(call);\n});", "expect(readFileSync(join(repository,path),\"utf8\"),path).not.toMatch(call);\n},120000);"), ("expect(shape,row.name).toEqual(row.expected);expect(independent[index],row.name).toEqual(row.expected);", "expect<unknown>(shape,row.name).toEqual(row.expected);expect<unknown>(independent[index],row.name).toEqual(row.expected);")],
 "f05.ts": [("spawn(\"sh\",[\"-c\",'read go; exec \"$@\"',\"sh\",cargo,\"build\",\"-p\",\"t\"]", "spawn(\"sh\",[\"-c\",'read go; exec \"$@\"',\"sh\",cargo!,\"build\",\"-p\",\"t\"]"), ("\"sh\",process.execPath,script,leases,work,variant,log,cargo],", "\"sh\",process.execPath,script,leases,work,variant,log,cargo!],"), ("const run=Bun.spawnSync([python,\"-I\",join(here,\"🐍️.py\"),log])", "const run=Bun.spawnSync([python!,\"-I\",join(here,\"🐍️.py\"),log])"), ("expect(Bun.spawnSync([python,\"-I\",join(here,\"🐍️.py\"),overlapping]).exitCode).toBe(1);", "expect(Bun.spawnSync([python!,\"-I\",join(here,\"🐍️.py\"),overlapping]).exitCode).toBe(1);")],
 "f06.ts": [("const child=spawn(cargo,[\"-Zunstable-options\",\"-C\",work,\"build\"],{env:{...process.env,PATH:join(dirname(cargo))", "const child=spawn(cargo!,[\"-Zunstable-options\",\"-C\",work,\"build\"],{env:{...process.env,PATH:join(dirname(cargo!))"), ("const run=Bun.spawnSync([python,\"-I\",join(here,\"🐍️.py\"),log,String(variants.length),final])", "const run=Bun.spawnSync([python!,\"-I\",join(here,\"🐍️.py\"),log,String(variants.length),final])"), ("expect(Bun.spawnSync([python,\"-I\",join(here,\"🐍️.py\"),overlapping,\"2\"]).exitCode).toBe(1);", "expect(Bun.spawnSync([python!,\"-I\",join(here,\"🐍️.py\"),overlapping,\"2\"]).exitCode).toBe(1);")],
 "f07.ts": [("import{processPlatformCausesOfSuiteV1,", "import{type ProcessPlatformCauseV1,processPlatformCausesOfSuiteV1,"), ("{expect(actual[index],vector.name).toEqual(vector.expected);expect(python1[index],vector.name).toEqual(vector.expected);}", "{expect<unknown>(actual[index],vector.name).toEqual(vector.expected);expect<unknown>(python1[index],vector.name).toEqual(vector.expected);}"), ("facts=new Map<string,readonly string[]>()", "facts=new Map<string,readonly ProcessPlatformCauseV1[]>()")],
 "f08.ts": [("import {build} from \"esbuild\";", "import {build,type BuildOptions} from \"esbuild\";"), ("settings=(batch:{in:string;out:string}[])=>(", "settings=(batch:{in:string;out:string}[]):BuildOptions=>(")],
}
for name, pairs in EDITS.items():
    path = os.path.join(HERE, "files", name)
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if new in text and text.count(old) == (1 if old in new else 0):
            continue
        assert text.count(old) == 1, (name, old[:80], text.count(old))
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
path = os.path.join(HERE, "files", "f09.json")
fixture = json.load(open(path, encoding="utf8"))
schema = json.load(open(os.path.join(HERE, *([".."] * 10), "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json"), encoding="utf8"))
for language, labels in schema["const"]["graphPhase"]["labels"].items():
    fixture["labels"][language]["phases"]["prefetch"] = labels["phases"]["prefetch"]
names = {row["name"] for row in fixture["cases"]}
for language, env, text in (("en", "en_US.UTF-8", "repository:command · Preparing stale projects: 3 of 7 projects"), ("de", "de_CH.UTF-8", "repository:command · Veraltete Projekte vorbereiten: 3 von 7 Projekte")):
    name = "prefetch " + language
    if name not in names:
        fixture["cases"].append({"name": name, "environment": {"LANG": env}, "command": "repository:command", "resource": "build:graph-phase:prefetch:3:7", "mode": "shared", "elapsedMs": 0, "expected": text})
open(path, "w", encoding="utf8").write(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n")
