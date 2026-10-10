# r12 execution report: w2-wp18-psets-io (IFC part of WP-18: property set templates and classification systems)

`T` = this ticket folder, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `E` = `S/🚪️io/📤️export/🏗️ifc`, `M` = `S/🚪️io/📥️import/🏗️ifc`, `F` = `S/🧫️fixtures/🏗️ifc`, `O` = `S/🧪️tests/🏗️export-bim-1-ifc`.

## 1. Result

State: **WRITTEN, NOT BUILT IN THE REAL CRATE.** `cargo check -p semio-s-artifact-bim-model --lib` fails at HEAD for reasons outside BIM (framework store/retirement refactor of 677 in flight: last run 176 errors, none in a BIM file (log removed with the scratch); the set changes between runs: `SnapshotRetirementStep`, `InteractiveJobCloseStep::Pending { released_items, released_bytes }`, `ValueRetirementStep`, `RetainedCloneGrant`, `RetireOwned`). Because that made every in-crate claim impossible, I verified the new code in a **scratch crate** that compiles the *live* source files (copied by `T/r12-w2-wp18-psets-io-scratch.py`, never edited in place) against a stripped copy of the Part-21 codec and stub model types, and with ifcopenshell on the files the scratch wrote. What ran and what did not is listed in section 5.

| Package item | State |
|---|---|
| 1 IFC 2x3 export of classifications (many per holder, type holders, unattached rows, parents) | written; export + import code compiled and tested in the scratch crate (not in the real crate) |
| 2 type-level property sets in `HasPropertySets` | same |
| 3 IFC 2x3 import (systems, references, many codes per holder, type psets) | same |
| 4 `ModelLibraryIntoIfc4` (IFC4 library serializer, registered, tests) | module + tests compiled and run in the scratch crate (serializer impl and registry rows only reviewed); **library file written and validated by ifcopenshell** |
| 5 oracles (python audits, feature scenarios, Rust subject adapter, fixtures) | python audits run (prototype and library); fixtures: snapshot + IFC4 library + library table committed; 2x3 psets file, its table and the house re-bless are OPEN (need the real crate) |
| 6 unit tests | written for every new function; counts in section 5 |

## 2. Files (before -> after)

Writer kit and spatial (`E`):
- `🧰️writer/🦀️.rs`: `Ifc::new` now delegates to new `Ifc::started(author, organization, true_north, change_action)` (same bytes for `ADDED`); new `Ifc::units()` (the four SI units, moved out of `spatial`). Tests: `new_writes_added_and_started_...`, `the_units_are_the_four_si_units_...`.
- `🏛️spatial/🦀️.rs`: private `units(x)` removed, calls `x.ifc.units()`; unused `derived` import dropped.

2x3 export (`E`):
- `🧬️data/🦀️.rs`: `type_sets` (new) puts the authored sets of a type record into the `HasPropertySets` of its `IfcTypeObject` (after `Semio_Authoring`); the four type writers (layered, profiled, window style, door style) use it; `emit_links` no longer notes type ids as "not part of the export"; the old single `Classification` block is replaced by `emit_classifications` (new, pub) and `parent_key` (new, pub) and the const `PARENTS_SET = "Semio_ClassificationParents"`. Before: one `IfcClassification` per system *name* with `Source = name`, one reference per distinct (system, code, title), only elements as holders, first code only on import. After: section 3.
- `🔬️projection/🦀️.rs`: `Projection` gains `classifications` (`ClassificationRows { systems: name|edition -> SystemRow { source, entries "code|title|parent" in file order }, attached: holder -> sorted "name|edition|code" }`) and `type_properties` (type id -> set -> property -> (IFC type, JSON value)); new pub `classification_rows`, `type_property_rows`, `json_cell`, `plain`, `quote`; `to_json` writes the two new keys.
- `🦀️.rs` (header `//!`, `pub mod ifc4;`), `🧱️codec/🦀️.rs` (+ `encode_ifc4`, `decode_ifc4`; new `🧱️codec/🧪️tests/🔬️unit/🦀️.rs`).
- New module `📚️ifc4/🦀️.rs` + `📚️ifc4/🧪️tests/🔬️unit/🦀️.rs` (IFC4 library).
- Tests: `🧬️data/🧪️tests/🔬️unit/🦀️.rs` (+9 tests), `🔬️projection/🧪️tests/🔬️unit/🦀️.rs` (+4), `🧰️writer/🧪️tests/🔬️unit/🦀️.rs` (+2), `🧪️tests/🧰️testkit/🦀️.rs` (+`PSETS`, `psets()`).

