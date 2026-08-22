# P8s Independent Remodel Streaming Final Audit

## Verdict

**REJECT — P1 test-evidence gap remains.** The repaired production structure closes the prior P8q/P8r P0 findings, but the JPEG rope regression still allows up to two complete input-equivalents of random byte reads. That is not strict enough to detect a duplicate full compressed-input pass, which is an explicit acceptance condition for this audit.

This was a fresh read-only source/static audit on 2026-08-22. I read the repository instructions and the complete P8e, P8q, and P8r reports including their repair dispositions; inspected current source and the scoped read-only diff; and did **not** run Cargo, builds, tests, descriptor generation, cache/target operations, ticket status/JSON operations, or modifying Git commands.

## Rejection

### P1 — JPEG Access Bound Still Permits A Duplicate Full Pass

`CompressedChunkRope::byte` records every JPEG random-byte access in the actual `JpgByteSource` implementation (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs:256-296`). The metric is therefore real, and its unit size is correctly one byte.

However, the sole JPEG regression accepts `random_byte_reads <= bytes.len() * 2` (`…/🖼️images/🦀️component.rs:1393-1414`, specifically the `<= bytes.len().saturating_mul(2)` assertion). A decoder that reads the valid compressed stream once and then makes one additional complete input pass can remain within that ceiling. The assertion does not establish an input-derived upper bound that distinguishes the intended access pattern from a duplicate full pass; it only excludes more than two input-equivalents.

This directly conflicts with P8r's stated repair disposition (“below two input passes”) and with this audit's required duplicate-pass detection. The PNG assertions are materially stricter: total sequential bytes are capped at one encoded input length (`…/🖼️images/🦀️component.rs:1323-1328`), and the active snapshot-to-`FrameIngestion` PNG test repeats that one-pass total-byte bound (`…/🚀️run-reconstruction/🦀️component.rs:1201-1227`). No equivalent JPEG ceiling is present.

## Rechecked P8r Blockers — Cleared In Source

- **Synchronous action bridge:** private `args_bridge::{field,text,number,flag,vec3,unknown,command_from_action}` are synchronous, and the framework-required outer method returns the bridge result without an await (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:205-255,527-529`). No direct Future/value use from the P8r finding remains.
- **Legacy promotion exclusion:** `CreateAsset` rejects `remodel-content:` payload handles (`…/🧷create-asset/🔺️diff/🦀️component.rs:48-49`) and has no magic commit-key branch. `ReplaceMeshResult` rejects `mesh-stage:` (`…/🧱replace-mesh-result/🔺️diff/🦀️component.rs:7-10`). The only direct arbitrary mesh commit helper is `#[cfg(test)]` (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:747-755`); the arbitrary test mesh façade is also test-only (`:904-921`).
- **Canonical terminal promotion:** `CommitReconstruction` parses only compact staged handles, materializes document-owned durable values before calling the single validate-all/apply cleanup function (`…/🏁commit-reconstruction/🔺️diff/🦀️component.rs:6-64`). That function locks both staging stores, validates every asset and mesh before removing any, then only removes after success (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:769-785`).
- **No active whole-input reassembly:** snapshot input decoding builds individually decoded `Arc<[u8]>` leaves under checked aggregate and 4-KiB ceilings (`…/🗿artifacts/📸️remodel/🦀️component.rs:277-304`). The active constructor sends those leaves straight into `CompressedChunkRope::from_leaves`, carrying identity and MIME separately (`…/🚀️run-reconstruction/🦀️component.rs:299-315`). The whole-value `remodel_asset` facade does reassemble a `Vec` and base64 string, but is not called by active reconstruction (`…/🗿artifacts/📸️remodel/🦀️component.rs:312-320`); its observed callers are export/UI/test-facing.
- **Rope implementation:** leaves reject empty or >4-KiB input and use checked aggregate accounting (`…/🖼️images/🦀️component.rs:226-250`); the PNG reader records actual read calls/bytes/largest unit (`:312-336`) and JPEG routes through the rope's byte source.
- **Replay/freshness/caps authoring:** active terminal turns produce one mutation, with the final turn producing one `CommitReconstruction` (`…/🚀️run-reconstruction/🦀️component.rs:737-753,930-944`). The authored public test exercises snapshot-owned leaf ingestion and real PNG decode (`:1201-1227`), and surrounding authored tests cover process-clear replay/checkpoint restoration, two documents, cancellation, stale delivery/ABA, 4-KiB chunk admission, aggregate limits, 512/512 mesh limits, 32-stage backpressure, and overflow paths. These are static observations only; none was run.

