# Result fnsplit4b (function decomposition: `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand`)

Executor `fnsplit4b` (Sonnet 5.5), foreground, no sub-agents, no cargo, no `bun repo`, no modifying git command. `C` = this ticket folder. Stopped on the coordinator's TOTAL WRITE PAUSE: nothing outside `C/` was written after it; the last applied repo edit was complete (see "Open items" for the one prepared, not applied spec).

## Census (tool `C/fnsplit4b-tools/census.ts`, same as fnsplit2)

| scope (outermost functions) | start of run | end of run |
| --- | ---: | ---: |
| over 600 lines | 0 (one at exactly 600: cad vec `registerTests1`) | 0 |
| over 300 lines | 74 | 17 (see list at the end) |

Rust 60 of 74 started, TS 14. Whole-repo census at the end: 177 outermost over 300 (all roots), `fnsplit4a` owns `🧰️framework`.

## Method per shape (tools in `C/fnsplit4b-tools/`, snapshots in `orig/` and `orig/rs/`)

- Registration functions and module describes (TS): `ksplit2.cjs`, `dsplit.cjs` (kit factory, groups, describe parts). Fixes: `async function` exports, new `rootTail` spec key (module tail statements stay in the root), `stmts-mod.cjs`/`stmts-method.cjs`.
- Phase splits (TS): `psplit2.cjs`. Switch cases: new `switchsplit.cjs`. Class-method if-blocks: new `blocksplit.cjs`. Callback hoist helper: `hoist-decls.cjs` (not applied).
- Rust long `match`: `rsfn.py arms` with inline methods/free fns (`autoarms.py`, `armspec.py`; new options `inline`, `generics`, `prelude_ifs`, `wrap`, `call_suffix`, `call_wrap`, `flow_option`/`flow_wrap` for `return Ok(..)` arms via `Option`/`ControlFlow`).
- Rust statement runs: `rsfn.py apply` (new: per-phase `modes`, `drop_outs`, `try`, `prelude_ifs`, `allow(clippy::too_many_arguments, reason)` when more than 7 params), spec builder `phspec.py` with `keep: auto`; block-mode for loop bodies.
- Data matches: `match2table.py` (match to `const` table), `vec2chunks.py` (`vec![..]` to slice functions, token equality checked), `dedupe-blocks.py` (9 identical blocks to one helper), `chain2fns.py` (builder chains), `hoist-uses.py` (211 function-level `use` lines to module level), `hoist-nested-fn.py` (`ITEMS` hoists nested items), `ifchain.py` (if-else chain branches to functions).

## Functions decomposed (before -> root after)

TS: cad vec `registerTests1` 600 -> 13 (kit + 3 groups, `🧪️semio-tech-cad-js-core-vec/*`); box-length 497 -> 10 (kit + 2 groups); brepjs `registerTests1` 493 -> 9 (kit + suite + fixture + 4 parts); `proveGisMapProposalApprovalFixture` 422 -> ~70 (3 child folders in `🌎️hub/🧪️tests/🌍️gis-map-fixtures`); `proveInviteRedemptionTransaction` 329 -> ~90 (3 child folders); teaching `🧪️deploy` describe 367 -> ~49 (shared + 4 parts); react-deck describe 310 -> 12 (3 parts); `remodelingMutationOutcome` 301 -> 73 (26 case helpers, same file); `executeCommandDiff` 320 -> 128 (12 private methods, 5 hoisted consts).

Rust (all helpers in the same file unless noted): table conversions `vector` x2 (546, 369), `fixture_text` x2 (413, 395), `reject_vector_of` (313); `wire_probes` 322 -> 4 slices; energy `run` 533; draw `step` 518 and `turn` 470; puzzle5d `turn` 447; raster `step` 425 and `step` 331; process3d `turn` 382; fem3d `step_model` 406, `step` 395, `step_one` 366/333/322, `close_step` 328/311; gismap/generation2d/3d store-init `step`/`step_phase`; `apply_in_place` 366; mp4 `emit` 307; gltf `apply_structure` 311; `replacement` 324 (macro `replace_row!`); trinity owners test 443 (helper + 9 calls); `check_piles_walls_slopes_and_uplift` 528; `check_project` 579 -> 96 (+ `check_soil_layers`, `check_footings`, 3 footing phases); `check_dictionary_typing` 466; `check_part_2` 456 and `check_ports_primitives_and_spaces` 329; `check_full_actions` 362; `evaluate_zone` 355; `evaluate_beam` 497; `check_connection` 349; `check_uls_actions` 426 (24-param helpers); `check_load_cases` 331; `evaluate_structure` 329 (nested fn hoisted); `every_editable_leaf...` test 359 (nested items hoisted); `decode_entity_body` 433 and `decode_mleader_style_object` 351 (211 `use` hoisted); `decode_r2004_object_records` 539 (4 branch functions); fem/draw/hub `create_*_app` x3 (builder chains in 4 to 5 slices); `accept` x2 (generation2d/3d, 8 arms each).

## Verification (exact, all run from `/Users/ueli/Documents/semio`)

