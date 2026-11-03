# Spec-catalogue traceability, one format (wave 2, traceability agent)

Nothing was compiled or run except Python (`python -I`, jsonschema 4.26). All Rust is unverified by a compiler.

## Format (schema-first)
- Schema: `⚖️compliance/🧭️spec-trace/🧬️schema/🔣️.json` (draft-07). Document = `{code, standard, spec, specIds[], items[{id, clause, entity, realisation, note}]}`. Realisation is exactly one of `check {checks[]}` (check-id fragments, matched with `contains`), `data {path}`, `design-case {entry}` (catalogue entry run through the artifact `CaseCatalogue`), `not-modelled {reason >= 20 chars}`.
- Rust twin: `⚖️compliance/🧭️spec-trace/🦀️.rs` (`norm_compliance::spec_trace`): `Realisation`, `SpecTrace`, `SpecTraceTable`, `TraceCounts`, `Emitted`, `problems`, `assert_traced`, plus `standing(table, report)` (the generic form of the old EN 1998 SEI-54 overview). Unit tests in `🧪️tests/🔬️unit/🦀️.rs`.
- Shared test helper: `Emitted::default().with_checks(report ids).with_case_catalogue::<Cases>(annex)` then `assert_traced(&TABLE, &emitted)`. It fails on: spec id without row, row not in the spec, duplicates, any check fragment (every one, not just one per row) that no example document or design case emits, design-case entry that produces no check, empty data path, too short non-modelled reason.
- Per artifact: `✳️any/🧭️spec-trace/🔣️.json` (source of truth) and generated `🦀️.rs` (const `TABLE`, never edited). Generator and validator: `⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py` (`--write` regenerates; plain run = third-party jsonschema validation, spec order and completeness, fragment-in-sources check, stale-twin check). Both pass for all ten codes now.
- Editor: `app-surface` `spec_trace_table(&TABLE)` (columns spec id, clause, entity, realisation kind EN/DE, detail, note; table id `spec-trace`) appended in `reference_tables()` of all ten catalogue panels; the old `table-en1993-traceability` is gone.

## Mounts (all crates already depend on `semio-s-plugin-norm-compliance`; no Cargo.toml/Cargo.lock change)
`pub mod spec_trace` in: checks crate root of en1990, en1991, en1992, en1993, en1994, en1997, en1999; schema crate root of en1995, en1996; inferences crate root of en1998. Tests: en1990/en1991 `compliance-report`, en1993 `🗂️catalogue-trace`, en1994 inferences `🔬️unit`, en1995 checks `🧪️tests/🧭️spec-trace` (renamed from traceability), en1992/en1997 checks `🧪️tests/🧭️spec-trace` (new), en1996 schema `🧪️tests/🧭️spec-trace` (new), en1998 `⚖️compliance`, en1999 `resistance-worked-examples`.

## Counts per code (traced = row with a realisation; unlocated = not-modelled whose reason starts `unlocated:`)
| Code | Spec ids | Traced | check | data | design-case | not-modelled (reasoned) | unlocated | of check rows: matched by clause number |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| EN 1990 | 29 | 29 | 26 | 0 | 0 | 3 | 0 | 0 |
| EN 1991 | 60 | 60 | 36 | 0 | 24 | 0 | 0 | 0 |
| EN 1992 | 156 | 156 | 0 | 0 | 156 | 0 | 0 | 0 |
| EN 1993 | 142 | 142 | 129 | 5 | 0 | 8 | 0 | 0 |
| EN 1994 | 54 | 52 | 51 | 1 | 0 | 0 | 2 | 45 |
| EN 1995 | 68 | 68 | 27 | 0 | 41 | 0 | 0 | 0 |
| EN 1996 | 37 | 37 | 0 | 0 | 37 | 0 | 0 | 0 |
| EN 1997 | 42 | 42 | 0 | 0 | 42 | 0 | 0 | 0 |
| EN 1998 | 26 | 26 | 22 | 4 | 0 | 0 | 0 | 1 |
| EN 1999 | 42 | 40 | 38 | 1 | 0 | 1 | 2 | 15 |
| Total | 656 | 652 | 329 | 11 | 300 | 12 | 4 | 61 |