Import (`M`): `🧬️data/🦀️.rs` (`read_attached` now also reads the sets of every imported type object; old first-reference-only classification block replaced by `read_classifications`, `parent_rows`, `user_rows`; skips the `Semio_ClassificationParents` set), `🦀️.rs` (`//!`), `🧪️tests/🔬️unit/🦀️.rs` (+8 tests, `classification_systems` assertion on the house).

IO registry: `S/🚪️io/🦀️.rs` (+`serializer_entry::<ModelSnapshot, export::ifc::ifc4::ModelLibraryIntoIfc4>` right after the 2x3 entry), `S/🚪️io/🧪️tests/🔬️unit/🦀️.rs` (hop list now has the IFC4 row and the already existing JSON row that the old expectation missed; +`the_library_entry_turns_a_packed_model_into_the_ifc4_library_bytes`).

Oracle case `O`: `🐍️.py` (imports `ifcopenshell.util.classification`; `identity` also reads `Tag` of type products; new `classification_rows`, `type_property_rows`, `parent_rows`, audits `classification_problems`, `property_problems`, `library_table`, `library_problems`, `library_case`, case `🏷️psets`, handlers `export-ifc-psets`, `export-ifc-library`), `🥒️.feature` (+2 scenarios `@id-export-ifc-psets`, `@id-export-ifc-library`, header text), `🦀️.rs` (+2 subject functions and registrations).

Fixtures `F/🏷️psets/`: `📸️snapshot/🔣️.json` (generated by `T/r12-w2-wp18-psets-io-fixture.py` from the house: 3 classification systems with parents and editions, 6 holders incl. wall type, door type, storey, several systems per holder, type-level sets on 4 types, 4 property set templates), `📚️library.ifc` (4 643 bytes, written by the live module code through the scratch crate), `📚️measure/🔣️.json` (written by the oracle). Longest path: 153 UTF-16 units from the drive root.

Kept scripts in `T`: `r12-w2-wp18-psets-io-fixture.py`, `-scratch.py`, `-prototype.py`, `-ifc4-prototype.py`, `-schema-probe.py`.

## 3. Design (and where it differs from the brief)

