# Neural Ownership Review

## Executed Gates

Full regression r3 now **passes 43/43**, 0.01 s runtime after 2.43 s compilation, Nx exit 0. The added evaluation-owner law executes with every previous engine test. Log: `🧪️coordinator-neural-full-native-r3-2026-08-27.txt`. This updates the regression checkpoint; the remaining retained evaluation, host, and browser gates below are unchanged.

The repaired **full neural regression r2 passes 42/42**, 0.02 s runtime after 1.14 s compilation, Nx exit 0. Three caught negative guard panics are expected. The explicit cold/error ownership repair and all seven new retirement laws execute together. Log: `🧪️coordinator-neural-full-native-r2-2026-08-27.txt`. This certifies the native regression suite, not retained evaluation latency or Flow/Wasm integration.

The preceding full run was RED: 14 passed, 28 failed, 42 total. Its stricter Dictionary/cache final-owner guards exposed unfinished ownership adoption in ordinary engine evaluation and tests. That evidence remains in `🧪️coordinator-neural-full-native-r1-2026-08-27.txt`; the guards were not relaxed to obtain r2.

The coordinator ran the actual neural-engine Nx native target with `--lib retirement -- --nocapture`: **7 passed, 0 failed, 35 filtered**, 0.01 s runtime after 53.04 s compilation. Five domain laws cover nested bytes, exact aliases, replacement/error cleanup, strict guards and cross-worker ownership. Two cache laws cover shared/final owners, replaced nested values, byte-accounted cleanup and guarded final drop. Three panic messages are expected caught negative checks. Log: `🧪️coordinator-neural-retirement-native-2026-08-27.txt`.

The expanded independent source gate r2 also passed: two language-neutral value cases/four strict-schema hostile rejections and one cache sequence/three hostile rejections, grants 1/64/4096, fast-json-stable-stringify oracle. Log: `🧪️coordinator-neural-retirement-source-r2-2026-08-27.txt`. The previous value-only run remains historical evidence.

## Reviewed Ownership

Dictionary now retains the verified codebase-owned ordered map. Clone shares its root. Drop only releases a shared alias atomically or rejects an unretired final owner without traversing nested data. ValueRetirement retains exact map/value/Arc/string frontiers and transfers one structural owner or a byte-bounded string fragment per turn. ColdDictionaryBuilder and ColdValueOwner explicitly own full synchronous cleanup for cold decoding, replacement, errors and batch scopes; they are not interactive APIs.

NeuralCache final cleanup takes the exact Arc root, transfers numeric-key BTreeMap ownership, and drains one entry into nested retirement. Numeric keys avoid unbounded string comparison; pop_first avoids sparse HashMap scanning. Ordinary final nonempty cache destruction is guarded. The coordinator read these implementations and all seven laws.

## Remaining Integration Work

The coordinator read the new complete ColdOwner/ColdRetire module. Its synchronous domain cleanup is explicitly batch-only; wrapping a budgeted-named evaluator does not make its scope cleanup bounded. Native regression correctness is restored, while Flow host/session cleanup and public EvalChannels adoption still need their own runtime tests. String-key BTreeMap insertion/lookup cannot receive retained-step credit merely because pop_first is bounded; use the owned bytewise map where those operations are interactive.

NeuralCache::sweep remains a full iter/filter/collect/remove loop. node_hash traverses the complete dictionary, and get_or_insert_with still accepts opaque synchronous computation. They are not newly certified as bounded. The Flow host invokes sweep at evaluation completion, so that evaluation lifecycle remains open until cursorized and measured. All app-native, Wasm and browser integrations remain required.
