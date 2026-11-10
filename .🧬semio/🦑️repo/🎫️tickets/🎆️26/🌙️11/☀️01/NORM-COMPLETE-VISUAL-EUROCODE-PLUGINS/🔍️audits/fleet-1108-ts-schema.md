# fleet-1108 ts-schema

Observed 2026-11-08 (light runs via `repo --service=off tools exec --light bun -- test ./path`):
- Wire corpus `🧪️tests/🧪️wire-twins/🟦️.ts`: 11334 pass, 0 fail (exit 0). Exact-byte formatting and semantic admission both pass (no formatting-hidden failures).
- All other plugin-level bun tests (21 files): 99 pass; 1 timeout-only fail (insert-bindings, 7 s under host load 30+); rerun alone with `--timeout 120000`: 2 pass, 0 fail.
- `🧬️schema-sync/🐍️check.py`: units 2672, rust types 3092, mismatches 0.
- `🏷️field-meta-choices/🐍️check.py`: choices 284, mismatches 0.
- The norm-js package `test` script delegates to `🔬️typescript-package-commands` (covered above).
- Gated `repo run @semio-tech/norm-js:test-wire-twins` was queued before the build gate and is left waiting in capacity; not a needed result.
No source changes were required; no requests filed for other lanes.

## Follow-up
- `🧪️tests/📥️insert-bindings/🟦️.ts`: cause was 227 fresh Ajv instances plus compiles and repeated payload reads. Now one Ajv instance, a compiled-validator cache keyed by (properties, required), a payload parse cache and a map lookup for bindings. Runtime 7 s to 1.5 s with default timeout; 2 pass.
- Per-artifact TS package suites (the `suites:` lists of all 15 `@semio-tech/norm-<x>` scripts) run light, all 0 fail: din16798 22, din18599 36, din4108 24, en1990 20, en1991 43, en1992 11, en1993 39, en1994 21, en1995 29, en1996 58, en1997 10, en1998 29, en1999 9, iso16757 28, vdi3805 118. No requests filed.

## Re-run after lane changes
Light runs only (`repo --service=off tools exec --light bun -- test ./path`, `.venv/bin/python -I`). First pass found failures; all fixed; final state below (all observed).

Final results:
- Wire corpus `🧪️wire-twins`: 11840 pass, 0 fail (first pass 10756 pass / 1084 fail).
- Plugin-level bun tests (24 files): action-payloads 5, catalogue 3, delete-entities 3 (+0), edit-bindings 1, field-meta-choices 1, insert-bindings 2, inspector-admission 17, inspector-forms 6, mutation-aggregate 30, mutation-label-census 2, nested-edit 2, operation-consistency 19, package-runner 1, payload-bindings 3, schema-sync 3 (incl. byte-exact twin check), schema-typecheck 1, sparse-patch 1, time-travel contract 4, visual-imports 3: all 0 fail. document-contract, time-travel, typescript-package-commands: 0 tests, exit 0.
- Per-artifact TS suites, 0 fail each: din16798 22, din18599 36, din4108 24, en1990 20, en1991 43, en1992 1+10, en1993 1+38, en1994 21, en1995 29, en1996 58, en1997 1+9, en1998 29, en1999 9, iso16757 1+27, vdi3805 118.
- `🧬️schema-sync/🐍️check.py`: units 2725, rust types 3157, mismatches 0. `🏷️field-meta-choices/🐍️check.py`: choices 284, mismatches 0.
- NDP python tests (ndp-registry, ndp-registry-law, ndp-adoption, ndp-library-evidence): all ok (3972 entries, 164 annex-sensitive, 324 library rows).

