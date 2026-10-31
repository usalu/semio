# Typing State Cross-Language Plan

Read-only Source audit, 2026-10-29. No edits/tests/native/compiler/process/lease operations. Actual52482 was reported queued; this report identifies current Source mismatch, not an observed Rust compiler error. Files are changing concurrently; one OS follow path disappeared during the read, so owner freeze/recheck is required.

## Current Canonical Semantics

Rust tool-machine/⌨️typing/🦀️.rs:225 defines TypingState.entries:Vec<(String,M)>; TS tool-machine/🟦️.ts:819 defines the same ordered keyed entries. Typing schema $defs.TypingOpen requires entries:TransactionEntries (JSON rows {key,mutation}) and describes keyed net leaves.

Typing keys are **derived numeric positions**, not authored semantic identifiers: Rust typing:188 upserts index.to_string() and retracts the remaining prior indices; TS:775–776 uses String(index) identically. Folding consumes ordered mutations and returns ordered net leaves. Thus a persisted typing-only `net:Vec<M>` is semantically sufficient **provided** transaction reconstruction always enumerates net to exact contiguous decimal keys0..n-1 and replacement retracts all old trailing indices. Generic ToolTransaction, gestures and scrub are separately keyed contracts and should not be renamed wholesale.

The new typing/edit Source already assumes TypingState.net: fold_inputs at50, replacement at94, destructuring/construction at95–96. Current canonical TypingState still has entries. New typing/resident assumes sorted Vec<TypingWindow<M>>, while current TypingLedger at333 uses BTreeMap<SharedUtf8,RetainedOriginalSlot<TypingState<M>>>. Do not mount resident against the current map without completing all ledger methods/transfer macros coherently.

## Opinionated Coherent Change

Use one canonical **typing net vector** across Rust, TS and typing-specific JSON state. Change TypingState.entries→net, schema TypingOpen.entries→net (array of mutations), typing persisted/preview fixture outputs accordingly. Keep tool transaction protocol entries and its keyed generic operations unchanged. A transient conversion at the generic transaction boundary enumerating net is a representation operation, not a legacy compatibility API; do not accept both persisted field names.

Schema first: author TypingOpen.net and its description/required fields, plus neutral cases asserting numeric-key projection, replacement shrink to zero, repeated fold, split, resume, idle/explicit commit, abort and window ordering. Run the same neutral typing fixtures through Rust, TS and existing XState oracle. Do not migrate persisted assets at runtime or retain entries aliases.

If owner intends a **Rust-internal-only** optimization instead, keep JSON TypingOpen and all neutral fixture expected entries unchanged, explicitly enumerate net in both serializing fixture projections and generic transaction resume. That is a valid smaller representation change, but it must be labelled internal; exposing Rust net while leaving the TS public TypingState.entries is not a coherent cross-language API rename.

## Files and Assets to Change Together

Paths below are relative to `🧰️framework/`.

| Owner | Required coherent work |
|---|---|
| 🔨️modules/🛠️tool-machine/🧬️schema/🔣️.json | TypingOpen required/property/description becomes net for full canonical change. TypingFold and TypingInput already use leaves; no need to rename them. TransactionEntries, ScrubOpen and GestureOpen stay keyed. |
| 🔨️modules/🛠️tool-machine/⌨️typing/🦀️.rs | TypingState, resume context length and generic ToolTransaction reconstruction, persist extraction, TypingLedger preview/open/read/retirement and every window operation. Current entries-dependent lines252/254/322/366. |
| 🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs | Already consumes net; align final state metadata/retirement and exact resident exchange, preserving actual Original/grants. Its neutral JSON/Rust tests must compile against canonical package. |
| 🔨️modules/🛠️tool-machine/⌨️typing/🗃️resident/🦀️.rs | Complete sorted Vec resident ownership and mount exactly once only if that is final design. Preserve lexicographic windows order, source_revision, live_windows, read/write fencing, capacity/copy/release accounting and original-slot identity. |
| 🔨️modules/🛠️tool-machine/🟦️.ts | TypingState:819, Typing.resume:836+, persist:891+, TypingLedger preview:918. Enumerate net to generic transaction keys; do not alter generic transaction/gesture/scrub entries. |
| 🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts | typing ledger projection around706, invalid TypingOpen fixture794, random typing/resume oracle847+. Preserve generic open ToolStep entries output: it still reflects actual ToolTransaction entries, not TypingOpen persisted state. |
| 🔨️modules/🛠️tool-machine/🧪️tests/🔬️unit/⌨️typing-laws/🦀️.rs | ledger JSON projection140, persist/resume laws199+, net→keyed transaction equivalence. Generic fixture entries_json helper stays for transaction/scrub/gesture outputs. |
| 🔨️modules/🛠️tool-machine/🧪️tests/🧫️fixtures/🧫️typing-law/🔣️.json | All **typing open ledger persisted-state** expected entries rows become ordered net values for full canonical change. Do not blindly replace every entries token: generic step expectations remain keyed. |
| 🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/📐️.json and 🔣️.json and 🦀️.rs | Update field/state representation and add exact numeric ordering/shrink/split/resume cases matching canonical schema. |
| 🔨️modules/🛠️tool-machine/🔗️read/📤️transfer/🦀️.rs | Typing-specific original_transfer/map_slots registration currently assumes ledger slot implementation; change TypingLedger slot traversal when moving BTreeMap→Vec. Leave gesture/scrub transfer behavior unchanged. |
| 🔨️modules/🛠️tool-machine/🔗️read/🧪️tests/🦀️.rs | TypingState original read/issuer/commit fence test at22, include new vector backing and no stale slot identity. |
| 🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs | Type/factory/retained-read consumer of TypingState; recheck concrete field access and native retained projection after state change. |
| 🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🔁️follow/🦀️.rs | Discovered TypingState consumer; file disappeared during follow-up read, so classify peer-transient and re-enumerate after owner freeze. Do not recreate old path without owner agreement. |
| 🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json | Existing TypingState/TypingOpen schema references require normal schema/catalog regeneration if canonical JSON schema changes; no hand-added alias. |

The schema, typing-law corpus and TS/Rust conformance projections are the authored language-neutral contract; generated mirrors/catalogs must be regenerated from final schema through their owners. No other direct TypingState/TypingOpen consumer was found in the bounded framework traversal. This is a reference-name census, not proof that no indirect use exists.

## Required Semantic Invariants

net order equals transaction insertion order; numeric keys are contiguous canonical decimal indices; shorter replacement removes all stale trailing leaves; empty net settles as Empty without publishing an edit; fold split commits previous net before opening next; resume retains transaction reference/buffer/deadline and chart fingerprint; windows remain lexicographically ordered and commits/expiry/preview agree across TS Map and Rust resident representation. Actual retained Original exchange must move the same vector backing under funded work rather than clone/rebuild it outside admission.

Do not fix the stale Source mismatch by casting net to keyed entries, adding a second state field, exposing old-name methods or weakening neutral expected output. Wait for actual52482 terminal result before calling this its compiler failure.
