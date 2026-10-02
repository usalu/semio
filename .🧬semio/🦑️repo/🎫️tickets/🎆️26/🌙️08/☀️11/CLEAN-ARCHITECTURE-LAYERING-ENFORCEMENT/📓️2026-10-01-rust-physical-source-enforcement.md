# Rust Physical Source Direction Enforcement

## Implemented Cut

Reused the repository-owned discovery Rust tokenizer, balanced delimiter parser, compile-reference scanner and module graph. Extended the existing source direction owner with one shared target resolver and a physical lstat census for every referenced authored input and ancestor. Linked data files and linked directories/junctions cannot conceal product ownership. Missing inputs, non-directory ancestors, wrong input kinds and uncensused Rust sources fail closed. All framework source, tests, macro templates and configurations retain their existing strict direction rules; generated OUT_DIR inputs retain their explicitly declared generated provenance. No Cargo edges, general Rust implementation, fixtures assertions or source functionality were removed.

## Contract And Independent Oracle

The closed schema and language-neutral fixture now include nine physical census cases, native Windows separator and absolute/UNC/traversal forms, multiline raw-string module mounts, and a typed full-report contract. Existing raw strings, comments, char literals, conditional module mounts, nested/include module bases, unused literal macro templates and unsupported expressions remain covered. Independent rustc dep-info confirms actual compiler reads, including an authored linked product data input. pathe supplies independent portable path interpretation and Ajv validates fixture/report schemas. The Windows live link oracle uses an unprivileged junction; other native environments use a directory symlink.

## Executed Validation

- RED: actual Bun/Nx test-rust-source-direction failed before the live input inspector existed: exported inspectRustSourceInputs was absent.
- GREEN: Bun/Nx test-rust-source-direction passed all seven tests, 121 assertions, with rustc and pathe independent oracles; 1.92 seconds test runtime and 2.5 seconds Nx task runtime in the final rerun.
- Actual Bun/Nx lint-rust-source-direction remained RED: 2330 inventoried Rust files; 4312 authored compile-time references; 8 strict direction violations; 2 physical/source problems.
- Actual emitted typed report was separately accepted by the committed report schema through Ajv strict mode.

No timeout was increased. Targets continue routing through the owning package 📜️script.ts. Existing noncached source test/lint targets now declare their policy/parser/schema/source input closure. Seed entries 900.05747 and 900.05748 expose portable and live gates; generated launch is left to the genuine generator.

## Actual Typed Verdict

```json
{
  "schemaVersion": 1,
  "files": 2330,
  "references": 4312,
  "violations": [
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️standalone/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "kind": "include_str",
      "line": 44
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs",
      "kind": "path",
      "line": 13
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
      "kind": "include_str",
      "line": 129
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖼️scene-raster-ownership/🔣️.json",
      "kind": "include_str",
      "line": 14
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️component-unit/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎛️inline-tree-controls/🔣️.json",
      "kind": "include_str",
      "line": 168
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧫️fixtures/🔣️.json",
      "kind": "include_str",
      "line": 54
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧫️fixtures/🔣️.json",
      "kind": "include_str",
      "line": 438
    },
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🧬️schema/🌐️document-http/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json",
      "kind": "include_str",
      "line": 21
    }
  ],
  "problems": [
    {
      "code": "unsupported-expression",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs",
      "detail": "Unsupported Rust compile expression at line 595: include_str"
    },
    {
      "code": "missing-input",
      "to": "🧰️framework/🔨️modules/🌱️value/📦️paged/🧪️tests/🔬️unit/🦀️.rs",
      "kind": "path",
      "line": 372,
      "from": "🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs",
      "detail": "Authored compile input failed physical census: 🧰️framework/🔨️modules/🌱️value/📦️paged/🧪️tests/🔬️unit/🦀️.rs"
    }
  ]
}
```

The missing Value paged unit-test mount was physically absent at scan time during concurrent Value work. Its source points to the correct local relative test path; the directory existed but held no Rust file. The gate preserves this truthful census result for the owner to repair. The unsupported product derive input is quote-emitted include_str with an unbound interpolation; no waiver or false parser inference was introduced.

## Files Touched

- Repo library 🕸️dependencies/🧭️direction/🦀️source/🟦️.ts
- Repo library 🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts
- Repo library 🧬️schema/🧱️rust-source-direction/🔣️.json
- Repo library 🧫️fixtures/🧱️rust-source-direction/🔣️.json
- Repo library 🧪️tests/🧱️rust-source-direction/🟦️.ts
- Repo library 📦️packages/🟦️typescript/📜️script.ts
- Repo library 📦️packages/🟦️typescript/📋️project.json
- .vscode/🧩️launch.seed.jsonc
- This retained ticket report