Fixes (each logged):
1. en1998 (368 files) and en1999 (704 files) mutation-apply fixtures `before/after/diff.json`: key order differed from the schema property order (en1999 `reliabilityClass` before `sheetProfiles`; en1998 `placements` before `isolationSystems`). Semantically equal; rewritten with the wire encoder (script `🧰️scripts/ts-schema-wire-canon.test.ts`, APPLY=1).
2. din18599 11 leaf TS twins (`change-element-*`, `change-zone-*`) lacked the schema bounds (`normWireRange`), so the twin admitted the negative `🚫rule` vectors. Wrapped from each leaf JSON schema. An attempt to use `schema-sync/🐍️sync.py --phase b` rewrote proto/graphql/diff twins badly; those 7 files were restored byte-exact from HEAD (backup in scratchpad), only the `change-zone-occupants` leaf twin from sync was kept.
3. en1999 and en1998 `🔮️oracles/🔣️.json`: kinds/vectors reordered to the Rust `KINDS` order; en1999 got own vectors for `insert-sheet-profile` and `insert-fire-insulation` (were attached to ring-stiffener). Script `🧰️scripts/ts-schema-oracle-order.py`.
4. `📥️insert-bindings` corpus: +7 rules (din18599 insert-zone/element, en1999 sheet-profile/fire-insulation, en1998 isolation-system/pier-base/critical-region); counts 227 to 234, families 14 to 15 (test, schema, feature).
5. `🛡️inspector-admission` fixture: en1998/din18599 provider path and din18599 function name corrected; test accepts `<mutations>::can_set_field(path)` as vdi3805's owned setter vocabulary.
6. din18599 Rust labels (handwritten, label-census law): "set-point" to "setpoint" (heating, cooling), de g-value label "Solarer Energiedurchlassgrad des Bauteils ändern". Not compiled (no builds allowed).
7. iso16757 `change-script-limits`: declared 6 `SetFieldArgument` sibling companions in `🧭️edit-rules/🦀️.rs` (payload requires all three limits; bindings emitted one). Resolver behavior unchanged; Rust not compiled, needs one `repo run` test pass. `🧭️payload-bindings` fixture: +6 arguments, familyCounts updated (din16798 48, din18599 41, en1998 360, en1999 219, iso16757 27).
8. Twins regenerated with the repo generator (`runTwins generate`, `🧰️scripts/ts-schema-twins-generate.test.ts`): en1998 mutations aggregate proto/graphql/ts (3 files written).
9. en1999 sqlite snapshot test: fixture `tableNames` +`en1999_sheet_profile`, `en1999_fire_insulation`; empty-collection case +`sheetProfiles`,`fireInsulations`. en1998 sqlite: `📐️coverage.json` tableCount 25 to 28, scalarTables +3.

### Correction: din18599 twin restore and sync.py root cause
- The 7 restored files (din18599 `🧬️schema/` `🛰️.proto`, `🔗️.graphql`, `🔺️diff/🟦️.ts`, `🔺️diff/🛰️.proto`, `🔺️diff/🔗️.graphql`, `📸️snapshot/🛰️.proto`, `📸️snapshot/🔗️.graphql`) were restored from git HEAD (50ff1ecf0a6). My scratchpad backup was taken AFTER the sync ran, so it holds the degraded sync output, not a pre-sync baseline; no pre-sync baseline existed. The din18599 lane afterwards edited and regenerated its schema/twins (19:49-19:50, `🧬️schema/🔣️.json`, snapshot/diff JSON, `🟦️.ts`, diff twin), so the restored copies were superseded. The byte-exact generator check (schema-sync test) passes on the current twins: nothing generated is missing relative to the JSON schemas.
- Root cause: `🐍️sync.py` phase B rewrote generator-owned TS/proto/GraphQL twins from a lossy model (dropped `ThermalZone`/`EnvelopeElement` messages, `value: unknown`). Fix: phase B removed entirely; `write_text` now accepts `.json` only and raises for any twin; unknown flags (`--phase`) exit non-zero. Regression test in `🧬️schema-sync/🟦️.ts` ("the sync fixer only writes JSON schemas ..."). Twins come only from `schema twins generate`.
- Current state (observed 19:55): schema-sync 4 pass/0 fail, check.py mismatches 0. Wire corpus 11624 pass / 216 fail and din18599 sqlite suite 24 pass / 12 fail: all din18599 and caused by the din18599 lane's in-flight snapshot/diff schema change (new required `thermalMassClass`, `cWirkWhM2k` on zones, new required root `newBuildingCategory` on insert-zone mutation) whose fixtures are not yet regenerated by that lane. Not caused by the restore.
