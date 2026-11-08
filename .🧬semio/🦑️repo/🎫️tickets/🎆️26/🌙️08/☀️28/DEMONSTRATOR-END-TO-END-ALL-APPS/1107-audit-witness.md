# 1107 Witness Audit (read-only, lane witness)

Ticket: `DEMONSTRATOR-END-TO-END-ALL-APPS`. Read-only audit: no edits, no cargo, no sub-agents, no modifying git commands. Scan script lives in the session scratchpad, not in the ticket.

## Method

- Scope: every tracked `Cargo.toml` under the artifact roots of the 8 demonstrator panes (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) plus the `semio-plugin-result-page` package (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/📃️/📦️packages/🦀️rust`). That is 343 packages.
- Roots: each package's lib target (`[lib] path` or `src/lib.rs`, a unit-test binary), each `tests/*.rs`, `tests/*/main.rs` and `[[test]]` path. That is 480 roots.
- Reach: a root reaches `take_fixture_result_page` when a non-comment line in its module closure (`mod`, `#[path]`, `include!`, followed from the root) names `take_fixture_result_page` or one of the testing-laws wrappers that call it: `settle_registered_typed_operation`, `take_fixture_result_turn`, `assert_pane_smoke`, `assert_pane_smoke_by_example`, `assert_proposes_transaction`, `assert_registered_ingest_idempotent_with_members`, `assert_two_registered_instances_converge_with_members`, `assert_undo_redo_round_trip`, `declared_verb_dispatch`, `declared_verb_fixture_app`, `exchange_and_assert_convergence`, `settle_extension_invocations`, `settle_history_verb`.
- Witness: the root's module closure contains `install_fixture_heap_witness!` or a `#[global_allocator]` over `HeapWitness`.

Result: 37 reaching roots, 17 with witness, 20 missing. The rough 124-file scan overcounted: it matched callers by ancestor heuristic and did not see per-root closure.

Result-page package: 3 roots (lib `🔄️full-operation/📃️/🦀️.rs`, tests `📃️wire/🧪️tests/🦀️.rs`, `📃️pending/🧪️tests/🦀️.rs`). None reaches `take_fixture_result_page`, so none needs the witness. The two test roots already have it.

## Reaching binaries

| Pane | Kind | Root (repo-relative) | Witness |
|---|---|---|---|
| generator | test | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🪧️pane-smoke/🦀️.rs` | present |
| generator | test | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️action-producer/🦀️.rs` | **MISSING** |
| generator | test | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️editor-routes/🦀️.rs` | **MISSING** |
| generator | test | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️mutation-laws/🦀️.rs` | **MISSING** |
| verfolgen | lib-unittest | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️laws/🦀️.rs` | present |
| verfolgen | lib-unittest | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/🦀️.rs` | **MISSING** |
| verfolgen | lib-unittest | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs` | **MISSING** |
| statik | lib-unittest | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/🦀️.rs` | **MISSING** |
| statik | test | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️app/🦀️.rs` | **MISSING** |
| statik | test | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️app-2/🦀️.rs` | **MISSING** |
| statik | test | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪧️pane-smoke-target/🦀️.rs` | present |
| bearbeiten | lib-unittest | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | present |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪧️pane-smoke-target/🦀️.rs` | present |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| bearbeiten | test | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧪️tests/🔬️window/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪧️pane-smoke-target/🦀️.rs` | present |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| koordinator | test | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | **MISSING** |
| energie | lib-unittest | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | present |
| energie | lib-unittest | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mounted-bestest/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️task-resume/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️defects/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️manifest/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🦀️.rs` | present |
| aggregator | test | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️gestures/🦀️.rs` | present |
| aussuchen | lib-unittest | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | **MISSING** |
| aussuchen | test | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` | present |

## Missing witness: exact path and line to add

Add `semio_plugin::install_fixture_heap_witness!();` before the first item (after inner `//!` docs). For lib-unittest roots wrap it in `#[cfg(test)]`, because the lib is also built without tests. Every missing package already depends on `semio-plugin`.

| # | Pane | Kind | Absolute path | Insert before line |
|---|---|---|---|---|
| 1 | generator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️action-producer/🦀️.rs` | 3 |
| 2 | generator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️editor-routes/🦀️.rs` | 3 |
| 3 | generator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️mutation-laws/🦀️.rs` | 3 |
| 4 | verfolgen | lib-unittest | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/🦀️.rs` | 6 |
| 5 | verfolgen | lib-unittest | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs` | 5 |
| 6 | statik | lib-unittest | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/🦀️.rs` | 5 |
| 7 | statik | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️app/🦀️.rs` | 3 |
| 8 | statik | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️app-2/🦀️.rs` | 3 |
| 9 | bearbeiten | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 10 | bearbeiten | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 11 | bearbeiten | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 12 | bearbeiten | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 13 | bearbeiten | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 14 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧪️tests/🔬️window/🦀️.rs` | 1 |
| 15 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 2 |
| 16 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/🧪️tests/🔬️unit/🦀️.rs` | 7 |
| 17 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 18 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 19 | koordinator | test | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | 1 |
| 20 | aussuchen | lib-unittest | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs` | 4 |

## Limits

- Static text scan, not a build. `cfg` is not evaluated, so closures are supersets. Rows are a superset for cfg-gated `mod`s, and an exact root may need confirmation by compiling it.
- Wrapper names are the testing-laws wrappers only. A helper outside testing-laws that calls one of them, and is itself not in the scanned packages, is not traced.
- Lib-unittest witnesses are checked across the lib's module closure. A witness in a non-test module would also count.
- Nothing was compiled or run. The testing-laws crate was already reported as blocked (`1107-result-applaws.md`), so this is the static answer only.

Binaries checked: 37 reaching of 480 roots. Binaries missing: 20.
