# Current Fixture and Test Layout Snapshot

Completed at 2026-09-09T09:53:29.070Z. Authored file paths inspected: 114137. This snapshot covers physical layout; production dependency and fixture-content classification require their separate audits.

```json
{
  "counts": {
    "production-fixture-dependency": 31,
    "invalid-test-module-wiring": 1,
    "inline-test-body": 31,
    "test-implementation-depth": 2,
    "inline-self-test-declaration": 1,
    "legacy-fixture-directory": 5,
    "test-case-name": 15,
    "fixture-owner-delivery-scope": 1
  },
  "findings": [
    {
      "code": "production-fixture-dependency",
      "path": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs",
      "line": 44,
      "detail": "Non-test source resolves a dependency on fixture ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "invalid-test-module-wiring",
      "line": 34,
      "detail": "External Rust test modules must use #[path] to an existing canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 21,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🦀️.rs",
      "line": 23,
      "detail": "Non-test source resolves a dependency on fixture ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-implementation-depth",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🦀️.rs",
      "line": null,
      "detail": "Executable test sources must be direct children of 🧪️tests/<test-name>."
    },
    {
      "code": "inline-self-test-declaration",
      "line": 6,
      "detail": "Self-test declaration testRewritingDocumentRetirementOracle is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🟦️.ts"
    },
    {
      "code": "production-fixture-dependency",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🟦️.ts",
      "line": 7,
      "detail": "Non-test source resolves a dependency on fixture ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-implementation-depth",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🟦️.ts",
      "line": null,
      "detail": "Executable test sources must be direct children of 🧪️tests/<test-name>."
    },
    {
      "code": "inline-test-body",
      "line": 353,
      "detail": "Inline Rust test modules must move to a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 363,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 377,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs"
    },
    {
      "code": "legacy-fixture-directory",
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🧫️fixture/♻️lifecycle/🔣️.json",
      "line": null,
      "detail": "Fixture examples must use the canonical 🧫️fixtures directory. Test implementations and executable support must use their own semantic scopes."
    },
    {
      "code": "legacy-fixture-directory",
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🧫️fixture/🔣️.json",
      "line": null,
      "detail": "Fixture examples must use the canonical 🧫️fixtures directory. Test implementations and executable support must use their own semantic scopes."
    },
    {
      "code": "inline-test-body",
      "line": 179,
      "detail": "Inline Rust test modules must move to a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 183,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 365,
      "detail": "Inline Rust test modules must move to a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 369,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 382,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 395,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 289,
      "detail": "Inline Rust test modules must move to a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 293,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 304,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 212,
      "detail": "Inline Rust test modules must move to a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 216,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "inline-test-body",
      "line": 227,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
    },
    {
      "code": "legacy-fixture-directory",
      "path": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
      "line": null,
      "detail": "Fixture examples must use the canonical 🧫️fixtures directory. Test implementations and executable support must use their own semantic scopes."
    },
    {
      "code": "legacy-fixture-directory",
      "path": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🧬️.schema.json",
      "line": null,
      "detail": "Fixture examples must use the canonical 🧫️fixtures directory. Test implementations and executable support must use their own semantic scopes."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts",
      "line": 11,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🔨️modules/🌱️value/💾️resident/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts",
      "line": 15,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts",
      "line": 11,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧫️fixtures/🔢️numeric-index.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/📜️script.ts",
      "line": 13,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧫️fixtures/🔗️references.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🧫️fixture-🧬️mutations-🔢️set-value/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🧫️fixture-🧬️mutations-🔢️set-value/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧫️fixtures/🧬️mutations/🔢️set-value/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🧫️fixture-🧬️mutations-🔢️set-value/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🦀️.rs",
      "line": 3,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🦀️.rs. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🦀️.rs",
      "line": 5,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🪤️lossy/🧬️mutations/🦀️.rs. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🦀️.rs",
      "line": 7,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🦀️.rs. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🦀️.rs",
      "line": 9,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/⏱️timestamped/🧬️mutations/🦀️.rs. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit/🦀️.rs",
      "line": 11,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🛂️validated/🧬️mutations/🦀️.rs. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-↩️restore-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-↩️restore-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/⏱️timestamped/🧬️mutations/↩️restore-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-↩️restore-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-🔢️set-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-🔢️set-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/⏱️timestamped/🧬️mutations/🔢️set-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-⏱️timestamped-🔢️set-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-↩️restore-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-↩️restore-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/↩️restore-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-↩️restore-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-➕️add-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-➕️add-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/➕️add-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-➕️add-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🔢️set-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🔢️set-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🔢️set-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🗑️delete-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🗑️delete-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🗑️delete-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🧮️demo-🗑️delete-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🪤️lossy-🔢️set-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🪤️lossy-🔢️set-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🪤️lossy/🧬️mutations/🔢️set-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🪤️lossy-🔢️set-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-↩️restore-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-↩️restore-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/↩️restore-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-↩️restore-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-⚠️set-warning-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-⚠️set-warning-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/⚠️set-warning-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-⚠️set-warning-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🔢️set-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🔢️set-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🔢️set-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🔢️set-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🚫️set-error-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🚫️set-error-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🚫️set-error-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🚫️set-error-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🛑️set-fatal-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🛑️set-fatal-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🚦️severity/🧬️mutations/🛑️set-fatal-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🚦️severity-🛑️set-fatal-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-↩️restore-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-↩️restore-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🛂️validated/🧬️mutations/↩️restore-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-↩️restore-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "inline-test-body",
      "line": 2,
      "detail": "Rust test attribute is outside a canonical test implementation.",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-🔢️set-n/🦀️.rs"
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-🔢️set-n/🦀️.rs",
      "line": 4,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/🛂️validated/🧬️mutations/🔢️set-n/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "test-case-name",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixture-🛂️validated-🔢️set-n/🦀️.rs",
      "line": null,
      "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
    },
    {
      "code": "fixture-owner-delivery-scope",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧫️fixtures/🔣️browser-entry-authority.json",
      "line": null,
      "detail": "Fixtures require a language-neutral semantic owner outside package, target, and implementation delivery folders."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
      "line": 96,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧫️fixtures/🧵️production.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
      "line": 118,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧫️fixtures/🩹️receipt.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
      "line": 185,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "production-fixture-dependency",
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
      "line": 255,
      "detail": "Non-test source resolves a dependency on fixture 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧫️fixtures/🔣️.json. Move executable support out of fixtures or place this read in a canonical test implementation."
    },
    {
      "code": "legacy-fixture-directory",
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️fixtures/🧪️nested-cargo-package-purity/🔣️.json",
      "line": null,
      "detail": "Fixture examples must use the canonical 🧫️fixtures directory. Test implementations and executable support must use their own semantic scopes."
    }
  ]
}
```
