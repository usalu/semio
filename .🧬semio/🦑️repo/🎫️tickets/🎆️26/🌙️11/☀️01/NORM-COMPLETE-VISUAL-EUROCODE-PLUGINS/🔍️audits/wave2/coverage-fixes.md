# Coverage fixes (wave 2)

Nothing was compiled or run except Python. All Rust is unverified by a compiler.

## 1. EN 1999 A-20 / A-21
- A-20 (flexural buckling, 6.3.1.1) is realised by `en1999.6.3.1.fb` (axis y) and `en1999.6.3.1.fb.z`; trace unchanged, note added.
- A-21 (welded members, HAZ factors kappa and omega_x) had no check of its own. New check `en1999.6.3.1.haz` in `🧱️member-checks/🦀️.rs` (clause 6.3.1.1, N_Ed against N_b,Rd of the governing axis, with kappa, kappa_0 and omega_x in the explanation, load remedies). It is emitted only for members with a welded element or a transverse weld.
- Trace JSON `🧭️spec-trace/🔣️.json` edited, `🦀️.rs` twin regenerated (generator `write`; jsonschema is not installed here, so no schema validation ran).
- Test: `every_spec_id_is_traced_to_a_check_family_an_example_emits` (weld-runs-and-outlines) gets a `welded` document (compliant roof purlin with `transverse_weld = true`, offset 0.75 m).
- Open: oracle for the new check (no independent value for kappa/omega_x; the existing worked example `transverse_weld_and_tig_reduce_the_resistances` covers the same factors).

## 2. Composed check ids
`assert_traced` is already wired in all ten artifacts (EN 1990 to EN 1999) and evaluates the example documents and design cases (`Emitted::with_checks` and `with_case_catalogue`); fragments are compared with `contains` against emitted ids, so `format!`/`head`/`mk`/`base` ids are verified at test time. Reviewed the document lists of the composed groups:
- EN 1990 e10/e11/e12 (`extended`, `analysed` documents) and EN 1991 `dyn-frequency`, `section-gradient`, `section-expansion` (`wave_3_document` with dynamic elements and thermal sections): documents present.
- EN 1995 T-nn (examples + `compliant_building_beam`, `noncompliant_building`, `catalogue_demo_document`, plus design cases) and EN 1999 `1-4.bend|defl` (`sheet-pass` in `wave5_document`): documents present.
- Added document: EN 1999 `welded` (for the new A-21 check). No other document was found missing by reading; whether every fragment is emitted is decided by the first test run.
- The static audit's 4 defects (E90.06/E90.17/E90.18 `6.11b`/`6.12b` vs key `e611b`) are outside this task; the EN 1990 test already checks those fragments separately.

## 3. EN 1997 JSON demo
- `🌍️en1997/.../✳️any/🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json`: 67 cases, one per registry entry in registry order (numbers and choice/text examples of the vocabulary, as `design_case::seed`). Generator: ticket `🧰️scripts/en1997/catalogue-demo-gen.py`.
- Test `the_committed_demo_document_equals_the_seeded_cases_and_covers_the_registry` (same checks as EN 1992: case count, ids, entry ids, value count, every registry entry has a case) appended to `📇️catalogue/🧪️tests/📚️demo-document/🦀️.rs`.
- The JSON uses empty entity lists; the Rust document uses `compliant_demo()` as base, so only `cases` are pinned.

## 4. Editor "new document" palette
`ArtifactEditor::examples()` feeds the navbar/new-document picker. Each editor of EN 1990, 1991, 1992, 1995, 1996, 1997 now appends `full_catalogue_example()` (id `full-catalogue`, label "Full catalogue example" / "Vollständiges Katalogbeispiel"), a DSL encode (`encode_enXXXX_dsl`) of the demo document (existing `catalogue_demo_document` for EN 1995/1997, `Snapshot { cases: catalogue_demo_cases(), ..default }` for the others). No Cargo change (all editors already depend on io and checks/schema). EN 1993, 1994, 1998, 1999 have no demo document yet; tests that count examples per editor may need a +1.

Files: the six `✏️editor/🦀️.rs`; en1999 `🧱️member-checks/🦀️.rs`, `🧭️spec-trace/🔣️.json` and `🦀️.rs`, weld-runs test; en1997 fixture and demo-document test; `📌️important/requests.md`; build logs `latest.md` ("ready for rebuild: coverage fixes").
