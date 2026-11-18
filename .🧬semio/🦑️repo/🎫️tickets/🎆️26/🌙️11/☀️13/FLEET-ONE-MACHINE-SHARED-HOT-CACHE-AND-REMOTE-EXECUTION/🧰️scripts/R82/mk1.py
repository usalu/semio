import sys,json;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
H=__import__("os").path.dirname(__import__("os").path.abspath(__file__))+"/"
rd=lambda n:open(H+n,encoding="utf8").read()
IN=B+"🕸️graph/📥️inference/";SI=IN+"🧭️script-inputs/";OBS=B+"👁️observation/";SCH=IN+"🧬️schema/🔣️.json"
s=Stage(SCR+"stage1")
limits={"layerIndexDictionaryMaximumBytes":2147483648,"layerIndexMaximumRowBytes":1048576,"layerIndexMinimumRows":8,"layerIndexReadBlockBytes":65536,"layerIndexLockWaitMilliseconds":3000,"layerIndexLockStaleMilliseconds":30000}
en={"layerIndexDictionaryMaximumBytes":"The largest size (2 GiB, row ids are 32 bit) of the persisted row dictionary of the closure layers (`x/rows.dict` of a closure store); beyond it no new row is added and the sweep of the store resets the index, which the next parses rebuild.",
"layerIndexMaximumRowBytes":"The longest stored row (1 MiB) a reader of the row dictionary accepts; a longer record is damaged.",
"layerIndexMinimumRows":"The fewest rows a layer needs to be indexed by row ids; a smaller layer is parsed, which costs less than a list.",
"layerIndexReadBlockBytes":"The size (64 KiB) of the block a reader of the row dictionary reads at once, because the rows of one layer that were new together lie together.",
"layerIndexLockWaitMilliseconds":"How long a writer of the row dictionary waits for another writer before it gives up and parses the layer again next time.",
"layerIndexLockStaleMilliseconds":"The age (30 s) from which the lock of the row dictionary counts as abandoned and is taken over."}
de={"layerIndexDictionaryMaximumBytes":"Die größte Größe (2 GiB, Zeilennummern haben 32 Bit) des gespeicherten Zeilenwörterbuchs der Abschlussschichten (`x/rows.dict` eines Abschlussspeichers); darüber kommt keine Zeile mehr hinzu und die Bereinigung des Speichers setzt den Index zurück, den die nächsten Zerlegungen neu aufbauen.",
"layerIndexMaximumRowBytes":"Die längste gespeicherte Zeile (1 MiB), die ein Leser des Zeilenwörterbuchs annimmt; ein längerer Eintrag ist beschädigt.",
"layerIndexMinimumRows":"Die wenigsten Zeilen, die eine Schicht für den Zeilennummernindex braucht; eine kleinere Schicht wird zerlegt, das kostet weniger als eine Liste.",
"layerIndexReadBlockBytes":"Die Größe (64 KiB) des Blocks, den ein Leser des Zeilenwörterbuchs auf einmal liest, weil die Zeilen einer Schicht, die zusammen neu waren, beieinander liegen.",
"layerIndexLockWaitMilliseconds":"Wie lange ein Schreiber des Zeilenwörterbuchs auf einen anderen Schreiber wartet, bevor er aufgibt und die Schicht beim nächsten Mal wieder zerlegt.",
"layerIndexLockStaleMilliseconds":"Das Alter (30 s), ab dem die Sperre des Zeilenwörterbuchs als verlassen gilt und übernommen wird."}
t=open(R+"/"+SCH,encoding="utf8").read();d=json.loads(t)
d["definitions"]["Limits"]["const"].update(limits);d["definitions"]["LimitsDescriptions"]["const"]["en"].update(en);d["definitions"]["LimitsDescriptions"]["const"]["de"].update(de)
i=str(s.n);s.n+=1;open(s.S+"/"+i+".base","w",encoding="utf8").write(t);open(s.S+"/"+i+".new","w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n");s.rows.append((i,SCH,"edit"))
mod=rd("layer-index.src.mjs").replace('"OBSERVATION"','"../../../../👁️observation/🟨️.mjs"').replace('"ROWKEY"','"../../../🕰️generations/🧮️fact-rows/🟨️.mjs"').replace('"SCHEMA"','"../../🧬️schema/🟨️.mjs"')
s.new(SI+"🧾️layer-index/🟨️.mjs",mod)
s.new(SI+"🧾️layer-index/🟨️.d.mts",'''/** 🪪️ The persisted row dictionary and row-id lists of the stored closure layers. */
export type LayerIndexV1 = Readonly<{
  idsSync(digest: string): Uint32Array | undefined;
  rowSync(id: number): unknown;
  indexSync(digest: string, rows: readonly unknown[]): Uint32Array | undefined;
  sweepSync(layers: string): { removed: number; reset: boolean };
  stats(): { indexed: number; read: number; rowReads: number; full: number; fingerprints: number };
}>;
export function createLayerIndexV1(options: { directory: string; limits?: Record<string, number> }): LayerIndexV1;
''')
s.new(SI+"🧾️layer-index/🧪️tests/🟦️.ts",rd("layer-index.unit.test.ts"))
s.new(IN+"🧪️tests/🪪️layer-ids/🟦️.ts",rd("layer-ids.law.ts"))
s.new(IN+"🧪️tests/🪪️layer-ids/🐍️.py",rd("layer-ids.law.py"))
s.edit(SI+"💾️entries/🟨️.mjs",[
('import{readFileSync}from"node:fs";','import{existsSync,readFileSync}from"node:fs";',1),
('import{graphStringInternerV1,internJsonStringsV1}from"../../../../🧠️memory/🟨️.mjs";','import{graphStringInternerV1,internJsonStringsV1}from"../../../../🧠️memory/🟨️.mjs";\nimport{createLayerIndexV1}from"../🧾️layer-index/🟨️.mjs";',1),
(' const layerBytesSync=digest=>{',' const layerIndex=createLayerIndexV1({directory,limits});\n const layerBytesSync=digest=>{',1),
('   const rows=JSON.parse(bytes.toString("utf8"));\n   for(let at=0;at<rows.length;at+=size)yield rows.slice(at,at+size);','   const rows=JSON.parse(bytes.toString("utf8"));\n   layerIndex.indexSync(digest,rows);\n   for(let at=0;at<rows.length;at+=size)yield rows.slice(at,at+size);',1),
('  /** 🌊️ The layers named by `digests` one after the other','''  /** 🪪️ The dictionary ids of the rows of the stored layer `digest` in layer order (the layer is decided by the verdicts of its distinct rows, never parsed), or undefined while the layer is lost or no process has parsed it yet; a layer parsed by `layerBatchesSync` is listed for every later reader. */
  layerIdsSync(digest){return existsSync(layerPath(digest))?layerIndex.idsSync(digest):undefined;},
  /** 🔍️ The row of one dictionary id of {@link layerIdsSync}, or undefined while it cannot be read. */
  layerRowSync(id){return layerIndex.rowSync(id);},
  /** 📊️ How many layers this process listed and read and how many rows it read from the dictionary. */
  layerIndexStats(){return layerIndex.stats();},
  /** 🌊️ The layers named by `digests` one after the other''',1),
('  await collectClosureStoreV1({directory,graceMilliseconds:limits.closureStoreGraceMilliseconds});','  await collectClosureStoreV1({directory,graceMilliseconds:limits.closureStoreGraceMilliseconds});\n  layerIndex.sweepSync(layers);',1),
])
s.edit(SI+"💾️entries/🟨️.d.mts",[('export type ClosureStoreV1 = Readonly<{ stamp(id: string): Promise<string | undefined>;','export type ClosureStoreV1 = Readonly<{ layerIdsSync(digest: string): Uint32Array | undefined; layerRowSync(id: number): unknown; layerIndexStats(): { indexed: number; read: number; rowReads: number; full: number; fingerprints: number }; stamp(id: string): Promise<string | undefined>;',1)])
s.edit(OBS+"🟨️.mjs",[('export function observationLayerHoldsV1(layer){return memoRowsHoldV1(layer,scopes.getStore());}','''export function observationLayerHoldsV1(layer){return memoRowsHoldV1(layer,scopes.getStore());}
/** 🧪️ Whether one stored row still holds, probed against the tree now and remembered nowhere: for a caller that keeps its own verdict per distinct row (the row dictionary of the closure layers), which asks once per row and pass, so no per-round table of rows is filled beside it. */
export function observationRowFreshV1(row){const seen=rebaseRowV1(row);return seen.kind==="file"?fileFactHolds(seen.path,seen.fact)&&JSON.stringify(facts.file(seen.path)?.sha256??null)===JSON.stringify(seen.fact.sha256??null):current([seen]);}''',1)])
s.edit(OBS+"🟨️.d.mts",[('export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;','export function observationLayerHoldsV1(layer:readonly unknown[]):boolean;\n/** 🧪️ Whether one stored row still holds, probed against the tree now and remembered nowhere. */\nexport function observationRowFreshV1(row:unknown):boolean;',1)])
s.edit(SI+"🟨️.mjs",[
('observationRowsHoldAsyncV1, OBSERVATION_LIMITS_V1 }','observationRowsHoldAsyncV1, observationRowFreshV1, OBSERVATION_LIMITS_V1 }',1),
('layerCountsV1 = { hits: 0, joined: 0, checked: 0, milliseconds: 0 };','''layerCountsV1 = { hits: 0, joined: 0, checked: 0, indexed: 0, milliseconds: 0 };
const rowVerdictsV1 = new WeakMap(), trackRowVerdictsV1 = round => { const map = new Map(); rowVerdictsV1.set(round, map); verdictMapsV1.add(new WeakRef(map)); return map; };
/** 🪪️ The verdict on one stored layer from the row ids a process listed for it earlier (`layerIdsSync` of the store): the layer is not read, decompressed or parsed, each distinct row of the dictionary is probed against the tree once per round and the layer holds while all its rows do; undefined while the layer has no list, so the caller reads it. A fact that changed since the list was made stales the layers that name its row, exactly. */
async function indexedLayerHoldsAsyncV1(entry, digest, round, step) {
  const store = entry.store; if (typeof store?.layerIdsSync !== "function") return undefined;
  const ids = store.layerIdsSync(digest); if (ids === undefined) return undefined;
  const known = round ? rowVerdictsV1.get(round) ?? trackRowVerdictsV1(round) : new Map();
  let probed = 0;
  for (let at = 0; at < ids.length; at++) {
    const id = ids[at]; let holds = known.get(id);
    if (holds === undefined) { const row = store.layerRowSync(id); if (row === undefined) return undefined; holds = observationRowFreshV1(row); known.set(id, holds); if ((++probed & 63) === 0) await step(); }
    if (!holds) return false;
  }
  return true;
}
/** 🪪️ The synchronous form of {@link indexedLayerHoldsAsyncV1}. */
function indexedLayerHoldsV1(entry, digest, round) {
  const store = entry.store; if (typeof store?.layerIdsSync !== "function") return undefined;
  const ids = store.layerIdsSync(digest); if (ids === undefined) return undefined;
  const known = round ? rowVerdictsV1.get(round) ?? trackRowVerdictsV1(round) : new Map();
  for (let at = 0; at < ids.length; at++) {
    const id = ids[at]; let holds = known.get(id);
    if (holds === undefined) { const row = store.layerRowSync(id); if (row === undefined) return undefined; holds = observationRowFreshV1(row); known.set(id, holds); }
    if (!holds) return false;
  }
  return true;
}''',1),
('    const started = performance.now();\n    let holds = true;\n','''    const started = performance.now();
    const listed = await indexedLayerHoldsAsyncV1(entry, digest, round, step);
    if (listed !== undefined) { layerCountsV1.indexed++; layerCountsV1.milliseconds += performance.now() - started; verdicts?.set(digest, listed); return listed; }
    let holds = true;
''',1),
('  if (known !== undefined) return known;\n  let holds = true;\n','''  if (known !== undefined) return known;
  const listed = indexedLayerHoldsV1(entry, digest, round);
  if (listed !== undefined) { layerCountsV1.indexed++; verdicts?.set(digest, listed); return listed; }
  let holds = true;
''',1),
])
s.edit(SI+"🟨️.d.mts",[('export function layerVerdictCountsV1(): { hits: number; joined: number; checked: number; milliseconds: number };','export function layerVerdictCountsV1(): { hits: number; joined: number; checked: number; indexed: number; milliseconds: number };',1)])
s.finish([SI+"🧾️layer-index/🧪️tests/🟦️.ts",IN+"🧪️tests/🪪️layer-ids/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SI+"💾️entries/🧪️tests/🟦️.ts",IN+"🧪️tests/closure-store-limits.test.ts"] if False else [SI+"🧾️layer-index/🧪️tests/🟦️.ts",IN+"🧪️tests/🪪️layer-ids/🟦️.ts",IN+"🧪️tests/🧮️pass-verdicts/🟦️.ts",SI+"💾️entries/🧪️tests/🟦️.ts",SI+"🧪️tests/closure-store-limits.test.ts",SI+"🧪️tests/walk-members-merge.test.ts"],"- Step 1 (R82 a): the closure store lists every layer it parses by dictionary ids (`ce/x/rows.dict`, `ce/x/<digest>.ids`, each distinct row once, fingerprints of 80 bits, lock, torn tail repair, identity of the dictionary in every list, sweep with the store collection); the layer verdict of a later round or process reads the ids and probes each distinct row once per round instead of reading, decompressing, hashing and parsing the layer (counter `indexed` beside `checked`). `observationRowFreshV1` probes one row without a round table. Laws: `🧾️layer-index/🧪️tests` (5), `🧪️tests/🪪️layer-ids` (warm process: every layer indexed, none parsed, rows probed once; changed fact stales exactly its entries, counts from an independent Python model).")
open(s.S+"/before.txt","w",encoding="utf8").write(IN+"🧪️tests/🪪️layer-ids/🟦️.ts\n")
print("staged",s.S)
