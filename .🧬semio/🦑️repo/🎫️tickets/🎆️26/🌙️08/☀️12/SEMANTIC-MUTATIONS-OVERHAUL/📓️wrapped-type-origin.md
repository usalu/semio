# Wrapped Mutation Type Origin

## Scope

FND-WRAPPED-TYPE-ORIGIN-21 extends the existing aggregate reachability proof through the single exported `inspectMutationRootReachability` inspector. Every successful proof now includes `origin`, either `null` or the repository-owned exact triple `{ sourcePath, declarationName, modulePath }`.

The proof resolves direct public declarations, root public aliases, renamed aliases, and one allowed public child-facet reexport. It fails closed for missing, private, conditional, ambiguous, shadowed, escaping, and symlinked evidence. It does not authorize child origins for metadata policy and does not activate a second graph or mutation policy.

The conditional-mount requirement exposed missing parser facts. Under the approved contract amendment, `RustModuleGraphFact`, `RustModuleUseFact`, `RustMutationMetadataDeclarationFact`, `RustEnumFact`, and `RustEnumVariantFact` carry `conditional: true` only when direct, inherited-module, inner, or `cfg_attr` metadata makes the item conditional. The existing metadata attribute parser provides those facts; no local second Rust parser was added. Restricted aliases are separately identified so only exact `pub` reexports can participate in an origin proof.

Raw source locators are rejected before any filesystem operation if they are non-NFC, absolute, empty, dot/parent, backslash, colon, control-character (including U+2028/U+2029), or case-fold to a `compose` path segment. The gate covers the supplied mutation root, leaf, Rust filename, and child `#[path]` locator. Conditional declarations, aliases, aggregate enums, and variants remain in candidate counts; they cannot be discarded to approve a remaining candidate.

The repository base is independently an untrusted source boundary. Before any access, it must be raw-NFC, control-free, native-host absolute, and free of dot/parent/case-folded-`compose` segments. A Windows drive spelling is accepted only on its native host and only with its single required colon; foreign drive strings and ADS/extra-colon paths fail before resolution. The existing discovery `workspaceAuthorityPath` and `noFollowDirectoryAncestry` helpers are now exported, documented, and reused: once lexical checks pass, the base and every ancestor must be a real no-follow directory. Leaf names and Rust filenames must each be a single component.

## Verification

- `SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️wrapped-type-origin' bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' test -t 'projects the actual wrapped mutation declaration origin'`
  - Passed: one focused group, 33 schema-validated vectors, 212 expectations, and 13 accepted Rust sources compiled by `rustc`. The vectors cover direct, renamed, grouped, external-child, and inline-child positive proofs; unbound, ambiguous, private, declaration/aggregate/variant/ancestor direct-inner-`cfg`-`cfg_attr` conditional, shadowed, conflicting, escaping, symlinked, and unsafe raw-locator negatives. Four unsafe mutation-root/leaf/filename vectors are virtual and never materialize excluded paths; unsafe child locators are never materialized. Compiler records are retained under `🧪️wrapped-type-origin/`.
- `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' test -t 'projects the actual wrapped mutation declaration origin'`
  - Passed through the registered workspace route: one group, 212 expectations, 295 filtered, exit 0.
- `bun '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts' test -t 'proves direct leaf reachability through exact public canonical mounts and wrapped types'`
  - Passed unchanged: one existing group, 19 fixture cases, 20 expectations.
- `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️wrapped-type-origin-root-review/📜️script.ts'`
  - Root's independent exact-origin replay passed all 14 pre-existing cases with zero failures (`run-Ue7JLJ`).
- `git diff --check -- <review surface>`
  - Passed with no whitespace diagnostics.

## Current Boundary Verification

The root-owned virtual source-boundary replay first passed 18 mocked filesystem vectors (`run-XzNCAc`), and the direct 33-vector focused suite passed with 212 expectations; the existing 19-case reachability suite passed with 20 expectations; and the root-owned exact-origin replay passed all 24 cases (`run-Pgpr2S`). The registered workspace route also passed the focused suite with 212 expectations and 295 filtered tests when `SEMIO_TEST_ARTIFACT_DIR` used a ticket-contained, no-follow base.

The final native-root correction adds two foreign-drive/extra-colon zero-I/O vectors to the root-owned virtual probe and promotes repository raw-path, nested-filename, base-symlink, and ancestor-symlink coverage into the permanent schema-first fixture (39 vectors total). Source transpilation and `git diff --check` pass. The final dynamic replays are presently blocked before assertions by unrelated concurrent `wgpu-frame-worker` generated-output validation in `loadTaxonomy`; no post-correction pass is claimed. Runtime replay must retain a ticket-contained artifact root to avoid macOS `/var` ancestry aliases.

## Review Surface

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🔣️vectors.json`
