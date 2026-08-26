# Fixed Contracts and Package Analyzer Schema Closure

## Result

Taxonomy v7 now loads strictly with tagged fixed scopes, one deterministic winner, explicit decisions for all 32 residual fixed-looking paths, no historical suffix or Cargo-cache blanket, and first-class JavaScript plus fail-closed C/C++ package analyzer authority.

The closure is schema-first. No physical source/package/ticket tree was moved, no normalization/shared test file was edited, and neither opaque Compose path was read.

## Fixed-contract schema

`FixedContractScope` is the closed tagged union:

```text
{ kind: exact-path, path }
{ kind: repository-root }
{ kind: package-root, ecosystemId }
{ kind: directory-kind, directoryKindId }
{ kind: path-pattern }
```

Legacy sibling `ecosystemId`/`directoryKindId` fields are not accepted. Exact-path scope requires a wildcard-free `scope.path` identical to `pathPattern`; repository-root requires one wildcard-free basename; package and directory scopes require resolvable registry IDs.

`fixedFilenameContractIdsForPath()` now returns zero or one winner. Matching candidates are ordered by literal segments, literal code points, wildcard count, and scope strength. Two top candidates with the same specificity throw rather than selecting lexically. The same rule applies to fixed directories.

All 69 current filename/directory records use tagged scopes. A Git-index/current-untracked census of 64,835 non-opaque paths resolved 4,046 fixed filename winners and found zero equal-specificity errors.

## Exact closure of the 32 misses

Nine current paths now have exact-path contracts:

| Contract group | Count |
|---|---:|
| `.cargo/config.toml` and `.codex/config.toml` | 2 |
| Repository and window-kits `tsconfig.json` | 2 |
| Repository `pyproject.toml` | 1 |
| Repo CLI, MCP client, library, and coordinator `go.mod` | 4 |
| **Total** | **9** |

The remaining 23 paths are exact members of three `fixedFilenameRejectionContracts`, not exceptions:

| Rejection contract | Identities | Disposition |
|---|---:|---|
| `nested-ticket-manifests` | 19 | relocate |
| `ticket-scratch-go-modules` | 2 | normalize |
| `ticket-progress-documents` | 2 | normalize |
| **Total** | **23** | |

Every identity is NFC, workspace-relative, wildcard-free, present in the current tree, and unique across the rejection registry. `fixedFilenameRejectionContractIdForPath()` uses exact identity membership and returns no result for a similar unregistered path.

Broad contracts were narrowed or removed:

- root Cargo and Node manifests now match only `Cargo.toml` and `package.json` at repository root; package-root contracts remain ecosystem-scoped;
- root README and LICENSE now match only repository-root `README.md` and `LICENSE.md`;
- the Nx package/project manifest pattern requires at least one path segment, leaving the root manifest to `root-project`;
- `cargo-cache-tag` was removed;
- `scopedFileKinds` is empty; `historical-ticket-evidence` and its 37-suffix `^.+$` allowance were removed.

Nested README/LICENSE/cache/evidence files are consequently renameable inputs. They did not gain substitute exceptions.

## Package authority

### Active JavaScript boundary

The existing registered package member `🟨️javascript` now has its own ecosystem, `boundary-only` package identity, boundary rule, and `javascript` analyzer. It does not alias the TypeScript ecosystem contract even though both analyzers share the same conservative import/export implementation branch.

The 12 current JavaScript package leaves are no longer unsupported. Exact content classification reports:

```json
{"files":12,"roles":{"implementation":12}}
```

Those 12 are now explicit `package-implementation` candidates and remain physical-move work for normalization.

### C/C++ boundary

`packageGlueGrammar.c-cpp` admits include aggregation, declarations, and immediate ABI delegation; it rejects type definitions and non-trivial function bodies. `packageBoundaryProfiles.c-cpp` binds the analyzer to `c-source` and `cpp-source`, recursion, and fail-closed uncertain/implementation roles.

There is no current repository semantic package-language directory precedent for C or C++. The profile therefore has `admission: blocked-until-language-directory-registered`. No emoji or language directory was invented, and an unexpected native package directory remains unknown/unadmitted.

### Source-format fixed/configurable dispositions

Exactly six source-format contracts have required `packageSourceDispositions`:

| Contract | Kind | Disposition / validator |
|---|---|---|
| `root-script` | fixed | tool-metadata / command-router |
| `python-init` | fixed | adapter-source / package-glue |
| two Rust entries | configurable | adapter-source / package-glue |
| two TypeScript entries | configurable | adapter-source / package-glue |

Validation derives this required key set from physical source kinds, rejects extra/missing dispositions, and requires authority plus verification. The package walker no longer skips a source leaf merely because its basename is fixed/configurable: it selects the disposition and analyzer explicitly. All active boundary rules admit the exact Nx manifest and command-router contracts as package tooling.

Across 172 current package `📜️script.ts` files, the structural command-router check finds 171 valid `ScriptRouter`/`runBundleScriptMain` routers and one unresolved empty file:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📜️script.ts
```

The empty file remains a real physical blocker; it was not whitelisted.

Manifestless `boundary-only` ecosystems are now traversed for package purity, closing the prior production blind spot for JavaScript and .NET boundaries.

## Reusable APIs

- `fixedFilenameRejectionContractIdForPath(path, taxonomy)`
- `classifyPackageSourceRole(content, grammar)`
- `classifyPackageSourceDisposition(content, disposition, grammar)`

All accept repository-owned types only and use no new runtime dependency.

## TDD and verification

Failing-first ticket harness:

```text
bun .../🧪️fixed-package-authority.ts
exit 1: historical-ticket-evidence was still present
```

Final ticket harness:

```json
{"exactFixedContracts":9,"fixedRejectionIdentities":23,"packageBoundaryProfiles":1,"packageSourceDispositions":6}
```

Strict load and CAD/Draw regression:

```text
schemaVersion=7, fixedFilenameContracts=56, fixed rejection identities=23, problems=[]
Draw: 11 files, 9 directories, 20 nodes, 2 reference edits
Draw digest: 1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b
```

Focused Bun tests:

```text
bun test ./🧪️index.test.ts --test-name-pattern='parses ...|closes generator ownership'
2 pass, 231 filtered out, 0 fail, 46 assertions
```

Focused Nx test with a shell-safe selector:

```text
bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=parses
1 pass, 232 filtered out, 0 fail, 30 assertions
NX successfully ran @semio-tech/repo-lib:test
```

An earlier combined selector containing a bare `|` reached one passing Bun test but caused `/bin/sh: closes: command not found`; it is command-quoting evidence, not a test failure. The shell-safe rerun above is authoritative.

`bun nx run @semio-tech/repo-lib:lint` reached TypeScript and reported only the existing UI `ImportMeta.env/glob` and cross-project `rootDir` diagnostics; it emitted no taxonomy/discovery diagnostic.

## Remaining exact physical decisions

1. Relocate the 19 nested ticket roots transactionally or author first-class repo-MCP child tickets; the schema does not authorize them in place.
2. Normalize the two scratch Go module filenames and two historical progress notes as retained evidence.
3. Move the 12 JavaScript implementation leaves out of package boundaries, leaving only canonical adapters.
4. Resolve the one empty MCP Go package command router.
5. Register a C/C++ package language directory only when a unique repository authority exists; until then the analyzer profile remains intentionally unadmitted.
6. Normalize nested README/LICENSE/cache and historical evidence leaves now exposed by removing blanket contracts.
