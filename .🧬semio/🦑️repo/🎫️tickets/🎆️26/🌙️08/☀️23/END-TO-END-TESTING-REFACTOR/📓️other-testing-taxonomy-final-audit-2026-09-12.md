# Final Nonframework Testing-Taxonomy Audit

Read-only live audit on 2026-09-12. This report covers authored content outside `🧰️framework` and excludes the Procedural moves and `contract.ts` renames assigned to the root agent. No source was changed and no build or test was run.

The requested `🗑️generated/testing-taxonomy/census/findings.json` was not present in the live tree, so its reported `214` count could not be independently reused. The findings below come from the current filesystem, including collision checks for every proposed destination.

## Live Residuals

| Residual | Live count | Classification |
| --- | ---: | --- |
| Case-local fixture payloads | 3 files | Test data incorrectly nested below a test case. |
| Puzzle 3D inline Rust test modules | 2 modules / 10 `#[test]` functions | Executable test code in production implementation files. |
| `🔬️testkit`, `🔮️oracle`, `🧪️oracle` with descendant authored files | 0 | No populated alias root remains at this observation point. Remove any empty alias directory left by concurrent moves; it has no authored content to relocate. |

The only other live `/🧪️tests/<case>/<extra>/🦀️.rs` matches were the 14 Procedural window cases already assigned to the root agent. No additional case-local helpers, examples, harnesses, supports, or oracle aliases were found by that focused scan.

## Case-Local Fixture Payloads

Each JSON file is an input/vector corpus consumed exclusively by its `document-contract` test implementation. It is a testing fixture, not production static data: the consumers validate schema/parser behavior with Ajv or Rust `serde_json`; no production consumer was found. Move each file to its semantic owner's `🧫️fixtures` tree and change the indicated relative literal.

| Current file | Canonical file (destination verified free) | Consumers and required literal |
| --- | --- | --- |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🧪️fixtures/🔣️.json` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json` | `.../🧪️tests/🪪️document-contract/🟦️.ts:5`: `./🧪️fixtures/🔣️.json` → `../../🧫️fixtures/🪪️document-contract/🔣️.json`. |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧪️tests/🪪️document-contract/🧫️fixtures/🔣️.json` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json` | `.../🧪️tests/🪪️document-contract/🟦️.ts:20` and `🦀️.rs:7`: `./🧫️fixtures/🔣️.json` / `🧫️fixtures/🔣️.json` → `../../🧫️fixtures/🪪️document-contract/🔣️.json`. |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧪️tests/🪪️document-contract/🧫️fixtures/🔣️.json` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json` | `.../🧪️tests/🪪️document-contract/🟦️.ts:57`: `./🧫️fixtures/🔣️.json` → `../../🧫️fixtures/🪪️document-contract/🔣️.json`. |

`🧪️fixtures` in the CAD source is both an invalid category spelling and wrongly nested. The Stdio sources use the canonical fixture spelling but retain the invalid case-local scope.

## Puzzle 3D Inline Test Bodies

Both files retain executable `#[cfg(test)]` modules inside their production implementation. Extract the complete module body, including private test helpers, to the listed direct test implementation. Keep the module declaration in the production file as `#[path = "..."] mod ...;`; that preserves Rust's lexical scope. No `include!` conversion is needed. Both destinations are currently absent.

| Production owner and inline module | Test body | Canonical test implementation | Production registration after extraction |
| --- | ---: | --- | --- |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs:247`, `mod tests` | 6 functions (`:305`–`:406`) | `.../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:721`, `mod vortex_payload_laws` | 4 functions (`:770`–`:801`) | `.../🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs` | `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod vortex_payload_laws;` |

The ellipses in the destination column mean the unchanged absolute owner prefix from the production-owner column. These paths retain the required shape `<semantic-owner>/🧪️tests/<test-name>/🦀️.rs`.

## Non-Authored Residue

One Python bytecode file remains under authored plugin paths: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/__pycache__/🐍️.cpython-314.pyc`. It is generated cache output, not a testing category or authored implementation, and should be removed without relocation.
