# Fixed Script Semantic Enforcement

Date: 2026-09-12

## Result

Exact fixed filenames and executable body ownership are now checked independently. A file selected by a fixed filename contract keeps its canonical fixed name, null file kind and real package role. When that contract also declares an explicit source grammar, normalization evaluates the body through the contract's validator and reports either:

- `fixed-source-disposition-unresolved` when the body is readable but not proven by the declared grammar and validator;
- `fixed-source-content-unreadable` when the source body cannot be decoded or read.

The gate does not infer package membership, move a mandatory `📜️script.ts`, or convert an unresolved body into implementation. Fixed metadata without an explicit source grammar remains metadata. The current catalog has exactly one fixed disposition with an explicit grammar: `root-script`, using `typescript` and `command-router`.

## Gate Seam

`fixedSourceDispositionDecision` is a pure discovery-layer decision over an exact fixed contract identity, source text and catalog taxonomy. It returns no decision unless the source disposition is fixed and declares its own grammar. This makes the source/metadata distinction reusable without hiding it in package-role logic.

Normalization calls that decision only after `canonicalFile` has selected the exact fixed contract. It then projects a semantic violation independently of `classifyPackageRole`. A successful read must supply actual bytes before UTF-8 decoding; a failed read no longer becomes an accidental empty string.

The command-router grammar gained two vocabulary forms grounded in the live process routing API:

- an imported value `Script` can be the single command class base alongside `BundleScript`;
- imported value `runWorkspaceScriptMain` can be the single terminal alongside the established package terminals.

Both retain original imported export identity, lexical binding and recursive argument checks. Type-only `Script` and type-only `runWorkspaceScriptMain` remain rejected.

## Portable Contract

The existing language-neutral package-boundary fixture and schema now include ten fixed-script cases:

| Placement | Manifest | Proven route | Rejected control |
| --- | --- | --- | --- |
| repository root | not applicable | `Script` plus `runWorkspaceScriptMain` | direct `Bun.write` |
| repository root | not applicable | runtime value imports | type-only `Script` and type-only `runWorkspaceScriptMain` |
| neutral domain | not applicable | `BundleScript` plus `runBundleScriptMain` | ambient `fetch` |
| TypeScript package | present | imported artifact package terminal | direct process output |
| TypeScript package | absent | imported artifact package terminal | ambient `fetch` |

Every path is independently checked against the exact `root-script` fixed contract. The same bodies are checked by the owned parser and the installed TypeScript AST oracle. Runtime workspace forms have no TypeScript semantic diagnostics; the paired type-only forms produce TS1361. Manifest-present and manifest-absent package cases also pass through public `discoverPackageProblems`, where rejected bodies remain `package-role-unresolved`.

The unavailable-content vector returns `fixed-source-content-unreadable`. A native EACCES inventory probe created an exact fixed script under the ticket-generated root with mode `000`; its row contained both `path-read-failed` and `fixed-source-content-unreadable`, then the probe restored permissions and removed the file. The cancellation control still stops inventory with `Taxonomy operation cancelled`, and successful scoped inventories report progress.

All newly added package, cancellation and read-failure controls use a unique `mkdtemp` child beneath `SEMIO_TEST_ARTIFACT_DIR`, with a ticket-generated fallback for ordinary direct runs. Before allocation, every path segment from the repository root through the artifact base is checked with `lstat` and must be a plain directory rather than a symbolic link. Each control removes only its own allocated child.

The durable read-failure control uses `inventoryTaxonomyWithCapturedSourceRead`. Source admission, no-follow path observation, fixed-name resolution, package classification, cancellation and progress remain on the ordinary inventory path. Only the admitted leaf-byte read is supplied by an explicit synchronous provider. The portable fixture's provider throws for its exact valid `📜️script.ts`; the inventory row retains `root-script`, `fileKind:null` and `not-package`, and contains both `path-read-failed` and `fixed-source-content-unreadable`. This avoids permission-bit behavior that differs across Windows, Unix and privileged test processes.

## Live Inventory Evidence

Focused inventory read all three current source bodies and preserved their structural facts:

| Path | Duration | File kind | Fixed contract | Package role | New finding |
| --- | ---: | --- | --- | --- | --- |
| `📜️script.ts` | 21,783.4 ms | null | `root-script` | `not-package` | `fixed-source-disposition-unresolved` |
| `♻️mit-bestand/🧺️demonstrator/📜️script.ts` | 5,996.7 ms | null | `root-script` | `not-package` | `fixed-source-disposition-unresolved` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | 7,240.6 ms | null | `root-script` | `configuration` | `fixed-source-disposition-unresolved` |

These findings expose bodies that the fixed-name/package-role interaction previously skipped. They do not prove that every rejected source contains domain implementation. In particular, the demonstrator's `import.meta.vitest` form remains a vocabulary review.

The permanent live integration assertion does not require these files to stay unresolved. It reads each current body, derives the expected disposition with the independent installed TypeScript AST oracle, and requires normalization plus the pure owned decision to agree. The table records this execution's observations only and is not a body-hash assertion.

A fresh no-follow `rg --files -g '📜️script.ts'` census observed 454 current scripts. Direct classification reported 278 `tool-metadata` and 176 `unresolved`. This is a current shared-checkout observation, not a causal comparison with the earlier 455-source census and not 176 confirmed extraction defects.

## Registered And Native Verification

- Initial red: the new fixture stopped because `fixedSourceDispositionDecision` did not exist. After that seam was implemented, the runtime `Script` case exposed the missing workspace vocabulary before the parser change.
- Focused durable read-failure regression: `1 pass, 0 fail, 4 expect() calls` in 3.82 seconds after an initial red missing-export failure.
- Direct Bun suite: `121 pass, 0 fail, 530 expect() calls` in 20.71 seconds. This retains the accepted lexical scope, type-only fallback, manifestless discovery, live-oracle, progress and cancellation controls.
- Registered Nx target with isolated workspace and cache data plus `--skip-nx-cache`: `121 pass, 0 fail, 530 expect() calls`; Bun 25.73 seconds, Nx 26.0 seconds, one task.
- Strict `loadTaxonomy()` followed by `validateTaxonomy()`: schema version 7, zero diagnostics.
- JSON parser and AJV fixture validation: schema and fixture accepted.
- `git diff --check` over the five implementation/contract files: no diagnostics.
- Existing `@semio-tech/repo-lib:test-package-body-policy` remains the single registered Bun/Nx route for this shared classifier suite. Its test verifies the project target, package script and both launch authorities, so no duplicate launcher was added.

## Exact Changed Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
  - added the pure explicit-grammar fixed-source decision;
  - added imported value `Script` and `runWorkspaceScriptMain` to the closed router grammar.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
  - projected fixed-source unreadable/rejected findings after exact filename resolution;
  - required real bytes before decoding source content;
  - added an explicit captured leaf-byte provider entry while preserving ordinary physical source admission.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification/🔣️.json`
  - declared the portable fixed-script placement/manifest/body and read-failure vectors.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json`
  - added the ten runtime, hostile and type-only fixed-script cases plus the deterministic captured-read failure case.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`
  - added portable, TypeScript AST/semantic, public discovery, live inventory, progress, read-error and cancellation checks;
  - made live expectations observational from the independent oracle, isolated new temporary controls beneath a no-follow ticket artifact root, and asserted both integrated read-failure diagnostics.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-fixed-script-semantic-enforcement-2026-09-12.md`
  - retained this execution record.

## Limits And Follow-Up

This lane does not extract or approve the 176 scripts observed as unresolved in the earlier census. Direct `router.run`, `import.meta.vitest`, the named Stdio terminal and Puzzle's scalar stack expression remain distinct vocabulary/source reviews recorded in `📓️small-script-disposition-review-2026-09-12.md`. Concurrent owner extraction subsequently reduced the plugin registry and descriptor scripts to accepted tool metadata without any router grammar expansion in this lane. The LaTeX `.sty` and `.cls` metadata/body gap requires its own grammar contract and is outside this fixed-script correction.
