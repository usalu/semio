# No defaults v2 (wave 2, 2026-11-03)

Nothing compiled; verified by reading and a dependency script. Script: `🧰️scripts/no-defaults-v2.py <n> <folder>`.

## Done: En1992, En1993, En1995, En1996, En1997, En1998, En1999
- `impl Default for EnNSnapshot` removed. New `EnNSnapshot::example(annex)` (`AnnexChoice`; en1998 keeps its string annex: `example(&str)`), built from the named reference constructor with the annex field set explicitly. en1997: `impl En1997Snapshot { example }` over `compliant_demo()`.
- Call sites (tests, demos, editors, viewers, diff tests, benches, oracle helpers) now `EnNSnapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)` (en1998: `example("de")`): 1992: 33, 1993: 25, 1995: 28, 1996: 24, 1997: 27, 1998: 32, 1999: 28.
- `norm_latency_bench!` in each editor passes `seed = <model>::EnNSnapshot::example(..)` (default arm needed `Snapshot: Default`). `assert_case_edit_paths` seed in case-edits (1992, 1995, 1996, 1997) is the explicit example.
- Cargo: `semio-s-plugin-norm-compliance` added where a crate now names `AnnexChoice` (viewers of 1992/1995/1996/1997/1999 as normal dependency; examples of 1992/1993/1997 as dev-dependency). Verified for every touched crate with a script.
- Real compile bug fixed in all of en1990..en1999 (except en1994, below): `#[derive(Clone, Debug, Default)]` on `EnNBuilderConstruction` required `EnNSnapshot: Default`; `Default` dropped (the builder has an explicit `empty()`).
- `unwrap_or_default()` reviewed: remaining uses are empty `Vec`/`String`/ids for "none" (inverse ops, sqlite child rows, point lists, fire-table row lists, remedy lists), no normative or user choice. Only one to watch: en1996 scene `annex_e::basis_id(&class).unwrap_or_default()` (label id, empty when the class has no basis; display only) and en1998 `resolve_annex(..).map(ndp_ids).unwrap_or_default()` (no NDPs when the annex cannot be resolved; the outer check reports the annex problem).

## Deferred: En1994 (build-en1994 frozen since 03:24 at the time of this report)
Remaining: remove `impl Default for En1994Snapshot` (explicit `example(annex)` taking the body, `annex` set from the argument), rewrite `En1994Snapshot::default()` call sites (92 files/lines), `norm_latency_bench!` seed, case-edits seed, `Default` derive on `En1994BuilderConstruction`, viewer/examples Cargo dependency on norm-compliance. Not started to avoid restarting the graph-plan.

## Notes
- Cargo.lock does not list the en199x crates; no lock edit.
- Ready-for-rebuild lines appended to `🗑️generated/build-en{1992,1993,1995,1996,1997,1998,1999}/latest.md`.
