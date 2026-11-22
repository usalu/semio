# EN 1998 example decode failure (`missing required field 'cases'`)

## Root cause
The EN 1998 artifact schema (`🫨️en1998/.../✳️any/🧬️schema/🔣️.json`, generated Rust `🧬️schema/🦀️.rs` `pub cases: Vec<CaseRow>`, TS reader `normWireRequired(normWireArray(parseCaseRow))`, proto field 23) lists `cases` as a required table, like EN 1990/1991/1992/1994/1995/1996/1997 and DIN 4108. All five authored EN 1998 DSL assets ended after `critical-regions [...]` and never declared the `cases` table. The decoder `decode_en1998_dsl` (used by the picker, descriptor and `setActiveExample`) therefore failed at the end of the 62-line `seismic-rc-frame` document (62:1) with `missing required field 'cases'`. The schema is right (the other families author `cases`), so the assets were fixed.

## Fix
Appended the empty `cases` table with the same header the other families use to all five hand-authored assets:
`cases [id:TEXT label-en:TEXT label-de:TEXT entry-id:TEXT numbers:TABLE texts:TABLE] {` / `}`.
No defaults, no migration, no schema change, no regeneration needed.

Also removed the leftover temporary probe `[DEBUG] FEM WIT poll semantic failure` (the one-shot `inspect_err` block in `poll_wit`); parentheses of the surrounding `with_entry(...)` expression were rebalanced (`})))?;` became `))?;`).

## Checks
- `check-example-required-fields.py` (ticket folder): for every norm family, compares required array-typed top-level schema fields with the table headers of every authored `🗣️.dsl.semio`. Before the fix EN 1998 was the only family flagged (`cases`, 5 files); after the fix: 0 files with missing required tables across all 15 families (66 assets). Other families (EN 1990, 1991, 1992, 1994, 1995, 1996, 1997, DIN 4108) already author `cases`; EN 1993, EN 1999, DIN 16798, DIN 18599, VDI 3805, ISO 16757 do not require it.
- Real-decoder check: the existing test `every_example_asset_decodes_to_its_code_built_snapshot` in `semio-s-artifact-norm-en1998-examples` covers the four non-showcase examples. NOT verified: two `repo test semio-s-artifact-norm-en1998-examples` runs failed in the repo resident service (13 to 37 minutes of project-graph reading, then service errors "Original immutable input closure bytes required" and "Inventory record content differs from its name", caused by the busy shared tree), not by test assertions. The reactor-context edit is also not compile-checked.
- Open risk to check on the next successful run: the committed binary twin `🖼️assets/🏢️seismic-rc-frame/🎒️.pack.semio` may predate `cases`; if `committed_pack_twin_matches_the_code_built_snapshot` fails, rerun with `SEMIO_UPDATE_GOLDEN=1`. The `seismic-showcase` asset is not covered by that Rust test.

## Changed files
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/{🏢️seismic-rc-frame,⚠️seismic-rc-frame-fail,🏢️seismic-multipart,⚠️seismic-multipart-fail,🏢️seismic-showcase}/<same name>/🗣️.dsl.semio`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` (probe removed)
- ticket: `check-example-required-fields.py`, this report

## Rebuild
Browser component `norm-eurocode` (hub eurocode extension that composes EN 1998; assets are `include_str!`ed). The probe removal touches the shared reactor-context guest, so other components only drop the debug print when rebuilt; none is functionally required.
