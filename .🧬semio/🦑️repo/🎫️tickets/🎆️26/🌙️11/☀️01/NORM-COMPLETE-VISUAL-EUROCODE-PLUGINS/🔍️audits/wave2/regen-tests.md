# REGEN-TESTS (wave 2)
Nothing compiled or run (no cargo). All edits are test code plus the shared helper; unverified by the compiler.

- Pack-name grep over all ten artifacts: 29 committed packs are all `🎒️.pack.semio`; the only writers of `📦️.pack.semio` were the en1991 and en1997 regen tests, now replaced. The helper additionally fails on any `*pack.semio` not named `🎒️.pack.semio`, so duplicates cannot appear.
- Shared helper: `semio_s_plugin_norm_compliance::regen` (`NORM/⚖️compliance/🧪️tests/🧬️regen/🦀️.rs`, mounted for `test`, `test-support`, `compliance-testing`): `blessing()`, `bless_or_assert`, `bless_dsl_and_pack_assets`, `SemanticExtent`, `semantic_contract_text`, `bless_semantic_contract`. Single switch `SEMIO_UPDATE_GOLDEN=1`.
- Pack/DSL regen test `regen_dsl_and_pack_assets_match_the_codecs_or_bless`: added to en1990/92/93/94/96 (editor assets test file); en1991 and en1997 old tests rewritten in place. Rewrites canonical DSL and the pack beside it (only where a pack is committed); asserts equality without the switch. `.op/.spr/.cmd` are protocol fixtures, not codec outputs, and are not generated.
- Semantic contract regen test: en1990/91/93/94/95/96/97/99 (emits identical json.dumps(indent=2) format, checked against the committed files). en1992 and en1998 have no `semantic.json` and no independent-extent test: nothing to regen (documented, left to owners).
- Cargo: en1990 root crate got dev-dep `semio-s-plugin-norm-compliance` (test-support). No Cargo.lock entry exists for these packages.
- Docs: `post-build.md` row (c) rewritten, stale known-risk removed; `post-build.sh` `bless_env` drops the en1991/en1997 env; `requests.md` announced; "- ready for rebuild: regen tests" appended to build-en1990..en1999 latest.md.
- Caveats: after a bless the semantic.json needs a second run (existing tests `include_str!` it); a stale pack now fails the ordinary test run (intended).
