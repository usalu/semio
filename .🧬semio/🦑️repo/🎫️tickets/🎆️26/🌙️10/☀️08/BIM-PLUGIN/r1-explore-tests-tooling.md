# 🧪️ BIM plugin exploration r1: tests, oracles, tooling, launch

Read-only exploration of `C:\git\semio` (2026-10-08). No repo file modified. Paths are relative to the repo root unless marked absolute. Plugin examples use `✏️s/🔌️plugins/…`.

## 0. Summary of findings

- Language-neutral test = `🧪️tests/<emoji-kebab>/🥒️.feature` (Gherkin with tags) plus one native adapter per implementation (`🦀️.rs`, `🟦️.ts`, `🐍️.py`). Fixtures are JSON in `🧫️fixtures/` (per case or per owner).
- Oracles are third-party libraries declared per owner in `🔮️oracles/🔣️.json` (`oracleHostPackages`) and per subset in `…/🪆️subsets/…/🔮️oracles/🔣️.json` (`oracles`, `noOracleDecisions`). Real oracles exist for stdio, norm and draw. GIS has only a Rust `geo`/`geojson` dev-dependency.
- 571 feature files: 381 use `@oracle-…`, 164 use `@no-oracle-…` (semio-native artifacts). AGENTS.md requires a third-party reproduction, so the no-oracle cases are recorded debt.
- `.venv` is stale: no ifcopenshell, shapely, pypdf, trimesh, sympy, cadquery, OCP. `uv.lock` pins ifcopenshell 0.8.4.post1 and shapely 2.1.2. Trimesh, sympy, cadquery and OCP appear nowhere in the repo.
- `.vscode/launch.json` is the AGENTS.md launch registry, but ticket DASHBOARD-LAUNCH-COCKPIT (2026/09/23) deletes it and its seed. This conflicts with AGENTS.md; see section 4.
- Plugin crates are in the separate workspace `✏️s/Cargo.toml` (root `Cargo.toml` excludes `✏️s`). `cargo … -p <crate>` from the root is auto-routed to the owning workspace.
- The crate name `semio-s-plugin-gis` in the brief does not exist. The GIS crates are `semio-s-artifact-gis-gismap` and `semio-s-artifact-gis-gisterrain`. The hub composition is `semio-hub-gis`. A `bim` crate already exists as `semio-s-plugin-flow-extension-bim` at `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim`.

## 1. Language-agnostic tests and fixtures

### 1.1 Layout (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md`)

```
<owner>/
├── 🖼️assets/                       static production data
├── 🧫️fixtures/                     immutable testing input owned by this domain
├── 📚️examples/<example>/           executable usage projects, with their own 🧪️tests
├── 🔮️oracles/<oracle>/             real reference or comparator implementations, with their own 🧪️tests
├── 🧪️tests/<emoji><kebab-case>/    one leading emoji identity
│   ├── 🧫️fixtures/                 immutable, private to this case
│   ├── 🥒️.feature                 the normative, language-neutral contract
│   ├── 🦀️.rs / 🟦️.ts / 🐹️.go / 🐍️.py / 🔷️.cs   one adapter per implementation
└── 📦️packages/<language>/          the implementations under test
```