## Production-Reachable Helper Challenge

The old deduplication verification protocol remains production-reachable through `CreateAsset` verification keys (`…/🧷create-asset/🔺️diff/🦀️component.rs:16-20,32-36`) and the `verify_staged_remodel_*_chunk` helpers. It no longer promotes content: production's terminal commit removes staging and persists leaves only in the snapshot, while the only `committed` insertion is test-only. Consequently `remodel_{asset,mesh}_content_exists` cannot become true through the normal production path, making the reconstruction verification branches (`…/🚀️run-reconstruction/🦀️component.rs:836-868` and raster equivalent) inert in that path. This is not the P1 rejection above, but it is obsolete production-reachable surface and should be removed or justified; it cannot provide production deduplication after the process-cache removal.

## Required Unrun Gates

All remain **UNRUN** under the disk-space restriction:

1. Native debug: `cargo test -p semio-s-plugin-remodel --lib`.
2. Native release/timing: `cargo test --release -p semio-s-plugin-remodel --lib`.
3. Wasm compile: `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. Wasm lint: `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Runtime ActionBus replay, total process-clear/checkpoint restore, two-document isolation, cancellation, stale/ABA, capacity, malformed, and exact-cap cases.
6. Runtime PNG/JPEG maximum-envelope, malformed-entropy, bounded-progress, and duplicate-read instrumentation in debug and release.
7. Sanctioned descriptor regeneration/comparison for `cancelReconstruction` and `CommitReconstruction`.

The JPEG metric ceiling must be tightened to a defensible, input-derived bound that fails a second complete pass before the runtime gates can establish acceptance.

## Repair Disposition — 2026-08-22

The P8s rejection and production-reachable helper challenge are repaired in current source:

- **Strict JPEG evidence:** the positive real-JPEG rope fixture now requires cumulative random-byte accesses to satisfy a checked strict less-than 2 * input_len predicate, with maximum access unit exactly one and zero sequential join bytes. Its additional checked fixture ceiling models the owned decoder's monotone no-restart source pass plus the bounded SOF0 safety probe and is itself proven below the second-pass threshold.
- **Negative instrumentation:** explicit source fixtures cover zero input, the one-byte boundary, checked-multiplication overflow, an invalid multi-byte unit, and a simulated second complete input pass whose 2 * input_len access count is rejected. Counters are neither reset nor bypassed between admission and decode.
- **Obsolete dedup surface:** asset/mesh verification keys and parsers, CreateAsset verification effects, DeleteAsset verification handling, staged comparison helpers, process content-existence checks, verification cursors, verified-against state, and staging committed state are removed. Raster and mesh reconstruction no longer enter inert verification branches.
- **Authoritative durability:** terminal output continues through one CommitReconstruction, which copies bounded staged values into snapshot-owned durable artifacts before the existing two-store validate-all/apply cleanup transaction. The cross-thread mesh source fixture now publishes to and resolves from RemodelDurableArtifactStore; only arbitrary test meshes retain an isolated test-only convenience registry.

rustfmt --edition 2021 completed on the five touched Rust files. The interactivity verifier exited 0 with 775 bounded production rows out of 775, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch path, and 773 unique rows. Scoped static, obsolete-protocol, reassembly, cumulative-metrics, diff, and global-state scans found no remaining P8s production blocker.

Native debug/release Cargo tests and timing suites, runtime ActionBus/replay/process-clear/checkpoint/two-document/cancel/stale/ABA/capacity/malformed cases, Wasm check/clippy, descriptor regeneration/comparison, Cargo/build operations, cache/target operations, ticket metadata operations, and modifying Git commands remain explicitly **UNRUN**. This disposition requests independent source re-audit and does not claim runtime acceptance.
