import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

G = BUILD + "🕸️graph/"
b = Batch("graphcore", "the graph module reads its capture ceilings from the revision policy, its snapshot ceiling from the snapshot policy and its strides, caps and sweep from its own schema, which joins the implementation inventory (re-keys: the implementation digest of the graph)")
b.new_schema(G + "🧬️schema/🔣️.json", "Graph module limits / Grenzen des Graph-Moduls", [Def("Limits", "The strides, caps and the observation sweep of the graph module.", "Die Schrittweiten, Obergrenzen und der Beobachtungs-Sweep des Graph-Moduls.", [
    ("invocationCheckStride", 1024, "After how many visited values a deep walk checks its invocation for cancellation.", "Nach wie vielen besuchten Werten ein tiefer Durchlauf seinen Aufruf auf Abbruch prueft."),
    ("discoveryProgressStride", 128, "After how many discovered files the discovery reports progress.", "Nach wie vielen entdeckten Dateien die Entdeckung Fortschritt meldet."),
    ("sweepMinimumBlocks", 24576, "The fewest observation blocks that must exist before the sweep of superseded blocks runs.", "Die wenigsten Beobachtungsbloecke, die existieren muessen, bevor der Sweep abgeloester Bloecke laeuft."),
    ("sweepBlocksPerHeader", 6, "The blocks one header is expected to hold; with the minimum it decides whether the sweep is worth its cost.", "Die Bloecke, die ein Kopf erwartungsgemaess haelt; entscheidet mit dem Minimum, ob sich der Sweep lohnt."),
    ("ownerRowListMaximum", 50000, "The most files whose stamp one owner folds into its content stamp; an owner with more has no stamp.", "Die meisten Dateien, deren Stempel ein Eigentuemer in seinen Inhaltsstempel faltet; ein Eigentuemer mit mehr hat keinen Stempel."),
    ("reasonCharacters", 200, "The most characters of the reason a failed prefetch reports.", "Die meisten Zeichen des Grundes, den ein fehlgeschlagener Vorabruf meldet."),
    ("snapshotReadChunkBytes", 65536, "The size of one read while a retained graph snapshot is staged (64 KiB).", "Die Groesse eines Lesevorgangs, waehrend ein behaltener Graph-Schnappschuss bereitgestellt wird (64 KiB)."),
])])
M = G + "🟦️.ts"
b.subs(M, [
    ("import{captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,", "import{REVISION_POLICY_V1,captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,"),
    ('import{refuseBuildV1}from"../📖️texts/🟦️.ts";', 'import{refuseBuildV1}from"../📖️texts/🟦️.ts";\nimport graphSchema from"./🧬️schema/🔣️.json"with{type:"json"};'),
    ("const graphFormatStampsV1=Object.freeze([\"graph.generation\",\"graph.record\",\"graph.inferrers\"]);\n", "const graphFormatStampsV1=Object.freeze([\"graph.generation\",\"graph.record\",\"graph.inferrers\"]);\n/** ‹E› The declared limits of the graph module; its schema file is their single source. */\nconst GRAPH_LIMITS_V1=Object.freeze({...graphSchema.definitions.Limits.const});\n"),
    ("fileURLToPath(graphSnapshotPolicyURLV1),65536,true)", "fileURLToPath(graphSnapshotPolicyURLV1),REVISION_POLICY_V1.maximumTextBytes,true)"),
    ("fileURLToPath(exactReadPolicyURLV1),65536,true)", "fileURLToPath(exactReadPolicyURLV1),REVISION_POLICY_V1.maximumTextBytes,true)"),
    ("fileURLToPath(graphSnapshotPolicyURLV1),65536).sha256", "fileURLToPath(graphSnapshotPolicyURLV1),REVISION_POLICY_V1.maximumTextBytes).sha256"),
    ("fileURLToPath(exactReadPolicyURLV1),65536).sha256", "fileURLToPath(exactReadPolicyURLV1),REVISION_POLICY_V1.maximumTextBytes).sha256"),
    ("capturePhysicalSourceTextV1(url,65536)", "capturePhysicalSourceTextV1(url,REVISION_POLICY_V1.maximumTextBytes)"),
    ("captureBoundedSourceRevisionV1(fileURLToPath(path),8388608).sha256);return {implementation", "captureBoundedSourceRevisionV1(fileURLToPath(path),REVISION_POLICY_V1.maximumSourceBytes).sha256);return {implementation"),
    ("semioPhysicalSourceURL))),65536).sha256};};", "semioPhysicalSourceURL))),REVISION_POLICY_V1.maximumTextBytes).sha256};};"),
    ("captured=captureBoundedSourceRevisionV1(path,8388608);return {kind:\"digest\"", "captured=captureBoundedSourceRevisionV1(path,REVISION_POLICY_V1.maximumSourceBytes);return {kind:\"digest\""),
    ("path=>captureBoundedSourceRevisionV1(path,8388608).sha256);}catch{diagnostic", "path=>captureBoundedSourceRevisionV1(path,REVISION_POLICY_V1.maximumSourceBytes).sha256);}catch{diagnostic"),
    ("Math.min(65536,bytes.length-offset)", "Math.min(GRAPH_LIMITS_V1.snapshotReadChunkBytes,bytes.length-offset)"),
    ("Buffer.byteLength(source)>536870912||Buffer.byteLength(authority)>536870912", "Buffer.byteLength(source)>graphSnapshotPolicyV1.maximumSnapshotBytes||Buffer.byteLength(authority)>graphSnapshotPolicyV1.maximumSnapshotBytes"),
    ('+" authority bytes, 536870912 allowed"', '+" authority bytes, "+graphSnapshotPolicyV1.maximumSnapshotBytes+" allowed"'),
    ("status.size>536870912", "status.size>graphSnapshotPolicyV1.maximumSnapshotBytes"),
    ("visited++%1024===0)checkScriptInvocation(invocation);\n    const item=pending.pop()", "visited++%GRAPH_LIMITS_V1.invocationCheckStride===0)checkScriptInvocation(invocation);\n    const item=pending.pop()"),
    ("visited++%1024===0)checkScriptInvocation(invocation);\n      if(!item||typeof item!==\"object\")return item;", "visited++%GRAPH_LIMITS_V1.invocationCheckStride===0)checkScriptInvocation(invocation);\n      if(!item||typeof item!==\"object\")return item;"),
    ("Math.floor(visited/128)!==Math.floor(previous/128)", "Math.floor(visited/GRAPH_LIMITS_V1.discoveryProgressStride)!==Math.floor(previous/GRAPH_LIMITS_V1.discoveryProgressStride)"),
    ("{minimumBlocks:24576,blocksPerHeader:6}", "{minimumBlocks:GRAPH_LIMITS_V1.sweepMinimumBlocks,blocksPerHeader:GRAPH_LIMITS_V1.sweepBlocksPerHeader}"),
    ("if(list.length>=50000)failed.add(name)", "if(list.length>=GRAPH_LIMITS_V1.ownerRowListMaximum)failed.add(name)"),
    ('say("failed",{reason:String((error as Error)?.message??error).replace(/\\s+/gu," ").slice(0,200)})', 'say("failed",{reason:String((error as Error)?.message??error).replace(/\\s+/gu," ").slice(0,GRAPH_LIMITS_V1.reasonCharacters)})'),
    ('new URL("./🕰️generations/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))', 'new URL("./🕰️generations/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))'),
])
b.subs(BUILD + "👁️observation/🧪️tests/📥️reuse/🟦️.ts", [
    ('{GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1}=await import("../../../🧠️memory/🟨️.mjs"),', '{GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1}=await import("../../../🧠️memory/🟨️.mjs"),graphLimits=JSON.parse(readFileSync(new URL("../../../🕸️graph/🧬️schema/🔣️.json",import.meta.url),"utf8")).definitions.Limits.const,'),
    ("GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1,checkResourceLeaseControl:", "GRAPH_LIMITS_V1:graphLimits,GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1,checkResourceLeaseControl:"),
])
T2 = BUILD + "📥️inputs/💾️storage/🧪️tests/🔏️closure/🟦️.ts"
b.subs(T2, [
    ('import {createHash} from "node:crypto";\n', 'import {createHash} from "node:crypto";\nimport {REVISION_POLICY_V1} from "../../../🔏️revision/🟨️.mjs";\n'),
    ("lstatSync:(path:string)=>facts.get(path),captureBoundedSourceRevisionV1:capture}", "lstatSync:(path:string)=>facts.get(path),REVISION_POLICY_V1,captureBoundedSourceRevisionV1:capture}"),
])
b.row("graph", "library", "🏗️build/🕸️graph/🧬️schema/🔣️.json", ["Limits"], ["🏗️build/🕸️graph/🟦️.ts"])
b.row("revision-graph", "library", "🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json", ["Policy"], ["🏗️build/🕸️graph/🟦️.ts"], minimum_number=8000)
b.write(tests=["./" + G + "🧪️tests/🕰️generation-protocol/🟦️.ts", "./" + G + "📥️inference/🔖️stamps/🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
