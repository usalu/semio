# CW8 — Shim removal + policy lints

Single agent, critical section. Prerequisite: CW7 complete — the `CollectionOperation` shape
migration (18 crates) and the broader `Operation`/`OpText`/`OperationDiff` import-path sweep
(~30 more crates, plus the kernel wave and vcs-plugin) are done. `framework/sync/rs` was already
fixed directly (its last 2 `vcs::Operation<P>` bounds → `protocol::Operation<P>`).

Read first: CW3's report shim list and `/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md`'s
CW8 row ("Shim removal (vcs re-exports), policy lints finalized, allowlists asserted empty,
dead-code sweep") and Part 3's "Policy lints" section (lines 98-102).

## 1. Verify the shim is truly dead, then delete it

`vcs/rs/lib.rs` has a region marked `//#region 🚧TEMPORARY protocol shim` (or similar — re-read
the file fresh to find its exact current marker and line range) that does `pub use protocol::{
Operation, OperationDiff, OpText, OperationMeta, Edit, merge_concurrent_diffs, ...};`.

Before deleting it, grep the ENTIRE repo (excluding `target/`, `.repo/`, `.claude/worktrees/`) for
real (non-comment) usage of `vcs::Operation`, `vcs::OpText`, `vcs::OperationDiff`,
`vcs::OperationMeta`, `vcs::Edit` (as a type/trait reference — `impl vcs::OpText for X`, `X:
vcs::Operation<P>`, `use vcs::{OpText, ...}`, etc., NOT doc-comment prose mentioning "vcs::OpText"
descriptively). As of this wave's start, the one known real remaining consumer is
`compose/client/lib/rs/lib.rs` (two `impl vcs::OpText for ...` blocks, around lines 7908 and
10858-10864 — re-check exact line numbers, the file may have moved). This is a PURE rename (OpText
is shim-identical between `vcs::` and `protocol::` — no shape change, unlike `CollectionOperation`)
— safe to do even though CW6's compose-hub agent separately, correctly deferred the much larger
"wire a real network client" work. Do NOT attempt any other compose changes — only the
`vcs::OpText` → `protocol::OpText` import/impl-path rename, nothing else in that file.

If your grep finds any other real (non-comment, non-worktree, non-archived-ticket-backup) consumer
beyond compose, migrate it the same pure-rename way if it's simple, or report it clearly and leave
the shim in place for that one case rather than forcing a risky change — shim removal must not
regress a currently-working crate.

Once the whole repo is confirmed clean (verify with the same grep again, zero real hits outside
`vcs/rs/lib.rs` itself and any legitimate historical comment text), delete the shim region from
`vcs/rs/lib.rs` entirely.

## 2. Policy lints (root `script.ts`)

Add, per the plan's Part 3 "Policy lints" section:
- `policyProtocolMigrationBreaches` — flags any remaining reference to `vcs::Operation`/
  `vcs::OpText`/moved `framework_core` types (the ones CW3 extracted: `HybridLogicalTimestamp`,
  `OperationEnvelope`, `OpDag`, `UndoPolicy`, `MergeStrategyKind`, the `HubProtocol` frame types)/
  `Hub*Frame`. Should be a zero-breach lint by the time you're done (you just proved the repo is
  clean in step 1) — its value going forward is regression prevention.
- `policyCommandEnvelopeCompletenessBreaches` — mirrors `policyPackCompletenessBreaches`'s shrinking-
  allowlist pattern: any file calling the pack-law asserts must also call
  `vcs::test_support::assert_command_envelope_round_trip` (added in CW7). Seed the allowlist with
  whatever's currently missing it (compute via the same `grep -rl` approach prior policy additions
  used), understanding this may be a non-trivial list — that's fine, it's meant to shrink over
  future work, not be empty on day one.
- `policyDbServerOnlyBreaches` — no `db` family Cargo dependency outside `db/`,
  `framework/product/os/hub/`, `compose/**/hub/**` (grep root Cargo.toml + every crate's Cargo.toml
  for a `db_*`/`db =` path dependency; the allow-list is directory-prefix based, not a file
  allowlist).
- `POLICY_ALWAYS_ALLOWED_DEP_PREFIXES` gains `"protocol/"` (apps now depend on it directly per
  CW7's import sweep — unlike `pack`/`db`, which stay reached only through `vcs`/`db::Database`
  respectively). Do NOT add `"db/"` to this list — db stays reached only through the hub servers
  and (behind a feature) `db_engine`, per `policyDbServerOnlyBreaches`'s whole point.

Register all three in the `policy` export next to the existing pack/protocol-completeness policies.

## 3. Dead-code sweep

Run `cargo build --workspace` and skim for `never used`/`never read`/`never constructed` warnings
specifically on anything that was part of the CW3-CW7 migration surface (leftover vcs-side
duplicate types/functions that are no longer referenced now that everything moved to `protocol::`
or was cleanly renamed). Do NOT do a repo-wide dead-code cleanup — scope this strictly to
migration-artifact dead code (e.g. if `vcs/rs/lib.rs` still has an old, now-unused private helper
that only the deleted shim's implementation used). If nothing migration-related shows up, say so
plainly rather than inventing cleanup work.

## Ownership discipline

Touch: `vcs/rs/lib.rs` (shim deletion), `compose/client/lib/rs/lib.rs` (the two `impl` sites only),
root `script.ts` (the three new policy functions + registration). Nothing else. Re-read every shared
file fresh immediately before editing (live concurrent repo). No git commands, no worktrees, no
AGENTS.md edits. Scratch/progress files in this ticket folder as `.txt`.

## Verify

`cargo build --workspace` clean. `cargo test -p vcs -p compose-hub` (and compose's own client crate
if its build lives outside the root workspace — check) pass. `bun ./script.ts lint` shows the three
new policies registered and firing correctly (zero breaches for `policyProtocolMigrationBreaches`
and `policyDbServerOnlyBreaches`; a real, computed allowlist for
`policyCommandEnvelopeCompletenessBreaches`).

## Report

Write `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw8-report.txt`: the
exact shim region deleted, every real consumer found and how it was handled, the three lints' exact
implementation and current breach counts, dead-code findings (or their absence), and full
build/test verification results.
