#!/usr/bin/env python3
"""🔗️ [sealprep] Process-chain law, one run per mode: with the sealed default the run is the trampoline, the snapshot door and one loader (3 heavy processes, a waiting run is the 25 MB trampoline plus the 124 MB snapshot door), so the law that says `the door and one loader` / `the door alone` is pinned per mode (live = unchanged bounds, sealed = measured bounds from the corpus). The mode variable comes from the sealed policy, never a copy.
usage: chainlaw.py <root> [apply]   (anchored, idempotent; refuses on a moved anchor; keeps the file mode; edits the 3 files of 🧭️invocation/🧪️tests/🔗️process-chain in ROOT, so give it a mirror)"""
import json, os, stat, sys
root = os.path.abspath(sys.argv[1])
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/"
def edit(rel, pairs):
    path = os.path.join(root, D + rel); mode = stat.S_IMODE(os.stat(path).st_mode); text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if new in text and old not in text: continue
        if text.count(old) != 1: sys.exit("anchor not unique (%d) in %s: %s" % (text.count(old), rel, old[:90]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text); os.chmod(path, mode)
S = lambda s: s
edit("🟦️.ts", [
 ('import schema from"./🧬️schema/🔣️.json";\n', 'import schema from"./🧬️schema/🔣️.json";\nimport sealed from"../../../🏛️sealed/🔣️.json";\n'),
 ('test.skipIf(process.platform==="win32")("a graph-free run is the front door and one loader, and its whole tree stays below the bound",()=>withBuildTestInvocationV1(async original=>{',
  'for(const mode of["live","sealed"]as const)test.skipIf(process.platform==="win32")("a graph-free run is the front door and one loader, and its whole tree stays below the bound ("+mode+")",()=>withBuildTestInvocationV1(async original=>{'),
 ('environment:Record<string,string|undefined>={...process.env,SEMIO_LANGUAGE:"en",SEMIO_BUILD_SERVICE:"off"};\n for(const key of Object.keys(environment))if(key===SCRIPT_PROCESS_INVOCATION_ENV||key.startsWith("SEMIO_HOST_CAPACITY_"))delete environment[key];',
  'environment:Record<string,string|undefined>={...process.env,SEMIO_LANGUAGE:"en",SEMIO_BUILD_SERVICE:"off",[sealed.modeVariable]:mode};\n for(const key of Object.keys(environment))if(key===SCRIPT_PROCESS_INVOCATION_ENV||key.startsWith("SEMIO_HOST_CAPACITY_"))delete environment[key];'),
 ('expect(peakHeavy,"the door and one loader").toBeLessThanOrEqual(law.maximumHeavyProcesses);', 'expect(peakHeavy,"the door and one loader").toBeLessThanOrEqual(mode==="sealed"?law.sealed.maximumHeavyProcesses:law.maximumHeavyProcesses);'),
 ('test.skipIf(process.platform==="win32")("a run that waits for host capacity is the front door alone: small and without any loader",()=>withBuildTestInvocationV1(async original=>{',
  'for(const mode of["live","sealed"]as const)test.skipIf(process.platform==="win32")("a run that waits for host capacity is the front door alone: small and without any loader ("+mode+")",()=>withBuildTestInvocationV1(async original=>{'),
 ('SEMIO_HOST_CAPACITY_DIRECTORY:area,SEMIO_LANGUAGE:"en",SEMIO_BUILD_SERVICE:"off"};', 'SEMIO_HOST_CAPACITY_DIRECTORY:area,SEMIO_LANGUAGE:"en",SEMIO_BUILD_SERVICE:"off",[sealed.modeVariable]:mode};'),
 ('.toBeLessThanOrEqual(waiter.maximumHeavyChildren);\n', '.toBeLessThanOrEqual(mode==="sealed"?waiter.sealed.maximumHeavyChildren:waiter.maximumHeavyChildren);\n   expect(Math.max(0,...heavy.map(row=>megabytes(row.kilobytes))),"resident megabytes of the heaviest child of the waiting run").toBeLessThanOrEqual(waiter.maximumHeavyChildResidentMegabytes);\n'),
])
path = os.path.join(root, D + "🔣️.json"); mode = stat.S_IMODE(os.stat(path).st_mode); corpus = json.load(open(path, encoding="utf8"))
if "sealed" not in corpus["run"]:
    corpus["run"]["sealed"] = {"maximumHeavyProcesses": 3}
    corpus["waiter"]["maximumHeavyChildResidentMegabytes"] = 200; corpus["waiter"]["sealed"] = {"maximumHeavyChildren": 1}
    raw = open(path, encoding="utf8").read()
    out = json.dumps(corpus, ensure_ascii=False, indent=1) + ("\n" if raw.endswith("\n") else "")
    open(path, "w", encoding="utf8").write(out); os.chmod(path, mode)
path = os.path.join(root, D + "🧬️schema/🔣️.json"); mode = stat.S_IMODE(os.stat(path).st_mode); schema = json.load(open(path, encoding="utf8"))
if "sealed" not in schema["properties"]["run"]["properties"]:
    run, waiter = schema["properties"]["run"], schema["properties"]["waiter"]
    run["required"].append("sealed"); run["properties"]["sealed"] = {"type": "object", "additionalProperties": False, "required": ["maximumHeavyProcesses"], "properties": {"maximumHeavyProcesses": {"type": "integer", "minimum": 1}}}
    waiter["required"] += ["maximumHeavyChildResidentMegabytes", "sealed"]; waiter["properties"]["maximumHeavyChildResidentMegabytes"] = {"type": "integer", "minimum": 1}
    waiter["properties"]["sealed"] = {"type": "object", "additionalProperties": False, "required": ["maximumHeavyChildren"], "properties": {"maximumHeavyChildren": {"type": "integer", "minimum": 0}}}
    raw = open(path, encoding="utf8").read()
    open(path, "w", encoding="utf8").write(json.dumps(schema, ensure_ascii=False, indent=1) + ("\n" if raw.endswith("\n") else "")); os.chmod(path, mode)
print("chainlaw applied to", root)
