# Captured Source Import Diagnostics

Status: source and independent static DATA validation only. Both actual Bun regression bodies are unrun.

The three existing immutable loader refusal predicates remain exact: an uncaptured declaring module, a missing require resolution, and a missing/unmapped import image. They now include a closed diagnostic containing the actual declaring importer, requested specifier, declaring-image membership, declared-map membership and target-image membership. Captured paths are reported relative to the repository. No target, file, mapping, provider execution, or fallback is admitted by this change. Existing error prefixes remain recognizable.

The schema and eight language-neutral fixtures were authored before production changes. Each context string is bounded to 256 Unicode code points; the complete JSON remains below the declared 4096-byte ceiling. The helper is self-contained when serialized into either loader, using only system Array/Object operations. It grants no Source or dependency authority.

Existing Ajv validated the closed fixture contract; independent SQLite JSON/substr projections matched all eight exact expected objects, including astral/BMP boundaries and escaped context. TypeScript parsed the three changed/new TS definitions without syntax errors. Static AST projection of both generated loader templates parsed successfully and confirmed the three refusal predicates remain unchanged. These static projections do not execute the product or prove actual captured runtime behavior.

Two named tests are mounted in the existing Source0 suite, preserving the previous 63 names and producing 65 required names. Its 27 explicit inputs, six runtime entries and live uncached classification remain unchanged. The actual helper is imported by the tests and production loader, so it participates in their normal inferred import closure; no new explicit AST-only subject was introduced.

Exact six source paths, current hashes, stable physical observations and AST/DATA checks are retained under generated/e2e-source-import-diagnostics-inputs.json. Maximum absolute path length is 146 characters. The previous actual Source0 whole 1 remains historical evidence; no new runtime producer has been launched.

## Source Scope

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts`

## Bounded Work Correction

Low caught that the initial Array.from implementation materialized the complete input before truncation. The saved implementation now stops its codepoint loop after 256 characters, bounding work and allocation. The existing named neutral test also includes an instrumented iterable that refuses a 257th read and records iterator closure; this actual-helper runtime assertion is authored and unrun. All three final TS files still parse without diagnostics. Exact six current source hashes were refreshed after this correction. No guard, cap, runtime producer or required test name changed.

## Concurrent Guard Observation

After Source0 96118, a fresh loader read showed the declaring-map predicate had been removed. Root prepared a one-predicate reconciliation, but the immediate byte-equality-before-write check detected another change and refused the write. A fresh reread already had the complete original predicate and exactly matched Root’s reviewed loader SHA. Root therefore made zero source edits in this reconciliation. No actor or loaded-byte attribution is inferred. Observations are retained in generated/e2e-source-import-guard-reconciliation.json; High owns the separate authenticated capture-query repair.
