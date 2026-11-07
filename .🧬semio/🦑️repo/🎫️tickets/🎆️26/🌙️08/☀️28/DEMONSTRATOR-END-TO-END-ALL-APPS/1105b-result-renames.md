STATUS: sweep applied; lock refresh clean (renames-2 exit 0); lock-ok touched; renames-1 wasm one-item check hit a cargo feature-resolver panic (open)

Rule: os/plugin/ui packages drop the "framework-" prefix (matches ~7600 existing dependency keys). 18 package names renamed (retained-command, result-page, reactor-*, runtime-ops, host-*, interpreter-imports, tests-01/16, instance-*, native-identity-tests, store-member-open, store-one-item-batch, os-kernel-dsl-derive-expand incl. its lib name). 568 Cargo.toml/.rs files rewritten (7629 exact-token replacements, only when the target package exists; also os-renderer-wgpu -> renderer-wgpu). Tools: 1105b-tools/renames/{census,apply,names,wsm}.ts. Census over all non-scratch manifests (path deps, package aliases, workspace=true keys against the owning root incl. nested roots): 0 mismatches. tomlscan: 0 bad. Hub's 67 renames agree.
Proof: renames-2 = repo cargo check, value/ordered manifest, exit 0 (Cargo.lock refreshed). Log 🗑️generated/1105b/renames-2.txt.
Open: renames-1 (wasm check -p semio-os-store-one-item on framework/Cargo.toml) planning-capture panics in cargo resolver/features.rs:326 "did not find features for semio-os-store-one-item"; not a name mismatch (resolution/lock succeeded); log renames-1.txt line 267.
Remaining prose-only old names in doc comments of plugin 🦀️.rs untouched.

## Feature strings + the renames-1 panic (follow-up)
Feature census (1105b-tools/renames/feat.ts: every `dep:X`, `X/f`, `X?/f`, bare name in [features], 1510 manifests with features): 0 stragglers; nothing to apply after sync-done. Census with explicit `package.workspace` roots honoured: still 0 mismatches.
The cargo features.rs:326 panic was a wrong invocation, not a rename: `--manifest-path 🧰️framework/Cargo.toml -p semio-os-store-one-item` while that crate declares `workspace = <repo root>` (also listed in framework members). With the crate's own manifest (renames-4, wasm, -p semio-os-store-one-item) the panic is gone and it reaches rustc: next error is E0753 "expected outer doc comment" at value/decode/owned/../../🪡️field/🦀️.rs:3 (source, not names; owner of value-native-owned).

## value lane (receive refactor)
FIXED, wasm check rc 0 each (logs renames-6..11): manifest-media, replication-command-ingress, actor-retained-turn, os-presence-native-snapshot, io-schema, ui-contract-text. value-ok touched.
Changes: derive default native path -> ::semio_framework_value; value facade re-exports OriginalReceivingPrimary; 9 inner `//!` docs moved to file top in value-native-owned (E0753); presence json decode/encode ported to NativeSnapshotDecodeOwner::receive<T,O,E> + native_snapshot_turn (work counted via work.performed()); ArtifactPresenceSnapshot trait now returns RetainedWorkOutcome<_,NativeSnapshotFailure> (NoPresence, publication fixture impl, peer callers and their unit tests updated; peer drops outcome.work: open).
Open (not done, other callers of the old 2-generic receive): ~dozens, e.g. spr/channel archive/decode, plugin media intrinsic/fault, io-mechanism, mcp workspace json/sqlite, trinity graph io, stdio sqlite receiving artifacts. Presence crate unit tests (json/tests) still on old Result API.

## value lane: owner.receive callers (follow-up)
FIXED (compile rc 0): os-channel archive/decode (renames-12), mcp probe-document json+sqlite+receive_pack (renames-17).
Edited, not compile-verified: plugin media intrinsic/fault (energie took over), store codec IoError .into() (62), trinity graph io (repo cargo cannot build that standalone workspace: prototype profile needs cargo-features trim-paths; rustfmt parses). Added NativeSnapshotFailure::into_value_error (io-control failure) as the single flattening boundary for callers without a ledger.
Taken by others: io-mechanism (runtime), plugin-decl-app/guest-contract (koordinator), space-history/sqlite-snapshot receiving (store).
NOT done: stdio sqlite artifacts' ArtifactSqliteSnapshotHost::{to,from}_sqlite_database_receiving and their per-artifact receiving modules (stl, epw, jpg, semio-text, deflate, txt, binary, json, las, csv, tsv, obj ...), each with bespoke Settlement code; port when the demonstrator closure compile reaches them.

stdio workspace check (renames-19, wasm --workspace --keep-going, rc 1): only 2 errors, both in semio-plugin-reactor-context (JobsWire::JobSource missing at plugin/reactor/context/🌉️call/🎭️exports/🦀️.rs:182); every stdio artifact depending on it is unreachable, so the real stdio sqlite error list is pending that fix. Trinity: off-closure, skipped.

## canonical-architecture run (renames-21) and stdio json-schema
renames-21 (repo run @semio-tech/mit-bestand-demonstrator:canonical-architecture via slot.sh) ran 2h25m: build:graph-plan verified 2840/2840 input closures (~15-35/min under fleet load), then 464262/464262 graph facts, then RESTARTED the input-closure verification at 886/2840: the graph was invalidated by concurrent source edits of the fleet. Killed by me on coordinator order (own tree only). Never reached Source admission, so the engine.json declaration remains unproven; rerun in the activation freeze window.
stdio json-schema (geojson mirror, 20 errors): FIXED, wasm check renames-22 rc 0 (receiving receive_decoding/encoding ported to RetainedWorkOutcome + 3-generic receive, trait impl signatures, binary pack encode signature). deflate-snapshot belongs to flow (claim withdrawn). Open for value lane: root semio-framework-value test closure is 39 files vs 32 cap (nativetests report); needs a crate split, not started.

## value root test-closure split (renames-23)
Census 37 -> 32 files (1105b-tools/valuesplit/census.mjs; label root2). Tests use private fields so they stayed in-crate; fixture consolidation (append close test + close/receive-work fixtures folded into append/work, shared-utf8 native fixture folded into admission fixture, schemas merged) and ⚠️refusal/🦀️.rs inlined. No new Cargo.toml.
DEBT: (1) zero margin 32/32, the next file breaks it; real fix = split value production modules into member crates. (2) inlined `pub mod refusal{..}` breaks one-🦀️.rs-per-member-dir taxonomy; restore ⚠️refusal/🦀️.rs once the split gives room.
PROOF: renames-24 (repo cargo test -p semio-framework-value --lib, native) rc 0, 7 passed 0 failed, log 🗑️generated/1105b/renames-24.txt. Source admission passed at 32 files. Also fixed a pre-existing never-compiled test bug: struct update `..Default::default()` on a Drop type (E0509) in the paged append close test, now field assignment.

DEBT (value): testing/🧾️wire/🦀️.rs physical_test_close_grant now a literal equal to os-store emission fixture physicalCloseGrant {1,4096,1048576,1048576,64}; add an os-store law pinning equality.
