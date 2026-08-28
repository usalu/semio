# Coordinator Native Actor and Canonical UI Review

Root read the complete retained reports and actual native output files for the runs below on 2026-08-27. Root did not start another Cargo process.

## Actor

The focused return-wire R2 gate executed four tests with 108 skipped, 0.148 s, exit 0. The initial exhaustive R4 run selected 112 but stopped at 61 passed, one failed and 50 not run. The failure was the existing replay fixture dropping a still-owned JobReplayLog at its strict guard; it was not a passing full regression.

R5 selected zero tests and earns no execution credit. After the fixture explicitly closed its empty replay log while preserving its original publication-refusal assertions, R6 executed the exact quick test: one passed, 111 skipped, 0.036 s. R7 then explicitly selected the exhaustive profile and actually passed **112/112, zero skipped, 0.618 s, exit 0**. This supersedes the R4 native Actor failure only. It does not implement native retained-return admission or prove consumed Wasm behavior.

Evidence: `📓️actor-return-wire-green-r2-native-2026-08-27.md`, `🧪️member-actor-return-wire-green-r2-native-2026-08-27.txt`, `📓️actor-return-full-r4-native-2026-08-27.md`, `🧪️member-actor-return-full-r4-native-2026-08-27.txt`, `📓️actor-replay-close-r5-r6-native-2026-08-27.md`, `🧪️member-actor-replay-close-green-r6-native-2026-08-27.txt`, `📓️actor-return-full-green-r7-native-2026-08-27.md` and `🧪️member-actor-return-full-r7-native-2026-08-27.txt`.

## Canonical Existing Components

R45 stopped before Cargo on the new fixture's schema-dialect mismatch; no native test credit. R46 actually passed both original existing-component laws, 104 skipped, 0.052 s. Its runtime logs show refusal before allocation (zero allocation, source unchanged), and a changed final byte requiring 35 turns with an allocation ledger of 32,768 bytes while the old source remains unchanged.

R47 actually passed two real reconciler laws, 104 skipped, 0.067 s. Its runtime logs show nine live SurfaceReconcileJobs with nine exact-root readers, all nine original roots still readable after owner close, then typed reader close. Replacement at grants 1, 64 and 4096 preserves original bytes and resident credit until the old reader closes.

These are scoped live native reconciler tests, not mocked phase-name tests. They do not yet prove the full resident-footprint census, original inline-census RED, paired transaction output collection, all refusal/unwind joins, full current runtime suite, Process budget fit, callback latency or browser consumption.

Evidence: `📓️runtime-canonical-existing-green-r45-r46-native-2026-08-27.md`, `🧪️member-runtime-canonical-join-r46-native-2026-08-27.txt`, `📓️runtime-canonical-jobs-green-r47-native-2026-08-27.md` and `🧪️member-runtime-canonical-jobs-r47-native-2026-08-27.txt`.

## Remaining Ownership Requirements

The new fixed shared output-entry pool must reserve before producer work and root seal. Saturation, refused publication and unwind must retain the exact transaction and ReadyPatch; fixed metadata and checked epochs must join physical accounting. The native producer census also identifies the existing semio_owned_poll_v1 JSON route: the same canonical retained result must replace whole variable output there as well as in WIT. Root has not approved a second ABI or an interim whole-copy result path.