- **2x3 classifications.** `IfcClassification(Source = system.source or '', Edition = edition or '', $, Name = name)` per system of the table (all systems, used or not: the table must come back); `IfcClassificationReference($, code, title, #system)` per table row in table order (attached or not); `IfcRelAssociatesClassification` per used (system, code) listing every element *and type object* that carries it (`links.elements` + `links.types`), so one holder appears in several relations. A code that is no table row is still written (without title) and noted; a missing system or holder is noted. 2x3 `Source` and `Edition` are mandatory strings, hence `''` for none.
- **Parents (2x3 has no slot).** `IfcPropertySet 'Semio_ClassificationParents'` on the `IfcProject`, one `IfcPropertySingleValue` per row with a parent: Name = `"<system name>|<edition>|<code>"` (qualified, because codes repeat across systems: a bare code as in the brief could collide), value = `IFCLABEL(parent code)`. Foreign files have no such set and import flat tables. Loss documented in the module `//!`. System ids are re-derived on import (`cs-<slug of name>`, made unique with `-2`, ...); a model whose ids differ from the slug of the name round-trips with its slug id.
- **Type psets.** Authored sets of a type record are written in `HasPropertySets` after `Semio_Authoring`; defaults and inherited values are never written; elements keep only their own sets (oracle audit `property_problems` checks no leak).
- **Import.** Many codes per holder (first wins, the rest is a note), type holders, unattached rows become table rows, a reference without classification or item reference is skipped with a note, a parent column that is no forest (`entries_problem`) is dropped for its system with a note.
- **IFC4 library (`ModelLibraryIntoIfc4`).** `IFC4` file schema, `Ifc::started(.., "NOCHANGE")` because the IFC4 owner-history rule demands a modification date next to `ADDED` (found with `ifcopenshell.validate` on `T/r12-w2-wp18-psets-io-ifc4-prototype.py`). `IfcProject` declares `IfcProjectLibrary` (`IfcRelDeclares`), the library declares the templates and is associated (`IfcRelAssociatesClassification`) with each `IfcClassification`. Corrections of the brief, checked against ifcopenshell's IFC4 schema (`T/r12-w2-wp18-psets-io-schema-probe.py`): `IfcPropertySetTemplateTypeEnum` has no `PSET_TYPEDRIVEN`; I write `PSET_TYPEDRIVENONLY` (type targets only), `PSET_OCCURRENCEDRIVEN` (element targets only), `PSET_TYPEDRIVENOVERRIDE` (both: type values inherited unless an occurrence overrides, the model's semantics), `NOTDEFINED` (none). `IfcClassification` IFC4 ADD2 attributes are `Source, Edition, EditionDate, Name, Description, Location, ReferenceTokens` (no `Specification`). `IfcSimplePropertyTemplate.Enumerators` is one `IfcPropertyEnumeration` (not a set). A definition's required flag, unit, default, range and help text go into the simple template's `Description` as `description=..;unit=..;required=..;default=..;minimum=..;maximum=..;` (fixed key order, `required` always, `%`, `;`, `=`, CR, LF percent-escaped). `ApplicableEntity` = comma list of the IFC4 entities (`TemplateTarget` -> `Ifc*`; `Ceiling` -> `IfcCovering`, `Void` -> `IfcOpeningElement`, `CeilingType` -> `IfcCoveringType`, ...). Hierarchy: `IfcClassificationReference.ReferencedSource` = parent reference or the classification, `Sort` = zero-padded table index (`000000`), parents written before children whatever the table order; a missing/circular parent or repeated code is noted (`bim.ifc.library.skipped` diagnostic). A template without properties is skipped with a note (IFC4 needs at least one).
- No inference is needed for the library: it only reads authored data.

## 4. Commands and results

