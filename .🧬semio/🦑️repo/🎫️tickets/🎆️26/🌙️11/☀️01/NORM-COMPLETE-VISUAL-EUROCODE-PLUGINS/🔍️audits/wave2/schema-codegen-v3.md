# Schema codegen v3 (wave 2): canonical JSON layout, no Default on snapshots

No cargo, no `repo run`, no git write. All `schema twins` runs through `heavy.sh`.

## Result
- `schema twins check --strict` repo-wide: identical=7568 style=0 drift=0 (json 2394, typescript 2389, protobuf 205, graphql 205, rust 2375).
- `⚖️compliance/🧪️tests/🧱️source-budget`: 0 crates over (was 4: en1998-diff, en1998-model-json-schema, en1991-schema, en1991-model-json-schema); 15 above 85 % (unchanged families).
- Layout test `python -I 🧬️twin-projection/🧪️tests/🗜️schema-layout/🐍️.py`: 6 vectors, 0 failures (bun not on this host; the TS twin of the test is `🟦️.ts` beside it).

## Root cause (differs from the request wording)
The generator never wrote a pretty "JSON twin": the embedded JSON is the schema document `🔣️.json` itself (`FacetLeaves.json_schema = include_str!`). Its layout came from the writer: `annotate`/`serializeSchema` knew only "exactly compact" or "two-space indent" and turned any other layout (H's width-320 reflow) into indent; Python writers kept the file layout. So large schemas (en1998 diff 4969 lines, snapshots 5346) were indented and H's reflow was undone by the next writer. `generate` and `check` did not look at the JSON at all.

## Decision: fix the layout at the source, do not relax the gate
- The gate is the kernel's real admission limit (`readRustUnitSourceClosureV1`: 32 files / 262144 B / 4000 active lines, `include_str!` inputs counted, error "Rust crate Source boundary exceeded before compiler"). Excluding generated twins in `source-budget` would only hide builds that the kernel refuses. Rejected.
- The JSON is source, humans read it, so most schemas stay indented (2521 files, zero churn). Only a document whose indented text exceeds 1500 lines goes compact (one line per top-level key and per definition, the layout the 114 diff/aggregate schemas already had). 1500 keeps module closure (json + 4 twins, ratio about 1.6) under about 2600 lines. A compact file stays compact.
- Layout is now a pure, idempotent function of the document: `serializeSchema` + `SCHEMA_INDENT_LINES` in `🧰️framework/…/🧬️schema/🧬️twin-projection/🧩️model/🟦️.ts`.
- New facet `json` in the twin projection (`🟦️.ts`): `generate` writes it, `check --strict` compares to it (equivalence `⚖️equivalence`: parsed value equal = style, never drift), `--only json` supported, report prints `style <path>` lines. No separate compaction step; writers may leave any layout, the next `generate` normalises it.

## NO-DEFAULTS in the generator
- The Rust generator emits exactly the `x-semio-rust.derive` list of the schema (harvested by `annotate`). New rule in `🦀️rust/🟦️.ts`: a record of a `📸️snapshot` module that derives `Default` is a `problem` (generate/check exit 1): "snapshot entities are constructed explicitly". Diff, patch and inference roots keep `Default` (empty value = no change; En1990Diff::default() 23 uses, `..Default::default()` in every mutation diff).
- Grep first: the 8 en1994 snapshot rows (CharacteristicAction, SlabAction, ProfiledSheeting, HeadedStuds, CompositeBeam, CompositeColumn, CompositeSlab, ColumnAction) have 0 consumers; `Default` removed from their `x-semio-rust.derive` and the Rust regenerated. en1990 `BridgeSls` was used with `..Default::default()` in 3 test literals (`🎬️scene/🧪️tests/🔬️unit` x2, `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report` x1): now spell all 16 fields, `Default` removed from the en1990 snapshot schema. The DSL/value derives do not need `Default` of the field types (other snapshot rows already compile without it).

## Regeneration
Repo-wide state was already consistent when I finished (`freeze.txt` named only build-en1992 until 07:53+, then was removed). Observed: a concurrent repo-wide `generate` by another agent (08:01:38, ~55 schema files plus the Rust snapshots) applied the new code, including en1992 files (`🧬️schema/🔣️.json`, snapshot, mutations, 14 leaf schemas) while build-en1992 was in graph-plan. I only ran `check` and `generate --only json --out <scratch>` (scratch deleted). en1992 may need a graph-plan restart; nothing for me to redo.
Changed schema layouts (55): en1998 diff 4969 -> 125 lines (102 KB -> 50 KB), en1998 snapshot 5346 -> 51 (153 KB -> 91 KB), en1992/en1996 mutation aggregates and snapshots, spec-trace/design-case/oracle schemas (hybrid H layout -> indent), a few leaf schemas (number literals).

## Deleted / updated
- Deleted: `🧰️scripts/crate-budget/h_compact_json.py`, `h_json_in_crates.py` (its only driver), their rows in `🧰️scripts/README-generators.md`.
- Updated: `📌️important/schema-first.md` (section "JSON layout (canonical, generated)"), `🧰️scripts/field-adder/schema_twins.py` docstring, `📌️important/requests.md` (done line), `- ready for rebuild: codegen v3` in `build-en1990..en1999/latest.md`.
- Added: `🧬️twin-projection/🧪️tests/🗜️schema-layout/{🔣️.json,🟦️.ts,🐍️.py,🥒️.feature}` (language-agnostic vectors; Python `json` as the third-party witness).

## Open / unverified
- bun test run and Rust compilation of the regenerated snapshots (rebuild requested).
- Python tool `schema_twins.py` still writes its own layout until the next `generate`; its compact dump equals `compactLayout` (verified by the shared vectors only for the layout test, not run through the field adder).
- 15 crates remain above 85 % (mut-* at 28 files, scene crates, en1993-model 92 %); not a codegen matter.
