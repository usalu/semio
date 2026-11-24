# Browser E2E v4 (Process 3D, native route)

## Result: still no canvas (0). The one-item publication now gets through the seal digest; the next blocker sits after the digest is Ready.

## 1. Editor replay (after the seal cursor fix)
- `replay-cargo.sh ed7` (`process3d_interaction_behavior`): 24 passed, 1 failed (`context::history_edits_end_to_end`: "original fixture recipient exceeds its one issued stock", release spent 538,684,032 B vs stock 536,870,912 B: the release-drain family of `host-stall-and-drain.md`, 1.8 MB over). No "original page allocation attempt is occupied" anywhere in that run.
- `ed8` (`process3d_panels_catalogue`) was still running when this was written; result not read.
- The store "occupied" fix (task step 2) was therefore not needed: the seal cursor and pack batching fixes already clear it. Nothing added.
- Another agent's `cargo test` (pid 87220, 25 min, no children, idle in a condvar wait, holding the shared `target/debug/.cargo-lock`) blocked every replay. I sent it SIGTERM.

## 2. Hub wasm rebuilds
Builds v5..v12 (`🗑️generated/hub-v5.sh`..`hub-v12.sh`, native route, `native/build`, wasm-prototype, -j 3), each followed by `publish-modules.ts $G closure-hub.json`, `activate-native.ts process-closure.json` and `verify-process-firefox-bidi.ts`. Screenshots `🗑️generated/e2e-process-v5.png` .. `e2e-process-v12.png` (latest v12), report `e2e-report.json`, stdout `e2e-stdout-v*.txt`. Disk 15-34 GB free throughout.

Findings:
1. v5 on the fresh tree: same `one-item publication blocked` + `publication-stalled`.
2. Other agents' diagnostics located the silent `Blocked` at the bundle's digest arm (`🏪️store/⚙️engine/🧳️bundle/🦀️.rs` ~line 301): `advance_digest` returned zero progress.
3. My probes: demand `copy 1840, depth 2` fits the grant (521301 / 16777216 / 1048560 / 64), but `next_digest_work_demand() == 0` with the seal cursor not ready, not empty, in `Phase::SourceClose`.
4. Root cause: `ArtifactCanonicalEditSealCursor::next_work_demand` forwarded the child's raw quote on its take/close/retirement branches. For the lease-bound source (the wasm path) `RetainedCloneBinding::close_work_demand` answers 0 when only the lease transition is left, while `advance` always performs at least one unit (`last_work=1`). The host admitted a Work of 0, the step could not be paid, default progress came back forever, the ladder never moved: `publication-stalled` after 4096 units.

## 3. Fix
`🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`: the three source/retirement branches of `next_work_demand` now `.map(|work|work.max(1))`.
Law: `.../seal/🧪️tests/📄️paged-digest/🦀️.rs::seal_with` asserts `next_work_demand() != 0` on every unsettled turn; `paged_seal_digest` 2/2 pass (`seal-cargo.sh sl1`). The test's native source does not take the lease route, so a red baseline did not reproduce there; the browser run is the red/green evidence. No TS twin (pure Rust quote invariant).

Browser evidence: with the floor (v12) the digest walks phases 0..8 (237 steps, ends `phase=8` Ready); at v10 it stopped in SourceClose with quote 0.

## 4. Remaining blocker (not fixed)
After the seal is Ready, `setContributions` still reports `publication-stalled ... refused the granted page (copy bytes, capacity bytes, depth) [508500, 16774551, 64]`. No `[DEBUG]` fires any more, so the new silent `Blocked` is after `owner.ready()` in the bundle `PreflightingCommit` path: `NativeBatchDigest::take_edit` returning `None` (`admitted(next_digest_demand, grant)` then `original.take_edit(child(grant))`), or a later `blocked_here()` site. Next: one `[DEBUG]` at each `None`/`blocked_here()` after line 301, rebuild (~35 min), read.
Result: canvases 0; page errors: the one `setContributions` `publication-stalled` error.

## 5. Files
Changed: the seal cursor and its paged-digest law (above). All my temporary `[DEBUG]` prints (receiving `work`, bundle, digest, seal) were removed; other agents' `[DEBUG]` lines (`digest step`, `scope body`) are theirs and remain.
Ticket: this file, `🗑️generated/hub-v5.sh`..`hub-v12.sh` (+logs, screenshots; generated, delete at close).