All cargo calls through `"$T/🚦️gate.sh" w2-wp18-psets-io -- ...`, foreground, from `/c/git/semio`.

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --message-format=short` (6 runs) | FAIL, 176 errors, all in framework crates (store/job/value refactor); none in a BIM file |
| `cargo test --offline --manifest-path T/🗑️generated/w2-wp18-psets-io/scratch/Cargo.toml` (env `PSETS_JSON` = the psets snapshot) | **45 passed, 0 failed**: writer kit 11 (incl. 2 new), IFC4 codec 2, IFC4 library 16, 2x3 export data 9 new (associations, table rows, parents set, notes, type sets, no inherited sets on instances, determinism, parent key), 2x3 import 7 of 8 new (the byte-stable test needs the real exporter; round trip of systems/parents/many codes/type holders/type psets, missing classification, two codes, duplicate names, non-forest parents, flat file) |
| `cargo run` scratch `library <psets snapshot> <out>` | wrote `F/🏷️psets/📚️library.ifc`, 4 643 bytes, 0 notes |
| `python 🐍️.py` `library_case(write)` (ifcopenshell 0.8.4.post1: `validate` with EXPRESS rules, audit against the snapshot) | "oracle agrees": 0 validate errors, 0 audit problems; `📚️measure/🔣️.json` written |
| scratch `report <library>` vs `📚️measure/🔣️.json` | identical JSON (subject report == oracle table) |
| `T/r12-w2-wp18-psets-io-prototype.py` (ifcopenshell builds a 2x3 psets file and an IFC4 library with the same structure; runs the oracle's `classification_problems`, `property_problems`, `library_table`, `library_problems`) | 0 audit problems; `validate` of the 2x3 prototype only reports the missing spatial decomposition of the prototype itself, the IFC4 library validates clean |
| scratch `project`/`tables` on the prototype 2x3 file vs the python tables | counts, classification tables and typed type properties identical |
| `python 🐍️.py check F 🏠️house` (old committed house file) | 12 problems: 9 phase rows and the classification `source` + stale measure table: the committed house file predates the phase export and the new classification layout (re-bless needed); `classification_problems` and `property_problems` otherwise agree with the real exporter output |

NOT RUN (tree does not compile): `cargo check`/`test` of the real crate, wasm32-wasip2 check, `cargo test --lib` counts, `BIM_BLESS`, the harness `oracle quick --case 🏗️export-bim-1-ifc`, `fixture audit`, `parity`.

## 5. Open items (precise)

1. **Real-crate build.** Files never compiled in the real crate: `E/🦀️.rs`, `E/🧬️data/🦀️.rs` (`emit_links` edit, the four hoisted `type_sets` calls in `emit_types`), `E/🔬️projection/🦀️.rs` (struct, `project`, `to_json` were compiled in the scratch, not the surrounding file), `E/📚️ifc4/🦀️.rs` serializer region, `S/🚪️io/🦀️.rs`, all `🧪️tests` files that need the real exporter (data/projection/import/io tests; their classification subset ran in the scratch), `O/🦀️.rs`, `O/🥒️.feature`.
2. **Bless once the tree compiles** (order matters): `"$T/🚦️gate.sh" w2-wp18-psets-io -- env BIM_BLESS=1 cargo test --manifest-path <A>/Cargo.toml -p semio-s-artifact-bim-model --lib -- the_committed_house_file_is_the_current_export the_committed_psets_file_is_the_current_export the_committed_library_file_is_the_current_export` (the library test should pass without rewriting), then `PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe "$O/🐍️.py" write "$F" 🏠️house 🏷️psets` (writes the two `🔬️measure` tables; the house table gains the keys `classifications`, `type_properties`), then `cargo test --lib -- io::` and `bun ./📜️script.ts oracle quick --case 🏗️export-bim-1-ifc`. Other cases of this oracle (`🪧️notated`, `🔲️ceilings`, `🛝️ramps`, the wall-depth case) have no committed `.ifc` in `F` at the moment, so their tables could not be rewritten; they gain the two keys when their owners run `write`.
3. Expected counts after the bless (from the scratch): psets 2x3 file has `IfcClassification` 3, `IfcClassificationReference` 9, `IfcRelAssociatesClassification` 6; house file keeps 2 / 2 / 2.
4. `🌎️hub/🧩️compositions/🏙️bim/🧪️tests/🔬️surface/🦀️.rs` asserts four hops and is already stale (misses CSV and JSON); it also needs the `(s.bim.model, s.stdio.ifc)` library row. Outside my scope, not touched.
5. `S/🚪️io/🪶️sqlite/📸️snapshot/*` still names the removed `Classification` entity; the tree is not mounted anywhere (dead), left alone.
6. The python audits check properties of elements that carry a `Tag` (`IfcElement`) and of types; properties on storeys, sites, spaces, zones are exported by the same code but not audited. Zones are keyed by `ObjectType` (their category) in the classification table.
7. No non-ASCII text is in the fixture: the stdio Part-21 writer escapes it (`\X2\`), but the round trip through ifcopenshell of such titles was not exercised.
8. No progress/cancellation argument exists on `Serializer::serialize`; the library export is one cheap pass over authored data (no inference).

Scratch output under `T/🗑️generated/w2-wp18-psets-io/` is removed at the end of the run (regenerate the scratch crate with `python -X utf8 -I T/r12-w2-wp18-psets-io-scratch.py <S> <part21 source> <dir>`).
