# Mutation Root Discovery Checkpoint 38

## Scope and Correction

The exact production write is `policyFindAllMutationsDirs` in root `📜️script.ts`. It now shares `MUTATION_TAXONOMY_SOURCE_SKIP`, includes repository-authored packages, tests, fixtures, examples, assets, targets, visible generated source and manifests, and keeps descending after discovering a mutation facet. Opaque/cache/hidden paths are pruned before access. Owned-root ancestors and discovered children are checked without following symlinks; non-directory, unreadable and vanished discovered inputs fail closed. Only absent configured roots are skipped. Relative and absolute escaping roots are rejected before access.

No general taxonomy helper, library target, launch registration, production normalization, generated output, or peer source was changed for this correction. Taxonomy's bounded Draw hold was honored until explicit release. The peer preserved the separate Dag launch entry and mounted it in its canonical seed.

## Tests Actually Executed

Both registered cases run through the existing library Bun/Nx test route, filtered by `mutation root discovery`, with `SEMIO_TEST_ARTIFACT_DIR` set to this ticket's `🧪️mutation-root-discovery-38`. All physical fixtures and results remain retained. No real `compose/**` was created or traversed; opaque and excluded trees exist only in the virtual fixture.

- Initial virtual red: `🧫️discovery-virtual-rfTINS`, 0/32 cases, unchanged source. This exposed both authored-root omissions and unsafe/silently swallowed traversal outcomes.
- Actual physical red: `🧫️discovery-physical-hqrhF1`, each compiler found only1of13 required facets; independent fast-glob found13. Log `🧪️mutation-root-discovery-physical-red-38.log`, exit1.
- First green: `🧫️discovery-virtual-SSTKZa`,32/32; `🧫️discovery-physical-vQvnem`,13/13 from each compiler.
- Final expanded green: `🧫️discovery-virtual-fGwQLk`,52/52 across POSIX/Windows path semantics and Bun/TypeScript compilation; `🧫️discovery-physical-s6SEjm`,13/13 from both actual-function implementations and fast-glob. Both registered tests passed,310 unrelated tests filtered out,13 assertions,6.18s, exit0. All four source/fixture/test input hashes remained stable during both final cases.

The language-neutral fixture is schema-validated with Ajv. The virtual test evaluates the actual extracted function and constants, not a reimplemented walker. The physical test evaluates the actual owned-root selector and walker against a newly manufactured, retained filesystem with fixture-supplied taxonomy areas. These are facet-discovery tests, not proof that a facet has a correct aggregate/leaf bijection; the structural policies remain required.

## Separate Public Inventory Boundary

The initial public `inventoryMutationTaxonomy` fixture attempt stopped in current `loadTaxonomy` before reaching physical discovery because five `wgpu-frame-worker` tracked outputs were missing: Rust builder/binary, TypeScript library, renderer registry and generated frame-worker JavaScript. That exact failure remains in `🧪️mutation-root-discovery-red-38.log`. No outputs were restored, regenerated or fabricated. The subsequent direct-helper gate does not establish public inventory or full-monorepo readiness.

A later fresh standalone replay, after taxonomy independently reported its real-workspace source-phase check green, did **not** reproduce that error. `🧪️mutation-discovery-public-38/🧫️run-Wwqvaj` loaded the public taxonomy successfully and the public inventory found exactly13of13 retained fixture facets, with root/discovery/controller input hashes stable and exit0. Its inventory digest is `a079b0a64975085992e2bc6c93a5fe43aacb1d0c0844fc2b6c105e5dd2aef8c2`. The fixture deliberately has2structural violations; this is discovery integration, not structural acceptance. No root script or WGPU source/output changed in this replay. The earlier error is a historical process observation; fixture/global-context or transient-state causality is unproven, and it must not be presented as a current missing-output authority failure. No real-workspace inventory ran.

## Released Hashes

| Input | SHA-256 |
| --- | --- |
| Root `📜️script.ts` | `ea4ce1967af1e7ec122a26491393d8ef79a5b7beac59e3b8da8db801e943efeb` |
| Extracted virtual helper and two constants | `44e3358145be5aaa664189cf9fda43c914429e8459b36a0466e86c57bf704f41` |
| Library TypeScript `🧪️index.test.ts` | `a87d53c14db003629708a87819b29d6a39cbee96990e67c310a7f8299906133f` |
| Discovery neutral fixture | `f47b28fe1093ca1556309c4c899634c499b5c9c749ac283972e5e024c00e6fcd` |
| Discovery fixture schema | `5b3c138f02c6ed7b48e437f66cad87fbe08d1ec7b1c13350bdfa2f311868eab4` |

The read-only review's claimed missing-constant assertion failure is contradicted by actual red/green execution: both top-level declarations were found. Its proposed exclusion of authored package/test/fixture roots reverses the required contract and was rejected. Its facet-discovery versus structural-proof distinction is retained here.

This bounded correction is released. The repository goal and ticket remain active; no global zero-violation or all-plugin acceptance is claimed.
