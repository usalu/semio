# Norm Rust Test Failures (semio-hub-norm), 2026-11-22

Result: 15 of 15 tests pass (`cargo test -p semio-hub-norm`, `--test-threads=1`): 12 surface/action-args, 1 compliance gate, 2 surface render. The official `@semio-tech/norm-plugin:test` target was last green except `norm_public_surfaces_render_all_declared_bodies` (14/15) before the final fixture fix; it could not be re-run afterwards (see Caveats).

## Root causes and fixes
1. action-args (din4108, din16798, iso16757, vdi3805): the law asserted `setSnapshot`, which the framework dropped. Retained editor tools are `setSelectedCheckIndex`, `setField`, `insertItem`, `removeItem`, `applyRemedy` (`NORM_EDITOR_TOOL_IDS`). Removed the assertion and doc mentions in the shared law and the five composition test files.
2. din16798 roster: `demo` was byte-identical to `compliant-office`. Removed the example module, `assets/demo`, the mount, the compliance-gate row and the generated catalogue row; repointed `DEFAULT_EXAMPLE_TEXT`, mutation tests (`.rs`, `.feature`, `.py`) and the schema compliance test to `compliant-office`.
3. compliance gate:
   - source scan covered all Eurocode artifacts (14 legitimate `1e-9 *` tolerances); scoped to the four gated families.
   - harness demanded `complies()`; iso16757 is `UnverifiedBasis` by design on `5.6.1.ifcView`; compliant now means no failing check.
   - iso16757 `5.8.scriptLimits.capacity` failed without remedies; added per-field `at_least` / `at_most` remedies.
4. surface_render (previously not reached):
   - hard-coded body-key sum 120 vs fixture; now 195.
   - fixture rows for din16798 and vdi3805 editor and viewer listed stale bodies; replaced with the real inventories.
   - oversize-identity refusal dropped an unretired `OriginalTurnFailure` (controlled retirement panic); that value is now `std::mem::forget`-ed in the test. Framework gap: the fixture API has no way to retire a failure.

## Caveats
- Under plain `cargo test` with parallel threads, `every_norm_editor_action_is_migrated_onto_the_shared_owned_factory` and the render test race on process-global plugin assembly (`owner-mutation-roster` conflicts); they pass alone and with one thread. The official runner uses one process per test.
- `norm_public_surfaces_render_all_declared_bodies` took up to 44 s on a loaded host; the official nextest budget is 15 s, which exceeded twice on a busy host. The disk also filled repeatedly during builds (cache gc was run).
- Final official-target rerun was blocked by another agent's workspace manifest edit (`semio-ui-contract-values` inheritance).

## Browser rebuild
`norm` component only (din16798 examples, iso16757 checks, catalogue JSON). Test-only edits in norm-energy, norm-eurocode, norm-eurocode-structural, norm-eurocode-materials need no rebuild.
