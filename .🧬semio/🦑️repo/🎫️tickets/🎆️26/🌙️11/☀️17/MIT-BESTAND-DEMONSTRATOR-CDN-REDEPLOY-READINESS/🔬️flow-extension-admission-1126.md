# Flow extension registry-admission refusal (2026-11-26)

## Root cause
`SharedRegistryIssuer` (neural engine, `📔️registry/🏗️admission/🦀️.rs`) compared the full `SourceIdentity` (including the Source epoch and the per-turn recipient local) exactly on every slice. Flow extension actors admit the registry across successive turns, where the Source epoch advances, so every `evaluate` refused with framework `invariantViolated`: "registry admission requires its original source slot and caller" (258-byte Pack, SHA-256 e9edb2b7...). The nine staged Wasm binaries (linked 2026-11-24) predate the fix.

## Fix (already in source, authored change 9b948fc54a4)
`SourceIdentity::continues` keeps operation, generation, source parent, cancel node, clock and caller slot as exact identity, lets the epoch only advance, and drops the per-turn recipient from identity. No source change was needed in this pass.

## Test
`📔️registry/🏗️admission/🧪️tests/🦀️.rs`: `original_registry_issuer_follows_its_source_forward_and_refuses_every_other_source` (forward epochs, backwards epoch refusal, other actor/reborn Source refusal, moved caller slot refusal, cancelled close returns payload). Previously recorded as passing (3 tests) in the custody note.

## Rebuild
Queue: `JOBS=1 bash native-queue.sh flow-extension-{math,text,logic,primitive,list,dictionary,draw,bim,brep}`.
- First attempts failed with `cannot update the lock file ... --locked` in planning-capture: transient, caused by concurrent peer manifest/lock edits; `cargo metadata --locked` on the describe crate passes now and a peer's puzzle build passes the same stage.
- Relaunched; it queues on the exclusive product lane `q53457c` behind other agents' builds (puzzle, procedural, demonstrator/process).
