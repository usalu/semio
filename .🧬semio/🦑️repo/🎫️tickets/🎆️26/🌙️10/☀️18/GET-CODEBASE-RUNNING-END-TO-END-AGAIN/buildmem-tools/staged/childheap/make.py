#!/usr/bin/env python3
"""🧒 [buildmem] Builds the staged batch `childheap` from the CURRENT files of a root (default: live, walked up from here): anchored edits, every anchor must occur exactly once; writes files/, base/, manifest.json beside this script. Rebase = run it again on the moved tree.
usage: python3 -I make.py [--root DIR]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
EN = "EN: The graph child that plans a build is told a fixed JavaScriptCore memory size (childHeapMegabytes), never the capacity grant of the whole run: the grant pays for every process of the run, and a larger size only lets the child keep more garbage before it collects. DE: Dem Graph-Kindprozess, der einen Build plant, wird eine feste JavaScriptCore-Speichergröße genannt (childHeapMegabytes), nie die Kapazitätszusage des ganzen Laufs: die Zusage bezahlt jeden Prozess des Laufs, und eine größere Angabe lässt das Kind nur mehr Abfall halten, bevor es aufräumt."
DOC = "/** 🧒️ The memory JavaScriptCore is told the graph child process may use (`BUN_JSC_forceRAMSize`), which sets how far its heap grows before it collects: the declared child size, never the capacity grant of the run, because the grant pays for every process of the run and a larger size only lets the child keep more garbage (36 refreshed projects in one process: 2 GiB peaks at 5.5 GB resident and 2.0 GB footprint, 8 GiB at 7.5 GB and 2.8 GB, with identical rows and wall time).\n * @see ./🧬️schema/🔣️.json */\n"
EDITS = {
    B + "🧠️memory/🧬️schema/🔣️.json": [
        ('es gibt also keine Standardsprache."', 'es gibt also keine Standardsprache. ' + EN + '"'),
        ('    "maximumParsedManifests": 256,\n', '    "maximumParsedManifests": 256,\n    "childHeapMegabytes": 2048,\n'),
    ],
    B + "🧠️memory/🟨️.mjs": [
        (" return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.maximumBudgetMegabytes,derived))*MEGABYTE;\n}\n",
         " return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.maximumBudgetMegabytes,derived))*MEGABYTE;\n}\n\n" + DOC + "export function graphChildHeapBytesV1(policy=P){return policy.childHeapMegabytes*MEGABYTE;}\n"),
    ],
    B + "🧠️memory/🟨️.d.mts": [
        ("readonly maximumBudgetMegabytes:number;", "readonly maximumBudgetMegabytes:number;readonly childHeapMegabytes:number;"),
        ("export function graphMemoryBudgetBytesV1(", "/** 🧒️ The fixed JavaScriptCore memory size of the graph child process in bytes, independent of the capacity grant. */\nexport function graphChildHeapBytesV1(policy?:GraphMemoryPolicyV1):number;\nexport function graphMemoryBudgetBytesV1("),
    ],
    B + "🧠️memory/🧪️tests/🔣️.json": [
        ('  "maximumParsedManifests": 256,\n', '  "maximumParsedManifests": 256,\n  "childHeapMegabytes": 2048,\n'),
    ],
    B + "🧠️memory/🧪️tests/🐍️.py": [
        ('        "messages": [render(policy, messages, row["key"], row["fields"], row["environment"]) for row in fixture["messageCases"]],\n',
         '        "messages": [render(policy, messages, row["key"], row["fields"], row["environment"]) for row in fixture["messageCases"]],\n        "childHeap": {"bytes": policy["childHeapMegabytes"] * MEGABYTE, "grantIndependent": True},\n'),
    ],
    B + "🧠️memory/🧪️tests/🟦️.ts": [
        ('test("the governor keeps the laws of every scenario, checked by the independent model",async()=>{',
         'test("the graph child heap is the declared size and equals the independent model",()=>{\n'
         ' const directory=mkdtempSync(join(tmpdir(),"memory-child-heap-"));\n'
         ' try{\n'
         '  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]),bytes=memory.graphChildHeapBytesV1();\n'
         '  expect(expected.childHeap).toEqual({bytes,grantIndependent:true});\n'
         '  expect(bytes).toBe(moduleSchema.definitions.Policy.const.childHeapMegabytes*MEGABYTE);\n'
         '  expect(memory.graphChildHeapBytesV1({...memory.GRAPH_MEMORY_POLICY_V1,childHeapMegabytes:1024})).toBe(1024*MEGABYTE);\n'
         ' }finally{rmSync(directory,{recursive:true,force:true});}\n'
         '});\n\n'
         'test("the governor keeps the laws of every scenario, checked by the independent model",async()=>{'),
    ],
    B + "🧭️invocation/🕸️graph/🟦️.ts": [
        ("/** 🧠️ The memory JavaScriptCore is told the child may use, which sets how far it lets the heap grow before it collects: the megabytes the admission granted this invocation, at least 2 GiB. Without it the heap grows toward a quarter of the host before the first full collection. */\n"
         "export function buildGraphChildHeapBytesV1(environment:Readonly<Record<string,string|undefined>>):number{\n"
         " let granted=0;try{const plan=JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN??\"null\") as {memoryMegabytes?:unknown;fixedMegabytes?:unknown}|null;const part=(value:unknown)=>typeof value===\"number\"&&Number.isFinite(value)&&value>0?value:0;granted=part(plan?.memoryMegabytes)+part(plan?.fixedMegabytes);}catch{}\n"
         " return Math.max(2147483648,Math.floor(granted)*1048576);\n"
         "}\n\n", ""),
        ('{profileMarkV1}=await import("../../⏱️profile/🟨️.mjs"),', '{profileMarkV1}=await import("../../⏱️profile/🟨️.mjs"),{graphChildHeapBytesV1}=await import("../../🧠️memory/🟨️.mjs"),'),
        ("BUN_JSC_forceRAMSize:String(buildGraphChildHeapBytesV1(process.env))", "BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1())"),
    ],
    B + "🧭️invocation/🧪️tests/🧊️isolated-plan/🟦️.ts": [
        ('test("the child heap is told the granted megabytes, never less than 2 GiB",()=>{\n'
         ' expect(invocation.buildGraphChildHeapBytesV1({})).toBe(2147483648);\n'
         ' expect(invocation.buildGraphChildHeapBytesV1({SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({memoryMegabytes:0,fixedMegabytes:5120})})).toBe(5120*1048576);\n'
         ' expect(invocation.buildGraphChildHeapBytesV1({SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({memoryMegabytes:512,fixedMegabytes:0})})).toBe(2147483648);\n'
         ' expect(invocation.buildGraphChildHeapBytesV1({SEMIO_HOST_CAPACITY_PLAN:"{"})).toBe(2147483648);\n'
         '});',
         'test("the child heap is the declared graph child size, never the capacity grant of the run",async()=>{\n'
         ' const text=readFileSync(fileURLToPath(new URL("../../🕸️graph/🟦️.ts",import.meta.url)),"utf8"),start=text.indexOf("export async function loadBuildGraphInChildV1("),body=text.slice(start,text.indexOf("\\n}\\n",start));\n'
         ' expect(start).toBeGreaterThan(-1);\n'
         ' expect(body).toContain("BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1())");\n'
         ' expect(body.match(/BUN_JSC_forceRAMSize/gu)?.length).toBe(1);\n'
         ' expect("buildGraphChildHeapBytesV1" in invocation).toBe(false);\n'
         ' const memory=await import("../../../🧠️memory/🟨️.mjs");\n'
         ' expect(memory.graphChildHeapBytesV1()).toBe(memory.GRAPH_MEMORY_POLICY_V1.childHeapMegabytes*1048576);\n'
         '});'),
    ],
}
def sha(data): return hashlib.sha256(data).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/": here = os.path.dirname(here)
    return here
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    for sub in ("files", "base"):
        shutil.rmtree(os.path.join(HERE, sub), ignore_errors=True); os.makedirs(os.path.join(HERE, sub))
    manifest = {}
    for index, (rel, edits) in enumerate(EDITS.items()):
        raw = open(os.path.join(root, rel), "rb").read(); text = raw.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1: sys.exit(f"anchor count {text.count(old)} in {rel[-70:]}: {old[:80]!r}")
            text = text.replace(old, new)
        name = f"{index:02d}" + os.path.splitext(rel)[1]
        open(os.path.join(HERE, "base", name), "wb").write(raw); open(os.path.join(HERE, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(raw), "new": sha(text.encode("utf8"))}
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(f"staged {len(manifest)} files from {root}")
main()
