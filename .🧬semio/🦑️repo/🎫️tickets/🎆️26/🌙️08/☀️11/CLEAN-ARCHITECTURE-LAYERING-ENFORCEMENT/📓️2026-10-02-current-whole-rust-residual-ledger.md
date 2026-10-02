# Current Whole Rust Residual Ledger

Unchanged registered target: bun nx run @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache. Terminal RED in 3m28s. This census walked the concurrently changing live tree and is not a coherent final source capture. No problem or violation was waived.

Source report SHA256: 187f7643b72ef9c2c73bfb76fba2d2e52dc42b14b00f440bb893ee317fc0925c

Files: 24374; authored references: 62020; strict violations: 1; problems: 17.

Exact report rows:

```json
{
  "violations": [
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
      "kind": "include_str",
      "line": 8
    }
  ],
  "problems": [
    {
      "code": "unsupported-expression",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs",
      "detail": "Unsupported Rust compile expression at line 596: include_str"
    },
    {
      "code": "unproven-family-binding",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs",
      "detail": "Opaque macro contains family syntax: macro_rules"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌀️mutate-procedural-2d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/💠️mutate-lowpoly-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "unresolved-template-scope",
      "from": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs",
      "kind": "include_str",
      "line": 10,
      "expansion": {
        "kind": "local-macro",
        "macro": "vector",
        "definitionLine": 7,
        "invocationLine": 18,
        "definitionOffset": 138,
        "templateOffset": 223,
        "invocationOffset": 879,
        "scope": {
          "kind": "local-block",
          "startOffset": 132,
          "endOffset": 2420,
          "compilerAttributes": [
            "test"
          ]
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=138"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 44,
      "from": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🖐️mutate-puzzle-5d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-puzzle-3d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-5d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs",
      "kind": "path",
      "line": 40,
      "from": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-3d-1/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"
    },
    {
      "code": "unresolved-template-scope",
      "from": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧪️tests/🔬️unit/🦀️.rs",
      "kind": "include_str",
      "line": 177,
      "expansion": {
        "kind": "local-macro",
        "macro": "vector",
        "definitionLine": 173,
        "invocationLine": 185,
        "definitionOffset": 10910,
        "templateOffset": 11014,
        "invocationOffset": 11660,
        "scope": {
          "kind": "local-block",
          "startOffset": 10904,
          "endOffset": 12791,
          "compilerAttributes": [
            "test"
          ]
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=10910"
    },
    {
      "code": "unresolved-template-scope",
      "from": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs",
      "kind": "include_str",
      "line": 19,
      "expansion": {
        "kind": "local-macro",
        "macro": "committed",
        "definitionLine": 16,
        "invocationLine": 29,
        "definitionOffset": 1105,
        "templateOffset": 1222,
        "invocationOffset": 2052,
        "scope": {
          "kind": "local-block",
          "startOffset": 1099,
          "endOffset": 2309
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: committed; definitionOffset=1105"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🔣️.json",
      "kind": "include_str",
      "line": 17,
      "from": "✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧪️tests/🔬️unit/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🔣️.json"
    },
    {
      "code": "missing-input",
      "to": "✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🔣️.json",
      "kind": "include_str",
      "line": 30,
      "from": "✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🧪️tests/🔬️unit/🦀️.rs",
      "detail": "Authored compile input failed physical census: ✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🔣️.json"
    },
    {
      "code": "unresolved-template-scope",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🦀️.rs",
      "kind": "include_str",
      "line": 48,
      "expansion": {
        "kind": "local-macro",
        "macro": "vector",
        "definitionLine": 45,
        "invocationLine": 60,
        "definitionOffset": 2024,
        "templateOffset": 2110,
        "invocationOffset": 3062,
        "scope": {
          "kind": "module",
          "modulePath": []
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=2024"
    }
  ]
}
```

Cargo registered gate also completed RED before a verdict: unchanged 30s metadata budget expired after normal workspace preparation waiting. 289 inventoried packages are observed; complete metadata and an edge verdict are unavailable. The gate remains pending a normal Native replay.

## Registered Live Census Two

