# EN 1998 example tests (2026-11-22)

## Results
- `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1998-examples --offline` (cwd: en1998 workspace), first run: 4 passed, 1 failed.
  - `every_example_asset_decodes_to_its_code_built_snapshot`: ok (confirms the `cases` fix in the DSL assets).
  - `committed_pack_twin_matches_the_code_built_snapshot`: FAILED, `missing required field 'cases'` while decoding the stale `seismic-rc-frame/🎒️.pack.semio` (Nov 20).
- Rerun with `SEMIO_UPDATE_GOLDEN=1`: twin rewritten (1152 -> 1158 bytes, +6 bytes, consistent with an added empty `cases` table), all 5 tests passed.
- Other `.pack.semio` goldens of this crate: only `seismic-rc-frame` has one.

## Added
- `showcase_asset_decodes` in the example test file: `seismic-showcase` has no code-built snapshot or Rust module, so the test decodes its DSL via `decode_en1998_dsl`. NOT yet compiled or run: three later runs aborted before cargo with a repo-tooling error (`Closed compiler builtin-read import coverage refused`, `redirectCapturedBuiltinReadsV1`), unrelated to this change (it appeared after the green runs, probably another agent editing repo tooling).
- Note: the showcase DSL got the empty `cases` table like the other four, and the decoder is the same one that passed for the other examples, so the test is expected to pass.

## Not done
- Other EN/DIN family example tests were not run (tooling error above; each run costs 10+ min on the busy tree). Static check from the earlier ticket report shows all authored DSL assets carry the required tables; committed `.pack.semio` goldens of other families may still be stale and are only detectable by their pack-twin tests.

## Rebuild
Browser component `norm-eurocode` needs rebuilding (assets are `include_str!`ed; DSL assets changed). The `.pack.semio` golden is test-only.
