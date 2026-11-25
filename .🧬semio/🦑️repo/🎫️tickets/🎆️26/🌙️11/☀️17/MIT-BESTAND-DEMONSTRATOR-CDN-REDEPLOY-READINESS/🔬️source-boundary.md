# Source Boundary Refusal of the PDF Lift Crate

Refusal: `Rust crate Source boundary exceeded before compiler: semio-s-artifact-stdio-pdf-lift (active lines 4075/4000)`.

## Counting Rule
- Admission: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` builds the message and compares against `maxActiveLines` (policy 4000, per-unit override through `unit.boundary`). The crate breaches when the sum is greater than 4000.
- Counter: `readRustSourceClosureV1` in `.../📥️source/🟦️.ts`. It follows `mod` / `#[path]` from the crate entry and sums `activeLines` over every reachable source file.
- Per file: `selectRustSourceV1(text, cfg).split("\n").length`. Blank lines and comments count. Items removed by `cfg` (for example `#[cfg(test)]`) do not count. `include_str!` files count 0. A closure also breaches on file count and bytes.
- Consequence: only a crate boundary lowers the number. Moving code between modules of one crate does not.

## Why Not the Controlled Lift Alone
`🧭️controlled` (2163 active lines) drives `Lifter<'_>`, `DocumentLiftState`, `FontLift` and the resource lanes. Those types and their inherent `impl Lifter` blocks live in the lifter modules, and the orphan rule forbids moving the impls apart from the type. Extracting `controlled` into a lower crate would need the lifter below it, which would be a cycle. So the lifter went down and the resumable document lift stays up.

## Split
New crate `semio-s-artifact-stdio-pdf-lifter` (1528 active lines), `⬇️lift/🧱️lifter/📦️packages/🦀️rust`, path-preserving carve. Root `⬇️lift/🧱️lifter/🦀️.rs` mounts the unmoved module files:
- `🧩️lifter-and-document` (state, `Category`, `Lifter`, `lift_page`)
- `🪟️resources-and-fonts`
- `🧭️navigation-and-annotations`
- `📚️catalog-and-page-mode`

`semio-s-artifact-stdio-pdf-lift` (2548 active lines) keeps `🧩️lifter-and-document/🧭️document-lift` (cursor, controlled lift and the `lift_document` / `lift_document_with` entry points), now mounted directly from its root. It depends on the lifter, page-content and page-tree crates and re-exports the clients' API explicitly: `Category`, `DocumentLiftState`, `Lifter`, `FontLift`, `FontStart`, the page content items and the page tree items. `page_mode_from_name`, `page_layout_from_name`, `lift_viewer_preferences` and `CATALOG_KEYS` are imported privately for the controlled lift.

Visibility widened by necessity: lifter items the document lift calls across the crate boundary moved from `pub(crate)` or private to `pub`.

## Registration (mirrors the page-content and page-tree split)
- `📖️pdf/Cargo.toml` workspace member.
- `📖️pdf/📦️packages/🦀️rust/📜️script.ts` package list.
- `📋️build-owners.json` owner row.
- `🧰️framework/.../📏️paths/🧪️tests/🧫️fixtures/🔣️.json` path fixture row.
- `Cargo.lock` of every workspace that locks the lift crate with the page crates (10 files), by `register-pdf-lifter.py`.
- Carve `owned` list of the lift manifest reduced to the modules it still owns.

## Scan
`source-boundary-scan.ts` (node 24, `--experimental-strip-types`, no compiler) reads every `Cargo.toml` outside tickets, caches and `target`, takes the `[lib]` / `[[bin]]` entries and counts with the admission code itself. cfg: aarch64 macos unix, no features, no `test`. `THRESHOLD=3900` is the default; set `THRESHOLD=3500` for a wider view.

Crates over 3900 active lines:

| Lines | Crate |
| --- | --- |
| 4000 | semio-framework-3d-brep-engine |
| 4000 | semio-framework-3d-brep-solid-transaction |
| 4000 | semio-framework-3d-brep-native-registry |
| 4000 | semio-framework-3d-brep-boolean |
| 3999 | semio-framework-3d-brep-representation |
| 3975 | semio-s-artifact-puzzle-5d-puzzle5d |
| 3928 | semio-framework-3d-brep-construction |

The 4000 entries sit exactly on the limit: one more line refuses them. Next below 3900: vdi3805-inferences 3886, mesh-engine 3856, en1998-schema-formulas 3853. Features that add cfg-selected code can raise a count; three entries could not be evaluated without compiler ownership.

## Check
`repo cargo --store=shared --configuration=prototype -- check -j 1 -p semio-s-artifact-stdio-pdf-lift --message-format short` run from the pdf workspace (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`), because the root workspace excludes the standards crates. Result: Finished, no errors.
