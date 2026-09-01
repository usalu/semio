# 🌳️ Kind-Only Leaves — Final Session Status

Goal: the monorepo follows the domain-driven multi-implementation tree taxonomy — **folders carry
semantics, files are kind-only leaves** (`🦀️.rs`, `🟦️.ts`, `🔣️.json`) — with implementation-neutral
folders for schema, components, fixtures, tests and oracles.

Session baseline: `bb06c41f73f0122fbed315b7487428b976f99921`.

## 1. Repo-wide position

| | files |
|---|---:|
| kind-only leaves | **7 343** |
| legacy named leaves | 28 634 |
| **kind-only share** | **20.4%** |

Legacy stems by area: `✏️s` 27 960 · `🧰️framework` 652 · `🌎️hub` 18 · `♻️mit-bestand` 4.

The framework layer is now the small remainder. `✏️s` is the overwhelming majority and is being
migrated by a concurrent session using the same mechanism this ticket repaired.

## 2. Fully migrated framework modules (kind-only leaves, zero legacy stems)

`⏱️trace` 6 · `⏳️async` 8 · `🎠️kernel` 15 · `🎯️action-bus` 3 · `📐️geometry` 3 ·
`📡️replication` 27 · `🔄️machine` 5 · `🔢️hash` 2 · `🔢️number` 2 · `🔺️mesh` 1 ·
`🔺️mesh-engine` 1 · `🧮️action-argument-resolution` 1 · `🧮️math` 2 · `⚠️diagnostic`

Plus the `🖍️draw` editor-command projection (11 moves) and the `#⃣hash` → `🔢️hash` directory rename.

## 3. What actually made this possible

Before this session **no rename had ever been applied** — the pipeline could not converge. Thirteen
distinct defects were found and fixed, each a guard checking a PROXY for the property it enforced:

| # | guard | proxy checked | property meant |
|---|---|---|---|
| 1 | reference closure | file is tracked/untracked | file is a live reference |
| 2 | closed-ticket prose | path token appears in text | reference must be rewritten |
| 3 | generator inputs | path is under repo root | path is authored source |
| 4 | frozen coordinates | one fixture's exact bytes | evidence is pinned |
| 5 | `rustFiniteManifestTargets` | raw-text `/[#!]/` scan | text can contribute module structure |
| 6 | trust scan | `#[cfg]` present | module structure is conditional |
| 7 | authority proof | glob import present | file participates in module graph |
| 8 | macro trust | macro invoked | macro can emit `mod` |
| 9 | `targets?.length !== 1` | one message for two states | no-targets vs many-targets |
| 10 | `inspectRustJoinArgumentSpans` | `.join("literal")` textually | filesystem path join |
| 11 | `jsonTokens` | `{workspaceRoot}/…/**/*` only | Nx path reference |
| 12 | `resolveReferencePath` | repo-root before sibling | which file a bare token names |
| 13 | `impl Trait for Type` | parsed as a `for` loop | end of file scan |

Defect 12 is the most serious: it produced a **false `unresolved=0`** — a plan that reported clean
and still failed at apply.

Other structural fixes: the pipeline's own artifacts re-entering its reference closure (504 vs 124
unresolved on identical input); generator input enumeration recursing into the `♻️mit-bestand`
submodule; historical evidence (ticket reports, ticket workspace, `.cursor` plans, dev prompt logs)
treated as live references; `fileKinds.svg` registered `🎨️` while 690 of 692 files on disk use `🔣️`;
`model-3d` conflating four physically distinct formats; ~2 346 lines of duplicated proc-macro
implementation across two `✨️derive` crates.

Vocabulary grew from 135 `semanticDirectoryKinds` to 300, and `fileKinds` from 73 to 81.

## 4. Recurring hazards worth remembering

- **Two directory kinds must never share a `parentKindIds` while sharing a `slugPattern`.** Bit twice
  (`asset-binary-subject` vs `asset-subject`, 471 rows and ~470 suppressed moves; `json-fixture-case`
  vs `fixture-case`). Fix by SCOPING the parent, not by `inferWithoutEmoji:false` — that flag
  suppresses the kind's own no-emoji catch-all and two workers then fought over adding/removing it.
- **A file-kind's leaf emoji must differ from its paired subject-folder emoji** (`video` 🎬️ vs
  `asset-video-subject` 🎥️), or any no-emoji stem silently collides with `asset-subject`.
