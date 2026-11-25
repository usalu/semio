# Slice worker and pool child memory (2026-11-25)

## Symptom
`repo run semio-hub-flow-component:component -c prototype --exclude-dependencies`: the slice worker reached 4.5 GB RSS (heap 2.6 GB, 62-65 M objects, 27.8 M of them `Array`, 8-18 M `string`) while "checking the file inventory", pool children 1.6-4.2 GB; the host warden cancelled them.

## Measurement
- Live `slice-memory.json` samples of the worker (afterCollect after a full GC): 27.8 M arrays stay live, yet the tracked collections (rowTable, settledReads, owner memos, scopes) sum to under 250 MB; the arrays appear in 30 s of the inventory and plateau.
- Memo hold (`slice-worker-memory-replay.ts`) and a standalone owner discovery (`slice-worker-memory-discovery.ts`) stay at 80 MB / 600 MB, so neither is the source.
- The service log named the phase: `build:closure-graph`, `command-source-*` right before the census. `slice-worker-memory-closure-graph.ts` loads the machine's closure graph store (`.cache/f/closure-graph`, 2758 nodes, 12 k edges): heap 5058 MB, 14.3 M arrays, 20 M strings.
- The store held 2.4 GB of JSON text for 2758 nodes: every module node carries `extra.witness` of about 1.4 MB. The nodes' witnesses are 8 051 typed rows (median) of which 8 030 are shared by more than 1000 nodes: 14.3 M typed entries and 2.9 M path entries stand for 20 862 and 5 982 distinct ones (a factor of 700).

## Root cause
`closureGraphPassV1` stamped, for each module node, the witness of `[...node.parts, ...node.shared, ...config.parts]`. `node.shared` holds the row arrays that every import lookup of every node shares by identity (the compile index of generated outputs, about 8 000 rows). The witness copied them into each node, so the store and every process that loaded or rebuilt the graph (the slice worker, each inference pool child) held nodes x 8 000 entries (quadratic in the tree): 2.4 GB of text on disk, 4-5 GB of heap in memory.

## Fix
- `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`: `layeredWitnessV1`: the witness of a node is what it read itself plus a `base` witness of the wide shared row arrays (at least 64 rows), computed once per pass per distinct set of arrays.
- `👁️observation/🟨️.mjs` `witnessVerdictV1`: a witness may name a `base`; it is judged once per round through the round's `verdicts` (keyed by the base object) and every node holds only its own part.
- `🌳️closure-graph/🟨️.mjs` + schema: the store keeps each distinct base once in a pool chunk the head names (`pool`, digest or null); a node's witness names its base by index; a loaded graph holds one copy per base. A head without `pool`, or with a missing or altered pool chunk, refuses the graph. The fleet offer lists the pool chunk too.

## Before / after
| | before | after |
|---|---|---|
| closure graph store | 2.4 GB text, 54 MB brotli (2758 nodes) | 36 MB brotli (5292 nodes) |
| load of the store | heap 5058 MB, 14.3 M arrays | heap 94 MB, 0.27 M arrays |
| slice worker, flow-component dry run | rss 4.5 GB peak, heap 2.6 GB, 27.8 M arrays, did not finish | rss 1.35 GB peak, heap 509 MB, 0.69 M arrays, read done in 71 s |
| pool children | 1.6-4.2 GB | about 1.2 GB peak |

## Tests
`🌳️closure-graph/🧪️tests/🪪️layered-witness/🟦️.ts` (3): layered verdict equals the flat verdict (oracle) under edits of either layer; 400 nodes with a 6 000 row base store in under 1/50 of the flat bytes and load in under 1/20 of the flat heap, one shared base object, round trip equal, unchanged resave writes nothing; pool refusals. Green: closure-graph tests 8, walk-pass 1, fleet 1, observation witness 5.

## Not done
Pool children still carry about 1.2 GB (owner state); the per-node witness arrays of `typed` remain plain arrays (small now).
