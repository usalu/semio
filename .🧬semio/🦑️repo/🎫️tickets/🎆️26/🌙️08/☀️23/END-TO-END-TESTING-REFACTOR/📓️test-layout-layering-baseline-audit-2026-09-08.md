# Test Layout Layering Baseline Audit

Read-only audit on 2026-09-08 of the test-layout paths in the root `🧅️layering.json`. It does not change the baseline, production source, Git state, or ticket state.

## Counting Rule

`layeringReferences` scans authored repo-wide and framework files. For every implementation area in `areaLayers`, it counts literal occurrences of `<area>/` in file text. `layeringCounts` sums those occurrences by file. A baseline value is therefore a file-specific upper bound on implementation-area references. It is not a test count, line count, or test-body budget.

The baseline comment requires a zero-count file to be removed. A path rename may carry its existing cap to one or more replacement files only when the moved body proves the relation. It must not absorb unrelated current drift.

## Proven Direct Moves With No Current Reference

The TypeScript and native-path move records establish these replacements. Each current target has zero literal implementation-area references, so no cap should be copied. Remove the stale entries after the path move.

```json
[
  {
    "stalePath": "🧪️vitest.config.ts",
    "allowed": 1,
    "currentPath": "vitest.config.ts",
    "currentCount": 0,
    "action": "remove"
  },
  {
    "stalePath": "⚡️vitest.config.ts",
    "allowed": 1,
    "currentPath": "vitest.config.ts",
    "currentCount": 0,
    "action": "remove"
  },
  {
    "stalePath": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️test/🟦️s.ts",
    "allowed": 1,
    "currentPath": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
    "currentCount": 0,
    "action": "remove"
  },
  {
    "stalePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️.go",
    "allowed": 10,
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go",
    "currentCount": 0,
    "action": "remove"
  },
  {
    "stalePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go",
    "allowed": 150,
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go",
    "currentCount": 0,
    "action": "remove"
  },
  {
    "stalePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts",
    "allowed": 36,
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts",
    "currentCount": 0,
    "action": "remove"
  }
]
```

## Pre-existing Overages Must Remain Visible

The baseline first appears in commit `fe7c8a8f8b`. At that revision its renderer entry allowed 2 although the old renderer body contained 4 references. Its library entry allowed 39 although the last source before the same commit deleted it contained 66 references. The current semantic test leaves retain these bodies, but changing a location does not authorise a budget increase.

```json
[
  {
    "stalePath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts",
    "baselineCommit": "fe7c8a8f8b",
    "allowed": 2,
    "preGoalSourceReferenceCount": 4,
    "preGoalReferenceAreas": { "✏️s": 2, "♻️mit-bestand": 2 },
    "currentPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
    "currentReferenceCount": 4,
    "currentReferenceAreas": { "✏️s": 2, "♻️mit-bestand": 2 },
    "bodyEvidence": { "preGoalNonblankLines": 4796, "currentNonblankLines": 5753, "sharedExactNonblankLines": 4733 },
    "action": "retarget existing cap 2; leave the current 4-reference overage visible"
  },
  {
    "stalePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts",
    "baselineCommit": "fe7c8a8f8b",
    "allowed": 39,
    "preGoalSourceRevision": "fe7c8a8f8b^",
    "preGoalSourceReferenceCount": 66,
    "preGoalReferenceAreas": { "✏️s": 66 },
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
    "currentReferenceCount": 72,
    "currentReferenceAreas": { "✏️s": 70, "🌎️hub": 2 },
    "bodyEvidence": { "preGoalNonblankLines": 5106, "currentNonblankLines": 5535, "sharedExactNonblankLines": 4891 },
    "action": "retarget existing cap 39; leave the current 72-reference overage visible"
  }
]
```

Neither record supports an increase to 4 or 72. The test-layout work preserves source provenance; it does not forgive the older reference excess.

## Proven Rust Sweep Split

The stale aggregate Rust source was successively renamed and then split into canonical test leaves. The source budget of 18 maps exactly to five current implementation files. These counts are current literal `✏️s/` references; their sum is exactly 18.

```json
{
  "stalePath": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture/🦀️-sweep.rs",
  "allowed": 18,
  "currentPathCounts": {
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-auto-discovery/🦀️.rs": 13,
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️example-asset-discovery/🦀️.rs": 1,
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-grammar-conformance/🦀️.rs": 1,
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-protocol-conformance/🦀️.rs": 1,
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🧹️fixture-sweep/🦀️.rs": 2
  },
  "total": 18,
  "action": "replace the one stale entry with these five exact entries"
}
```

The adjacent TypeScript command-check leaf `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts` has count 0 and is not part of the Rust split.

## Entries Deliberately Left Untouched

The stale math component entry (`🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/🦀️component.rs`, 2) and directory schema component entry (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️component.rs`, 1) have current plausible canonical test locations but no retained source-origin evidence in this audit. Leave both baseline entries unchanged until their owning migration supplies that proof.

## Applied Baseline Verification

The final root baseline preserves the two proven caps without a budget raise:

~~~
{
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts": 2,
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts": 39,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️example-asset-discovery/🦀️.rs": 1,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-auto-discovery/🦀️.rs": 13,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-grammar-conformance/🦀️.rs": 1,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-handcrafted-protocol-conformance/🦀️.rs": 1,
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🧹️fixture-sweep/🦀️.rs": 2
}
~~~

The five Rust sweep entries total 18. The six obsolete test-layout baseline paths documented above are absent. The old CLI production allowance (10), math component allowance (2), and directory-schema component allowance (1) remain unchanged because this audit did not prove their body-origin transfer.
