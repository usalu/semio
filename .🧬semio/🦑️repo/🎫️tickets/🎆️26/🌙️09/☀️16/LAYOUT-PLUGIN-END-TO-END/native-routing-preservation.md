# Native Routing Preservation Audit

2026-10-28. Fresh bounded read-only snapshot. No source edits, test, compiler or graph. Python tomllib parsed the current nested workspace and package manifests, not stale report copies. SHA256 values below identify the exact snapshot; any subsequent mutation must reread and compare current content rather than replace this complete snapshot.

## Current Test Route

- semio-s-artifact-layout-layout-engine
- semio-s-artifact-layout-layout
- semio-s-artifact-layout-layout-editor-tests
- semio-s-artifact-layout-layout-engine-scene-tests
- semio-s-artifact-layout-layout-engine-flow-tests
- semio-s-artifact-layout-layout-editor-commands-tests
- semio-s-artifact-layout-layout-mutations-content-tests-1
- semio-s-artifact-layout-layout-mutations-content-tests-2
- semio-s-artifact-layout-layout-mutations-frames-tests-1
- semio-s-artifact-layout-layout-mutations-frames-tests-2
- semio-s-artifact-layout-layout-diff-plan
- semio-s-artifact-layout-layout-sqlite-member-tests
- semio-s-artifact-layout-layout-engine-export-unit-tests

All13current entries are unique and all13expected prior/additive entries are preserved. The formerly lost diff-plan, sqlite-member-tests and engine-export-unit-tests are now restored. Mutations-decisions is a production owner rather than a previously declared test addition; no request to silently drop existing test packages is implied.

## Six New Owner Checks

```json
[
  {
    "name": "semio-s-artifact-layout-layout-editor-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 14,
    "owner_rows": 1,
    "absolute_manifest_length": 156,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-scene-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 19,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-flow-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 5,
    "owner_rows": 1,
    "absolute_manifest_length": 165,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-diff-plan",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 2,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-sqlite-member-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 5,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-decisions",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 9,
    "owner_rows": 1,
    "absolute_manifest_length": 176,
    "issues": []
  }
]
```

## Routing Issues

None: exact package workspace roots and member membership remain correct; six new owners retain one matching build-owner row and all declared dependency aliases/relative paths resolve.

## Snapshot Hashes

```json
{
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/📜️script.ts": "ab3af06efebf7efdf5cbf49b42b05455087c0f09765d9fc829220a9a22bdd114",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml": "f6445c07e7c7341e7d8811af9c8011277cd062e2a8dc1c7ebe4948db6188c5ec",
  "📋️build-owners.json": "273873fe844738f79bdaa9c98699c2eee89d27810f00003d71427c390d636353",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️member/📦️packages/🦀️rust/Cargo.toml": "fc7372f97657140ddc69b960a6875c77db0b3c1fe5f4c2034080260c2b890a61",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/🧪️tests/🍰️companion-2/📦️packages/🦀️rust/Cargo.toml": "622cd38f970be8c3281384b743f7c4da3a25b1c6d0ce494da1348e3008904cea",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️content/🧪️tests/🍯️companion-1/📦️packages/🦀️rust/Cargo.toml": "6478092d292f9e166eb7cac33c93d518fc8fe08655a193fda8fd6201d90ec324",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/🧪️tests/🍰️companion-2/📦️packages/🦀️rust/Cargo.toml": "379c9b50eacfef245abf0f1d4f1902c405c7e840d7162cec1a1394dd6974ecf6",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️frames/🧪️tests/🍯️companion-1/📦️packages/🦀️rust/Cargo.toml": "44463a88dab4acb031c6b203313be6ea7faf4ac2fbc5f431f64c0ca24ea97894",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🧭️plan/📦️packages/🦀️rust/Cargo.toml": "ce90dda13014b7ce162a289da05a173d6d3a762834605c0cd29af41f37a8da19",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml": "c8d901eacadf71cfc3e7fe723849f818c5490b2b7d858baedb4cf456c7c0cc96",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml": "5b395febb5c65bb4483d498b5d85c701b798ded00c34c1382811370c7d4295e3",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🎬️scene/📦️packages/🦀️rust/Cargo.toml": "358b630101e09f39598c7f9a5b9b7415ccad75f6dc3b86c125f7fd6751cf46a1",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🧵️flow/📦️packages/🦀️rust/Cargo.toml": "dde1d25004afc2ba3d0dd26d8e42fae92f4bfdc2e3775fdb57aaac599a60ec3a",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust/Cargo.toml": "1bf0070f336b4a3dd1089a218c20ec0b544b74864944507e9cb35f0ddec1cf9e",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml": "4031d339934275d5c05afd8e563cae62d7faf259c870afbaa128c11eb53f2996",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust/Cargo.toml": "8a0800fa34088c0417fa612a674fbeb9b1ff37c84deb05e6fa3770b7a8d44903"
}
```

## Canonical Selected Test Mount Check

Current13selected cfg(test) owners measured through readRustSourceClosureV1. Duplicate test-source ownership among selected packages:

```json
[
  [
    "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/📕️pdf-structure/🦀️.rs",
    [
      "semio-s-artifact-layout-layout-editor-commands-tests",
      "semio-s-artifact-layout-layout-engine-export-unit-tests"
    ]
  ]
]
```

Unselected broad IO/diff owners intentionally are not treated as executed duplicates. Existing runtime/editor testkit context modules are shared support rather than duplicate behavior cases.

The only shared selected test-source path above is the PDF structure utility. Its body contains 0test attributes; utility reuse must be distinguished from executing the same behavioral suite twice. No duplicate runtime implementation was introduced by the companion ownership changes.