Untraced (genuinely unlocated, owner to resolve): EN 1994 C-11 (stress-block integrator, support for C-07/C-08/C-12) and C-44 (fire framework F:2); EN 1999 A-24 (HAZ torsional buckling, 6.3.3.3) and A-51 (temperature of unprotected members, 1-2:4.2.5). EN 1995 has 68 ids (T-98 added to the id list: the coverage audit extraction missed it; the registry lists it).

## Migration of the five variants (old modules deleted, no aliases)
- EN 1993: `catalogue::ENTRIES` (142 rows) merged with the v4 JSON (87 rows). Richer realisation per id: check prefixes are the union of both (de-duplicated ignoring a trailing dot); where one side was Data/NotModelled and the other a check family, the check wins and the other becomes the note (V-040, V-050, V-075, V-102, V-113a, V-125, V-126, V-250, V-253, V-363/364/367/369/370/374, V-376, V-402, V-407, V-411, V-452, V-500, V-502, V-600, V-604, V-800/802/803, V-851, V-860, V-880 etc.); data/not-modelled kept where v4 said open/kernel (V-001, V-014, V-015, V-366, V-601, V-504..506, V-609, V-805). Deleted: `catalogue` module in `📏️parts/🦀️.rs`, scripts `en1993-spec-trace.py` and `en1993-vids-entries.py`; `V-` test `the_catalogue_lists_the_142_ids...` folded into the helper.
- EN 1999: `catalogue::ENTRIES` (22 rows) kept verbatim as check/data rows, 20 more ids added (15 by clause number, 3 by reading the check ids, A-81 not modelled because the spec itself excludes it, 2 unlocated). Module deleted from `📏️parts/🦀️.rs`; checks root got the mount.
- EN 1995: `TRACE` (Member/Connection -> check, Case -> design-case entry with entity `cases`) migrated; `⚖️checks/🧭️traceability` and its test removed; the case-run test and registry-subset test kept.
- EN 1990: `SPEC_TRACE` + `SPEC_OPEN` migrated (3 open ids became not-modelled with reasons). EN 1991: `TRACE` rows kept with their clause strings; 24 further ids are design-case rows (registry entries). EN 1998: `SPEC_TRACE` migrated; names/de labels dropped (the schema has no per-row labels), `spec_overview` replaced by the generic `standing`; SEI-06 now `.timeHistory.` checks (the test in the file already required them), 14 more ids mapped by reading the check literals (kernel spectrum/modal functions as data rows).
- New traces: EN 1992 (156 registry entries as design-case rows), EN 1996 (37), EN 1997 (42) from the case registries; EN 1994 (54) by clause match against the check-id families plus 7 manual rows. Spec ids and clauses come from `📐️specs` (same extraction as the coverage audit; `🧰️scripts/spec-trace/{spec_ids,migrate}.py`, one-shot).

## Open items / unverified
- 61 check rows are traced by clause number (marked in `note`): the family exists in code, but the owner must confirm it realises that id. Mostly EN 1994 (45) and EN 1999 (15).
- `assert_traced` is strict; it will fail at first run for any fragment that the listed example documents do not emit (en1999 uses only the three documents the old test used, en1994 five snapshot constructors, en1998 one document). Fix by adding a document or correcting the row. Dynamic ids (`head("c90","6.1.5",...)`) are accepted by the static checker through key plus clause.
- The legacy sources of the migrated variants are deleted, so `migrate.py` cannot be re-run for EN 1990/91/93/95/98/99; edit the JSON and run the generator.
- EN 1996 `masonry` crate is already over the line budget (4134) independent of this work.
- Entities on design-case rows are `cases` (assumed snapshot collection of the case bridge); entities exist only for EN 1993 and EN 1995, others are empty.
- Appended `- ready for rebuild: traceability ...` to `🗑️generated/build-en1990..1999/latest.md`; announcements in `📌️important/requests.md`.
