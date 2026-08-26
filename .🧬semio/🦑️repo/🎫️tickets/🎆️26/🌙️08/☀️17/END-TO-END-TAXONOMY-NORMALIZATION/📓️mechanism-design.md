# Taxonomy Normalization Mechanism Design

## Production Ownership

| Family | Sole writer | Intended files |
| --- | --- | --- |
| Schema and registries | `S-CORE-SCHEMA` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, taxonomy types and validation |
| Inventory, canonicalization and plans | `S-CORE-ENGINE` | New repo-library semantic normalization region/module and its exports |
| Transaction, journal and rollback | `S-CORE-TRANSACTION` | Same normalization module, disjoint transaction regions after frozen interfaces |
| CLI and launch integration | `S-CLI` | Root `📜️script.ts`, root `📋️project.json`, launch seed and generated launch catalog |
| Tests and fixtures | `S-TEST` | Repo-library test family and active-ticket golden fixtures |
| Shared reference families | `O-00` until split | Workspace manifests/configuration and generated registries |

## Frozen Interfaces

The mechanism exposes `inventoryTaxonomy`, `planTaxonomy`, `applyTaxonomyPlan`, and `verifyTaxonomy`. The CLI is a thin adapter. Each operation accepts a repository root, optional scope, progress callback, and cancellation predicate. Planning accepts the immutable baseline commit and the recorded opaque-tree digest. Apply additionally accepts the exact plan path and digest.

Inventory enumerates tracked paths from the Git index and the active ticket only. It filters an excluded prefix before any filesystem read and never resolves a symlink below an opaque prefix. Directories are derived from admitted file paths; untracked build and dependency trees are outside the versioned-path contract.

```text
TaxonomyInventoryEntry
TaxonomyMove
TaxonomyEdit
TaxonomyRegeneration
TaxonomyPlan
ReferenceEdit
TaxonomyJournal
TaxonomyVerification
```

All public records contain only repository-owned primitives and types. JSON rendering recursively sorts object keys and stable-sorts arrays by their contract identifiers. Digests cover the canonical UTF-8 bytes, never in-memory object order.

## Schema Version 7

Version 7 adds `fileKinds`, `semanticDirectoryKinds`, `fixedFilenameContracts`, `configurableEntryContracts`, `packageBoundaryRules`, `packageGlueGrammar`, `pathExclusions`, `unicodeNormalization`, `variationSelectorPolicy`, `collisionPolicy`, and `areaEnforcement`.

The incompatible cut removes semantic leaf, entry, example, story, specification, test-contribution, surface-schema and root-data filenames once all consumers read kind IDs and exact entry contracts. Broad packaging suffix exemptions are removed. `compose/` is represented only as an opaque path exclusion, not as an enforceable area.

## Canonicalization

1. Normalize each admitted path segment to NFC and canonical VS16 form.
2. Match exact fixed-name contracts before file-kind classification.
3. Resolve the longest registered extension chain and its kind-only basename.
4. Drop generic stems when the parent already owns the concern.
5. Otherwise resolve a semantic directory from the central registry and move the file below it.
6. Apply directory-prefix normalization to every destination segment.
7. Group byte, NFC, case-folded, VS16-folded, same-kind and platform-reserved collisions.
8. Block on every unknown semantic, uncertain package role, unsupported reference or collision without a deterministic destination.

No fallback assigns a decorative emoji. Context defaults exist only as explicit registry rules for tests, fixtures, assets, configuration, documentation and generated output.

## Reference Model

Every edit records an adapter, a structured location and byte preimage. Initial adapters are Rust, TypeScript/JavaScript, Go, Python, .NET, native/CMake, JSON/JSONC, YAML, TOML, XML, Markdown, Nx/launch/task configuration and generated registry/template ownership. A literal replacement is legal only inside a parser-recognized path-bearing token with a unique old target.

## Transaction

Apply validates plan bytes, plan digest, baseline identity, source hashes and opaque-tree digest. It records preimages under the active ticket, stages every moved file under collision-safe operation IDs, creates destinations, installs staged files, edits final-path preimages, runs declared regenerations, verifies locally, and only then commits the journal state. Failure or cancellation restores edited bytes and reverses installed/staged paths. The audit report and journal remain in the ticket; transient staging is removed on success.

## Verification Proof

The pilot covers named Rust and TypeScript leaves, package implementation extraction, test-case and asset directory creation, generated output, fixed Cargo metadata, same-kind collision, case/VS16 normalization, shared manifest rewrite, injected rollback and cancellation. Convergence requires two byte-identical inventories, two byte-identical pre-apply plans, one successful apply, an empty second plan, zero verification violations, and the unchanged `compose/` digest.