- **Cross-check every new word against `semanticDirectoryMemberKinds.memberNames` repo-wide.**
- **A row that disappears because it stopped being DETECTED, not because it became REWRITABLE, is a
  silent regression.** Prove which by diffing the plan's `edits` for real `oldValue`→`newValue`.
- `--scope` takes ONE path; a comma-separated list silently matches nothing (`moves=0 unresolved=0`).

## 5. The remaining structural obstacle

**The apply window is longer than the interval between concurrent writes.** Each plan is a 10-15
minute full-tree scan, and three consecutive applies failed on churn rather than defects:
`Plan affected pre-state digest cannot be rederived`, `Transaction repository index changed since
capture`, and `🖥️platform` oscillating 0 → 1 blockers between consecutive plans.

Root cause: the `plugin-registry` generator enumerates **40 232 repo-wide inputs**, so every plan is
sensitive to every edit anywhere. Narrowing that input scope is the highest-value remaining
engineering task — without it, large applies cannot be landed reliably in a live tree.

## 6. Path-budget finding (measured, changes the plan)

Shortening test-case slugs was applied to all 14 offending `🌍️gis` scenarios with a deterministic
rule (`📓️goal-slug-rule.md`, `📜️goal-slug-tool.ts`). Result: over-budget paths **73 → 65**, longest
**289 → 278 bytes**. It cannot close the gap: the fixed scaffolding before any scenario name already
costs 231-236 bytes, leaving under 10 for the name; even `✂️delete-route/removes-tram-route`
overflows by 7-8 bytes.

**The fix is structural.** `🏅️standards/🔖️1/🪆️subsets/✳️any` (~40 bytes) collapses to `🪆️1-any`
(~9 bytes) — a projection the engine already performs, applied to `🖍️draw` this session. That is
~31 bytes back on every plugin path, the right order to close a 7-38 byte overflow.

## 7. Known-remaining, precisely located

- `🖱️ui` ~1 152 moves · `🖼️assets` 1 089 moves — both blocked on small, identified row sets.
- `🖼️assets`' 4 wgpu rows: `repositoryReferenceCandidatePaths` never consults
  `generatorContracts.outputRoots` for ANY contract, so generator outputs are reference-scanned when
  the engine's own intent is that they should not be. Architectural gap, located, not fixed.
- `🎠️kernel`-class correct refusals: constructs the provers decline on purpose, with oracle-tested
  suppression. Documented, not overridden.
- `📺️renderer` wgpu package projection (179 blockers) gates the assets regeneration chain.
- `✏️s` 27 960 legacy stems — the bulk, concurrent session's territory.

## 8. Why `✏️s` cannot be migrated by this pipeline right now — measured

`✏️s` holds ~27 900 of the 28 716 remaining legacy stems (≈96% of the work). It is not reachable
from this session, and the reason is concurrency, not a defect.

Attempted the smallest plugin, `🎪️demonstrator` (84 legacy leaves). The plan ran ~25 minutes over
55 000 reference candidates and then failed:

```
error: Reference edit preimage mismatch at
  ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🧪️tests/🧪️mutate-playground-1/🥒️.feature
  :gherkin-description-inline-code:34:4@2687
```

The scan saw `🧪️tests/🧪️mutate-playground-1/`; by write time the directory was
`🧪️tests/mutate-playground-1/` — the concurrent session had dropped the `🧪️` prefix mid-plan.

Churn rate, measured directly:

```
plugin files modified in the previous 30 minutes: 1 327
```

**A plugin plan takes ~25 minutes; the plugins tree changes ~1 300 files in 30.** No plan can hold a
coherent view of `✏️s` while that restructuring runs. Retrying does not help — every attempt races
the same writer.

This is the same constraint as §5, at a scale that makes it decisive rather than merely annoying.
Two things would change it, in order of value:

1. **Narrow the plan's reference-candidate scope.** The `plugin-registry` generator enumerates
   40 232 repo-wide inputs, which is why a plugin-scoped plan must scan the entire tree and why every
   plan is sensitive to every edit anywhere. A genuinely scope-local plan would shrink the window
   from ~25 minutes to something that can outrun the churn.
2. **A quiet window.** The framework modules landed precisely because that layer was not being
   concurrently restructured.

Until one of those holds, `✏️s` belongs to whoever owns that restructuring — and notably, that
session is already producing kind-only leaves using the vocabulary and adapters repaired here.
