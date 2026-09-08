# Test Layout Migration Snapshot

This is an intermediate scan taken while language agents were still moving files and rewriting references. It is a remaining-work queue, not final validation. The public Bun/Nx invocation completed source discovery and inspection with progress logs; no source exclusions were added.

The scanner inspected 29,246 source paths and returned 3,107 findings. Multiple findings may describe the same source.

| Finding | Count |
| --- | ---: |
| inline-test-body | 1017 |
| legacy-test-filename | 34 |
| test-case-name | 1 |
| test-implementation-depth | 306 |
| invalid-test-module-wiring | 1671 |
| test-owner-delivery-scope | 6 |
| inline-self-test-declaration | 58 |
| legacy-test-directory | 7 |
| test-implementation-filename | 7 |

| Extension | Count |
| --- | ---: |
| .ts | 107 |
| .py | 2 |
| .rs | 2963 |
| .js | 6 |
| .tsx | 29 |

Rust and TypeScript execution agents received the complete generated queue; native execution received the remaining Python case identity defect. The previous full contract snapshot included other testing-infrastructure contract failures and is superseded for layout migration by this dedicated scan.

## Remaining JavaScript and Python Paths

```json
[
  {
    "code": "inline-test-body",
    "line": 160,
    "detail": "Python test declaration is outside a canonical test implementation.",
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-21_quelle_remediation/agent_qext/🧪️tests/📄️v5-per-row-parser/🐍️.py"
  },
  {
    "code": "test-case-name",
    "path": "♻️mit-bestand/🔎️recherche/_neo4j/intake/runs/2026-05-21_quelle_remediation/agent_qext/🧪️tests/📄️v5-per-row-parser/🐍️.py",
    "line": null,
    "detail": "The test case directory must use one canonical emoji followed by a kebab-case name."
  },
  {
    "code": "test-implementation-depth",
    "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js",
    "line": null,
    "detail": "Executable test sources must be direct children of 🧪️tests/<test-name>."
  },
  {
    "code": "inline-test-body",
    "line": 4,
    "detail": "import.meta.vitest wiring must dynamically import an existing canonical test implementation.",
    "path": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist/assets/🌐️-DZ1JpS32.js"
  },
  {
    "code": "inline-test-body",
    "line": 1,
    "detail": "Test registration is outside a canonical test implementation.",
    "path": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist/assets/react.esm-NM2ar-8j.js"
  },
  {
    "code": "test-owner-delivery-scope",
    "path": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️tests/🖼️surface/🟨️.js",
    "line": null,
    "detail": "The test owner is below delivery scope 🎯️targets; move the case to its nearest language-neutral semantic owner."
  },
  {
    "code": "inline-self-test-declaration",
    "line": 9,
    "detail": "Self-test declaration testFlowBrowserClock is outside a canonical test implementation.",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js"
  },
  {
    "code": "test-implementation-depth",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js",
    "line": null,
    "detail": "Executable test sources must be direct children of 🧪️tests/<test-name>."
  }
]
```
