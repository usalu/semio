# Native Layout Normal Source Inventory

2026-10-28. Canonical readRustSourceClosureV1; macOS ARM64/unix cfg without cfg(test), with testkit; bounded layout artifact tree only. No compiler, graph or test launched. Normal entry means each Cargo lib target with cfg(test) disabled; test-only companions may consequently have an empty closure. Global bounds:32inputs,262144bytes,4000active lines.

Inspected 34 library manifests. Failing owners: 0.

```json
[
  {
    "name": "semio-s-artifact-layout-layout-host",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔨️modules/🏠️host/🧰️owned/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 3446,
    "lines": 66
  },
  {
    "name": "semio-s-artifact-layout-drawing-composition",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔨️modules/🧩️drawing-composition/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 431,
    "lines": 5
  },
  {
    "name": "semio-s-artifact-layout-layout-examples",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust/Cargo.toml",
    "files": 3,
    "bytes": 3276,
    "lines": 30
  },
  {
    "name": "semio-s-artifact-layout-layout-sqlite-member-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 3797,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-io",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml",
    "files": 32,
    "bytes": 253706,
    "lines": 1732
  },
  {
    "name": "semio-s-artifact-layout-layout-descriptor",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📇️descriptor/📦️packages/🦀️rust/Cargo.toml",
    "files": 16,
    "bytes": 164485,
    "lines": 27
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-content-tests-2",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/🧪️tests/🍰️companion-2/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 2869,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-content-tests-1",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/🧪️tests/🍯️companion-1/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 3037,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-content",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/📦️packages/🦀️rust/Cargo.toml",
    "files": 26,
    "bytes": 108391,
    "lines": 1615
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-content-2",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩️content-2/📦️packages/🦀️rust/Cargo.toml",
    "files": 15,
    "bytes": 62556,
    "lines": 1008
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-frames-tests-2",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/🧪️tests/🍰️companion-2/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 2530,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-frames-tests-1",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/🧪️tests/🍯️companion-1/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 2572,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-frames",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/📦️packages/🦀️rust/Cargo.toml",
    "files": 9,
    "bytes": 41244,
    "lines": 686
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-frames-2",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️frames-2/📦️packages/🦀️rust/Cargo.toml",
    "files": 12,
    "bytes": 61334,
    "lines": 1079
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml",
    "files": 30,
    "bytes": 253539,
    "lines": 1116
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-decisions",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️decisions/📦️packages/🦀️rust/Cargo.toml",
    "files": 18,
    "bytes": 170212,
    "lines": 694
  },
  {
    "name": "semio-s-artifact-layout-layout-descriptor-diff",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📇️descriptor/📦️packages/🦀️rust/Cargo.toml",
    "files": 6,
    "bytes": 111627,
    "lines": 12
  },
  {
    "name": "semio-s-artifact-layout-layout-diff",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml",
    "files": 28,
    "bytes": 244422,
    "lines": 2439
  },
  {
    "name": "semio-s-artifact-layout-layout-diff-plan",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🧭️plan/📦️packages/🦀️rust/Cargo.toml",
    "files": 6,
    "bytes": 37328,
    "lines": 227
  },
  {
    "name": "semio-s-artifact-layout-layout-model",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml",
    "files": 31,
    "bytes": 223479,
    "lines": 2552
  },
  {
    "name": "semio-s-artifact-layout-layout-viewer",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml",
    "files": 5,
    "bytes": 13252,
    "lines": 287
  },
  {
    "name": "semio-s-artifact-layout-layout-editor-panels",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml",
    "files": 13,
    "bytes": 122241,
    "lines": 1614
  },
  {
    "name": "semio-s-artifact-layout-layout-editor-commands-2",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪜️commands-2/📦️packages/🦀️rust/Cargo.toml",
    "files": 21,
    "bytes": 54228,
    "lines": 950
  },
  {
    "name": "semio-s-artifact-layout-layout-editor-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 50417,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-editor-commands-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 2716,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-editor-commands",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml",
    "files": 26,
    "bytes": 184821,
    "lines": 2697
  },
  {
    "name": "semio-s-artifact-layout-layout-editor",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",
    "files": 17,
    "bytes": 210538,
    "lines": 2655
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-scene-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🎬️scene/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 2389,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-flow-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🧵️flow/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 620,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-engine",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust/Cargo.toml",
    "files": 20,
    "bytes": 249092,
    "lines": 2225
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-json",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json/📦️packages/🦀️rust/Cargo.toml",
    "files": 12,
    "bytes": 118236,
    "lines": 2182
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-export-unit-tests",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 1617,
    "lines": 0
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-export",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust/Cargo.toml",
    "files": 22,
    "bytes": 245992,
    "lines": 3008
  },
  {
    "name": "semio-s-artifact-layout-layout",
    "manifest": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml",
    "files": 1,
    "bytes": 10313,
    "lines": 173
  }
]
```

Input script retained as native-inventory.ts. This measures package source ownership only, not transitive external package dependency readiness or compiler correctness.
