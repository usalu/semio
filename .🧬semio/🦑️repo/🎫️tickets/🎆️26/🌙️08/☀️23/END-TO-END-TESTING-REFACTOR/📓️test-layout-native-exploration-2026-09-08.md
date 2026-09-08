# Native Test Layout Exploration

## Canonical rule

Every test case has exactly this shape:

```text
<semantic-owner>/🧪️tests/<test-name>/<implementation>
```

`<implementation>` is the canonical language filename, for example `🦀️.rs`, `🐹️.go`, `🐍️.py`, or `🔷️.cs`. The semantic owner is the nearest domain scope of the implementation under test. It is never a delivery-package path such as `📦️packages/🦀️rust`; package folders only configure a language build.

For a Rust implementation at `…/🎮️commands/🌊️workflow/🦀️.rs`, a case belongs at `…/🎮️commands/🌊️workflow/🧪️tests/<test-name>/🦀️.rs`.

## Static inventory

The inventory used tracked source paths and excluded a file only when its suffix had exactly two path components after `🧪️tests/` (the case directory and implementation filename).

| Category | Nonconforming inventory |
| --- | ---: |
| Rust executable test entry-point attributes (`#[test]`, `#[tokio::test]`, `#[semio_framework_async_macros::async_test]`) | 21,918 attributes in 3,849 source files |
| Rust `#[cfg(test)]` gates | 8,300 attributes in 3,700 source files |
| Rust test-related attributes of all forms | 37,292 attributes in 5,053 source files |
| Rust legacy `tests/` integration files | 7 |
| Rust one-level `🧪️tests/<implementation>` files | 288 |
| Go `*_test.go` files | 8 |
| Python test-path files | 22 |
| C#, F#, Java, Kotlin, Swift, Zig, Ruby, PHP, Elixir test-path files | 0 |
| Native `*.test.*` filenames | 0 |

The Rust source-file count is the migration worklist for inline modules; attribute count is not a case count because a source file can have several cases. Each test function needs its own named case directory before extraction.

### Rust inline source distribution

The highest-volume areas are `✏️s/🔌️plugins/🗄️stdio` (1,544 source files / 7,712 test-related attributes), `🧰️framework/🛍️products/💻️os` (379 / 6,238), `✏️s/🔌️plugins/📕️norm` (786 / 3,035), `✏️s/🔌️plugins/🏛️architect` (302 / 1,820), `✏️s/🔌️plugins/🧩️puzzle` (169 / 1,790), `✏️s/🔌️plugins/🔋️energy` (88 / 1,715), and `🧰️framework/🔨️modules/🖱️ui` (132 / 1,684).

Next partitions are `🌀️procedural` (124 / 1,021), `📸️remodel` (60 / 951), `🧱️block` (181 / 951), `🏗️fem` (53 / 775), `🌍️gis` (93 / 429), `📏️layout` (63 / 420), `🌊️flow` (70 / 398), and `➗️mathematical` (30 / 384). These partitions do not share semantic owners and can migrate concurrently.

Framework-core owners with substantial inline work are `🖱️ui` (132 / 1,684), `🗺️surface` (5 / 254), `📡️replication` (17 / 241), `🧮️math` (1 / 192), `🕸️graph` (5 / 191), `🎭️actor` (10 / 149), `🛂️manifest` (1 / 112), `🌱️value` (11 / 111), `🧊️3d` (4 / 108), `🔢️number` (1 / 106), `🎒️pack` (6 / 102), `📐️geometry` (3 / 90), and `⏳️async` (5 / 90). The remaining framework-module scopes contain 1–86 files each.

### Directory violations

The 7 conventional Rust `tests/` files are:

```text
🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/tests/📐️first_party_geometry.rs
🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/tests/{✍️mutable-receiver,📈️scale,🕳️uninhabited,🧩️mixed-receivers}.rs
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs
🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/tests/🧬️typegen_export.rs
```

They must move to their nearest semantic owner and lose the `📦️packages/🦀️rust/tests` staging path.

The 288 one-level `🧪️tests/<implementation>` paths are already close but require a case-name directory. The largest groups are OS (95), norm (32), UI (25), energy (15), procedural (11), puzzle (10), block (6), and value (6); every other group has five or fewer. The remaining groups span framework modules, plugin artifacts, editor modules, and hub inference.

The eight Go files all belong to the repository product and use the Go suffix convention:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/{🔬️component,🤝️g1_contract}_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/{🎮️command/🧪️command,📡️eventstore/🧪️eventstore}_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️contract_test.go
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/{🗂️g3_filesystem,🧬️schema_contract,🧪️g3_event_store}_test.go
```

The 22 Python paths are all under `♻️mit-bestand`: one archive migration test, four review tests, eight `netz/tests` files, eight retained intake-log test scripts, and one Figma bridge test. Each has a `test_*.py` or `*_test.py` legacy name or sits in `tests/`.

## Rust extraction and build hazards

Cargo discovers `crate/tests/*.rs` automatically. It does not recursively discover the requested semantic layout. A layout migration therefore needs an explicit test target per canonical case, using Cargo `[[test]]` entries with a `path` pointing to `../../<semantic-owner>/🧪️tests/<test-name>/🦀️.rs`, or an equivalent generated manifest that registers every canonical test file. Leaving a conventional `tests/` runner merely to include nested cases violates the requested path rule.

Moving a `mod tests` body into a case file changes Rust visibility. Tests currently read private functions, private fields, and test-only symbols. An external Cargo test target can only use public or intentionally exposed test-support APIs. Before each extraction, parse the module boundary and classify imports as public API, `pub(crate)` API, private implementation access, generated include, fixture path, and test-only hook. Move reusable test support into the semantic owner, make only deliberate `pub(crate)` seams available, and remove every source-level `#[cfg(test)]` module after its cases move.

`#[path] mod` can retain private access only when a production source file still declares the external file as a nested module. That preserves the legacy inline-test topology and does not satisfy extraction. It is useful only as a temporary compiler check during a single atomic migration, never as the final arrangement.

Plugin artifacts add another boundary: generated artifacts can contain Rust cases but still need owners inside the artifact's semantic schema, mutation, example, or command scope. Do not consolidate them under a plugin-wide or package-wide test folder.

## Migration order

1. Add a structural validator that rejects `*.test.*`, `*_test.go`, `test_*.py`, `tests/`, and a `🧪️tests` directory without exactly one case segment and one canonical implementation segment.
2. Add explicit Rust test-target registration for the canonical tree, then prove one public-API case and one former-private-access case compile and run through the registered target.
3. Partition extraction by semantic owner, starting with stdio, OS, norm, FEM, puzzle, procedural, and UI. Each worker owns disjoint domain scopes, not language package folders.
4. For every source module, create one canonical case directory per test function, move its fixtures beside that case when they are not shared, remove the inline test module and test-only hooks, and run the owning target.
5. Migrate Go and Python after their language runners have explicit discovery of the canonical filename; do not rely on suffix or directory discovery. Delete the seven Rust `tests/` files and all one-level `🧪️tests` files in the same owner-local change.
6. End with the structural validator and full language test commands so no legacy discovery convention survives.

No source files were modified during this exploration.
