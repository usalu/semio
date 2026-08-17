# W2a — `semio`/`v1`/`cad` Subset — Report (written by the W2a closer, backfilling a missing agent report)

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/**` only.

## Why this file exists

The `cad` subset agent finished real implementation work but never wrote `w2a-cad-report.md` —
`w2a-verify-report.md` flagged this as a process gap ("cad's missing `w2a-cad-report.md` should be
written... the underlying work is real and passes, but the process gap... should not repeat").
This file backfills it from direct inspection of the subset's files (`git status`, `cargo test`),
matching the verifier's own independent findings — it does not add new work to `cad`.

## What is implemented (confirmed by direct inspection)

- **DIALECT/WRITES**: `Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset:
  SubsetId("cad") }` (`🎹️composer/🦀️component.rs:13`) — correctly wired, matches the `✳️cad/` path.
- **Diff** (`🧬️schema/🔺️diff/🦀️component.rs`): hand-rolled sparse diff with a real
  `impl protocol::DiffCodec for SemioCadDiff` (line 594) — no apply-and-capture, no
  `snapshot: Option<...>` full-replace escape hatch.
- **SubsetValidator**: real referential-invariant checks (`cad_referential_diagnostics` —
  dangling layer refs, dangling/self-referential block-insert refs), exercised by
  `validator_accepts_a_fully_referenced_snapshot`,
  `validator_flags_dangling_layer_and_dangling_block_insert`, and
  `validator_flags_self_referential_block_insert`.
- **Files touched**: 62 files under `✳️cad/**` (`git status --porcelain`), all within scope — no
  cross-subset or shared-file writes.

## Verification (re-run by the W2a closer, not self-reported)

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::cad"
```

```
running 13 tests
test artifacts::semio::standards::v1::subsets::cad::schema::diff::component::tests::between_roundtrip_law ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::mutations::component::tests::mutation_diff_law ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::diff::component::tests::field_sweep ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::mutations::component::tests::inverse_law ... ok
test artifacts::semio::standards::v1::subsets::cad::composer::tests::validator_accepts_a_fully_referenced_snapshot ... ok
test artifacts::semio::standards::v1::subsets::cad::composer::tests::validator_flags_dangling_layer_and_dangling_block_insert ... ok
test artifacts::semio::standards::v1::subsets::cad::composer::tests::validator_flags_self_referential_block_insert ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::diff::component::tests::absorb_law ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::snapshot::component::tests::json_pack_round_trips ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::snapshot::component::tests::dsl_text_round_trips ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::mutations::component::tests::op_text_binary_roundtrip_law ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::diff::component::tests::diff_codec_text_binary_roundtrip_law ... ok
test artifacts::semio::standards::v1::subsets::cad::schema::snapshot::component::tests::codec_retention_law ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 1485 filtered out; finished in 0.03s
```

**Policy**: filtered to `✳️cad`, 2 breaches — `taxonomy/emoji-prefix` on the (pre-existing, W1b-seeded)
`📄set-snapshot` triad dir name and `os-state-authority/item-scope-global` on the composer's
`VALIDATOR_ENTRY: OnceLock<...>` — both the exact same systemic pattern shared identically by every
other semio v1 subset's composer (see `w2a-verify-report.md` and `STATUS.md` for the repo-wide
disposition). Zero breaches attributable to `cad`'s own real implementation work.

`📜️script.ts`'s `POLICY_DIFF_COMPLETENESS_ALLOWLIST` no longer carries a `cad` entry — removed by
the W2a closer as part of this same ticket, confirmed satisfied (`impl protocol::DiffCodec for
SemioCadDiff` at `🔺️diff/🦀️component.rs:594`).