- The four collection names are exact: `🧪️tests`, `🧫️fixtures`, `📚️examples`, `🔮️oracles`.
- Case folder names: one leading emoji plus kebab slug (`testCaseSlugPattern` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`). Examples: `🎨️mutate-drawing-1-any-style`, `🎚️mutate-gis-gismap-1-any-viewer-view-map-config`, `🔬️unit` (Rust unit folder), `🔺️differential-ifc-4`.
- Adapter file names are the language emoji: `🥒️.feature`, `🦀️.rs`, `🟦️.ts`, `🐍️.py`, `🐹️.go`, `🔷️.cs`, `🔣️.json`.
- Feature count: 571 `.feature` files across `✏️s`, `🧰️framework`, `🌎️hub` and `🎓️teaching`.
- The harness is split across two modules:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`: runner, host, README, `📦️packages/🦀️rust` (crate `semio-repo-test-host`, exports `Adapter`, `Context`, `Outcome`, `law::vector`), `📦️packages/🟦️typescript`, `📦️packages/🐹️go`, `📦️packages/🔷️dotnet`, `🖥️host/🐍️.py` (Python host), `🧬️schema/🔣️.json` (protocol v2 schema).
  - `🧰️framework/🔨️modules/🧪️test/`: `🥒️gherkin/🟦️.ts`, `🔌️adapter/🟦️.ts`, `🎮️mutation/` (mutation inventory), `🧬️schema/`, `🧫️fixtures/`.
- Feature-file contract: `@capability-…`, `@oracle-…` or `@no-oracle-…`, `@comparison-…`, `@mutations-…`. Each scenario: `@id-…`, exactly one `@level-…` (`fundamental|quick|long|exhaustive`), exactly one `@mode-…` (`differential|conformance|round-trip|property|error`).
- Python host note: "It never parses a feature file." The coordinator parses features and passes a plan to `🐍️.py --plan <plan.json> --out <results.jsonl> --adapter <🐍️component.py>`.

### 1.2 Complete example: `🎚️mutate-gis-gismap-1-any-viewer-view-map-config`

Path: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎚️mutate-gis-gismap-1-any-viewer-view-map-config/`

`🥒️.feature`, verbatim:

```gherkin
@capability-gis-gismap-1-any-viewer-view-map-config-mutate
@no-oracle-gis-gismap-1-any-viewer-view-map-config-state-lane-semantics
@comparison-ordered-json-v1
@mutations-gis-gismap-1-any-viewer-view-map-config
Feature: Apply every config state-lane mutation of s.gis.gismap's 👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config to its committed vector
  … (description trimmed: the case records the no-oracle decision and asserts every law inside the subject
     handlers through semio_s_plugin_stdio_test_oracle::law::vector) …

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> vector under 👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through gis_map_viewer_window_config_mutation_report_json
    Then the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved
    Examples:
      | id |
      | set-camera |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> vector under 👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through gis_map_viewer_window_config_mutation_report_json
    Then the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly
    Examples:
      | id |
      | set-camera |

  @id-keep
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Re-applying <id> to a snapshot that already holds its value is a warned no-op
    Given the committed <id> vector under 👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through gis_map_viewer_window_config_mutation_report_json
    Then the snapshot is unchanged and the only diagnostic is mutation.no-op
    Examples:
      | id |
      | set-camera |
```

Fixture folder `🧫️fixtures/🎥️set-camera/✅️set/` (all files read in full):

- `📸️snapshot/⬅️before/🔣️.json`: `{ "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 } }`
- `🦠️mutation/🔣️.json`: `{ "kind": "set-camera", "camera": { "x": 1250.5, "y": -830.25, "zoom": 3.5 } }`
- `📸️snapshot/➡️after/🔣️.json`: `{ "camera": { "x": 1250.5, "y": -830.25, "zoom": 3.5 } }`
- `🔺️diff/🔣️.json`: `{ "camera": { "x": 1250.5, "y": -830.25, "zoom": 3.5 } }`
- `🎯️outcome/🔣️.json`: `{ "status": "applied", "messages": [] }`
- A no-op sibling `🟰️set/` with the same file set.

Rust adapter `🦀️.rs` (abridged; the vector table is `include_str!` of the files above):

```rust
//! … Recorded no-oracle decision …: every law is asserted INSIDE the subject handler through
//! `semio_repo_test_host::law::vector::Vector`. The oracle handlers answer with the committed after/before snapshots.
use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_repo_test_host::law::vector::Vector;
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-camera" => Vector {
            before: include_str!("../../👁️viewer/…/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("…/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("…/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("…/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("…/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}
```

The adapter is not complete here; read `🦀️.rs` in the case folder for the subject handler (`gis_map_viewer_window_config_mutation_report_json`).

Note: this case uses `@no-oracle-…`, so its oracle role answers with the committed fixtures themselves. That is not an independent reference.

### 1.3 Rust test that loads a language-neutral fixture (example with fixture)

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️unit/🦀️.rs` loads `include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/📐️infer-gismap-1/🔣️.json")`, then checks `vectors["subjectSchema"]`, `vectors["schemaVersion"] == 1`, and each `cases[i]`:
- subject: `gis_map_inference` (dispatched through `ArtifactInferenceExecutionRequest`)
- oracle: `geo::MultiPoint::bounding_rect()` from `geo = "0.31.0"` (dev-dependency)
- also `geojson = "1.0.0"` dev-dependency (`geojson` oracle for the rfc8259/geojson writer)

Gismap fixture example, `🧫️fixtures/🧩️map-create-region-group/🔣️.json` (first lines): `{ "schema": "semio.gis.map-create-region-group/v1", "jobId": "1111…", "base": { "positions": [ { "id": "point-a", "data": { "id": "point-a", "lon": 7, "lat": 47 } } ], … }, "expected": { … } }`.

### 1.4 Other fixture conventions

- Owner-level fixtures: `🧫️fixtures/<emoji-kebab>/🔣️.json` (e.g. `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔺️diff/🩹️typed-clear/🔣️.json`).
- Plugin-level fixtures: `✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🎛️mutation-inputs/🔣️.json`.
- Shared assets: `asset://…` URIs in feature steps (e.g. `asset://🎬️demo/🗣️.dsl.semio`), `shared://🧬️mutations` for specification vectors.
- Fixtures are immutable; the runner copies them into a case work directory before touching them (per the IFC differential ticket note).

## 2. Oracles (`🔮️oracles`)

### 2.1 Where oracle registrations live

Plugin-level, `…/🔮️oracles/🔣️.json` with `oracleHostPackages` only (the owner's Python/Rust host package):
- GIS `✏️s/🔌️plugins/🌍️gis/🔮️oracles/🔣️.json`: `oracles: []`, `oracleHostPackages: []`. Says the plugin "carries NO oracle and NO catalog".
- Norm `✏️s/🔌️plugins/📕️norm/🔮️oracles/🔣️.json`: `oracleHostPackages: [{ "implementation": "python", "package": "semio_norm_vocabulary", "path": "✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution", "module": "🐍️" }]`.
- Stdio `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔣️.json`: `oracleHostPackages` = `pypdf 6.14.2`, `simplejson 4.1.1`, `ifcopenshell 0.8.4.post1` (all python, no path = external distribution).
- Draw `✏️s/🔌️plugins/🖍️draw/🔮️oracles/🔣️.json`: `oracleHostPackages: []`; the native `.dsl.semio` format has no third-party reader.
- Energy `✏️s/🔌️plugins/🔋️energy/🔮️oracles/🔣️.json`: `semio-s-plugin-energy-oracle` (path `🔮️oracles/🏃️execution`) and `jsonschema 4.26.0`.

Subset-level, `…/🪆️subsets/<subset>/🔮️oracles/🔣️.json` (note: the file is `🔣️.json` inside `🔮️oracles/`, not `🔣️oracle.json` as the norm root comment says). Example: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json` registers `quick-xml 0.42.0` (MIT, `testOnly: true`, `productionReachable: false`) as a qualifying carrier oracle for SVG.

Counts across `✏️s`: 46 Rust oracle crates under `🔮️oracles/**/Cargo.toml`; 123 Python test adapters `🐍️.py` under `tests`.

### 2.2 Rust oracle example (complete): DXF reader oracle

Crate: `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust/Cargo.toml` (complete):

```toml
[workspace]
members = ["."]

[workspace.metadata.semio.repository]
exclude-patterns = []
schema-version = 1
member-manifests = ["Cargo.toml"]

[package]
name = "semio-s-plugin-stdio-drawing-test-oracle"
version = "0.1.0"
edition = "2021"
publish = false
description = "🖊️ Independent artifact-free DXF semantic reader"

[package.metadata.semio]
role = "test"

[lib]
name = "semio_s_plugin_stdio_drawing_test_oracle"
path = "../../🦀️.rs"

[features]
oracles = ["dep:dxf"]

[dependencies]
"dxf" = { version = "0.6", optional = true }
"semio-repo-test-host" = { path = "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust", default-features = false }

[dev-dependencies]
serde_json = "1"
```

Oracle test `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧪️tests/🔬️semantic/🦀️.rs` (complete):

```rust
use super::project_dxf_r12;
use dxf::entities::EntityType;
use semio_repo_test_host::Json;

#[test]
fn semantic_golden_entities_match_the_independent_dxf_reader() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖊️semantic/🔣️.json")).expect("declared semantic cases");
    for case in cases.as_array().expect("closed case array") {
        let bytes = case["input"].as_str().expect("declared DXF input").as_bytes();
        if let Some(prefix) = case["errorPrefix"].as_str() {
            let error = project_dxf_r12(bytes).expect_err("malformed input refuses");
            assert!(error.starts_with(prefix), "{}: {error}", case["id"]);
            assert!(dxf::Drawing::load(&mut &bytes[..]).is_err());
            continue;
        }
        let drawing = dxf::Drawing::load(&mut &bytes[..]).expect("independent library parses fixture");
        let independent: Vec<serde_json::Value> = drawing.entities().map(|entity| match &entity.specific {
            EntityType::Line(line) => serde_json::json!({ "start": [line.p1.x, line.p1.y, line.p1.z], "end": [line.p2.x, line.p2.y, line.p2.z], "entityKind": "line", "layer": entity.common.layer }),
            _ => panic!("golden cases declare only line entities"),
        }).collect();
        assert_eq!(serde_json::Value::Array(independent), case["entities"], "{}", case["id"]);
        let projected = project_dxf_r12(bytes).expect("owned semantic reader parses fixture");
        // … per-entity comparison of entityKind, layer, start, end (f64 literal equality) …
    }
}
```

Fixture `🧫️fixtures/🖊️semantic/🔣️.json` (verbatim head): `[{ "id": "line", "input": "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n1.0\n20\n2.0\n30\n3.0\n11\n4.0\n21\n5.0\n31\n6.0\n0\nENDSEC\n0\nEOF\n", "entities": [ { "start": [1.0, 2.0, 3.0], "end": [4.0, 5.0, 6.0], "entityKind": "line", "layer": "0" } ] }, { "id": "empty", … }, { "id": "malformed", "input": "not a group code\n", "errorPrefix": "dxf oracle: load failed:" } ]`

Comparison: the oracle output (dxf crate) is compared to the owned reader's projection with `assert_eq!` on JSON values; the case has `🥒️.feature` only if declared through the registry (not verified for this folder).

Document oracle (`🔮️oracles/📃️document/🦀️.rs`): `oracle_create_pdf` uses `pdf-writer` 0.15 and `lopdf` 0.44 behind `#[cfg(feature = "oracles")]`. Public API exposes only owned types (`PdfSpec`, `PdfPageSpec`).

### 2.3 Python oracle examples

- IFC 4 differential: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/`
  - `🥒️.feature` tags: `@capability-ifc-4-any-mutate`, `@oracle-ifcopenshell-ifc-4-any-differential`, `@comparison-semantic-ifc-v1`; scenarios `@mode-differential`.
  - `🐍️.py` imports only `json`, `ifcopenshell` and `semio_repo_test`. It applies each mutation through ifcopenshell 0.8.4.post1 to the real `nakagin-capsule-tower.ifc` (2 496 437 bytes, 24 792 entities), re-serializes via `ifcopenshell.file.to_string`, and reads back with its own ISO 10303-21 reader before the `semantic-ifc-v1` comparison.
  - Manifest: `…/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🔣️.json` registers `ruststep 0.4` (reader only). The ticket note says ruststep is reclassified as a cross-semio implementation, not an oracle, because its expected results were computed by the owner's `🦀️oracle.rs`.
- Norm: `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py` is one independent Python engine (`semio_norm_vocabulary`) imported by 15 norm subset adapters. Its docstring says PyPI has no en1990/din18599/vdi3805/iso16757 packages and the nearest ones (`structuralcodes`, `concreteproperties`, `anastruct`) implement formulas, not the interchange format. So it is a second implementation, not a third-party oracle.
- Energy: `✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python/pyproject.toml` is a standalone uv project, `requires-python = ">=3.12,<3.13"`, with `default-groups = ["test"]`: `honeybee-energy==1.123.32`, `honeybee-openstudio==0.7.2`, `jsonschema==4.26.0`, `ladybug-core==0.44.59`, `openstudio==3.11.0`. Run via `bun ./📜️script.ts setup|status|run|native|epjson|emit|test` in the same folder.

### 2.4 How external libraries are provisioned and compared

- `path` set: local in-repo source. The Rust host writes a Cargo.toml with a path dependency (crates.io coordinates refused).
- `path` absent: external distribution. The Python host builds a cache venv under `.🧬semio/🦑️repo/⚡️cache/tests/hosts/`, keyed by base interpreter and package set. It uses `semio-base-interpreter.pth` to reuse the repo `.venv`.
- TypeScript: resolved from repo `node_modules`.
- Comparison profiles: `@comparison-ordered-json-v1`, `@comparison-semantic-ifc-v1` (tags on the feature); `comparisonProfiles` list in the manifest; `law::vector` in Rust; `law::mutation_is_observable`.
- Purity: oracle packages must not be production-reachable; `🔒️dependencies.json` classifies them as `test-oracle`.

### 2.5 Python libs available

`pyproject.toml` (root, requires-python `>=3.14,<3.15`) test group: `pytest`, `pytest-cov`, `pytest-timeout`, `deepdiff`, `jsonpatch`, `jsonschema`, `mercantile`, `networkx`, `shapely 2.1.2`, `lxml`, `python-pptx`, `python-docx`, `openpyxl`, `html5lib`, `steputils`, `ifcopenshell==0.8.4.post1`, `anastruct`, `scikit-fem`, `PyNiteFEA`. Dev group: jupyter, ruff, black, pandas, numpy 2.5.0, matplotlib, seaborn, scipy, scikit-learn.

Actual `.venv\Lib\site-packages` (python 3.14.4, `include-system-site-packages = false`): present: numpy 2.4.3, scipy 1.17.1, scikit-learn 1.8.0, pandas 3.0.0, matplotlib 3.10.8, pytest 9.0.2, pytest-cov, jsonschema 4.26.0, deepdiff 8.6.2, black, ruff, jupyter stack, lark. Absent: ifcopenshell, shapely, pypdf, simplejson, jsonpatch, lxml, python-pptx, python-docx, openpyxl, anastruct, scikit-fem, PyNiteFEA, steputils, mercantile, networkx, trimesh, sympy, cadquery, OCP. So the venv is out of sync with pyproject; run `uv sync` before any ifcopenshell oracle.

Geometry: no trimesh, sympy, cadquery or OCP in any lock or manifest. BREP is checked with Rust crates (parry3d and topol are mentioned in ticket `26/10/05-06 FIXTURES-ARE-TESTING-EXAMPLES-ONLY/current-embedded-eight-closure`).

## 3. Running plugin tests

### 3.1 Plugin nx targets (gismap Rust package)

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/📋️project.json` (nx project `@semio-tech/gis-gismap-rs`) has `build`, `check`, `test` (all `bun ./📜️script.ts <cmd>`), and `verify-*`, `test-snapshot-sqlite*`. Example:

```json
"test": {
  "executor": "nx:run-commands",
  "options": {
    "cwd": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust",
    "command": "bun ./📜️script.ts test",
    "forwardAllArgs": true
  }
}
```

`package.json` (same folder) has only `verify-*` scripts that call `bun nx run …`.

TS sibling `…/🗿️gismap/📦️packages/🟦️typescript/📋️project.json` has `inference-check`, `cold-document-pair-check`, `inference-presentation-check`, `inference-bridge-*-check`.

Rust script: `…/📦️packages/🦀️rust/📜️script.ts` extends `BundleScript`. Tests run native: `runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--lib", filter, "--", "--nocapture"], this.repoRoot)`.

Plugin-level: `…/📦️packages/🟦️typescript/📜️script.ts` and `🏭️bridge/📜️script.ts` are routers via `ScriptRouter` + `runScriptMain`.

### 3.2 Repo-level targets (`📋️project.json` root)

- `test`: `bun ./📜️script.ts test`, `dependsOn` every project's `test` (excluding `workspace`, `@semio-tech/repo-test-domain`).
- `test-fundamental`, `test-quick`, `test-long`, `test-exhaustive`: same pattern per level, `cache: true`.
- Harness project `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` (`@semio-tech/repo-test-domain`) exposes `test-discover`, `test-contract`, `test-oracle`, `test-subject`, `test-parity`, `test-report`, `test-doctor`, `test-inventory`, and many others. Per-case Nx targets are generated (`test`, `test-quick`, `test-long`, `test-exhaustive`, `test-contract`, `test-oracle`, `test-subject`, `test-parity`).
- Commands (from the harness README): `bun ./📜️script.ts discover | doctor | contract | oracle <level> | subject <level> | parity <level> | run <level> | report | clean | dependency | inventory | fixture | probe | matrix | gc`.
- Level tag: `SEMIO_TEST_LEVEL` env (launch entries use `quick`).

### 3.3 Cargo invocations

- Plugin crates in `✏️s/Cargo.toml` (`[workspace] resolver = "2"`, `members` ~219 `…/📦️packages/🦀️rust` entries, `exclude` of `**/🏅️standards/**`, `**/🔮️oracles/**`, `**/👽️guest/**`).
- Root `Cargo.toml` excludes `"✏️s"` and the same globs. The repo has no `[workspace]` member for the plugins. Nested `[workspace.dependencies]` in root and `✏️s` both exist.
- `runCargo` (library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:2224`) calls `selectedCargoArguments(root, args)` (`🗂️workspaces/🦀️cargo/🟦️.ts:171`), which routes `-p <name>` to the workspace that owns the package, unless `--manifest-path` is given.
- wasm target for plugins: `cargo check -p <crate> --target wasm32-wasip2 --message-format=short`. Repo `rust-toolchain.toml` lists targets `wasm32-unknown-unknown` and `wasm32-wasip2`. The `verify rust-warnings` gate uses `--target wasm32-wasip2` with `--lib` for plugin packages (root `📜️script.ts:646`).
- Tests are native: `cargo test -p <crate> <filter>`. No wasm test run.
- `.cargo/config.toml` (repo root): `[build] target-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/target"`, `build-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/build"`, `rustflags = ["-Z","threads=8"]`, unstable `build-dir-new-layout`, `fine-grain-locking`, `checksum-freshness`. Target-specific rustflags for `wasm32-wasip2`: `--max-memory=536870912`, `-zstack-size=8388608`.
- `RUSTC_WRAPPER`: not set in repo config. It is cleared in code at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/…/fresh-component/🟦️.ts:71` (`["RUSTC_WRAPPER", ""]`, `CARGO_INCREMENTAL=0`) and stripped in `📚️library-search-path` tests. In ticket notes, `RUSTC_WRAPPER="" cargo check -p … --offline` is prefixed by agents (QUIZ-PETS, `26/10/02`). LOWPOLY (`26/08/29`) says `RUSTC_WRAPPER=""` was tried and OOM-killed twice with ~60 MB free. Treat it as an ad hoc workaround, not a convention.

### 3.4 Build time evidence (from ticket notes, not measured here)

- Cold `wasm-release` cdylib of `semio-s-plugin-gis` (name as written in that note, now stale): 23 min 24 s (`tc3c-n-plugin-bootstrap-and-note-creation.md`).
- Same crate in a later run, 0 errors, 2 min 27 s (`tc3e-three-package-hub-and-note-creation.md`; cache state not stated).
- `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --target wasm32-wasip2`: 2 min 39 s (`W1-A`).
- `cargo check -p semio-s-artifact-sequence-sequence`: 1 min 04 s (native).
- `cargo check -p semio-framework-plugin --lib` under load: 5 min 59 s.
- `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target-s-e2e cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --keep-going`: 86 min 52 s, EXIT 0 (`S-END-TO-END/stdio-check-census.md`; cache state not stated).
- Estimate for a new `bim` artifact crate on `wasm32-wasip2`: 2–3 min warm, 20–25 min cold, more under concurrent load.

### 3.5 The build gate used by recent tickets

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES/🚦️gate.sh` (verbatim, 916 bytes):

```bash
#!/usr/bin/env bash
# 🚦️ Build gate: blocks until fewer than MAX_RUSTC rustc processes run and swap has headroom, then holds one of 4 slots.
# Usage: "$T/🚦️gate.sh" <label> -- <command...>
set -u
label="$1"; shift; [ "${1:-}" = "--" ] && shift
dir="$(cd "$(dirname "$0")" && pwd)/🗑️generated/gate"; mkdir -p "$dir"
max_rustc="${MAX_RUSTC:-10}"
while :; do
  for slot in 1 2 3 4; do
    if mkdir "$dir/slot-$slot" 2>/dev/null; then
      if [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt "$max_rustc" ]; then
        echo "$label $$ $(date +%T)" > "$dir/slot-$slot/owner"
        trap 'rm -rf "$dir/slot-'"$slot"'"' EXIT
        CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-3}" "$@"; exit $?
      fi
      rm -rf "$dir/slot-$slot"
    elif [ -f "$dir/slot-$slot/owner" ] && ! kill -0 "$(awk '{print $2}' "$dir/slot-$slot/owner")" 2>/dev/null; then
      rm -rf "$dir/slot-$slot"
    fi
  done
  sleep 15
done
```

Executor brief rule (`📋️executor-brief.md:33`): every cargo call through the gate, foreground only, never `CARGO_TARGET_DIR`; plugins `cargo check -p <crate> --target wasm32-wasip2 --message-format=short`, then `cargo test -p <crate> <filter>`.

## 4. launch.json

### 4.1 Files and status

- `.vscode/launch.json` (3.3 MB, tracked in git): the AGENTS.md registry. Groups by `presentation.group` (counts): `4_gate` 3200, `3_dev` 639, `9_gates` 453, `4_build` 260, `🧹clean🛡️gates` 29, `4_test` 6, `0_dev`/`2_mouse`/`1_keyboard` few. Config types: `node-terminal`.
- `.vscode/🧩️launch.seed.jsonc` (2.7 MB, tracked): the authored seed. The same entries as the launch.json in the same order.
- `.claude/launch.json` (18 KB, tracked): the Claude preview config, `version 0.0.1`, `runtimeExecutable`/`runtimeArgs`/`port`/`env` (79 entries). Used by `preview_start`.
- Ticket DASHBOARD-LAUNCH-COCKPIT (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/`): `fleet-plan.md:14` states that no file named `launch.json` or `launch.seed.jsonc` exists and nothing generates, reads or documents one (target state). The frozen-seal ledger (`🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154-155`) records both files as `consumerDeletions` with reason "The dashboard is the only developer control plane". Memory note says this is the 10-08 goal ("Dashboard sole control plane"). This conflicts with AGENTS.md ("register all executable commands there"). Confirm with the owner before registering BIM commands in `.vscode/launch.json`.

### 4.2 Naming and ordering convention (from entries)

- Test entries: `"name": "🧪️test<plugin-emoji-segments>📜️<ticket-or-view>🦀️<adapter>"`, e.g. `🧪️test🗄️stdio📜️committed🦀️current`.
- Dev entries: `"name": "🛠️dev<plugin>…⚛️react"` or `…🧊️wgpu🌐️wasm`, e.g. `🛠️dev🌍️gis🗺️gismap⚛️react`.
- Gate entries: `⚖️gate…`, `🧫️fixtures-testing-only-…🧪️`.
- Name grammar: emoji for plugin/artifact, then a short kebab or emoji word, then renderer/adapter emoji (`⚛️react`, `🧊️wgpu`, `🌐️wasm`, `🦀️native`, `🟦️source`).

Test entry (verbatim, from `.vscode/launch.json` line 680):

```json
{
  "name": "🧪️test🗄️stdio📜️committed🦀️current",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- bun \".🧬semio/…/NON-DESTRUCTIVE-HISTORY-EDITING/native-matrix/📜️script.ts\" run \"semio-s-artifact-stdio-ifc,…\" assembly",
  "cwd": "${workspaceFolder}",
  "env": { "NX_DAEMON": "false", "NEXTEST_SUCCESS_OUTPUT": "immediate" },
  "presentation": { "group": "9_gates", "order": 900.0581154 }
}
```

Dev entry (verbatim, from `.vscode/launch.json` line 1095):

```json
{
  "name": "🛠️dev🌍️gis🗺️gismap⚛️react",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run workspace:dev -- gis2d",
  "cwd": "${workspaceFolder}",
  "env": { "S_OS_PORT": "6040", "SEMIO_PLUGIN": "gis2d", "SEMIO_RENDERER": "react", "SEMIO_APP": "s.gis.gismap@1/*#editor" },
  "presentation": { "group": "3_dev", "order": 160 },
  "serverReadyAction": { "action": "openExternally", "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6040)", "uriFormat": "%s" }
}
```

Preview entry (`.claude/launch.json`, line 553):

```json
{
  "name": "norm-react",
  "runtimeExecutable": "bun",
  "runtimeArgs": ["nx", "run", "@semio-tech/framework-os-dev:serve-din4108-react-dev"],
  "port": 6091,
  "env": { "SEMIO_RENDERER": "react", "S_OS_PORT": "6091" }
}
```

Plugin dev lookup: `bun nx run workspace:dev -- <plugin>` → root `dev` target (`bun ./📜️script.ts dev`, `forwardAllArgs`).

## 5. `✏️s/🧑‍💻dev/*`

These are the dev host layer for the plugins, not test folders. Contents found:
- `🌊️flow/`, `🗄️stdio/`, `📐️cad/`, `📕️norm/`, `🏗️fem/`, `📋️forms/`, `🗒️note/`, `🧩️puzzle/`, `🧱️block/`, `🪐️space/` and others. Each has `📦️packages/…` (`🟦️typescript/📋️project.json` + `📜️script.ts`, or `🦀️rust/`), `🧪️tests/`, `🧫️fixtures/`, sometimes `🧬️schema/`.
- Shared dev composition: `🚀️entry/🟦️.ts` boots the OS dev host (`bootFrameworkOsDev({ brands: [], documentServices: [GIS_INFERENCE_PRESENTATION_V1], backboneWorkerFactory: () => new Worker(…"../🧩️service-composition/👷️worker/🟦️.ts"…), surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES })`).
- `🗄️stdio/🟦️.ts` is a TS barrel: `export * as json from "@semio-tech/stdio-json"` (and similar per format).
- Packages: `🗄️stdio/📦️packages/🟦️typescript/📋️project.json` (nx name `@semio-tech/stdio-js`) has `build`, `check`, `test`, `package-contract`, `package-graph`, and named inputs for the composition.
- Setup pattern: a `📦️packages/<lang>/📋️project.json` with `targets.build|check|test` calling `bun ./📜️script.ts <cmd>`; the `📜️script.ts` is a `ScriptRouter` (`register("build", …)`). Root `workspace:dev` starts the host.
- Not verified: the exact mapping from each `🧑‍💻dev/<plugin>` folder to its launch entry.

## 6. Other notes and gaps

- AGENTS.md (checked in) vs current state: launch.json registry (AGENTS) vs dashboard deletion (ticket). Surface to the owner.
- Oracle manifest name: the norm root comment and the README mention `🔣️oracle.json`. The actual files are `🔮️oracles/🔣️.json` inside each subset.
- AGENTS.md says every feature needs a language-agnostic test and a third-party reproduction. Repo state: 164 features are `@no-oracle-…` (e.g. gismap viewer config, drawing). The no-oracle decision is "debt, not verdict" in the repo's own words.
- For BIM: the closest existing pattern is `ifcopenshell 0.8.4.post1` as a Python differential oracle (stdio IFC 4 case). It is already pinned in the root `pyproject.toml` test group and `uv.lock`, and in stdio `oracleHostPackages`. A BIM IFC export could reuse it.
- For BIM geometry: no shapely-free geometry oracle besides `geo` (Rust, GIS) and `parry3d`/`topol` (BREP ticket). Shapely 2.1.2 is locked but not installed in `.venv`.
- Name collision: `🌊️flow/🧩️extensions/🏗️bim` (crate `semio-s-plugin-flow-extension-bim`) already exists.
- Not checked: the full `🧪️tests` Go and .NET adapters; `🩺️environment` doctor checks; whether `bun ./📜️script.ts discover` runs in this session (not run, no builds run).
