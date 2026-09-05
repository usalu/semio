# Unique semantic path emoji statutes and repository migration

> Retracted: the completion and zero-breach claims below were incorrect. The implementation accepted stacked emoji identities and corrupted path references and runtime values. This document is retained as incident evidence, not current status. See [Workspace Repair](📓️workspace-repair.md). Exactly one handpicked emoji is now required; subsequent discriminator emojis are forbidden.

## Outcome

The repository now defines and enforces one schema-first policy for emoji-bearing paths. Every governed Git-visible file and directory must start with a canonical, non-generic emoji identity, and the complete leading identity must be unique across the shared file-and-directory sibling namespace.

The migration renamed 6,256 path entries: 4,108 directories and 2,148 files. The final independent audit covers 74,091 files, 41,635 directories, and 77,399 governed entries with no statute breaches.

| Finding | Initial | Final |
| --- | ---: | ---: |
| Missing leading emoji | 2,176 | 0 |
| Generic emoji | 243 | 0 |
| Presentation mismatch | 219 | 0 |
| Prefix spacing | 0 | 0 |
| Duplicate sibling identity | 3,644 | 0 |

The command family reported by the user is now sibling-unique:

- `🧬️add-generation`
- `🧬️✏️rename-generation`
- `🧬️📅️update-generation-values`
- `🧬️🚫️remove-generation`
- `🧬️🟤️select-generation`

The independent `emoji-regex` diagnostic reports two oracle differences for the valid ZWJ profession graphemes `🧑️‍🎨️` and `🧑️‍💻️`. These are not statute breaches: the repository parser treats each full grapheme as one identity, while that oracle returns the base person emoji for these two inputs.

## Statutes

The taxonomy's `pathEmojiPolicy` is the authoritative contract:

1. Govern every Git-visible file and directory except a declared excluded or externally reserved scope.
2. Require a leading emoji identity on every governed basename.
3. Reject generic `📁`, `📂`, and `📄` identities.
4. Parse the complete contiguous leading grapheme sequence as the identity.
5. Compare identities after folding emoji variation selectors.
6. Require canonical emoji presentation and prohibit whitespace between the identity and semantic stem.
7. Enforce uniqueness across files and directories with the same parent.
8. Preserve the semantic role in the first emoji and use later emojis as deterministic sibling discriminators.
9. Reserve fixed external names only through scoped contracts. Fixed-directory contracts can declare their descendants reserved, as used for the Next.js `app` router tree.
10. Infer Cargo, Go, and TypeScript package-root filename contracts from an adjacent `Cargo.toml`, `go.mod`, or `package.json`; the exemption does not become a repository-wide basename exemption.

Go's compiler-mandated `internal` import hierarchy remains ASCII through its scoped Go contract. Dotfiles, tool-owned generated trees, declared fixture/example trees, and other taxonomy exclusions remain reserved. Authored artifact taxonomies remain governed beneath artifact owners.

## Implementation

- Extended the taxonomy schema/catalog with `pathEmojiPolicy`, scoped fixed-name contracts, and reserved-descendant directory contracts.
- Added shared TypeScript discovery helpers for full grapheme identities, variation-selector folding, semantic kind recognition after discriminators, and language-neutral statute findings.
- Wired the root repository policy to the Git-visible inventory, scoped package-manifest inference, dotfile reservation, and the shared findings engine.
- Added matching Go-side statute support and robust Git inventory behavior.
- Added a language-neutral JSON fixture and JSON Schema consumed by the statute tests, plus an independent `emoji-regex` Unicode oracle.
- Added the Nx `test-path-emoji-statutes` target and launch configurations.
- Migrated all initially detected paths and repaired active repository references. A recorded-coordinate repair projected 1,281 unresolved relative literals across 157 files through the rename plan.
- Repaired explicit Cargo target paths for renamed Rust source roots and added explicit binary targets where Cargo defaults no longer applied. All 50 artifact manifests now resolve through `cargo metadata`.
- Repaired the plugin SDK manifest, math crate path, semio subset-base references, repository-library test routes, schema references, registry references, and styling references exposed by validation.
- Added focused integration coverage for missing prefixes, adjacent package manifests, VS16-equivalent sibling collisions, file-versus-directory collisions, and valid distinct identities.

## Verification

Passed:

- Production repository emoji policy over the complete current Git-visible inventory: 0 breaches.
- Ticket audit: 0 missing, 0 generic, 0 presentation, 0 spacing, and 0 duplicate findings.
- `bun nx run '@semio-tech/repo-lib:test-path-emoji-statutes' --skip-nx-cache`: 6 tests, 42 assertions.
- `bun nx run '@semio-tech/repo-lib:test' -- --test-name-pattern '^emoji-prefix policy'`: 4 tests, 7 assertions.
- `bun nx run repo-go-lib:test-quick --skip-nx-cache`.
- `cargo metadata --no-deps --format-version 1` at the workspace root.
- `cargo check -p semio-framework-math --quiet`.
- `cargo metadata --no-deps --format-version 1` for all 50 artifact manifests.
- `cargo check --all-targets` for 47 of 50 artifact manifests; the remaining STEP, BREP, and mesh bridges reach semantic compilation and fail on generated `flow_dag`/`ToValue`/`FromValue` diagnostics rather than missing renamed paths.
- `@semio-tech/repo-lib:lint` no longer reports unresolved-module (`TS2307`) diagnostics. Its remaining diagnostics are non-path `rootDir`, typed-array, `ImportMeta`, generated-export, and implicit-any issues.

The reference repair is intentionally conservative: a relative literal was changed only when its current coordinate did not resolve, its pre-migration coordinate mapped through the recorded plan, and the projected target exists.
