# Neural Strict Retirement Adoption

## Source Checkpoint

Dictionary now owns `OrderedMap<Value>`. Clone shares the exact immutable root. Ordinary Dictionary destruction only performs atomic shared-alias release; a final live owner is rejected without recursive payload destruction. It never runs cold cleanup implicitly. `Dictionary::into_retirement` transfers its exact root to the domain cursor.

`ValueRetirement` transfers map nodes, nested dictionaries, shared values, strings, and byte buffers under explicit one-item byte grants. Final `OwnedValue` from OrderedMap is handed into the neural domain; no generic `drop(Value)` is used. Strings retire by actual remaining UTF-8 byte length, including grants smaller than a code point. The retained representation is bytes, so no UTF-8 slicing panic or capacity-sized admission gate is involved. Every frontier is terminal-guarded with ManuallyDrop.

`ColdDictionaryBuilder` explicitly completes update cursors, retires displaced roots, closes all update aliases, and handles final nested payloads through the domain cursor. Custom Dictionary serde decoding uses this builder, including duplicate replacement and partial decode failures. `ColdValueOwner` provides a visibly cold batch/test scope. No generic OrderedMap<Value> insert/Deserialize cleanup is used. These cold APIs confer no interactive evaluation credit.

The private NeuralCache entry index now uses numeric-key BTreeMap so retirement can pop one minimum entry through bounded machine-width metadata without HashMap sparse-capacity scans or long-string comparison. Displaced dictionaries enter retained domain cleanup. Cache retirement uses exact `Arc::into_inner`, not a strong-count race. The final cache transfers both its live index and pending domain retirement into the guarded close state. Whole `sweep`, hashing, and evaluator compute remain cold/unmigrated and are explicitly not credited as bounded jobs.

## Validation

The canonical source route `@semio-tech/neural-engine-rs:test-source` passed two language-neutral fixtures and four strict hostile rejections at 1/64/4,096 bytes. The fixtures include a 16,384-byte nested Unicode string and duplicate-key replacement with 4,800 semantic bytes. Existing third-party fast-json-stable-stringify reproduces the JSON output; exact UTF-8 payload byte totals are independently checked. Log: `🧪️neural-value-retirement-source-2026-08-27.txt`.

Seven native tests are authored and were handed to the coordinator's exclusive compiler queue: five nested domain/decoder/shared-root/worker-transfer/guard laws and two cache replacement/final-owner laws. Native outcome is pending at this checkpoint and must not be inferred from source checks.

### Native Regression Follow-up

The coordinator ran all seven new retirement laws successfully, then the complete neural suite. The first complete run correctly exposed 28 ordinary final-owner violations (14 passed). Repairs added visibly cold `ColdOwner`/`ColdRetire` scopes to internal evaluator partial maps, merged inputs, local caches, cluster work, Registry replacement/teardown, and SchemaComponent construction/validation errors. Named test roots now retire explicitly. The strict Dictionary/cache guards were not relaxed. After these changes the coordinator reran the full suite: **42 passed, zero failed**, exit 0, 1.14 seconds compile and 0.02 seconds test. Three expected caught guard panics belong to negative ownership tests. Exact log: `🧪️coordinator-neural-full-native-r2-2026-08-27.txt`.

The cold scope module is explicitly synchronous; even APIs historically named `budgeted` still contain cold evaluation/hash/topology work and do not earn retained interactive credit. EvalChannels output/input maps and private TreeSnapshot indexes now use BTreeMap for owned retirement without sparse-capacity scanning. Live long-key lookup/update construction remains cold; this does not replace the retained OrderedMap comparison API.

The cache lifecycle gained a strict schema-first fixture for two seeds, displaced ownership, shared release, final release, exact 21,193 bytes, and terminal emptiness. Its source oracle passed one fixture/three hostile rejections alongside the original two/four; log `🧪️neural-cache-retirement-source-2026-08-27.txt`.

Flow session close now transfers private ordered cache/set owners, byte strings, channels, snapshots, and final cache ownership into retained frontiers. Three native session laws are authored, including 42,405 semantic bytes with a 16 KiB string reserved to 64 KiB, 1/64/4,096-byte grants, worker transfer, empty reserved text, and a strict live-owner guard. The source fixture/third-party oracle passed one fixture/three hostile rejections; log `🧪️flow-session-retirement-source-2026-08-27.txt`. Native Flow session execution remains unrun. Existing process-global geometry retention merging in begin_close and FlowHost opaque bridge/ghost/index ownership remain explicit unfinished boundaries.

Two authoritative launch seed gates were registered for neural source and filtered native retirement tests. Canonical plugin-registry generation passed and regenerated launch.json. Log: `🧪️neural-launch-registry-2026-08-27.txt`. `git diff --check` passed at this checkpoint.

## Remaining Integration

Framework Flow's shared retirement now consumes the neural domain cursor, coordinated with the Generator executor. App-local Flow retirement delegates nested domain owners to shared Flow retirement and has its own terminal guard. FlowWorkingScene layout and app source-copy map/set/dictionary branches share immutable roots.

Host/evaluation owner adaptation remains active. Public EvalChannels outputs/inputs, host outputs/export payloads, and TreeSnapshot indexes still require coherent ordered ownership traversal to eliminate sparse HashMap close scans. Private session text caches must retire actual byte lengths, not require capacity-sized grants. Ordinary engine errors, local caches, aggregate decode failures, operator metadata, and batch/test scopes must adopt explicit cold ownership or retained retirement; strict Dictionary backing alone does not complete these callers. The five live Flow Artifact recipes, typed parameter factory integration, Store seal/hash wiring, and native end-to-end undo/cancel tests remain unfinished.
