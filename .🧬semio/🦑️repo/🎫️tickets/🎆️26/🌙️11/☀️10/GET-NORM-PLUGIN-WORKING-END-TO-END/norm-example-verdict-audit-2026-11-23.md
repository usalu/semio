# Norm Example Verdict Audit, 2026-11-23

Read-only pass over the 15 family example crates. Editor trees were left out. Seventeen example packages: one per family, plus `semio-s-artifact-norm-vdi3805-examples-failing` and `semio-s-artifact-norm-vdi3805-failing-blatt-4-to-19`. No `[DEBUG]` string remains in those example tests.

A real verdict asserts `failing()` empty or non-empty, a named check id, or a remedy. `inference_determinism_law` and `inference_default_law` are equality-only and sit beside a verdict test.

Every primary asset has a verdict test except the one fixed in this follow-up. `failing-prestressed-beam` only asserted `!complies()` plus a prestress field. An unverified NDP basis makes `complies()` false even when `failing()` is empty, which the compliant prestressed sibling already documents. That test is now `primary_asset_fails_and_every_failure_has_a_remedy`.

Packs older than the spoken DSL, so a later rebuild must pick them up only when the core is older than the asset:

- EN 1992 office frame DSL 2026-11-22 22:42, pack 2026-11-20 01:18. The structural core is 2026-11-22 10:21, so the DSL is newer than the core.
- EN 1999 purlin DSL 2026-11-23 02:43, pack 2026-11-20 01:08, 1246 bytes. The materials core being built started after the DSL change and still embeds the old pack until `REGEN_EN1999_ASSETS=1` runs.

DIN 4108, DIN 16798, ISO 16757, EN 1991, EN 1994, EN 1995, and EN 1996 packs are older than the cores that already published after those DSL files.