- TS registration traces (old vs new, stubbed module graph, `C/fnsplit4b-tools/run-reg.sh` + `cmp-reg.py`, `run-mod2.sh` for module describes): TRACE IDENTICAL for cad vec (64 events), box-length (28), brepjs (33), teaching deploy (27), react-deck (10).
- TS type differential `node tsdiff.cjs <root> <snapshot> <children>`: TSDIFF CLEAN for all 9 TS splits (cad vec, box-length, brepjs, gis-map-fixtures, invite, teaching deploy, react-deck, remodeling, wasm-engine). `bun build --no-bundle` on all 37 changed TS/TSX files: 0 failures.
- Function differentials for gis-map-fixtures and invite-redemption (`run-fn.sh`): identical outcome (both throw at the same first law because stubbed schemas are not available); this does not exercise the split bodies, so those two are verified by type differential only.
- Rust: `rustfmt --edition 2021 --emit stdout` parse of all 46 touched files: 0 failures (before the very last two accept splits: both parsed ok at apply time). `rscap.py` inline format capture audit per file: 0 problems. `ptrarg.py` applied to the new helper params (one unrelated pre-existing `replace_string` change reverted by hand).
- Rust name sweep `C/rs-sweep-tools/main.ts` (`RS_BARE=1 bun main.ts --scope <36 manifests>`): 66 findings in 3 crates, all in lines/files I did not edit and present in my pre-split snapshots (`semio-s-fem-analyses-assembly` imports, `semio-s-artifact-stdio-dwg-snapshot-drawing/geometry`, hub `client-contract` test). Zero findings in edited code. The accept splits were applied after this sweep (rustfmt + rscap only).
- Token equality: `vec2chunks` (wire_probes 6604 = 6604 tokens). `match2table`: arm counts 76/54/68/65/43 equal table entries; key and body text verbatim.
- Not run: cargo build/check/test, `bun repo`, any Rust test. All Rust claims are static. Points a compiler could still reject: `&mut` reborrows in extracted arm calls (draw `access`), `ControlFlow`/`Option` return wrapping of `return Ok(..)` (fem2d/fem3d `close_step`, fem2d `step`), closure-captured `doc` in gltf `apply_structure`, `Vector` now `#[derive(Clone, Copy)]` in 3 test files, `macro_rules! replace_row` definition order, `&String` kept in `check_object_id_and_spaces`.
- Path rule: new paths measured in UTF-16 units (<= 255) and components in bytes (<= 255) via `newcheck.py` (patched; the old byte-total check is stricter than the repo rule and refused 276-byte paths, which were renamed shorter). `tax.ts` taxonomy resolution could not run (missing `@semio-tech/framework-transient`).

## Tool defects found

- `rsfn.statements()` swallowed following statements after a nested `fn` item (hoisting a nested fn moved the rest of the function; caught by rustfmt, both files restored from snapshots and redone after the fix).
- `ksplit2` did not export `async function` module declarations and moved module-level tail statements into the kit; fixed.
- `reg-trace` could not import the repo graph (missing workspace packages) and bun maps `vitest` to `bun:test`; replaced by a stubbed graph walk (`mock.module`) and an `onLoad` rewrite of `"vitest"` to the recording shim.

## Open items

- Still over 300 (17): `registerInteractionE2eTests` 527 (one 386-line case table inside the describe), `step23` 453 (closure-heavy Playwright probe), `InteractionRepl` 447 (437-line JSX, aside is a 332-line sub-element; plan: `InteractionReplAside({ core })` with 52 names from `core`), plugin-coverage script callback 374, browser-actor `run` 308 (in-browser `page.evaluate`), `step_one` 504 (macros capturing `self`), `gis_map_abandoned...` test 474 and `gis_map_approval_committed...` test 311 (flat async tests, Arc locals), `serve_document_socket` 465, `verify_selected` 333 (async, 11 cross-phase outs), `from_native` 432 (closure `allocate`), `build` 413 (shared builders), `decode_table_record_object` 398 (six tuple-typed lets), `decode_r2010_header_section` 331 (inner macros, `&mut value.field` borrows), `encode_r2010_table_record_frame` 320, `print_content` 317.
- `encode_r2010_table_record_frame`: spec `fnsplit4b-tools/dwg-table-frame.json` (5 arms, `argexpr data/strings`) prepared and dry-run clean, NOT applied because of the write pause. Apply with `python3 fnsplit4b-tools/rsfn.py armsapply dwg-table-frame.json` after the lift.
- Second pass wanted: `check_uls_actions`, `check_load_cases`, `check_connection`, `evaluate_beam`, `check_footings` helpers carry 8 to 24 parameters (`allow(clippy::too_many_arguments, reason = ..)`); a context struct would be the clean form.
- Crate gate: no new files were added to any Rust crate (all Rust helpers are inline); lines per crate grew by 10 to 60. Not re-measured with the gate tool.
- Files edited by others during the run were skipped (`accept` x2 were skipped at first, then done when older than 15 minutes; `InteractionRepl` was edited at 11:04 and left).