Unchanged original gate completed RED in 4m33s. Root had exposed the six fixture interfaces and replaced two wildcard unit-test imports before this launch; private fixture-reader modules and UI source work changed during the live census. This is not a coherent final capture. Exact report SHA256 c898128e8fa3913c8ff0774c636f18ce2c7b6d67e9bac2e879b02575830583aa. Files 24379, references 62010, violations 1, problems 3.

```json
{
  "violations": [
    {
      "rule": "framework-modules-no-products",
      "from": "🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
      "kind": "include_str",
      "line": 8
    }
  ],
  "problems": [
    {
      "code": "unsupported-expression",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs",
      "detail": "Unsupported Rust compile expression at line 596: include_str"
    },
    {
      "code": "unresolved-template-scope",
      "from": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs",
      "kind": "include_str",
      "line": 19,
      "expansion": {
        "kind": "local-macro",
        "macro": "committed",
        "definitionLine": 16,
        "invocationLine": 29,
        "definitionOffset": 1105,
        "templateOffset": 1222,
        "invocationOffset": 2052,
        "scope": {
          "kind": "local-block",
          "startOffset": 1099,
          "endOffset": 2309
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: committed; definitionOffset=1105"
    },
    {
      "code": "unresolved-template-scope",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🦀️.rs",
      "kind": "include_str",
      "line": 48,
      "expansion": {
        "kind": "local-macro",
        "macro": "vector",
        "definitionLine": 45,
        "invocationLine": 60,
        "definitionOffset": 2024,
        "templateOffset": 2110,
        "invocationOffset": 3062,
        "scope": {
          "kind": "module",
          "modulePath": []
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=2024"
    }
  ]
}
```

## Third Actual Registered Source Census

Original unchanged @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache is terminal RED5m44sNx. This is a live-tree source census while UI and other authorized shared work continue; it is not a coherent frozen compiler epoch. Exact report SHA-256 cf4ec43cfb24197348d9f5835d5705dfc6f2b7ebca1ea8b3d96a0b8d48e3dc98.

```json
{
  "schemaVersion": 1,
  "files": 24384,
  "references": 62015,
  "violations": [],
  "problems": [
    {
      "code": "unsupported-expression",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs",
      "detail": "Unsupported Rust compile expression at line 596: include_str"
    },
    {
      "code": "missing-input",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework",
      "kind": "include_str",
      "line": 163,
      "from": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
      "detail": "Authored compile input failed physical census: 🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework"
    },
    {
      "code": "missing-input",
      "to": "🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework",
      "kind": "include_str",
      "line": 279,
      "from": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
      "detail": "Authored compile input failed physical census: 🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework"
    },
    {
      "code": "unresolved-template-scope",
      "from": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs",
      "kind": "include_str",
      "line": 8,
      "expansion": {
        "kind": "local-macro",
        "macro": "committed",
        "definitionLine": 5,
        "invocationLine": 18,
        "definitionOffset": 224,
        "templateOffset": 341,
        "invocationOffset": 1186,
        "scope": {
          "kind": "local-block",
          "startOffset": 218,
          "endOffset": 1443
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: committed; definitionOffset=224"
    },
    {
      "code": "unresolved-template-scope",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures/🦀️.rs",
      "kind": "include_str",
      "line": 29,
      "expansion": {
        "kind": "local-macro",
        "macro": "vector",
        "definitionLine": 26,
        "invocationLine": 41,
        "definitionOffset": 841,
        "templateOffset": 927,
        "invocationOffset": 1902,
        "scope": {
          "kind": "module",
          "modulePath": []
        }
      },
      "detail": "Finite Rust macro requires an exclusive live lexical scope: vector; definitionOffset=841"
    }
  ]
}
```

## Fourth Actual Registered Source Census

Unchanged registered full source route terminal RED2m16sNx. Current live-tree report SHA-256 2306301d0b3c6b47112f1956caf43a5c99bff991c0829d98875f2290416b8070. This run follows Root literal fixture-reader inputs and corrected two WFC manifest suffixes. It still does not certify a coherent frozen compiler epoch or unresolved generated producer.

```json
{
  "schemaVersion": 1,
  "files": 24387,
  "references": 62022,
  "violations": [],
  "problems": [
    {
      "code": "unsupported-expression",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs",
      "detail": "Unsupported Rust compile expression at line 596: include_str"
    }
  ]
}
```
