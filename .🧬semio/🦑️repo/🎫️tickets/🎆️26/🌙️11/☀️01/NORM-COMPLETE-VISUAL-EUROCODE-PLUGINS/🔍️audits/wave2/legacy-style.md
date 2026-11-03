# Legacy and Style Audit: Norm Plugin (Wave 2)

Read-only audit of `✏️s/🔌️plugins/📕️norm/**` against `/home/deck/semio/AGENTS.md`. No source was edited.

Paths below are relative to `✏️s/🔌️plugins/📕️norm/` unless marked. Scanner and generator: `🗑️generated/scan.py` and `🗑️generated/report.py` (scan output was intermediate and removed).

## Summary

| Check | Count |
|---|---|
| Forbidden words (legacy, compat, migrat*, SDK GAP, placeholder, stub, deprecated) and `[DEBUG]` | 463 hits |
|  of which `legacy` | 75 |
|  of which `compat*` | 53 |
|  of which `migrat*` | 232 |
|  of which `SDK GAP` | 36 |
|  of which `placeholder` | 34 |
|  of which `stub*` | 27 |
|  of which `deprecated` / `TODO` / `FIXME` / `todo!` / `unimplemented!` | 0 |
| `-2` groups (directories): 9 end in `-2`, 4 contain `-2` mid-name | 13 |
| `-2` suffixed crate/package names and path refs in Cargo.toml | 18 |
| Unmounted leftover window dirs (Rust not reachable from any `mod` mount) | 3 |
| Comment lines inside bodies (`//#region` markers) | 74 |
| Comment lines inside bodies (`//` prose) | 3 |
| TS interface field JSDoc inside definitions (`/** @state */`) | 511 |
| Indented `///` doc lines inside Rust definitions (judgement, see section 4) | 1294 |
| Docstrings not starting with an emoji | 2 (plus 5 keycap-emoji `#️⃣` starts, treated as compliant) |
| `render_document_json` in UI code | 0 |
| `to_string_pretty` / `JSON.stringify` in UI code (editor, viewer, panels, windows, modes, commands) | 0 |
| `to_string_pretty` in io/test/snapshot code (not UI) | 56 |
| `[DEBUG]` logs | 6 |
| Hard-coded utilisation constants | 5 |
| Paths over 256 bytes from repo root `/home/deck/semio/` | 11 (over 256 characters: 0) |

## 1. Legacy, compat, migration, SDK GAP, placeholder, stub wording

Grouped by artifact. `compat*` also matches `compatible()` API calls, which are not compatibility layers. `Migrated` is an enum variant name in `✏️editor` code and is not a migration script.

### ⚖️compliance (14)

- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs:224` [compat] pub fn compatible(self) -> impl Iterator<Item = UnitId> {
- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs:231` [compat] self.compatible().find(|unit| unit.factor() == 1.0 && unit.offset() == 0.0)
- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🧪️tests/🔬️unit/🦀️.rs:32` [compat] let cases: Vec<_> = UnitId::ALL.iter().flat_map(|from| from.compatible().map(move |to| (*from, to))).collect();
- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🧪️tests/🔬️unit/🦀️.rs:36` [compat] let got = from.convert(123.456, *to).expect("compatible");
- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🧪️tests/🥒️.feature:14` [compat] Scenario: Conversion between compatible units
- `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🧪️tests/🥒️.feature:15` [compat] Given a value 123.456 in every compatible unit pair
- `⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs:111` [migrat] /// (📓️migration-recipe.md §1/§3), not a value type reused pervasively as a struct field.
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json:76` [compat] "asWritten": "NA-DE same (DE uses K_FI via the design supervision/inspection route, see E90.05). out: K_FI applied to unfavourable γF (Table B3): γF,e
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json:5301` [stub] "asWritten": "**V-420 6.2.4 + Table 6.2 — T-stub in tension equivalent / T-Stummel.** Failure modes: Mode 1 (complete flange yielding): method 1 FT,1,
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json:5548` [stub] "asWritten": "**V-850 3-1 — Lattice towers and guyed masts:** member classes (leg, diagonal, redundant); effective slenderness: λ̄eff = 0.9 + ... anne
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table.rs:62` [compat] row("ndp.en1990.e90-02.1", S16, "Table B3", "?", Unset, Unset, "", S17, S0, "Table B3", Unverified, "en1990-en1991-target.md:48", "NA-DE same (DE uses
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table.rs:337` [stub] row("ndp.en1993.v-420.1", S3, "6.2.4", "FT,1,Rd = 4 Mpl,1,Rd/m", Unset, Unset, "", S8, S0, "6.2.4", Unverified, "en1993-target.md:209", "**V-420 6.2.4
- `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table.rs:350` [stub] row("ndp.en1993.v-850.1", S3, "6.3.1.4", "λ̄eff = 0.9 + ...", Unset, Unset, "", S8, S0, "6.3.1.4", Unverified, "en1993-target.md:290", "**V-850 3-1 — 
- `⚖️compliance/🔤️op-text-and-pack-codec/🦀️.rs:50` [migrat] /// other P6-migrated document kind, e.g. 🪐️space, 🔁️workflow, 📖️playbook). Every one of norm's

### 📇️registry (13)

- `📇️registry/🧬️contract/🖥️app-surface/🔖️field-meta/🦀️.rs:221` [compat] FieldKind::Quantity { units, stored, .. } if units.is_empty() => stored.compatible().collect(),
- `📇️registry/🧬️contract/🖥️app-surface/🔖️field-meta/🦀️.rs:223` [compat] FieldKind::Angle => UnitId::Radian.compatible().collect(),
- `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️retained-disposition-oracle/🦀️.rs:28` [migrat] if route["id"] != *expected_id || route["admission"] != "migrated" {
- `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️retained-disposition-oracle/🦀️.rs:60` [migrat] return Err("cohort totals or publication contracts are not fully migrated".into());
- `📇️registry/🧬️contract/🖥️app-surface/🧵️retained-commands/🦀️.rs:301` [migrat] semio_framework_plugin::InteractiveJobClassification::Migrated
- `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧾️properties/🎛️controls/🦀️.rs:264` [placeholder] let mut input = ui::input(InputKind::Number).try_id(format!("{row_id}.input")).map_err(|_| ui_error("ui.node.id"))?.display_factor(unit.factor).precis
- `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧾️properties/🎛️controls/🦀️.rs:356` [placeholder] let input = ui::input(InputKind::Text).value(ui_text(current)?).placeholder(norm_ui_label(chrome("empty", "leer", locale))?).commit(ui_text("blur")?).
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:55` [migrat] "admission": "migrated",
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:67` [migrat] "admission": "migrated",
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:79` [migrat] "admission": "migrated",
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:91` [migrat] "admission": "migrated",
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:103` [migrat] "admission": "migrated",
- `📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json:115` [migrat] "admission": "migrated",

### 🔮️oracles (9)

- `🔮️oracles/🏃️execution/🐍️.py:38` [placeholder] 🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio is the repository-wide placeholder whose whole
- `🔮️oracles/🏃️execution/📄️document/🐍️.py:163` [compat] if key is not None and compatible(document[key], replacement):
- `🔮️oracles/🏃️execution/📄️document/🐍️.py:168` [compat] if parent is not None and compatible(parent[key], replacement):
- `🔮️oracles/🏃️execution/📄️document/🐍️.py:401` [compat] def compatible(current, replacement):
- `🔮️oracles/🏃️execution/📄️document/🐍️.py:438` [compat] if replacement is None or compatible(member[inner], replacement):
- `🔮️oracles/🏃️execution/📝️carrier/🐍️.py:86` [placeholder] "payload = OCTET+ placeholder and the framework's DSL notation module carries no grammar document "
- `🔮️oracles/🏃️harness/🔌️adapters/groundhog/🐍️.py:15` [stub] """🔇️ groundhog 0.15.0 declares no dependencies yet imports plotly, pyproj and jinja2 at module level for plotting only; those roots resolve to inert 
- `🔮️oracles/🏃️harness/🔍️coverage/🐍️.py:30` [stub] def stub_plotting():
- `🔮️oracles/🏃️harness/🔍️coverage/🐍️.py:122` [stub] stub_plotting()

### 🗿️artifacts/⚖️en1990 (22)

- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:71` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:368` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:376` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:386` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:388` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:390` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:407` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:85` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:129` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:115` [[DEBUG]] std::fs::create_dir_all(out).expect("[DEBUG] mkdir");
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:120` [[DEBUG]] let snapshot = parse_dsl(text).expect("[DEBUG] dsl");
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:121` [[DEBUG]] std::fs::write(out.join(format!("{name}.pack.semio")), en1990_to_pack(&snapshot)).expect("[DEBUG] write");
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️mutate-en1990-1/🥒️.feature:54` [placeholder] grammar: the committed 📖️.grammar.semio is the repository-wide payload = OCTET+ placeholder, so

### 🗿️artifacts/⚡️din18599 (15)

- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:64` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:262` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:263` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:272` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:281` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:289` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:297` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚡️mutate-din18599-1/🥒️.feature:39` [placeholder] repository-wide payload = OCTET+ placeholder), so the two implementations are compared on the
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:542` [migrat] /// lifted out of the pre-migration manifest's inline .artifact_kind(ArtifactKindSpec { .. }) so the

### 🗿️artifacts/🌍️en1997 (22)

- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:50` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:72` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:368` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:376` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:386` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:388` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:390` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:407` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:21` [[DEBUG]] const script="import{Database}from'bun:sqlite';import{deepStrictEqual,ok}from'node:assert';\nconst expected=await Bun.file(process.argv[1]).json(),mod
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:72` [[DEBUG]] console.error('[DEBUG] EN1997 independent complete SQLite annex='+annex+' tables='+names.length+' rows='+totalRows+' cells='+totalCells+' mode='+mode+
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🦀️.rs:255` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather

### 🗿️artifacts/🌬️din16798 (15)

- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:64` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:252` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:253` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:262` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:271` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:279` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:287` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:83` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:123` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌬️mutate-din16798-1/🥒️.feature:40` [placeholder] repository-wide payload = OCTET+ placeholder), so the two implementations are compared on the
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:267` [migrat] /// lifted out of the pre-migration manifest's inline .artifact_kind(ArtifactKindSpec { .. }) so the

### 🗿️artifacts/🏋️en1991 (19)

- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:72` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:332` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:342` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:351` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:359` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:367` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:375` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:385` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:387` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:389` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:397` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:406` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:83` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:123` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🥒️.feature:48` [placeholder] 📖️.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two

### 🗿️artifacts/🏛️en1992 (32)

- `🗿️artifacts/🏛️en1992/README.md:18` [compat] 156 catalogue entries in 11 groups (function level, member checks use the same functions): materials and time effects; section resistance (strain comp
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:73` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:368` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:376` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:386` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:388` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:390` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:407` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:83` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:123` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs:304` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️section-resistance/🦀️.rs:1` [compat] //! 📐️ Section resistance of EN 1992-1-1 §6.1 as catalogue entries S-01 … S-10: strain-compatibility integrator over arbitrary polygons with holes, N–
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️section-resistance/🦀️.rs:279` [compat] /// 💪 M_Rd (N·m) of the member section for N_Ed in N (tension positive, the snapshot convention) by strain compatibility, 0 when N_Ed is outside the r
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📐️section-resistance/🦀️.rs:522` [compat] entry_of("S-01", "DIN EN 1992-1-1", "6.1", "Strain-compatibility integrator", "Dehnungsebene", &["fck", "b", "h", "bar_y", "bar_z", "bar_a", "x", "alp
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔥️fire-design/🦀️.rs:272` [compat] Outcome::nmm("m_rd_fi", "M_Rd,fi(N_Ed,fi)", "M_Rd,fi(N_Ed,fi)", m, "strain compatibility on the reduced section".into()),
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️helpers-and-annex-parts/🦀️.rs:126` [compat] /// 💪 Rectangular-section flexural resistance M_Rd [N·m] by strain compatibility (parabola-rectangle, S-04); N_Ed tension positive.
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️helpers-and-annex-parts/🦀️.rs:253` [compat] /// 🔍 Required A_s [m²] for M_Ed by bisection on the strain-compatibility resistance.
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🐍️.py:242` [compat] """Strain compatibility by quadrature: parabola-rectangle concrete, bilinear steel, tension steel at depth d, N_Ed tension positive (units N, m, Pa)."
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🐍️.py:398` [compat] # Checks whose formulas follow the EN 1992 clauses directly (strain compatibility, variable strut, punching, bond, cover, fire tables, l/d, second ord
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:475` [placeholder] bad.push(format!("placeholder {}", gql.display()));
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:482` [placeholder] bad.push(format!("unknown/placeholder {}", ts.display()));
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql:1` [stub] # 🧬️ En1992 mutations — hierarchical artifact fields only (no flat design-effect stubs).

### 🗿️artifacts/🏭️vdi3805 (94)

- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs:63` [legacy] /// 📋 Filter classes for Blatt 19 (ISO 16890 / EN 779 legacy mapping used by VDI 3805-19).
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:64` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:321` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:322` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:331` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:340` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:348` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:356` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:229` [legacy] Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: "8".into(), new_choice: EditionProfileChoice::Legacy }),
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql:24` [legacy] CREATE TABLE vdi3805_edition (id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES vdi3805_document(id),map_key TEXT NOT NULL,choice TEXT NO
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs:75` [legacy] for(key,value)in maps.values(n::map(n::field(root,1)?)?){out.insert("vdi3805_edition",&[Cell::Integer(1),Cell::Text(key),Cell::Text(n::enumeration(val
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts:40` [legacy] function edition(value:string):"legacy"|"current"{if(value!=="legacy"&&value!=="current")throw Error("VDI3805 unsupported edition choice");return valu
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:26` [legacy] fn edition(value:EditionProfileChoice)-> &'static str{match value{EditionProfileChoice::Legacy=>"legacy",EditionProfileChoice::Current=>"current"}}
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:27` [legacy] fn read_edition(value:&str)->Result<EditionProfileChoice, ValueError>{match value{"legacy"=>Ok(EditionProfileChoice::Legacy),"current"=>Ok(EditionProf
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:42` [legacy] test("VDI3805 every quantity kind and edition choice retain exact semantic identity",async()=>{for(const kind of choices.quantityKind){const expected=
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json:9` [legacy] "editionProfile":{"":"legacy","任意\u0000":"current","__proto__":"current"},"correctionAsOf":{"year":65535,"month":255},"strictMode":false,
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🗺️duplicate-maps.json:186` [legacy] "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🗺️duplicate-maps.json:190` [legacy] "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🛂️choices.json:3` [legacy] "legacy",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🥒️.feature:25` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🦀️.rs:216` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/🧭️middle/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✂️remove-geometry-connection/🧭️middle/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎛️change-product-configuration/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏭️change-manufacturer-file/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-product/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-product/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📅️change-correction-as-of/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📈️add-curve/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📈️add-curve/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📉️remove-curve/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📉️remove-curve/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍️change-curve-points/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍️change-curve-points/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️resize-geometry/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️resize-geometry/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📦️add-product/✅apply/📸️snapshot/➡️after/🔣️.json:119` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📦️add-product/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌️add-geometry-connection/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒️change-strict-mode/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒️change-strict-mode/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔖️change-edition-profile/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-product/✅apply/📸️snapshot/➡️after/🔣️.json:24` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-product/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-product/🧭️middle/📸️snapshot/➡️after/🔣️.json:135` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-product/🧭️middle/📸️snapshot/⬅️before/🔣️.json:190` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚧️change-limits/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚧️change-limits/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚮️remove-geometry/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🚮️remove-geometry/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️add-geometry/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️add-geometry/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✅apply/📸️snapshot/➡️after/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧮️change-geometry-parameters/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹️remove-edition-profile/✅apply/📸️snapshot/⬅️before/🔣️.json:80` [legacy] "8": "legacy"
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:511` [stub] assert!(!text.contains("_placeholder"), "{label} must not stub GenericAttributes");
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧰️check-evidence/🦀️.rs:53` [legacy] let legacy = match sheet {
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧰️check-evidence/🦀️.rs:60` [legacy] EditionProfileChoice::Legacy => legacy,
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:27` [legacy] "legacy",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:1598` [legacy] "legacy",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:1603` [legacy] "legacy": {
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:1604` [legacy] "en": "Legacy edition",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛡️limits-and-code-tables/🦀️.rs:50` [legacy] /// 🏷️ Filter classes Blatt 19 (ISO 16890 / EN 779 legacy — VDI 3805-19 Table 1).
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:10` [legacy] editionProfile: { [key: string]: "legacy" | "current" };
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:195` [legacy] export type Vdi3805EditionProfileChoice = "legacy" | "current";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:203` [legacy] export const parseVdi3805Snapshot: NormWireReader<Vdi3805Snapshot> = normWireObject<Vdi3805Snapshot>({ catalog: normWireRequired(normWireRef(() => par
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:236` [legacy] export const parseVdi3805EditionProfileChoice: NormWireReader<Vdi3805EditionProfileChoice> = normWireLiteral("legacy", "current");
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:99` [migrat] /// lifted out of the pre-migration manifest's inline .artifact_kind(ArtifactKindSpec { .. }) so the
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🩺️diagnostics-and-edition-profile/🦀️.rs:46` [legacy] Legacy,
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json:27` [legacy] "legacy",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:46` [legacy] value: "legacy" | "current";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:166` [legacy] export const parseVdi3805EditionProfileEntry: NormWireReader<Vdi3805EditionProfileEntry> = normWireObject<Vdi3805EditionProfileEntry>({ key: normWireR
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs:83` [legacy] let sum = law(&base, &set(EditionProfileChoice::Legacy), |_| set(EditionProfileChoice::Current)).await;
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts:10` [legacy] editionProfile: { [key: string]: "legacy" | "current" };
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts:177` [legacy] export const parseVdi3805Artifact: NormWireReader<Vdi3805Artifact> = normWireObject<Vdi3805Artifact>({ catalog: normWireRequired(normWireRef(() => par
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️change-edition-profile/🧬️schema/🔣️.json:39` [legacy] "legacy": {
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️change-edition-profile/🧬️schema/🔣️.json:40` [legacy] "en": "Legacy edition",
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:16` [stub] //! 🟤️set-snapshot stub is deleted along with its dangling glue mount).
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:123` [migrat] // straight off this 🧬️mutations/ directory (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION,
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:46` [legacy] let change = Vdi3805Mutation::ChangeEditionProfile(change_edition_profile::ChangeEditionProfile { sheet: "8".into(), new_choice: semio_s_artifact_norm
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:48` [legacy] assert_eq!(after_change.edition_profile.get("8"), Some(&semio_s_artifact_norm_vdi3805_model::EditionProfileChoice::Legacy));
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:184` [migrat] // sibling facet iso16757's migration made the same call), and step (e) of the fan-out recipe

### 🗿️artifacts/📇️iso16757 (39)

- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:64` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:250` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:251` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:260` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:269` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:277` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:285` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:128` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql:27` [compat] CREATE TABLE iso16757_accessory (id INTEGER PRIMARY KEY, group_id INTEGER NOT NULL REFERENCES iso16757_accessory_group(id), ordinal INTEGER NOT NULL, 
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts:36` [compat] export type Iso16757SqliteSnapshot={catalogue:{id:string;metadata:{names:Iso16757SqliteNames;lifecycle:{revision:string;status:string;validFrom:string
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts:91` [compat] let n=0;for(const[key,values]of await sorted(c.accessories,p,o)){collection(values);const group=await insert(p,"iso16757_accessory_group",[1n,BigInt(n
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts:145` [compat] for(const v of await reader.ordered("accessory_group",1n)){const key=artifactSqliteText(v,3);if(out.catalogue.accessories.has(key))throw Error("ISO167
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:43` [compat] for(n,(key,values))in catalogue.accessories.iter().enumerate(){let group=p.insert("iso16757_accessory_group",&[Cell::Integer(1),ordinal(n)?,Cell::Text
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:123` [compat] for v in reader.ordered("iso16757_accessory_group",1,c)?{let key=text(v,3,c)?;if out.catalogue.accessories.contains_key(&key){return Err(invalid("ISO 
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:103` [[DEBUG]] let script=r#"import{Database}from'bun:sqlite';const c=JSON.parse(process.argv[1]);const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json:472` [compat] "compatibilityCondition": ""
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🥒️.feature:25` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🦀️.rs:332` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🦀️.rs:169` [placeholder] pub fn ensure_locale_on_names(names: &Names, locale: &str, placeholder: &str) -> Names {
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🦀️.rs:174` [placeholder] next.alternatives.push(LocalizedText { locale: locale.into(), text: placeholder.into() });
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:166` [stub] assert!(!schema_text.contains("\"catalogue\": {\n      \"type\": \"object\"\n    }"), "catalogue must not be a bare object stub");
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:239` [stub] assert!(hits.is_empty(), "facet stub/unknown patterns remain: {hits:?}");
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📚️part-1-catalogue/🦀️.rs:159` [compat] pub compatibility_condition: Option<String>,
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:2389` [compat] "compatibilityCondition": {
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:248` [compat] compatibilityCondition: string | null;
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts:376` [compat] export const parseAccessoryRelationship: NormWireReader<AccessoryRelationship> = normWireObject<AccessoryRelationship>({ accessoryProductId: normWireR
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧱️catalogue-core/🦀️.rs:37` [compat] /// 📊️ Physical dimension signature for unit compatibility.
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧱️catalogue-core/🦀️.rs:55` [compat] pub fn compatible(self, other: Self) -> bool {
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧾️artifact-definition/🦀️.rs:14` [migrat] /// lifted out of the pre-migration manifest's inline .artifact_kind(ArtifactKindSpec { .. }) so the
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:19` [stub] //! 🟤️set-snapshot stub is deleted along with its dangling glue mount).
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:152` [migrat] /// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
- `🗿️artifacts/📇️iso16757/🧪️tests/🔬️unit/🦀️.rs:8` [compat] assert!(DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH));
- `🗿️artifacts/📇️iso16757/🧪️tests/🔬️unit/🦀️.rs:9` [compat] assert!(!DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH_3));
- `🗿️artifacts/📇️iso16757/🧪️tests/🔬️unit/🦀️.rs:10` [compat] assert!(!DimensionSignature::DIMENSIONLESS.compatible(DimensionSignature::LENGTH));

### 🗿️artifacts/🔩️en1993 (30)

- `🗿️artifacts/🔩️en1993/README.md:20` [stub] Library level (functions with tests, not all wired into the model): global analysis (alpha_cr, amplification, sway imperfection, bracing), warping tor
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:73` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:368` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:376` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:386` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:388` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:390` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:407` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:83` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:123` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:553` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:798` [stub] /// 🔗 EN 1993-1-8 §6 component method of the end plate joint: T-stubs, column web components, row distribution, M_j,Rd and S_j,ini (mm, N, MPa).
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:825` [stub] let stub = jc::t_stub_modes(leff.l1(), leff.l2(), tp, fyp, m_use, jc::n_prying(e.min(ex.max(1.0)), m_use), ew, sum_ft, as_, n_b, lb, Mode1Method::Two,
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:826` [stub] mode = mode.max(stub.mode);
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:827` [stub] iso.push(col.ft_rd.min(stub.ft_rd).min(ft_wc));
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:854` [stub] let stub = jc::t_stub_modes(leff.l1(), leff.l2(), tp, fyp, m, jc::n_prying(e, m), 0.0, sum_ft, as_, f64::from(j.bolts_per_row), j.anchor_length * 1e3,
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs:855` [stub] let ft = stub.ft_rd.min(sum_ft);
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🏗️parts-2-6/🦀️.rs:53` [stub] /// 🏗️ Base plate compression resistance of one T-stub, F_c,Rd = f_jd b_eff l_eff.
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🔗️joint-components/🦀️.rs:1` [stub] //! 🔗️ EN 1993-1-8 §6 component method: effective lengths, T-stub modes 1 to 3 with and without prying, column web components, row reduction, M_j,Rd, 
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🔗️joint-components/🦀️.rs:81` [stub] /// 🔗️ T-stub flange in tension (Table 6.2): L_b* = 8.8 m³ A_s n_b/(Σ l_eff,1 t_f³), prying forces develop when L_b ≤ L_b*, absent for L_b > L_b*, the

### 🗿️artifacts/🧩️en1994 (21)

- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:72` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:333` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:343` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:352` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:368` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:376` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:386` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:388` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:390` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:398` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:407` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🥒️.feature:25` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs:269` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:332` [placeholder] /// 🧱 HEB 300 placeholder for insert mutations.

### 🗿️artifacts/🧱️din4108 (16)

- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:42` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:64` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:151` [migrat] /// the three verbs were already Migrated, retained and proven, and were still inert in the shell.
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:254` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:255` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:264` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:273` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:281` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:289` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:83` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:127` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-din4108-1/🥒️.feature:43` [placeholder] repository-wide payload = OCTET+ placeholder), so the two implementations are compared on the
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:342` [migrat] /// lifted out of the pre-migration manifest's inline .artifact_kind(ArtifactKindSpec { .. }) so the

### 🗿️artifacts/🪨️en1996 (25)

- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:74` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:335` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:353` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:361` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:369` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:377` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:387` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:389` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:391` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:399` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:408` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs:51` [stub] let stub = (0.5 * lc.slab_span_m.clamp(0.0, 6.0)).max(0.1);
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🕹️handles/🦀️.rs:52` [stub] out.push(handle(Spec { entity: &entity, tag: "span", kind: HandleKind::ScalarAlong { axis: pose.axis_y() }, anchor: pose.at(0.5 * l, t + stub, h + 0.1
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧱️walls/🦀️.rs:125` [stub] let stub = (0.5 * span).clamp(0.0, SLAB_STUB_MAX);
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧱️walls/🦀️.rs:127` [stub] let depth = a + stub.max(0.1);
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪨️mutate-en1996-1/🥒️.feature:38` [placeholder] repository-wide payload = OCTET+ placeholder), so the two implementations are compared on the
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/📇️catalogue/🦀️.rs:1` [legacy] //! 📇️ Catalogue tests: the worked examples X1 … X12 of the target specification, remedy and basis laws, and the legacy-trace scan.
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/📇️catalogue/🦀️.rs:301` [compat] for needle in ["Compatibility", "older durability", "pub fn compressive_strength_mpa"] {

### 🗿️artifacts/🪵️en1995 (21)

- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:74` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:335` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:353` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:361` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:369` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:377` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:387` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:389` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:391` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:399` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:408` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs:9` [SDK GAP] // 🚧️ SDK GAP: WindowKit/TableWindowKit/TableView are not yet in semio_framework_plugin's
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:558` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather

### 🗿️artifacts/🪶️en1999 (18)

- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:74` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:334` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:335` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:353` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:361` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:369` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:377` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:387` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:389` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:391` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:399` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:408` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs:213` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather

### 🗿️artifacts/🫨️en1998 (33)

- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs:4` [migrat] //! re-evaluates unconditionally) — so this command genuinely mutates nothing. The pre-migration
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:41` [SDK GAP] // 🚧️ SDK GAP: Dialect is not in semio_framework_plugin's curated crate-root re-export list
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:73` [migrat] /// pre-migration enum, never derived from one another.
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:338` [migrat] .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:339` [migrat] .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:348` [migrat] .action_interactive_job("setField", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:357` [migrat] .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:365` [migrat] .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:373` [migrat] .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:381` [migrat] .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:391` [migrat] .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:393` [migrat] .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:395` [migrat] .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:403` [migrat] .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:412` [migrat] .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:84` [migrat] assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:124` [migrat] /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the app_commands! decomposition
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🏷️field-meta/🦀️.rs:181` [compat] ("buildings[].members[].detailingCompatibleWithQ", m!("q-compatible detailing", "q-gerechte Durchbildung", choices BOOL)),
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:20` [SDK GAP] // 🚧️ SDK GAP: see the identical note in ✏️editor/🦀️.rs — Dialect is only reachable
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️seismic-multipart-fail/⚠️seismic-multipart-fail/🗣️.dsl.semio:20` [compat] } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=false min-dimension-m=0.2m rho=0.002 rho-prime=0 omega-wd=0.02 steel-section-clas
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/⚠️seismic-rc-frame-fail/⚠️seismic-rc-frame-fail/🗣️.dsl.semio:20` [compat] } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=false min-dimension-m=0.2m rho=0.002 rho-prime=0 omega-wd=0.02 steel-section-clas
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-multipart/🏢️seismic-multipart/🗣️.dsl.semio:20` [compat] } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=true min-dimension-m=0.35m rho=0.012 rho-prime=0 omega-wd=0.12 steel-section-clas
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio:20` [compat] } ] [ {id=col-c1 material=rc role=column detailing-compatible-with-q=true min-dimension-m=0.35m rho=0.012 rho-prime=0 omega-wd=0.12 steel-section-clas
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:197` [compat] En1998Mutation::ChangeMemberDetailing(p) => format!("change-member-detailing building-index={} member-index={} new-detailing-compatible-with-q={}", en
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:315` [compat] "change-member-detailing" => Ok(En1998Mutation::ChangeMemberDetailing(ChangeMemberDetailing { building_index: dec_json(&arg("building-index")?)?, memb
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🥒️.feature:31` [placeholder] 📖️component.grammar.semio is the repository-wide payload = OCTET+ placeholder, so the two sides are compared
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs:914` [placeholder] /// placeholder — so the identity scenario compares the two implementations at the carrier level rather
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🧱️structural-system-checks/🔗️ductility-and-masonry/🦀️.rs:253` [compat] lc("q-compatible detailing declared", "q-gerechte Durchbildung erklärt"),
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌍️full-seismic-check/🧱️structural-system-checks/🔗️ductility-and-masonry/🦀️.rs:266` [compat] lc("Provide q-compatible detailing (or reduce q0).", "q-gerechte Durchbildung herstellen (oder q0 reduzieren)."),
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json:545` [compat] "en": "Detailing compatible with q",
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:585` [compat] /// 🔩️ Steel cross-section class (1–4) for q-compatibility (§6.5 / §6.6).
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json:517` [compat] "en": "Detailing compatible with q",
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️change-member-detailing/🧬️schema/🔣️.json:63` [compat] "en": "Detailing compatible with q",

### 🧪️tests (3)

- `🧪️tests/🔬️typescript-package-commands/🟦️.ts:36` [migrat] expect(fixture.expected.retained === fixture.expected.identities, "every identity must be retained (migrated)");
- `🧪️tests/🔬️typescript-package-commands/🟦️.ts:41` [migrat] expect(route.admission === "migrated", route ${route.id} admission is ${route.admission});
- `🧪️tests/🔬️typescript-package-commands/🟦️.ts:78` [migrat] console.log(norm retained cohort ok: ${fixture.apps.length} editors × ${fixture.routes.length} migrated routes, ${classified} descriptor rows carried 

### 🧬️schema (2)

- `🧬️schema/🔣️.json:86` [placeholder] "placeholder"
- `🧬️schema/🔣️.json:90` [migrat] "const": "migrated"

## 2. `-2` suffixed groups and crates

Directories (group names). Items 1 to 9 end in `-2`; the 4 with `-2` mid-name are listed at the end of the note below:

- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️silos-2`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️traffic-2`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪜️failing-2`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📐️part-2-geometry`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️products-2`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🏗️parts-2-6`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️general-2`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🌉️bb-2-thermal-bridges`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️part-2-minimum-insulation`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️elements-2`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️members-2`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-storey-2`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️buildings-2`

Crate/package names and `path =` entries in Cargo.toml:

- `🗿️artifacts/🏋️en1991/Cargo.toml:182 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️silos-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🏋️en1991/Cargo.toml:194 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️traffic-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️silos-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1991-mut-silos-2"`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️traffic-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1991-mut-traffic-2"`
- `🗿️artifacts/🏭️vdi3805/Cargo.toml:147 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪜️failing-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪜️failing-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-vdi3805-examples-failing-2"`
- `🗿️artifacts/📇️iso16757/Cargo.toml:173 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️products-2/📦️packages/🦀️rust"`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️products-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-iso16757-mut-products-2"`
- `🗿️artifacts/🔩️en1993/Cargo.toml:171 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️general-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️general-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1993-mut-general-2"`
- `🗿️artifacts/🧱️din4108/Cargo.toml:163 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️elements-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️elements-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-din4108-mut-elements-2"`
- `🗿️artifacts/🪵️en1995/Cargo.toml:191 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️members-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️members-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1995-mut-members-2"`
- `🗿️artifacts/🫨️en1998/Cargo.toml:170 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️buildings-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🫨️en1998/Cargo.toml:203 path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-storey-2/📦️packages/🦀️rust"`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-storey-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1998-mut-buildings-storey-2"`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️buildings-2/📦️packages/🦀️rust/Cargo.toml:3 name = "semio-s-artifact-norm-en1998-mut-buildings-2"`

Non-suffix `-2` tokens (standard part references, not suffixes; listed for completeness): `part-2-geometry`, `bb-2-thermal-bridges`, `part-2-minimum-insulation`, `parts-2-6`.

## 3. Unmounted leftover directories (EN 1995 and others)

A window module is mounted only when its `🦀️.rs` is referenced by a `#[path]` mount in the same artifact. The check below found the following Rust window modules with no mount:

- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs`

Evidence for EN 1995:

- `🗿️artifacts/🪵️en1995/.../✏️editor/🎭️modes/✏️edit/🦀️.rs:47` (`pub mod windows`) mounts only `checks`, `diagrams`, `outliner`, `properties`, `viewport`. `📥️inputs` and `📊️results` are not mounted.
- `🗿️artifacts/🪵️en1995/.../👁️viewer/🎭️modes/👁️view/🦀️.rs:27-34` (`pub mod windows`) mounts only `checks`, `diagrams`, `viewport`. `📊️report` is not mounted.
- The unmounted Rust tests import the missing modules, e.g. `use crate::modes::edit::windows::inputs::*;` in `📥️inputs/🧪️tests/🔬️unit/🦀️.rs:1`, which cannot compile as-is.
- Other artifacts (vdi3805, din18599, iso16757, din16798, din4108) mount the same three windows in their edit and view modes, so EN 1995 is the odd one out.
- TypeScript twins still export these windows: `✏️editor/🟦️.ts:9-10` and `👁️viewer/🟦️.ts:9`. The TS and Rust trees disagree.

Not counted as unmounted: the norm-root `🪟️results` directory, which is a crate referenced by `🗿️artifacts/🪵️en1995/Cargo.toml:104`. Also the `update-*-inputs` mutation leaves, which are mutation names, not window dirs.

## 4. Comments inside definitions

### 4a. `//#region` marker lines indented inside bodies

House-style region markers. They sit inside `mod` or `#[cfg(test)]` bodies, so they count under the rule.

- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:201` `//#region ðï¸MediaPorts`
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:213` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️mutate-en1990-1/🦀️.rs:43` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️mutate-en1990-1/🦀️.rs:62` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️mutate-en1990-1/🦀️.rs:64` `//#region 🔖️Handlers`
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/⚖️mutate-en1990-1/🦀️.rs:175` `//#endregion 🔖️Handlers`
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:187` `//#region ðï¸MediaPorts`
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:199` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:202` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:215` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🦀️.rs:290` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🦀️.rs:311` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🦀️.rs:313` `//#region 🔖️Handlers`
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌍️mutate-en1997-1/🦀️.rs:416` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:177` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:189` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:202` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:214` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🦀️.rs:43` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🦀️.rs:62` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🦀️.rs:64` `//#region 🔖️Handlers`
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🦀️.rs:165` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:203` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:215` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs:339` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs:360` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs:362` `//#region 🔖️Handlers`
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs:465` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:197` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:210` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🦀️.rs:251` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🦀️.rs:272` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🦀️.rs:274` `//#region 🔖️Handlers`
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏭️mutate-vdi3805-1/🦀️.rs:373` `//#endregion 🔖️Handlers`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:174` `//#region 🔖️MediaPorts`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:187` `//#endregion 🔖️MediaPorts`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🦀️.rs:367` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🦀️.rs:388` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🦀️.rs:390` `//#region 🔖️Handlers`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️mutate-iso16757-1/🦀️.rs:493` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:203` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:215` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:588` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:609` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:611` `//#region 🔖️Handlers`
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔩️mutate-en1993-1/🦀️.rs:714` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:202` `//#region 🔖️MediaPorts`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:215` `//#endregion 🔖️MediaPorts`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs:304` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs:325` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs:327` `//#region 🔖️Handlers`
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs:430` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:179` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:191` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:204` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:216` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:204` `//#region 🔖️MediaPorts`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:216` `//#endregion 🔖️MediaPorts`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:593` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:614` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:616` `//#region 🔖️Handlers`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs:719` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:204` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:216` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs:248` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs:269` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs:271` `//#region 🔖️Handlers`
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs:374` `//#endregion 🔖️Handlers`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:208` `//#region ðï¸MediaPorts`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:220` `//#endregion ðï¸MediaPorts`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs:949` `//#region 🔖️FixtureDecode`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs:970` `//#endregion 🔖️FixtureDecode`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs:972` `//#region 🔖️Handlers`
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs:1075` `//#endregion 🔖️Handlers`

### 4b. `//` prose comments inside function bodies

- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🧩️part-libraries/🦀️.rs:462` `// 🔀️ O1 de-dyn: `ScriptRuntime` has exactly one impl (`DefaultScriptRuntime`) — the trait object is`
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🧩️part-libraries/🦀️.rs:463` `// deleted in favor of the concrete type (an enum of one is worse than none, R11).`
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📜️annex-parameters-and-parts/🦀️.rs:161` `// --- ventilation rate method Table B.6/B.7 ---`

### 4c. TypeScript interface field JSDoc (inside `interface` bodies)

These are emoji-led docstrings on fields, not prose comments. They still sit inside definitions. Counted per file:

- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (20 field docs, first at line 7)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (20 field docs, first at line 47)
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (20 field docs, first at line 7)
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (18 field docs, first at line 8)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (13 field docs, first at line 7)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (13 field docs, first at line 8)
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (13 field docs, first at line 7)
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (10 field docs, first at line 21)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (49 field docs, first at line 7)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (70 field docs, first at line 8)
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (49 field docs, first at line 7)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (10 field docs, first at line 7)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (10 field docs, first at line 8)
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (10 field docs, first at line 7)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (7 field docs, first at line 7)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (9 field docs, first at line 8)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (7 field docs, first at line 7)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (17 field docs, first at line 8)
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (8 field docs, first at line 7)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (17 field docs, first at line 7)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (17 field docs, first at line 105)
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (17 field docs, first at line 7)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (9 field docs, first at line 7)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (9 field docs, first at line 8)
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (9 field docs, first at line 7)
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (10 field docs, first at line 39)
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (5 field docs, first at line 8)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (3 field docs, first at line 7)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (3 field docs, first at line 8)
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (3 field docs, first at line 7)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts` (9 field docs, first at line 7)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (9 field docs, first at line 8)
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts` (7 field docs, first at line 7)
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts` (10 field docs, first at line 75)
- `🪟️results/🎚️config/🧬️schema/🟦️.ts` (1 field docs, first at line 3)

### 4d. Indented `///` doc lines inside Rust definitions

Judgement call. Method docs on `impl` blocks are arguably docstrings, not comments inside definitions. Struct and variant field docs are inside definitions. Counts per artifact, excluding tests and snapshots:

- `🗿️artifacts/🏋️en1991`: 180
- `🎬️structural-scene`: 150
- `📇️registry`: 111
- `🗿️artifacts/🧩️en1994`: 110
- `🗿️artifacts/🏛️en1992`: 99
- `🗿️artifacts/🫨️en1998`: 97
- `🗿️artifacts/🔩️en1993`: 96
- `⚖️compliance`: 63
- `🗿️artifacts/🌍️en1997`: 55
- `🗿️artifacts/🪶️en1999`: 52
- `🗿️artifacts/🪵️en1995`: 45
- `🗿️artifacts/⚖️en1990`: 37
- `🗿️artifacts/📇️iso16757`: 36
- `🗿️artifacts/🧱️din4108`: 32
- `🗿️artifacts/🪨️en1996`: 30
- `🗿️artifacts/🌬️din16798`: 30
- `🗿️artifacts/⚡️din18599`: 27
- `🧮️mechanics`: 23
- `🗿️artifacts/🏭️vdi3805`: 21

## 5. Docstrings not starting with an emoji

- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘change-thermal-bridge-psi/🦀️.rs:1` `//!` starts with `ψ `change-thermal-bridge-psi`.`
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️timber/🌲️material-and-annex-factors/🦀️.rs:104` `///` starts with `𝜓 EN 1990 Table A1.1 ψ₀/ψ₁/ψ₂ for categories A–E/H and snow/`

Compliant keycap starts (`#️⃣`), not counted: `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/#️⃣change-beam-stud-count/🦀️.rs:1`, `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/#️⃣change-bar-layer-count/🦀️.rs:1`, `⚖️compliance/⚡️incremental-evaluation/🔍️read-tracker/🦀️.rs:41`, `🎬️structural-scene/🎯️projection/🧩️fragment-cache/🦀️.rs:7`, `🎬️structural-scene/🧬️schema/🧩️entities/🦀️.rs:288`.

## 6. JSON rendered in UI

- `render_document_json`: 0 hits.
- `to_string_pretty` / `JSON.stringify` / `serde_json::to_string` in editor, viewer, panels, windows, modes, commands: 0 hits.
- The following are io, test, or snapshot code, not UI. Verify no UI path calls them:

- `📇️registry/🧬️contract/🟦️.ts:59` if ((minimum !== undefined && number < minimum) || (maximum !== undefined && number > maximum) || (exclusiveMinimum !== undefined && number 
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:161` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:89` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs:59` std::fs::write(&rep, serde_json::to_string_pretty(&report).expect("report")).expect("rep");
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:123` let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:152` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:104` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:337` std::fs::write(&snap_path, serde_json::to_string_pretty(&doc).expect("snap json")).unwrap();
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:338` std::fs::write(&report_path, serde_json::to_string_pretty(&report).expect("report json")).unwrap();
- `🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:367` std::fs::write(&snap, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:151` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:34` let mut child=Command::new("bun").arg("-e").arg(r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bu
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:296` let json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:73` let before_canon = serde_json::to_string_pretty(&before).expect("encode before") + "\n";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:79` let mutation_canon = serde_json::to_string_pretty(&mutation).expect("encode mutation") + "\n";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:85` let diff_canon = serde_json::to_string_pretty(&diff).expect("encode diff") + "\n";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:86` let after_canon = serde_json::to_string_pretty(&after).expect("encode after") + "\n";
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:68` Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(catalog)
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:76` Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(document
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:8` test("VDI3805 complete manufacturer catalogue agrees with independent snapshot JSON schema",()=>{const validate=new Ajv({strict:false}).comp
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:9` test("VDI3805 complete manufacturer catalogue agrees with independent artifact JSON schema",()=>{const validate=new Ajv({strict:false}).comp
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:82` let mut expected=fixture();let bytes=export_sqlite_database(&database(&expected),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let 
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:128` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs:26` let json = serde_json::to_string_pretty(&serde_json::Value::from(&tree)).expect("serialize snapshot");
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:69` Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(catalogu
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:77` Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(dictiona
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:30` const values=JSON.stringify(Array.from({length:f.wideListItems},()=>({kind:"null",state:"Unknown"})));
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:103` let script=r#"import{Database}from'bun:sqlite';const c=JSON.parse(process.argv[1]);const d=Database.deserialize(await Bun.stdin.bytes(),{saf
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:170` std::fs::write(&tmp, serde_json::to_string_pretty(&snap).unwrap()).unwrap();
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:189` std::fs::write(&tmp, serde_json::to_string_pretty(&bad).unwrap()).unwrap();
- `🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:29` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:7` test("EN1994 neutral complete composite snapshot agrees with independent JSON schema",()=>{const validate=new Ajv({strict:false}).compile(sc
- `🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:85` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:87` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:374` std::fs::write(&snap_path, serde_json::to_string_pretty(&doc).expect("snap")).unwrap();
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:375` std::fs::write(&report_path, serde_json::to_string_pretty(&report).expect("report")).unwrap();
- `🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:402` let json = serde_json::to_string_pretty(&doc).unwrap();
- `🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:104` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:105` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:165` let pretty = |compact: String| serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&compact).expect("codec json")).unwra
- `🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:33` use store::{os_store::serialization::ArtifactSqliteSnapshotHost as _,sqlite_snapshot::*};use std::{io::Write,process::{Command,Stdio}};let d
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:8` test("EN1998 independent snapshot schema retains complete native arbitrary annex subject",()=>{const validate=new Ajv({strict:false}).compil
- `🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:9` test("EN1998 artifact-root schema agrees with the exact current site and every closed nested record",()=>{const validate=new Ajv({strict:fal
- `🧪️tests/🏷️mutation-label-census/🟦️.ts:78` if (part === leaf.kind) found.push(`${locale}: equals its kind ${JSON.stringify(part)}`);
- `🧪️tests/🏷️mutation-label-census/🟦️.ts:80` if (identifier) found.push(`${locale}: untranslated identifier ${JSON.stringify(identifier[0])} in ${JSON.stringify(part)}`);
- `🧪️tests/🏷️mutation-label-census/🟦️.ts:84` for (const part of shared) found.push(`de reuses the English literal ${JSON.stringify(part)}`);
- `🧪️tests/🔬️typescript-package-commands/🟦️.ts:43` expect(JSON.stringify(contract?.lanes) === JSON.stringify(route.publicationLanes), `route ${route.id} publication lanes diverge from its con
- `🧪️tests/🧪️wire-twins/🟦️.ts:82` if (!chosen) throw new Error(`no branch admits ${JSON.stringify(value).slice(0, 80)}`);
- `🧪️tests/🧪️wire-twins/🟦️.ts:85` if (value === null || typeof value === "boolean" || typeof value === "string") return JSON.stringify(value);
- `🧪️tests/🧪️wire-twins/🟦️.ts:95` return `{\n${entries.map(([key, item]) => `${inner}${JSON.stringify(key)}: ${encode(schemas, item, member(key), owner, inner)}`).join(",\n")
- `🧪️tests/🧪️wire-twins/🟦️.ts:121` [...new Set([...schemas.byId.values()].flatMap((schema) => [...JSON.stringify(schema).matchAll(/"\$ref":"(https:[^"#]+)/gu)].map((match) => 
- `🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:57` assert(validate(value), JSON.stringify(validate.errors));
- `🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:58` assert.equal(JSON.stringify(parseNormResultsWindowConfig(value)), JSON.stringify(value));
- `🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:61` assert.equal(validate(value), false, `Ajv admitted ${JSON.stringify(value)}`);
- `🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:67` const documentBytes = JSON.stringify(fixture.document);
- `🪟️results/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts:78` assert.equal(JSON.stringify(fixture.document), documentBytes);

## 7. `[DEBUG]` logs

- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:115` std::fs::create_dir_all(out).expect("[DEBUG] mkdir");
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:120` let snapshot = parse_dsl(text).expect("[DEBUG] dsl");
- `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:121` std::fs::write(out.join(format!("{name}.pack.semio")), en1990_to_pack(&snapshot)).expect("[DEBUG] write");
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:21` const script="import{Database}from'bun:sqlite';import{deepStrictEqual,ok}from'node:assert';\nconst expected=await Bun.file(process.argv[1]).
- `🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:72` console.error('[DEBUG] EN1997 independent complete SQLite annex='+annex+' tables='+names.length+' rows='+totalRows+' cells='+totalCells+' mo
- `🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:103` let script=r#"import{Database}from'bun:sqlite';const c=JSON.parse(process.argv[1]);const d=Database.deserialize(await Bun.stdin.bytes(),{saf

## 8. Hard-coded utilisation constants

Named `f64`/`usize` constants that fix utilisation thresholds or display limits. Each should be derived from a norm table, a parameter, or the framework, not hard-coded here.

- `🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:25` `pub const NEAR_LIMIT_UTILISATION: f64 = 0.95;`
- `🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📜️norm-tables/🦀️.rs:143` `pub const DIN_V_18599_2_UTILIZATION_A: f64 = 0.95;`
- `⚖️compliance/✅️check-results/🦀️.rs:19` `pub const UTILIZATION_CAP: f64 = 1.0e6;`
- `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/✅️checks/📶️utilisation-bar/🦀️.rs:5` `pub const UTILISATION_SCALE_MAX: f64 = 1.2;`
- `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/✅️checks/📶️utilisation-bar/🦀️.rs:7` `pub const UTILISATION_BAR_CELLS: usize = 12;`

Not counted: inline `.utilization(x, dim(1.0))` calls, which use 1.0 as the unity capacity. Also not counted: `utilization() > 1.0` pass thresholds in `🗿️artifacts/🪨️en1996/.../🏋️load-case-checks/🦀️.rs:142,165`, which are the standard's pass criterion, not a hidden tunable.

## 9. Paths over 256 bytes

Measured from `/home/deck/semio/` (repo root). Byte count is UTF-8. Character count is Unicode code points.

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/📸️snapshot/➡️after/🔣️.json` bytes 258, chars 180
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-assessment-supported-building-id/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 259, chars 181
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-building-accidental-eccentricity-ratio/✅apply/📸️snapshot/➡️after/🔣️.json` bytes 264, chars 186
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-building-accidental-eccentricity-ratio/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 265, chars 187
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-building-claims-simple-masonry/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 257, chars 179
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-building-multiple-resisting-systems/✅apply/📸️snapshot/➡️after/🔣️.json` bytes 261, chars 183
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-building-multiple-resisting-systems/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 262, chars 184
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/📸️snapshot/➡️after/🔣️.json` bytes 258, chars 180
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-foundation-supported-building-id/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 259, chars 181
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-x-m/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 257, chars 179
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/✏️change-storey-centre-of-stiffness-y-m/✅apply/📸️snapshot/⬅️before/🔣️.json` bytes 257, chars 179

All 11 are en1998 mutation fixtures (`🧪️tests/🧫️fixtures/🧬️mutations/✏️change-...`). They exceed 256 bytes but not 256 characters. The rule counts bytes, so they are violations.

