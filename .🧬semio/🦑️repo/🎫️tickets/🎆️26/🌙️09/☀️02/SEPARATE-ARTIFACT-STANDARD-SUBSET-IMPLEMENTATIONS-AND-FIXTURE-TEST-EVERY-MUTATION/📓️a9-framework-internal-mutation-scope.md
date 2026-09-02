# Framework-Internal Mutation Vocabulary Scope

## Decision

Selected option 2: the `unregistered-mutation-vocabulary` contract now applies to every mutation
directory below `🏅️standards` and to every unprofiled mutation directory whose exact computed owner
has a discovered Gherkin case. It does not apply to an unprofiled, featureless framework-native
mutation tree.

The rule is an artifact/Gherkin completeness gate. Registering any of the 30 A9 framework trees
would create a manifest claim that the Gherkin platform cannot discharge, immediately replacing the
existing finding with `mutation-catalog-unclaimed`. The PNG exemplar confirms that a truthful claim
requires an owner-level feature, catalog tag, scenarios, fixtures and real adapter handlers; empty
catalog metadata would not be coverage.

## Census

The full non-excluded repository contains 31 mutation directories outside `🏅️standards`:

- one already-cataloged OS-config vocabulary;
- exactly the 30 A9 paths with neither a catalog at the validator's computed owner nor a feature
  discovered for that exact owner.

The 30 paths contain 341 files and divide structurally into 17 test-owned trees, seven fixture-owned
trees and six framework-source trees. Their Rust sources contain 107 local `#[test]` attributes;
the remaining fixture-only leaves are referenced by tests outside the leaf. They are native
framework mutation/diff/inverse coverage, not empty artifact-format catalogs.

Exact owner matching matters. The three OS-config features below `🔌️plugin/🖥️host` do not make the
unrelated `🔌️plugin` fixture trees Gherkin-owned. Conversely, a future unprofiled vocabulary at
`<owner>/🧬️schema/🧬️mutations` remains in scope as soon as `<owner>/🧪️tests/<case>/🥒️.feature` is
discovered. Catalogs that exist without a claiming feature remain independently rejected by
`mutation-catalog-unclaimed`.

## Implementation

`mutationVocabularyRequiresCatalog` documents and implements the boundary using the full discovery
set, not the caller's possibly narrowed case selection. `validateAllContracts` reuses that same full
case set for `mutation-catalog-unclaimed`, so the change adds no extra repository walk.

A focused Bun/Nx regression covers all four boundaries:

- profiled vocabulary without a feature: required;
- unprofiled vocabulary with an exact owner feature: required;
- unprofiled vocabulary without a feature: exempt;
- a feature on an ancestor of a nested fixture vocabulary: not an exact owner and therefore exempt.

## Cross-Shard Check

The companion nested-profiled-owner shard independently kept the three GIS editor/presence
vocabularies in scope and reported them among the 13 remaining findings. Its change permits nested
subset owners to register truthful catalogs later; this change preserves the standards side of that
contract unconditionally. The two boundaries therefore compose without hiding the GIS gap or any
other standards vocabulary.

## Verification

- Before `bun ./📜️script.ts test contract`:
  `unregistered-mutation-vocabulary: 43`, `mutation-catalog-unclaimed: 8`,
  `contribution-manifest-invalid: 0`, `mutation-vector-unregistered: 0`,
  `mutation-catalog-capability-mismatch: 0`.
- After the same command: `unregistered-mutation-vocabulary: 13`, with exactly the 30 A9 framework
  paths removed and no path added. `mutation-catalog-unclaimed` remained 8 with the exact same set:
  `gltf-2-0-animation`, `gltf-2-0-asset`, `gltf-2-0-buffer`, `gltf-2-0-camera`,
  `gltf-2-0-material`, `gltf-2-0-mesh`, `gltf-2-0-scene`, `gltf-2-0-skin`. The three other watched
  classes remained at zero.
- The focused Nx test passed: two tests, eight assertions, covering this scope rule and the companion
  nested-profile owner rule.
- The repository-wide contract command still exits non-zero because the live tree has 1,953
  unrelated high-priority breaches. The targeted mutation-catalog measurements above are green.
- The package type-check reaches the edited package but remains blocked by five unrelated existing
  errors: three in the shared UI/library dependencies and two in an unchanged validator section at
  line 5576. No diagnostic points to this change.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts`
- this report
