# Nonframework Testing-Taxonomy Acceptance Audit

Read-only acceptance audit of `✏️s/🔌️plugins`, `🌎️hub`, and root runner/configuration files on 2026-09-12. No source was edited and no build or test was run.

## Active `CARGO_MANIFEST_DIR` Defects

Ten active Rust test joins retain the pre-artifact-package prefix. Each source is compiled by the listed artifact package (`[lib] path = "../../🦀️.rs"`), so `CARGO_MANIFEST_DIR` is its `📦️packages/🦀️rust` directory. Every current literal resolves to a missing duplicated `🗿️artifacts/<artifact>/🗿️artifacts/<artifact>` path. Every replacement below resolves to a live target.

```json
[
  {
    "source": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs:164",
    "manifest": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:9",
    "manifest": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🧪️tests/🔬️unit/🦀️.rs:108",
    "manifest": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures"
  },
  {
    "source": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:6",
    "manifest": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:5",
    "manifest": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:6",
    "manifest": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:5",
    "manifest": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:5",
    "manifest": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets"
  },
  {
    "source": "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs:6",
    "manifest": "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
  },
  {
    "source": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🦀️.rs:604",
    "manifest": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/Cargo.toml",
    "oldLiteral": "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets",
    "newLiteral": "../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🖼️assets"
  }
]
```

## Taxonomy and Runner Results Before Those Fixes

- No populated `🔬️testkit`, singular `🔮️oracle`, or singular `🧪️oracle` path remains under the plugin or hub trees.
- No fixture payload remains below `🧪️tests/<case>/`; no test implementation remains below an extra child directory under its test case.
- No actual Rust `#[test]` attribute is outside `🧪️tests`. A regex hit in Remodel is prose only. The AVI oracle's `#[cfg(test)] include!("🧪️tests/🔬️oracles-unit/🦀️.rs")` has an existing canonical target and is intentionally retained.
- Every Rust `#[path = "…"]` reference that names a test or formerly noncanonical testing root resolves to an existing file.
- Active `project.json`, `package.json`, `📜️script.ts`, and `nx.json` contain no reference to a removed `contract.ts`, `🔬️testkit`, singular oracle, nested `🧪️fixtures`, or former `🧪️tests/<case>/🔬️unit` shape.
- The concurrently repaired CAD config fixture and Sourcing Curation schema snapshot/diff files exist. The Sourcing document-contract test's `include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")` resolves to an existing fixture.

The reported CARGO-manifest defects are executable test data/owner discovery paths, not historical documentation or negative vectors.
