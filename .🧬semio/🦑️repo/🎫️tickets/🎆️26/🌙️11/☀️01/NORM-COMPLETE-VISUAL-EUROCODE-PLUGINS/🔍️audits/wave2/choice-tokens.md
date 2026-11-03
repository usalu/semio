# Choice tokens (wave 2)

## Tooling (norm `🧪️tests/🏷️field-meta-choices/`)
- `🐍️choices.py` shared reader; `🐍️generate.py [--artifact K] [--skip K] [--dry]` rewrites the body of every `NormFieldChoice` table whose fields edit a schema enum (tokens plus EN/DE labels from `x-semio-ui.options`; old label reused by normalised token when an enum has no options). Tables shared by fields with different enums are kept.
- `🐍️check.py` (+ bun wrapper `🟦️.ts`): every choice token is an enum member, every member is offered, order equals enum, labels equal the schema options. Current: `choices=267 mismatches=1` (only en1994 `ANNEX` labels, frozen).
- Limitation: the generator is a Python norm tool, not yet a `schema twins` projection (no Rust build to validate a TS-engine change); 100+ fields in en1993/en1994/en1998/din4108/vdi are plain `string` in the schema (no enum), so they cannot be derived or checked; fix by giving those schema fields enums.

## Mismatches fixed (wire tokens are PascalCase/camelCase of the schema)
- en1995: `not-imposed`, `snow-high` -> `notImposed`, `snowHigh` (+ labels).
- en1996: 11 tables lowercase/camel vs Pascal wire (`LoadBearing`, `Class1`, `Clay`, `Group1`, `GeneralPurpose`, `WallTie`, ... `Seismic`); `annex`/RC labels. Schema drift fixed: snapshot property `designSituation` lacked `Seismic` (the `$defs` and Rust enum had it): JSON, snapshot/diff TS twins patched by hand (generator not run). Load-case situation is a lowercase free string: new table `LOAD_CASE_SITUATION`. Compliance perturbation test used lowercase tokens; now Pascal.
- en1992: 17 kebab/lowercase tables (`flat-slab`, `xc1`, `simply-supported`, `u-hook`, `self-weight`, ...) now schema-derived; `fire.slabSystem` lost `flat`/`ribbed` (not enum members).
- din4108 `climateZone`, iso16757 (7 tables, camelCase vs Pascal; unit test tokens updated), en1998 (`DE_GROUND`, `ROLE`, `RC`, `BASIS`), en1990/din18599 labels.
- Verified: en1997, en1999, en1990 tokens already fine. No other code used the old tokens (grep).

## en1990 item 20 (inferences dev cycle)
`-inferences` no longer dev-depends on `-io`, `-examples`, `-schema` (only the compliance-report test used examples/io). The test now mounts from the root crate (`⚖️en1990/🦀️.rs` `mod inferences_compliance_report`, same convention as en1991/en1994/vdi3805), imports explicitly instead of `crate::*`; root `Cargo.toml` dev-deps gained checks, examples, app-surface, test-fixtures, compliance `compliance-testing`. Unbuilt: possible glob ambiguity or missing import on first compile.

## Pending
- en1994: frozen (build-en1994 graph-plan); run `generate.py --artifact en1994` (ANNEX labels) once freeze.txt clears.
- Rebuild requests appended to `build-{en1990,en1992,en1995,en1996,en1998}/latest.md`; din4108/iso16757 have no build lane.
