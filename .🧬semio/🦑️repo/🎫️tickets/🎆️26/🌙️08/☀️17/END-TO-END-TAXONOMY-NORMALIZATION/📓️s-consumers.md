# S-Consumers: Version-7 Taxonomy Convergence

## Scope

This slice owns the schema-first v7 taxonomy contract, its strict discovery consumer, and the repo-library package consumer. It did not read or modify `compose/**` or `temp/compose/**`, did not edit `AGENTS.md`, and did not modify Git state. The intentional compose deletion remained untouched.

Production files changed:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`

## Contract outcome

The incompatible schema is version 7 and rejects removed v6 filename fields and blanket package allowances. Its current strict-load inventory is:

- 72 physical file kinds owning 119 unique extension chains.
- 119 one-to-one file-kind resolution rules.
- 90 semantic directory kinds, including parent-constrained test fixtures and ticket evidence.
- 74 owner-local member registries containing 3,130 exact NFC directory names.
- 39 fixed filename contracts and 18 fixed directory contracts.

`fileKinds` now represent physical/tool-consumed formats, not semantic roles. The same-extension role variants were removed: JSON resolves to `json`/`🔣️`, Markdown to `markdown`/`📝️`, Rust to `rust-source`/`🦀️`, and TypeScript tests to ordinary TypeScript/TSX `typescript-source`/`🟦️`. In particular, `.test.ts` and `.test.tsx` are not compound extension kinds; test semantics belong in directory kinds. Distinct physical compound formats such as `.d.ts` and `.schema.json` remain eligible for their own kinds.

The global resolution registry covers the non-compose extension census, including raster PNG/BMP, native C/C++/assembly, documents, configuration/data, CAD/model, font, media, archive, and binary formats. Longest-chain resolution is case-normalized and deterministic. Historical ticket-only odd suffixes remain in `scopedFileKinds`, require both their ticket path and source-name pattern, and are never promoted to global format kinds.

## Semantic directories

Global semantic kinds are exact emoji-plus-slug records with optional `parentKindIds`. Contextual fixtures include `test-case`, `asset-subject`, `test-fixture-member`, `test-fixture-window`, `test-fixture-segment`, and `test-fixture-asset`; the resolver uses the nearest parent first and then owner ancestry. Active-ticket test evidence is narrowly admitted only beneath `ticket-day`.

The exact owner-local overlay replaces an any-emoji wildcard. It resolves existing artifact, module, plugin, mutation, inference, example, asset, and governance members only under registered owners. NFC normalization and VS16 insertion occur before matching.

Final non-compose Git-visible census:

```text
64,725 files
60,437 global physical-kind resolutions
4,136 exact fixed-contract resolutions
138 scoped historical-ticket resolutions
14 unresolved source anomalies
37,989 derived directories
32,056 emoji-leading directories
32,056 resolved; 0 unresolved
```

The 14 file residues are retained migration/cleanup evidence rather than uncovered renameable formats: four `*undefined` outputs, one `.gitkeep`, one `.rs.broken_backup`, several extensionless probes/binaries/legacy names, a whitespace basename, and three emoji-prefixed external names (`CNAME`, `Caddyfile`, `Dockerfile`) whose normalized targets are exact external contracts. No suffix exemption was added for them.

## Fixed contracts and policies

Filename and directory contracts carry `pathPattern`, authority, reason, `configurability: "unconfigurable"`, scope, verification, and expiry. Supported scopes are repository root, package root, directory kind, and path pattern. NFC POSIX patterns support `*`, `?`, character classes, and whole-segment `**`.

Discovery exports the shared matching surface:

- `taxonomyPathPatternMatches(path, pattern)`
- `fixedContractFilename(contract)`
- `fixedContractSpecificity(contract)`
- `fixedFilenameContractIdsForPath(path, taxonomy, context)`
- `fixedDirectoryContractIdsForPath(path, taxonomy, context)`

Specificity is ordered by literal segments, literal code points, fewer wildcard tokens, and non-pattern scope. Equal specificity remains an ambiguity; lexical ID order is presentation-only.

Narrow contracts now preserve nested `AGENTS.md`, the exact `🎫️ticket.json` ticket manifest path, repo/package manifests, Git hooks, `CACHEDIR.TAG`, `CNAME`, and nested `go.work.sum`. Fixed directory contracts cover genuine external/governance names including `.🧬semio`, editor/agent metadata, ticket slugs, and `**/.git`. Nested `.git` is retained and governed, not excluded or opaque. `.storybook` deliberately has no fixed contract. The only opaque path exclusion is compose.

## Consumer migration

Discovery derives canonical leaf names from kind IDs and exact contracts, filters opaque paths before reads, and validates that old v6 keys are absent. Package discovery uses recursive boundary/glue contracts and treats uncertain content roles as problems rather than valid glue.

The repo TypeScript library consumer now imports `loadTaxonomy` and `fixedContractFilename`, derives layering inputs through v7 `repoWideContractIds` and generated contract IDs, and matches generated paths exactly from v7 locations. No editable non-compose consumer retains the removed semantic filename fields; remaining `leafFilename` search hits are local variables derived from kind IDs or explicit rejection checks for old shapes.

## Verification evidence

Successful checks:

```text
bun -e '<strict load and focused assertions>'
counts: 72 file kinds, 119 extension chains/rules, 90 directory kinds,
74 member kinds, 39 fixed files, 18 fixed directories
assertions: nested .git, ticket evidence, CACHEDIR.TAG, nested go.work.sum,
.test.ts -> TypeScript, .md -> Markdown, .json -> JSON, .rs -> Rust: all true

bun -e '<non-compose Git-visible file/directory census>'
file resolution: 60,437 global + 4,136 fixed + 138 scoped; 14 migration residues
directory resolution: 32,056 / 32,056 emoji-leading directories

bun build discovery/🟦️component.ts --target=bun --external @semio-tech/framework --outfile /dev/null
Bundled 1 module; exit 0

bun --check packages/🟦️typescript/📦️index.ts
exit 0

git diff --check -- <three owned production paths>
exit 0
```

`bun nx run @semio-tech/repo-lib:lint` reached TypeScript and reported no error in the taxonomy/discovery files. The project target remains red because of pre-existing out-of-scope errors: `ImportMeta.env`/`ImportMeta.glob` in UI styling and TS6059 `rootDir` violations from OS plugin imports. Bundling the large package index is likewise blocked by the existing unresolved `chromium-bidi` Playwright dependency; its direct Bun syntax check passes.

## Remaining coordinated work

The self-hosted taxonomy still lives at the semantic filename `.../library/🔣️taxonomy.json`. It is not being declared fixed. Relocating it to a kind-only leaf under a semantic directory (proposed `.../library/📇️taxonomy/🔣️.json`) requires an atomic change across loader literals, root/runtime consumers, tests, and reference fixtures. Per coordination instruction, this slice stops before that relocation so the affected lanes can be quiesced first.

Acceptance checks for the next boundary:

1. Perform the self-hosted taxonomy relocation atomically and prove default-load plus rewrite behavior.
2. Run the full normalization inventory with every consumer on the frozen helper interfaces.
3. Treat the 14 source anomalies as explicit moves/deletions/fixed-target normalization; do not add blanket suffix allowances.
4. Re-run the repository taxonomy suite after the concurrent engine and fixture migrations converge.
