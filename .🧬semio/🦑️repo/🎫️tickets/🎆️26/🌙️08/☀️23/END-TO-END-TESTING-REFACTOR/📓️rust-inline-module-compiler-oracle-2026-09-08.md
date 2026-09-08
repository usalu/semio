# Rust Inline Module Compiler Oracle

Read-only compiler audit performed on 2026-09-08. The fixture sources and their only temporary tool are isolated under 🧑‍💻rust-module-path-oracle. Compiler results and diagnostics are transcribed in this report; private generated captures are removed during closure. No production source, Git state, ticket state, or shared generated directory was changed.

## Execution

The compiler matrix was executed through the ticket's minimal Nx workspace:

```sh
NX_WORKSPACE_ROOT_PATH="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/🧫️nx-fixture" NX_DAEMON=false bunx nx exec -- bun "$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻rust-module-path-oracle/📜️script.ts"
```

Rustc 1.99.0-nightly (c4af71034 2026-07-06) compiled and ran six valid compiler test fixtures, and compiled one intentional failure. The exact results and diagnostics are transcribed below; the temporary machine-readable capture is removed during closure.

| Fixture | Source declaration | Compiler-opened test implementation | Result |
| --- | --- | --- | --- |
| `non-mod-imported` | `📦️crate/lib.rs:1` imports `🏛️owner/🦀️.rs`; `🏛️owner/🦀️.rs:3` declares `outer`'s path test | `📦️crate/🏛️owner/outer/🧪️tests/🔬️imported/🦀️.rs` | compiled; `owner::outer::canonical::imported_non_mod_path_executes` passed |
| `default-non-mod` | `🏛️default-non-mod/🦀️.rs:3`, no parent `path` | `🏛️default-non-mod/outer/🧪️tests/🔬️default/🦀️.rs` | compiled; `outer::canonical::default_path_executes` passed |
| `dot-parent` | `🏛️dot-parent/🦀️.rs:1` has `#[path = "."]`; leaf at line 3 | `🏛️dot-parent/./🧪️tests/🔬️dot/🦀️.rs` | compiled; `chosen::canonical::dot_path_executes` passed |
| `empty-parent` | `🏛️empty-parent/🦀️.rs:1` has `#[path = ""]`; leaf at line 3 | `🏛️empty-parent/🧪️tests/🔬️empty/🦀️.rs` | compiled; `chosen::canonical::empty_path_executes` passed |
| `named-parent` | `🏛️named-parent/🦀️.rs:1` has `#[path = "renamed-scope"]`; leaf at line 3 | `🏛️named-parent/renamed-scope/🧪️tests/🔬️named/🦀️.rs` | compiled; `chosen::canonical::named_path_executes` passed |
| `mixed-present` | `🏛️mixed-present/🦀️.rs:1` has `named-scope`; line 3 has `../reassigned-scope`; leaf at line 5 | `🏛️mixed-present/named-scope/../reassigned-scope/🧪️tests/🔬️mixed-present/🦀️.rs` | compiled and executed when `named-scope/` exists |
| `mixed-absent` | Same declarations as `mixed-present`, but no `named-scope/` exists | The normalized destination exists at `🏛️mixed-absent/reassigned-scope/🧪️tests/🔬️mixed-absent/🦀️.rs` | expected compile failure |

The intentional failure is exact and material: `rustc` tried to open `🏛️mixed-absent/named-scope/../reassigned-scope/🧪️tests/🔬️mixed-absent/🦀️.rs` and returned `No such file or directory (os error 2)`. The lexical normalization of that path therefore cannot establish a valid Rust module import.

## Compiler Rules Established

- A `#[path]` on an inline module replaces that inline module's directory component. `.` and `""` reset the child base to the enclosing base; a named value replaces it with the named directory. They do not append the inline Rust namespace name.
- A nested named path follows the literal path chain. `#[path = "named-scope"]` followed by `#[path = "../reassigned-scope"]` preserves `named-scope/../reassigned-scope` for filesystem lookup.
- A source filename other than `mod.rs` contributes no implicit filename-stem directory in either form tested. The direct crate-root source `🏛️default-non-mod/🦀️.rs` and the separately imported `📦️crate/🏛️owner/🦀️.rs` both resolve an unannotated inline `outer` below the physical source directory, not below a `🦀️/` virtual directory.
- `posix.resolve` is useful for the canonical destination identity, but a Rust-wiring validator must retain the unnormalized path chain and require each prefix through a `..` traversal to exist.

## Proposed Language-Neutral Source/Path Vectors

These are compact additions or complements to the layout vector corpus. A validator should inspect `sources`, while the compiler oracle compiles their first source as a test crate. `expected: []` means no `invalid-test-module-wiring` finding.

