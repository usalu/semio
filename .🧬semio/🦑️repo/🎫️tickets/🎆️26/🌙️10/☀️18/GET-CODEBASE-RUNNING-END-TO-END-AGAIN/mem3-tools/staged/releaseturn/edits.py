DOC = """🧎️ Staged batch `releaseturn` (lane mem3): the graph host releases the inference caches after one turn of the event loop and collects last. The observation scopes and the async contexts of a finished inference stay referenced by the runtime until the loop has turned once, so the release of the tick that ended the inference freed only part of what it names (12 projects of a print refresh: 1,213 MB live, 1,017 MB after the same-tick release, 721 MB after one turn): the shaping, the index patch, the pack and the assembly ran with up to 1 GB of dead inference data still resident. `releaseAfterTurnV1` (memory module) is the one place that orders turn, release and collection; the host release goes through it. Apply: python3 -I stage.py apply; revert: python3 -I stage.py revert; laws: sh verify.sh <root>."""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
EDITS = [
    (L + "🧠️memory/🟨️.mjs", [
        ("/** 📐️ The managed bytes that stay live after one collection:",
         "/** 🧎️ Releases what a construction held after one turn of the event loop and collects last, also when the release throws: the observation scopes and the async contexts of a finished inference stay referenced by the runtime until the loop has turned once, so a release in the tick that ended the inference frees only part of what it names (12 refreshed projects: 1,213 MB live, 1,017 MB after a same-tick release, 721 MB after one turn).\n * @param {() => void | Promise<void>} release\n * @param {{ports?: {collect: () => void}, turn?: () => Promise<void>}} [options]\n */\nexport async function releaseAfterTurnV1(release,{ports=createMemoryPortsV1(),turn=()=>new Promise(resolve=>setTimeout(resolve,0))}={}){\n await turn();\n try{await release();}finally{ports.collect();}\n}\n\n/** 📐️ The managed bytes that stay live after one collection:"),
    ]),
    (L + "🧠️memory/🟨️.d.mts", [
        ("export function createMemoryPortsV1():MemoryPortsV1;",
         "export function createMemoryPortsV1():MemoryPortsV1;\n/** 🧎️ Releases what a construction held after one turn of the event loop and collects last, also when the release throws. */\nexport function releaseAfterTurnV1(release:()=>void|Promise<void>,options?:{ports?:Pick<MemoryPortsV1,\"collect\">;turn?:()=>Promise<void>}):Promise<void>;"),
    ]),
    (L + "🕸️graph/🟦️.ts", [
        ('import{GRAPH_MEMORY_POLICY_V1,graphStringInternerV1,createMemoryPortsV1}from"../🧠️memory/🟨️.mjs";',
         'import{GRAPH_MEMORY_POLICY_V1,graphStringInternerV1,createMemoryPortsV1,releaseAfterTurnV1}from"../🧠️memory/🟨️.mjs";'),
        ('    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry){(await import("./📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();setupSharesV1.delete(invocation);}graphStringInternerV1.clear();createMemoryPortsV1().collect();},',
         '    release:()=>releaseAfterTurnV1(async()=>{observationV1.releaseInferenceMemoryV1();(await import("../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry){(await import("./📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();setupSharesV1.delete(invocation);}graphStringInternerV1.clear();}),'),
    ]),
]
NEW = {L + "🧠️memory/🧪️tests/🧎️release-turn/🟦️.ts": "release-turn.ts"}
