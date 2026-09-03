# Unique semantic path emoji statutes and repository migration

## Outcome

The repository now defines and enforces a single schema-first policy for emoji-bearing paths. Every governed file and directory must start with an emoji identity, generic file/folder identities are forbidden, presentation is canonical, emoji prefixes are adjacent to the semantic stem, and every sibling in the shared file-and-directory namespace has a unique complete leading emoji identity.

The repository-wide migration planned 946 path moves. The final audit covers 74,023 files, 41,614 directories, and 5,040 governed entries with zero statute breaches:

| Finding | Initial | Final |
| --- | ---: | ---: |
| Missing leading emoji | 323 | 0 |
| Generic emoji | 22 | 0 |
| Presentation mismatch | 0 | 0 |
| Prefix spacing | 0 | 0 |
| Duplicate sibling identity | 601 | 0 |

The independent `emoji-regex` diagnostic reports two oracle differences for ZWJ profession sequences (`🧑️‍🎨️` and `🧑️‍💻️`). These are not statute breaches: the repository parser correctly treats each complete grapheme sequence as one identity, while the oracle returns only the base person emoji in those two cases.

## Statutes

The taxonomy's `pathEmojiPolicy` is the authoritative contract. Its rules are:

1. Govern all Git-visible files and directories except declared excluded or reserved scopes.
2. Require a leading emoji identity on every governed basename.
3. Treat the full contiguous leading emoji sequence as the identity, preserving the first emoji as the semantic directory/file role and allowing later emoji as sibling discriminators.
4. Compare identities without variation-selector presentation differences.
5. Require canonical emoji presentation and no whitespace between the emoji identity and semantic stem.
6. Reject generic `📁`, `📂`, and `📄` identities.
7. Enforce uniqueness across files and directories sharing the same parent.
8. Reserve external fixed names only through declared, scoped contracts rather than broad basename exceptions.

External fixed-name contracts now explicitly cover package-root Bun `README.md` and `LICENSE.md`, Nx manifests, and other existing ecosystem files. Go's `internal` import hierarchy is reserved through Go import-grammar contracts because emoji directory segments are invalid package import paths; filenames within those directories remain governed.

## Implementation

- Extended the taxonomy schema/catalog with `pathEmojiPolicy`, fixed-name contracts, and semantic directory rules.
- Added shared TypeScript discovery helpers for complete leading identities, semantic kind recognition after discriminators, scoped fixed-name resolution, and first-class statute findings.
- Updated normalization to preserve complete leading identities and recognize discriminated package directories.
- Added matching Go statute evaluation and robust Git inventory handling.
- Added a language-neutral JSON fixture and JSON Schema consumed by TypeScript and Go tests.
- Added an independent `emoji-regex` oracle to validate Unicode identity parsing.
- Added the Nx `test-path-emoji-statutes` target to both generated and seed VS Code launch configurations.
- Migrated missing, generic, and colliding repository paths and repaired active references.
- Reconnected all 49 explicitly routed repository-library test sources to their migrated owner paths.

## Verification

Passed:

- `bun nx run @semio-tech/repo-lib:test-path-emoji-statutes --skip-nx-cache` — 4 tests, 33 assertions.
- `bun nx run @semio-tech/repo-client:test -- -run '^TestPathEmojiStatutesLanguageNeutralFixture$'`.
- `bun nx run @semio-tech/repo-client:build --skip-nx-cache`.
- `bun nx run @semio-tech/repo-lib:test-taxonomy-leading-grapheme --skip-nx-cache` — 9 tests, 21,221 assertions.
- Repository-library route audit — all 49 explicit test source paths exist.
- Ticket audit — 0 missing, 0 generic, 0 presentation, 0 spacing, and 0 duplicate findings.

The broader `@semio-tech/repo-lib:lint` target remains red on unrelated concurrent cross-project `rootDir`, typed-array, `ImportMeta`, and one existing Playwright callback typing diagnostic. The path migration's previously failing schema, registry, package, and styling path resolutions are no longer among the diagnostics.
