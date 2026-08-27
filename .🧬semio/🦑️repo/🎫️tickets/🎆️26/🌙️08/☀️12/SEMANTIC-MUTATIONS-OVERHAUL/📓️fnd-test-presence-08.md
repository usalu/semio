# FND Test Presence 08

The mutation structural policy now accepts a leaf only after proving one enabled, reachable, non-ignored `#[test]` function from the taxonomy-resolved canonical Rust source or an explicit leaf-owned `#[path]` mount. It no longer treats a `🧪️tests` directory, a `cfg(test)` module, a token, a comment, or a string as test evidence.

`inspectRustRunnableTests` uses the existing tokenized Rust structural parser. It follows inline modules, including explicit inline `#[path]` base overrides at every nesting level, records explicit external mounts, ignores nested functions, applies inner scope attributes, recursively expands enabled `cfg_attr` forms for `cfg`, `ignore`, and `path`, and refuses unknown conditional configurations or opaque `cfg_attr` payloads that could change executability or source identity. The policy resolver uses exact filesystem spellings and source-module-relative mount bases; it rejects missing targets, symlinks/junctions, escapes, absolute/drive-relative targets, backslash targets, non-regular files, and all non-leaf-owned path components.

The registered neutral fixture and Draft-07 schema contain 25 vectors: empty directory, unmounted source, empty module, comments/literals, disabled configuration, ignored-only coverage (with and without a reason), nested function decoy, inner disabled scope, inline law, direct canonical mounted law, enabled and ambiguous conditional attributes, conditional disabling/ignoring/path mounting, inline child-module path bases and overrides (including nested overrides), child facet, missing source, directory symlink/junction, and escape. Ajv validates the neutral schema; `rustc --test` compiles every valid vector and executes its discovered tests. The missing-source vector is required to fail compilation and is rejected by policy before any execution. Compiler and runtime oracle processes each have a 30-second timeout and a non-signal assertion.

## Registered Evidence

Executed on macOS with retained artifacts:

```text
SEMIO_TEST_BUDGET_MS=180000
SEMIO_TEST_ARTIFACT_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️test-presence-inline-override-final-artifacts
bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --timeout 90000 -t 'requires one enabled reachable non-ignored leaf test proven by parsed Rust items'
```

Result: one registered test passed, 148 assertions, 289 tests filtered, no failures. The fixture selects a directory symlink on POSIX and a Windows junction on Windows; this run executed on macOS. The independent 18-vector replay at `🧪️test-presence-preflight/🧫️run-hf6GdG` also completed with zero mismatches. No production Rust, derive, mutation leaf, or `compose/**` source was read or modified.

## Changed Paths

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-test-presence/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-test-presence/🛂️schema.json`
