# Current Universal Coverage Audit

Read-only source audit on 2026-10-02. No Cargo/Nx/Bun tests ran in this audit; all observations below concern physically read declarations and mounts. Runtime receipts remain owned by Root and execution workers. Read repository and s AGENTS.md; reused the associated existing ticket.

## Census and Authority

The retained inventory currently contains 149 subset entries and 96 distinct artifact roots. Its stored counts remain 89 stdio/60 nonstdio subsets and 118 dedicated snapshot roots. These are inventory counts, not universal admitted/provider counts. Inventory snapshot paths were checked physically; named owners below still exist. 110 snapshot-local SQL files exist across framework+s, but this count includes shared dialect/component files and proves neither independent authorship nor runtime registration.

The important retained-inventory omissions are framework-owned FlowHostSnapshot (OS flow artifact snapshot), Infinite DAG ownership and WorkflowSnapshot/RunArtifact. Workflow/Run physically have authored SQL/provider files. They must remain separate from similarly named s.flow/s.dag parents. Drawing component proof concerns SemioDrawingSnapshot, not the distinct persisted s.draw DrawingSnapshot.

## Genuinely Missing Parent Providers

No `ArtifactSqliteSnapshot` implementation was found anywhere in live Rust source for these seven existing persisted types. Their own roots contain no snapshot-local authored SQL/provider files, and repository-wide implementation searches found no relocated equivalent. Existing ordinary ArtifactPack/native mounts do not supply semantic SQL capability.

- `s.demonstrator.playground@1/*`: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.process.process3d@1/*`: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.trinity.rewriting@1/*`: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.trinity.jack@1/*`: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.dag.dag@1/*`: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.draw.drawing@1/*`: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
- `s.raster.raster@1/*`: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.

Process3d owns workshop, stock id/label/pose/payload, stock_solid, steps, step_payloads and tool_solids: its actual parent is not the child BREP/Flow provider. Jack/DAG own distinct graph child handles and parent metadata. Raster owns its parent maps/layers/assets; SemioImage child coverage cannot replace it. Rewriting persists before_fixture_json/lhs_json/rhs_json as domain Strings plus parameter_bindings/rule_layout; these Strings require exact text preservation, not carrier reinterpretation. Playground currently persists schema only.

## Mounted Schemas With Missing Actual Native Hooks

Store `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10873` explicitly refuses default controlled native input and output. Its erased codec calls these owner hooks. Consequently a mounted relational model or preflight alone cannot establish erased I/O. These physically read providers lack one or both overrides (not runtime failures claimed):

| Dialect | Missing Owner Hook | Provider |
| --- | --- | --- |
| `s.writer.writer@1/*` | input, output | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.mathematical.equation@1/*` | output | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.procedural.generation2d@1/*` | input, output | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.procedural.generation3d@1/*` | input, output | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.gis.gisterrain@1/*` | output | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.gis.gismap@1/*` | output | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.las@1.0/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.html@5/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.zip@2.0/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.svg@1.1/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.ifc@4/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.ifc@2x3/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.binary@raw/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.csv@rfc4180/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.step@ap214/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.tsv@iana/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.md@commonmark/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.png@1.2/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.txt@utf-8/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.tiff@6.0/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.deflate@rfc1950/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.gltf@2.0/*` | output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.json@rfc8259/*` | output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |
| `s.stdio.bmp@v3/*` | input, output | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` |

Writer has an actual ArtifactPack SQLite codec opt-in at its `🚪️io/📸️snapshot/💾️binary/🦀️.rs:17`, while its provider implements projection/reconstruction/preflight but neither native hook. This is a concrete mounted declaration versus erased boundary gap. Other table rows require their own real owner selector, not inferred generic behavior. Generation2d/3d are already Root-owned pending controls. ZIP is already I/O-owner scope. DWG AC1018 and GLTF lack local SQL because provider/schema sharing must be resolved before treating absent local paths as missing coverage; AC1024 and GLTF have current explicit providers. PDF1.7 likewise delegates, so absent local implementation tokens are not a separate RED.

## Staged Work and Receipts

Block2d/3d/5d current physical providers now contain trait implementations and native hooks, superseding older missing-implementation wording; this audit does not prove mounts/runtime. Puzzle3d likewise now has provider/hooks. Layout file has drafted helpers and owned component retirement but no `ArtifactSqliteSnapshot for LayoutSnapshot` at read time. Keep staged Block/Puzzle and Layout Native separate from Source proof. Root owns latest Neural63/63 and Pack108/108 receipts. Known Block Source152/954, Procedure+Playbook Source8/49 and Layout Source7+contract90snapshots39diff are separate Source scopes. Latest Generation stopped on shared Severity imports before assertions: no capability RED.

## Next Three Priorities

1. Author complete Process3d semantic parent schema/provider and actual typed+erased I/O laws, preserving all stock/workshop/step payloads and independent child/local target identities. This is the largest clearly unowned missing parent.
2. Cover Jack and DAG parents next, with their literal graph child identities, metadata and actual declaration/I/O; separately resolve framework Infinite DAG versus s.dag owner authority.
3. Complete Writer, Equation and GIS native owner hooks after genuine baseline assertions, then advance Drawing/Raster/Rewriting/Playground absent-parent schemas. Existing SQL/preflight and child coverage cannot satisfy those actual erased boundaries.

No production/test/script files were modified; this report is the sole created artifact. No generated log files were written.
