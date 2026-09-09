# Runtime Taxonomy Assets

Production normalization unconditionally reads the nested Cargo and exact README/license ownership catalogs through taxonomy authority paths. Current README normalization additionally reads its expectation authority. These three documents are runtime static data, so they moved to the library asset scope. Test-only vectors and schema definitions remain in their separate scopes.

The JCO catalog package source/destination owner now matches the live testkit guest owner; frozen source content hashes remain unchanged. Current catalog byte digests were refreshed from actual files. The reviewed expectation asset retains its data except the runtime catalog coordinate. Cross-reference/hash validation is ongoing.

```json
{
  "moves": [
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json",
      "sourceSha256": "46a9750c0e31aa2d778ce851d3c8a472cd0433bde7d0e4ea687668c4b0c323a7",
      "targetSha256": "136f78b1002df27bc05863123c6197ea5b957ac0a030cb4afa6d1ba5e1215572"
    },
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/⚖️readme-license-owner-authority/🔣️.json",
      "sourceSha256": "2dcb31628412a9e95b4adfc8756d8fbe9dfa1b397742b3f8a47c603ad15ccc79",
      "targetSha256": "2dcb31628412a9e95b4adfc8756d8fbe9dfa1b397742b3f8a47c603ad15ccc79"
    },
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🗺️testing-readme-coordinates/🔣️.json",
      "sourceSha256": "7c6c64ee9bb155ec4180e6bb9e476cbca84fe24752c868eb28ec53a6ee73f62c",
      "targetSha256": "b2a6129ad3abb7d1ea6140a7050c2200b0c29f5ab178bad7fe81f02cbcbdd0a3"
    }
  ],
  "consumers": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🗺️testing-readme-coordinates/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
  ],
  "hashes": {
    "📽️nested-cargo-package-projection": "136f78b1002df27bc05863123c6197ea5b957ac0a030cb4afa6d1ba5e1215572",
    "⚖️readme-license-owner-authority": "2dcb31628412a9e95b4adfc8756d8fbe9dfa1b397742b3f8a47c603ad15ccc79",
    "🗺️testing-readme-coordinates": "b2a6129ad3abb7d1ea6140a7050c2200b0c29f5ab178bad7fe81f02cbcbdd0a3"
  }
}
```

Dependent current identities were recomputed in dependency order: expectation bytes and size, reviewed manifest bytes, current revision envelope, revision input bytes, and nested Cargo catalog bytes. The canonical expectation asset and separately retained test expectation example were byte-identical after their shared catalog coordinate update. Independent runtime validation follows.

```json
{
  "revisionDigest": "ac0299064894934e2a5e4c2861a8b57a9dee84a46698f224a2eb48d2e42d28ee",
  "manifestSha256": "583645faf0e033dd08911947b6b02e3aba49bc6ae24244aa546ef283239976b1",
  "revisionInputSha256": "1d9700f8bcdebaa4622373c85faa58bf5d626cee34fc19cf57310feba543f3d7",
  "additionalAuthoredPaths": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json"
  ]
}
```

Runtime identity validation passed through Bun and Nx: 6 tests / 69 assertions in reviewed-input and README activation cases. Independent JSON parsers, SHA-256, stable JSON revision digest and production schema loading all agreed after asset reclassification.

The actual production catalog loaders completed successfully: 2 nested Cargo package records and 40 exact owner records. Both catalogs used asset paths, their declared digests matched file bytes, and Node and Bun SHA-256 outputs agreed. Retained verification input: 🧑‍💻coordination/🖼️runtime-catalogs/📜️script.ts.