```json
[
  {
    "id": "rust-module-base-non-mod-imported",
    "sources": [
      { "path": "📦️crate/lib.rs", "source": "#[path = \"🏛️owner/🦀️.rs\"]\nmod owner;\n" },
      { "path": "📦️crate/🏛️owner/🦀️.rs", "source": "mod outer {\n #[cfg(test)] #[path = \"🧪️tests/🔬️case/🦀️.rs\"] mod tests;\n}\n" },
      { "path": "📦️crate/🏛️owner/outer/🧪️tests/🔬️case/🦀️.rs", "source": "#[test] fn law() {}\n" }
    ],
    "expected": []
  },
  {
    "id": "rust-module-base-dot-and-empty-reset",
    "sources": [
      { "path": "🧩️domain/🦀️.rs", "source": "#[path = \".\"] mod dot { #[cfg(test)] #[path = \"🧪️tests/🔬️dot/🦀️.rs\"] mod tests; }\n#[path = \"\"] mod empty { #[cfg(test)] #[path = \"🧪️tests/🔬️empty/🦀️.rs\"] mod tests; }\n" },
      { "path": "🧩️domain/🧪️tests/🔬️dot/🦀️.rs", "source": "#[test] fn law() {}\n" },
      { "path": "🧩️domain/🧪️tests/🔬️empty/🦀️.rs", "source": "#[test] fn law() {}\n" }
    ],
    "expected": []
  },
  {
    "id": "rust-module-base-named-reset",
    "sources": [
      { "path": "🧩️domain/🦀️.rs", "source": "#[path = \"🧭️scope\"] mod named { #[cfg(test)] #[path = \"🧪️tests/🔬️case/🦀️.rs\"] mod tests; }\n" },
      { "path": "🧩️domain/🧭️scope/🧪️tests/🔬️case/🦀️.rs", "source": "#[test] fn law() {}\n" }
    ],
    "expected": []
  },
  {
    "id": "rust-module-base-missing-traversal-prefix",
    "sources": [
      { "path": "🧩️domain/🦀️.rs", "source": "#[path = \"named-scope\"] mod first { #[path = \"../reassigned-scope\"] mod second { #[cfg(test)] #[path = \"🧪️tests/🔬️case/🦀️.rs\"] mod tests; } }\n" },
      { "path": "🧩️domain/reassigned-scope/🧪️tests/🔬️case/🦀️.rs", "source": "#[test] fn law() {}\n" }
    ],
    "expected": [{ "path": "🧩️domain/🦀️.rs", "code": "invalid-test-module-wiring", "line": 1 }]
  }
]
```

The final vector deliberately omits `🧩️domain/named-scope/`: fixture setup creates the normalized destination but not the raw traversal prefix, so `rustc` must reject it.

## Current Repository Traversal Audit

The same ticket-local Bun script performed a read-only scan over every Rust source containing `🧪️tests`:

```sh
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻rust-module-path-oracle/📜️script.ts audit-current-traversals
```

It examined 4,592 Rust sources, reconstructed module bases for 967 path attributes that import 🧪️tests and whose raw compiler path contains .., and found every raw target and each intermediate prefix present (missing: 0). The traversal totals and result are transcribed here; the temporary records are removed during closure. This is a layout/wiring filesystem audit, not a claim that every production crate was compiled.

## Rust Wiring Classification Review

The live WFC engine is a direct counterexample to treating `#[cfg(test)]` alone as evidence that an external module is a test implementation. [🦀️.rs](../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🦀️.rs) at lines 3–5 declares `#[cfg(test)] #[path = "🔦️beam/🦀️.rs"] pub(crate) mod beam;`. The imported [beam module](../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔦️beam/🦀️.rs) defines the real `beam_search` algorithm, while test declarations reside in independently canonical test implementations. The same shape covers the engine's chunk, constraint, domain, grid, propagation, and solver modules.

External Rust module wiring should therefore be classified as test wiring only when the declaration is explicit: its module name is `tests`, or its literal `#[path]` carries the canonical `🧪️tests` marker (and legacy test markers where legacy detection remains in scope). A test-only configuration gate by itself must not impose a canonical-test destination.

This leaves no ordinary test escape if the scanner continues to inspect every authored Rust file independently: an arbitrary `mod verification;` that imports a file containing `#[test]`, `#[async_test]`, `#[wasm_bindgen_test]`, or a `self_tests` declaration is reported from that imported file's body. An opaque custom macro with no registered test signal could evade both the body scanner and explicit wiring rule; it needs a framework-specific test-signal registration, not a broad `cfg(test)` heuristic that misclassifies domain code.
