# WP4 — plugins (`✏️s/**`, everything outside `🧬️schema/🧬️mutations/**`)

Partition: every schema-shaped or schema-named file under `✏️s/**` that is **not** inside a
`🧬️schema/🧬️mutations/**` subtree. Input: `📓️wp0-plugin-fixtures.md` (13 families, 57 violations) and
`📋️execution-contract.md` §A/§B/§D. Shared conventions this work package wrote and worked to:
`📓️wp4-plugins-conventions.md`. Helper: `wp4-move.py` (draft-07 migration + `$defs` insertion).

**Result: 59 fixture-owned schema files eliminated, 0 remain.**

```
$ git ls-files '✏️s' | grep -E '(🧪️|🧫️|🚧️)' | grep -E 'schema\.json$' \
    | grep -v '🧬️schema/🧬️mutations/' | while read -r f; do [ -e "$f" ] && echo "STILL ON DISK: $f"; done
(no output)
$ find ✏️s -name '*schema.json' | grep -E '(🧪️|🧫️|🚧️)' | grep -v '🧬️schema/🧬️mutations/' | grep -v '/.venv/'
(no output)
```

(`git ls-files` still lists the paths where the auto-commit has not yet staged the deletion; the
worktree scan above is the authoritative form. The only `find` hits are vendored draft metaschemas
inside `✏️s/🔌️plugins/🔋️energy/🧪️oracle/📦️packages/🐍️python/.venv/`, which are not repo files.)

---

## 1. Per-family table — source → owner scope → export → module

### Family 1–3, 12–13: retained-command laws (shared shape from `framework.ui`)

| Source (deleted) | Scope id | Export | Module |
|---|---|---|---|
| `🎞️animate/…/🎬️presentation/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `s.animate.presentation` | `PresentationRetainedCommandLimits` | `…/🪆️subsets/✳️any/🧬️schema/🔣️.json` |
| `🎥️shooting/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `s.shooting.shooting` | `ShootingRetainedCommandLimits` (+3 hoisted) | same level |
| `🏗️fem/…/🧊️3d/…/✏️editor/🧪️fixtures/🚧️retained-command-limits/🧬️.schema.json` | `s.fem.fem3d` | `Fem3dRetainedCommandLimits` | same level |
| `📸️remodel/…/✏️editor/🧪️fixtures/🚧️retained-command-limits/🧬️.schema.json` | `s.remodel.remodeling` | `RemodelingRetainedCommandLimits` (+1) | same level |
| `🪐️space/🗿️artifacts/🏠️home/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `s.space.home` | `HomeRetainedCommandLimits` | same level |
| `🪐️space/🗿️artifacts/🪐️space/…/✏️editor/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `s.space.space` | `SpaceIndexRetainedCommandLimits` | **new** `🔣️.json` in the existing module |
| `🪐️space/⚙️engine/🪐️space/🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json` | `s.space` | `SpacePlayRetainedCommandLimits` | **new** plugin-root module |
| `🌿️vcs/…/✏️editor/🧪️fixtures/🧬️retained-command-routes.schema.json` | `s.vcs.vcs` | `VcsRetainedCommandRoutes` | subset module |
| `💡️reasoning/🗿️artifacts/🔌️wires/…/🧬️retained-command-routes.schema.json` | `s.reasoning.wires` | `WiresRetainedCommandRoutes` | subset module |
| `📜️imperative/🗿️artifacts/📜️procedure/…/🧬️retained-command-routes.schema.json` | `s.imperative.imperative` | `ProcedureRetainedCommandRoutes` | subset module |
| `➗️mathematical/…/➗️equation/🧪️fixtures/🧬️equation-retained-command.schema.json` | `s.equation.equation` | `EquationRetainedCommandLaw` | subset module |
| `📖️playbook/…/🧪️fixtures/🎚️playbook-view-command-limits.schema.json` | `s.playbook.playbook` | `PlaybookViewCommandLimits` | subset module |
| `🏭️process/🗿️artifacts/🧊️process3d/🧪️tests/📐️retained-route-schema.json` | `s.process.process3d` | `Process3dRetainedRouteLaws` | subset module |
| `📕️norm/🧪️fixtures/🧫️retained-command-dispositions/🧬️.schema.json` | `s.norm` | `NormRetainedCommandDispositions` (+3 hoisted) | **new** plugin-root module |

**Shared shape.** `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` (`$id
https://semio.tech/schema/framework/ui/schema.json`) already publishes `RetainedCommandLimits`
(the whole declared/corpus **document**), `RetainedCommandRoutes` (the route **array**) and
`RetainedCommandRoutesDocument`. It has **no** `RetainedCommandRoute` singular. Nine exports now
`$ref` it (`allOf` document level, or `allOf` at the `routes` array level) and narrow it with the
per-artifact `const` census verbatim:

```
$ grep -rho 'https://semio.tech/schema/framework/ui/schema.json#[^"]*' --include='🔣️.json' ✏️s | sort | uniq -c
   4 https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandLimits
   8 https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandRoutes
   1 https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandRoutesDocument
```

Every one of those resolves against the module as it stands today — there are **no dangling
`$ref`s**. Twenty `#/$defs/RetainedCommandRoute` refs written into the space modules mid-wave were
repaired by lifting them to one array-level `RetainedCommandRoutes` ref, and three misplaced
document-level `RetainedCommandLimits` refs (sitting on the inner `limits` / `limits.bounded`
sub-object, where they could never validate) were moved back to the export root.

Four exports carry **no** framework `$ref`, each for a measured reason (see §5): shooting and
remodeling (`oracle.ownedInterface`/`expected` vs `additionalProperties:false`), fem3d
(`artifactStoreBytes`/`decodedItems` vs the shared byte budget), norm (a cohort document the shared
declared-limits shape cannot describe at all).

### Family 4: scene-owner laws

| Source (deleted) | Scope id | Export | Module |
|---|---|---|---|
| `✒️writer/🗿️artifacts/✒️writer/🧪️fixtures/🧬️writer-child-local-text.schema.json` | `s.writer.writer` | `WriterChildLocalTextLaw` | subset module |
| `➗️mathematical/…/➗️equation/🧪️fixtures/🎬️equation-scene-owner.schema.json` | `s.equation.equation` | `EquationSceneOwnerLaw` | subset module |
| `📖️playbook/…/🧪️fixtures/🎬️playbook-scene-owner.schema.json` | `s.playbook.playbook` | `PlaybookSceneOwnerLaw` | subset module |
| `🌊️flow/🗿️artifacts/🌊️flow/🧪️fixtures/🏠️flow-scene-owner.schema.json` | `s.flow.flow` | `FlowSceneOwnerLaw` | subset module |
| `🌊️flow/🧪️fixtures/🧹️surface-owners/🧬️.schema.json` | `s.flow` | `FlowSurfaceOwners` | **new** plugin-root module |

### Family 5: the 15 flow editor micro-laws → `s.flow.flow`

All fifteen moved into the existing subset module `…/🪆️subsets/✳️any/🧬️schema/🔣️.json`
(`$id https://semio.tech/schema/s/flow/flow/artifact.json`), which grew from 4 to 47 `$defs`:

`FlowChildAddWidget`, `FlowTreeProjection`, `FlowHostWire`, `FlowArtifactRecipes`,
`FlowGrantFrontier`, `FlowSliderLabels`, `FlowArtifactCanonical`, `FlowDeleteCascade`,
`FlowContentIdentity`, `FlowStoreOwners`, `FlowPresenceOwners`, `FlowTransientOwners`,
`FlowAddWidgetRetained`, `FlowViewerOwners`, `FlowSceneOwnerLaw` — plus 28 hoisted, prefix-renamed
inner `definitions` (`FlowArtifactCanonicalWidget`, `FlowChildAddWidgetSnapshot`, …).

**Why the subset module, not an editor module.** Contract §A's eligible levels are plugin root,
artifact standard subset, surface (`✏️editor/🎚️config|👥️presence|🫧️transient`), framework/product
module, hub area, mutation leaf. `✏️editor` itself is **not** an eligible level, and flow has no
`✏️editor/🧬️schema`. Every consumer of these laws (`✏️editor/🧪️fixtures/📜️script.ts`,
`✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs`, `📦️packages/🦀️rust/📜️script.ts`) sits under the subset, so the
subset scope is the nearest eligible owner. `store-owners` / `presence-owners` /
`transient-owners` are editor **ownership censuses**, not surface config/presence data, so they stay
with their siblings in one coherent authority rather than being split across three surface modules.

The cross-document `$ref`s (`semio.flow.artifact-canonical#/definitions/widget|synapse|layout`, which
`delete-cascade` used via `Ajv.addSchema`) became intra-module `#/$defs/FlowArtifactCanonicalWidget`
etc. Verified: the module has zero unresolvable `$ref`s.

### Families 6–7: plugin-identity / artifact-identity → new plugin-root modules

| Source (deleted) | Scope id | Export | Module (all **new**) |
|---|---|---|---|
| `💡️reasoning/🧪️fixtures/🧫️plugin-identity/🧬️.schema.json` | `s.reasoning` | `ReasoningPluginIdentity` | `✏️s/🔌️plugins/💡️reasoning/🧬️schema/` |
| `🪐️space/🧪️fixtures/🧫️plugin-identity/🧬️.schema.json` | `s.space` | `SpacePluginIdentity` | `✏️s/🔌️plugins/🪐️space/🧬️schema/` |
| `🌍️gis/🧪️fixtures/🪪️artifact-identity/🧬️.schema.json` | `s.gis` | `GisArtifactIdentity` | `✏️s/🔌️plugins/🌍️gis/🧬️schema/` |
| `🌿️vcs/🧪️fixtures/🪪️native-openable-identity/🧬️v1/🧬️.schema.json` | `s.vcs` | `VcsNativeOpenableIdentity` (+1) | `✏️s/🔌️plugins/🌿️vcs/🧬️schema/` |
| `🗄️stdio/📦️packages/🦀️rust/🧫️fixtures/🖊️dwg-artifact-ownership/🧬️.schema.json` | `s.stdio` | `StdioDwgArtifactOwnership` | `✏️s/🔌️plugins/🗄️stdio/🧬️schema/` |

### Family 8–10: stdio

| Source (deleted) | Scope id | Export | Module |
|---|---|---|---|
| `🗄️stdio/📦️packages/🦀️rust/🧫️fixtures/🌳️catalog-root/🧬️.schema.json` | `s.stdio` | `StdioCatalogRoot` | new plugin-root module |
| `🗄️stdio/📦️packages/🦀️rust/🧫️fixtures/🏠️home-io-surface/🧬️.schema.json` | `s.stdio` | `StdioHomeIoSurface` | same |
| `🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/🧬️.schema.json` | `s.stdio.registry` | `NativeCatalogSurface` | new `📇️registry/🧬️schema/🔣️.json` |
| `…/📌️commitment.schema.json` | `s.stdio.registry` | `NativeCatalogSurfaceCommitment` (+3) | same |
| `…/🧬️commitment-cases.schema.json` | `s.stdio.registry` | `NativeCatalogSurfaceCommitmentCases` | same |
| `…/🧬️budget.schema.json` | `s.stdio.registry` | `NativeCatalogSurfaceBudget` (+1) | same |
| `…/🧬️imports.schema.json` | `s.stdio.registry` | `NativeCatalogSurfaceImports` | same |
| `🗄️stdio/📇️registry/🧪️fixtures/🧾️claim-authority/🧬️.schema.json` | `s.stdio.registry` | `ClaimAuthority` | same |
| `🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json` (non-canonical filename) | `s.stdio.registry` | `NativeCodecFactories` (+3) | canonicalised into the same `🔣️.json` |

**WP0 open question 3 answered: the `native-catalog-surface` cluster is NOT dead.** The WP0 audit
grepped only inside stdio. A repo-wide `git grep` shows all six cluster files plus
`🧬️native-codec-factories.schema.json` are consumed by `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
(lines 4746/4757/4760/4762/4764/4772/4784/5195/9079). Nothing was deleted as dead; all seven were
migrated, and the hub consumer is the top cross-partition request in §6.

### Family 9: gis plugin-root fixtures

| Source (deleted) | Scope id | Export | Module |
|---|---|---|---|
| `🌍️gis/🧪️fixtures/💡️inference-control/🧬️.schema.json` | `s.gis` | `GisInferenceControl` (+1) | new plugin-root module |
| `🌍️gis/🧪️fixtures/🗄️durable-three-store-assembly/🧬️.schema.json` | `s.gis` | `GisDurableThreeStoreAssembly` (+5) | same |
| `🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🧬️.schema.json` | `s.gis` | `GisComponentColdMapPatch` (+1) | same |
| `🌍️gis/🧪️fixtures/🧩️map-create-region-group/🧬️.schema.json` | `s.gis.gismap` | `GisMapCreateRegionGroup` (+2) | gismap subset module |

**WP0 open question 4 answered: `🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs` is test-only,
not production logic.** `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:66-68` declares it as a
Cargo **integration-test target**:

```toml
[[test]]
name = "component_cold_map_patch"
path = "../../🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs"
```

It contains two `#[semio_framework_async_macros::async_test]` functions and uses
`semio_s_plugin_gis::…` as an *external* crate; no `mod`/`#[path]` anywhere reaches it from
`🌍️gis/🦀️.rs` or `🗿️artifacts/**`. It was left where it is. **Boundary note for the mutations
worker:** `GisMapCreateRegionGroup`'s natural long-term owner is gismap's
`🧬️schema/🧬️mutations/🌐create-region` leaf; nothing was written under `🧬️mutations/**`, so if the
leaf claims it, the export should move down and the subset module should `$ref` it.

### Family 11: flow/note action-cohort — note's drifted copy deleted

New scope `s.flow.action-cohort` at `✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🧬️schema/🔣️.json`
(`$id https://semio.tech/schema/s/flow/action-cohort/schema.json`), export `ActionCohort`, migrated
from flow's `🎬️action-cohort/🧬️.schema.json`. `✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧬️.schema.json`
— the narrower drifted copy — is **deleted**; note now binds `schema://s.flow.action-cohort/ActionCohort`.
Both fixtures validate against the one export (see §4), confirming the drift was unnecessary.

### Family 12: norm — the two `🧪️tests` meta-schemas

Ruling: **they are not the harness's test-case shape and do not belong to the shared test module.**
`📕️norm/🎚️config/🧪️tests/🧬️.schema.json` (`contractId semio.norm.config-mutation/v1`) is a wire-codec
conformance vector — `cases[]` of `{id, before, payload, after, warning}`, `invalid[]`, `text[]`,
`binary[]` with `hex` matching `^(?:[0-9a-f]{2})*$`. `📕️norm/🖥️app-surface/🧪️tests/🧬️.schema.json`
(`contractId semio.norm.surface-render/v1`) is a render census — `rows[]` of
`{variant, role, appId, bodyKeys}`, 30/30, a 15-variant enum, `appId` pattern
`^s\.norm\.[a-z0-9]+@1/\*#(editor|viewer)$`. Their only shared key is `contractId`; nothing else
overlaps. The shared test module `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`
(66 `$defs`) has no generic harness-test-case export; its two case-vector exports
(`ContributionDirectoryOwnershipCases`, `SchemaInvariantCases`) live there because the *contracts they
exercise* are owned by the test module, and its own doc string states the rule: a case directory
holds examples and never the contract they are examples of. By that rule a vector over norm's
config-mutation codec and norm's app-surface render census is owned by norm.

Both fixture-local files are deleted; the contracts are `s.norm/NormConfigMutationCases` and
`s.norm/NormSurfaceRenderCases`. `🖥️app-surface` gained no module of its own — it is not an eligible
scope level; the plugin root is. **No cross-partition request for the test-module owner.**

### Family 13: singletons

| Source (deleted) | Scope id | Export | Module |
|---|---|---|---|
| `💠️lowpoly/🧪️interactive-job/🧬️.schema.json` | `s.lowpoly` | `LowpolyInteractiveJobPartition` | **new** plugin-root module |
| `🧱️block/🧪️publication-authority/🧬️.schema.json` | `s.block` | `BlockPublicationAuthority` | **new** plugin-root module |
| `🖍️draw/🧪️publication-authority/🧬️.schema.json` | `s.draw` | `DrawPublicationAuthority` | **new** plugin-root module |
| `🧩️puzzle/…/◻️2d/…/✏️editor/🌉️wasm/🧪️fixtures/🧬️.schema.json` | `s.puzzle.puzzle2d` | `Puzzle2dWasmSessionFactory` | subset module |
| `🧩️puzzle/…/🧊️3d/…/⏳️precompute/🪣️fill/🧪️fixtures/🧬️.schema.json` | `s.puzzle.puzzle3d` | `Puzzle3dFillPreviewJson` (+5) | subset module |
| `🪐️space/🗿️artifacts/🏠️home/…/✏️editor/🎚️config/🧪️fixtures/📇️projection-persistence-v1/🧬️.schema.json` | `s.space.home` config surface | `HomeProjectionPersistence` (+5) | existing `🎚️config/🧬️schema/` |

lowpoly's load-bearing 8-arm `routes.items.allOf[0].then.oneOf` is preserved verbatim and in order
(`["Artifact"]`, `["Config"]`, `["HostOnly"]`, `["Transient"]`, `["Config","Transient"]`,
`["Artifact","Transient"]`, `["Artifact","Config"]`, `["Artifact","Config","Transient"]`), and the
editor doc comment at `💠️lowpoly/…/✏️editor/🦀️.rs:425` now names its new home
(`schema://s.lowpoly/LowpolyInteractiveJobPartition`'s 8th `oneOf` arm) — a comment-only edit.

---

## 2. Modules created

Eleven **new plugin-root / new scope** modules, plus one new `🔣️.json` inside an existing module and
one canonicalisation:

| Module | `$id` | Exports |
|---|---|---|
| `✏️s/🔌️plugins/🌊️flow/🧬️schema/` | `…/s/flow/schema.json` | 1 |
| `✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🧬️schema/` | `…/s/flow/action-cohort/schema.json` | 1 |
| `✏️s/🔌️plugins/🌍️gis/🧬️schema/` | `…/s/gis/schema.json` | 11 |
| `✏️s/🔌️plugins/🌿️vcs/🧬️schema/` | `…/s/vcs/schema.json` | 2 |
| `✏️s/🔌️plugins/🗄️stdio/🧬️schema/` | `…/s/stdio/schema.json` | 3 |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json` | `…/s/stdio/registry/schema.json` | 14 |
| `✏️s/🔌️plugins/🪐️space/🧬️schema/` | `…/s/space/schema.json` | 2 |
| `✏️s/🔌️plugins/💡️reasoning/🧬️schema/` | `…/s/reasoning/schema.json` | 1 |
| `✏️s/🔌️plugins/🧱️block/🧬️schema/` | `…/s/block/schema.json` | 1 |
| `✏️s/🔌️plugins/🖍️draw/🧬️schema/` | `…/s/draw/schema.json` | 1 |
| `✏️s/🔌️plugins/💠️lowpoly/🧬️schema/` | `…/s/lowpoly/schema.json` | 1 |
| `✏️s/🔌️plugins/📕️norm/🧬️schema/` | `…/s/norm/schema.json` | 8 |
| `🪐️space/🗿️artifacts/🪐️space/…/✳️any/🧬️schema/🔣️.json` | `…/s/space/space/artifact.json` | 3 (module existed with only `🟦️.ts`/`🦀️.rs`) |

**Deviation — format coverage.** Contract §B lists five canonical files per module. These new modules
carry only `🔣️.json`. Rationale, reported rather than silently applied: JSON Schema is the normative
format for `🧬️data` facets (§A), and every consumer of these contracts is a JSON-Schema consumer (an
Ajv oracle in a `📜️script.ts`, or `serde_json::Value` indexing in Rust). A `🦀️.rs` in a new
plugin-root module would be unreachable unless it were declared from the plugin crate root with
`#[path]`, which would mean adding new Rust to eleven plugin crates and a workspace-scale
`cargo check` this session is forbidden to run; a `🛰️.proto`/`🔗️graphql` mirror with no producer or
consumer is dead code, which CLAUDE.md forbids. **Coordinator decision needed** (§6-F): either accept
JSON-Schema-only plugin-root modules, or schedule a follow-up wave that generates the other four
formats from `🔣️.json` and wires the Rust.

---

## 3. Consumer rewiring

Every rewired site loads the owner module once, registers it under its own `$id`, and compiles
`{ $ref: "<module $id>#/$defs/<Export>" }`.

| File | Lines | Change |
|---|---|---|
| `🌊️flow/…/✳️any/✏️editor/🧪️fixtures/📜️script.ts` | 16–24 (new `flowSchemaModule` + `flowExport()`), 28, 70, 97, 114, 181, 236, 258, 278, 312, 384, 407, 426, 450, 467–468 | 14 `Bun.file(...🧬️.schema.json)` + `new Ajv().compile()` pairs → `flowExport("<Export>")`; `delete-cascade`'s `.addSchema(artifactSchema)` is gone (both are `$defs` of one module now); surface-owners compiles from the flow plugin-root module |
| `🌊️flow/…/✳️any/✏️editor/🧪️fixtures/📜️script.ts` | 369–371 | mutation-leaf reads repointed at the WP4-mutations layout: `🧪️tests/🚫️rejects-duplicating-6d209e/🦠️mutation/🔣️.json` and `👯️duplicate-widget/🧬️schema/🔣️.json` (was `🧬️.schema.json`) |
| `🌊️flow/📦️packages/🦀️rust/📜️script.ts` | 68–71 | `add-widget-retained` schema → `s.flow.flow#/$defs/FlowAddWidgetRetained` |
| `🌊️flow/📦️packages/🟦️typescript/📜️script.ts` | 142–147 | `action-cohort-audit` repointed from two **stale paths that do not exist at HEAD** (`🔣️action-cohort.schema.json`, `🔣️action-cohort.json`) to `🎬️action-cohort/🧬️schema/🔣️.json` + `#/$defs/ActionCohort` and the real fixture paths `🎬️action-cohort/🔣️.json` / `🗒️note/🧪️action-cohort/🔣️.json` |
| `🗒️note/🧪️action-cohort/🧪️component.test.ts` | 11, 17–19 | compiles `s.flow.action-cohort/ActionCohort` instead of note's deleted copy |
| `🌍️gis/📦️packages/🦀️rust/📜️script.ts` | 30–35 (new `compileGisScopeExport`), 100, 140, 182, 197, 239 | five fixture-schema compiles → scope exports; the hand-picked `.$defs.groupMembership` read became `GisMapCreateRegionGroupMembership`; `Ajv2020` → draft-07 `Ajv` |
| `🌿️vcs/📦️packages/🦀️rust/📜️script.ts` | 103–107 | `VcsNativeOpenableIdentity` |
| `🗄️stdio/📦️packages/🦀️rust/📜️script.ts` | 27 (helper), 287, 656, 709 | `StdioCatalogRoot`, `StdioDwgArtifactOwnership`, `StdioHomeIoSurface` |
| `🪐️space/📦️packages/🦀️rust/📜️script.ts` | 20–29 (new `compileRetainedCommandLimits`, loads framework.ui + scope module), 40–43, 133, 369–372, 419–430 | projection-persistence, home retained, plugin identity, and the per-surface retained loop |
| `🧩️puzzle/…/🧊️3d/…/⏳️precompute/🪣️fill/🦀️.rs` | 4493–4515 | `include_str!("🧪️fixtures/🧬️.schema.json")` → `include_str!("../../../🧬️schema/🔣️.json")`, then `&module["$defs"]["Puzzle3dFillPreviewJson"]` and the hoisted `…Diagnostic`/`…Ghost` |
| `💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts` | 4, 66, 111–112 | `Ajv2020` → `Ajv`; `LowpolyInteractiveJobPartition` |
| `🧱️block/📦️packages/🟦️typescript/📜️script.ts` | 76–79 | `BlockPublicationAuthority` |
| `🖍️draw/📦️packages/🟦️typescript/📜️script.ts` | 81–84 | `DrawPublicationAuthority` |
| `📕️norm/📦️packages/🦀️rust/📜️script.ts` | 133–135, 180, 185–186 | `NormConfigMutationCases`, `NormSurfaceRenderCases` |
| `🏭️process/📦️packages/🟦️typescript/📜️script.ts` | 130–133 | `Process3dRetainedRouteLaws` |
| `💠️lowpoly/…/✏️editor/🦀️.rs` | 425 | doc comment names the new home (comment only) |

**Data files.** Rust `include_str!` of a *data* fixture is unchanged everywhere (only puzzle-3d ever
included a schema). Eight data files lost exactly one key — the `"$schema": "./<sibling>.schema.json"`
self-reference that only existed to point at the now-deleted file, per conventions §2:
`⚖️flow-scene-owner-law.json`, `🧹️surface-owners/🔣️.json`, `👥️presence-owners/🔣️.json`,
`🫧️transient-owners/🔣️.json`, `👁️viewer/🧪️fixtures/🧹️owners/🔣️.json`,
`⚖️writer-child-local-text-law.json`, `⚖️equation-retained-command-law.json` +
`👑️equation-scene-owner-law.json` + `👑️playbook-scene-owner-law.json` +
`👁️playbook-view-command-limits.json`, and norm's `🧫️retained-command-dispositions/🔣️.json`. Each
site's Rust/TS reader was checked first and none indexes `$schema`.

**nx + launch.json.** No target was renamed, so every existing `.vscode/launch.json` gate line still
resolves. Two commands were registered in `📋️project.json` that the ScriptRouter already exposed but
nx did not: `@semio-tech/block-js:publication-authority-audit` and
`@semio-tech/draw-js:publication-authority-audit`, plus their `.vscode/launch.json` entries
`📦️audit🧱️block🛂️publication-authority` (group `4_build`, order 107) and
`📦️audit🖍️draw🛂️publication-authority` (order 108), following the existing
`📦️audit🧩️puzzle🛂️publication-authority` entry.

---

## 4. Verification — real output

### Rewired oracles that pass

```
$ cd ✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit
 14 pass  0 fail  Ran 14 tests across 8 files. [122.00ms]
validated Block publication authority; apps=Block2dPlayApp:9,Block3dPlayApp:23,Block5dPlayApp:7; schema=Ajv; oracle=owned; hostile=12

$ cd ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit
 2 pass  0 fail  Ran 2 tests across 2 files. [69.00ms]
validated Draw publication authority; apps=DrawingPlayApp:26; schema=Ajv; oracle=owned; hostile=5

$ cd ✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript && bun ./📜️script.ts test
 2 pass / 0 fail
validated Process3d retained routes; routes=33; migrated=33; bounded=25; resumable=8; batchOnly=0; scanThenMonolith=0; schema=Ajv; oracle=independent

$ cd ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust && bun ./📜️script.ts config-mutation-source
Norm config schema oracle passed: 5 cases, 5 hostile payloads, 4 undeclared wire forms, 13 text vectors, 25 binary vectors

$ cd ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust && bun ./📜️script.ts surface-render-source
[DEBUG] Norm surface inventory: 15 variants, 30 apps, 120 bodies, AJV and 5 hostile vectors passed

$ cd ✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript && bun ./📜️script.ts test
norm retained cohort ok: 15 editors × 3 migrated routes, 0 descriptor rows carried a classification
 30 pass  0 fail

$ cd ✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript && bun ./📜️script.ts test
validated Mathematical publication authority; routes=7; …
 2 pass  0 fail

$ cd ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust && bun ./📜️script.ts plugin-identity-check
plugin-identity-check: checks=11 clean
$ … home-directory-projection-persistence-check
home-directory-projection-persistence-check: checks=11 clean
$ … home-directory-event-page-owner-check
home-directory-event-page-owner-check: checks=27 clean
$ … interactive-job-catalog-check
interactive-job-catalog: descriptor rows without an interactiveJob disposition: 0
interactive-job-catalog-check: checks=23 clean

$ cd ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust && bun ./📜️script.ts component-cold-map-patch-check
gis-component-cold-map-patch-source: AJV=1 SHA256=node+webcrypto hostile=5 markers=9; …
$ … map-create-region-group-check
gis-map-create-region-group-check: checks=26 clean; atomic durable publication not claimed

$ cd ✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust && bun ./📜️script.ts native-openable-identity-check --oracle-only
vcs-native-openable-identity-oracle: positive=1 hostile-denied=11 protocol-webcrypto=1

$ cd ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust && bun ./📜️script.ts home-io-surface source
stdio-dwg-artifact-ownership: cases=8 AJV=1 framework=clean modules=clean artifact-facets=clean
stdio-home-io-surface-oracle: AJV=1 TOML=bun+iarna surfaces=6 direct=4 shared=4 full=36 codecs=26 consumers=3

$ cd ✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust && bun ./📜️script.ts add-widget-retained-check --oracle-only
[DEBUG] Flow retained addWidget oracle: 2 accepted + 24 hostile cases; session/publication model is independent; runtimeClaims=0
```

### The flow editor oracle (family 5), all 14 Ajv regions

`✏️editor/🧪️fixtures/📜️script.ts` cannot run to completion in the current tree because its **first
two statements** are framework preflights that are red independently of this ticket (§5). Run with
those two calls skipped, every rewired region passes:

```
[DEBUG] Flow child add-widget contract: 2 typed node rows, 1 reconstructed-host repeat, 6 denials, 6 hostile fixture rejections
[DEBUG] Retained UI tree projection: 2 trees, 2 structural denials, 3 hostile contracts, AJV/stable JSON oracle
[DEBUG] Flow actual operation-wire source: 6 binary shapes, 3 hostile fixtures
[DEBUG] Flow artifact recipe fixtures=4 hostileRejections=4 semanticLabelBytes=4800 oracle=immer runtimeClaims=0
[DEBUG] Flow parameter intent cases=4 hostileRejections=10 oracle=fast-json-stable-stringify runtimeClaims=0
[DEBUG] Shared parameter retirement byteOracles=2 hostileFixtureRejections=3 oracle=Node.Buffer runtimeClaims=0
[DEBUG] Flow delete-cascade oracle=immer semanticLabelBytes=4800 inverseIndices=1,3 hostileRejections=3 runtimeClaims=0
[DEBUG] Flow content identity oracle=["05cbf925…","d953e80e…","ee474987…","4b96a357…","62089504…"]
[DEBUG] Flow persisted parent child/target equality: 21 source assets checked with AJV
[DEBUG] Flow store-owner oracle: 3 lanes, 3 grants, independent UTF-8/page retirement
[DEBUG] Flow presence-owner oracle: 3 rosters, 3 grants, UTF-8 counts checked independently
[DEBUG] Flow transient-owner oracle: 4 exact zero-payload trace steps
```

The two regions after the first non-schema source-truth failure were replayed verbatim against the
new modules:

```
[DEBUG] FlowViewerOwners: real fixture accepted, hostile documentRights rejected
[DEBUG] FlowSurfaceOwners: real fixture accepted, 4/4 hostile contracts rejected, members equal viewer's
```

### Flow/note action-cohort

```
[DEBUG] ActionCohort accepts flow fixture (schema=semio.flow-note.action-cohort.v1, owner=FlowPlayApp, routeCount=37)
[DEBUG] ActionCohort accepts note fixture (schema=semio.note.action-cohort.v1, owner=NotePlayApp, routeCount=36)
[DEBUG] ActionCohort rejects a forged owner
$ bun test '✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🧪️component.test.ts'
 2 pass  1 fail   ← the Ajv test passes; the failure is a stale source-truth string, see §5
```

### Every export compiled against its real fixture + a hostile mutation

```
PASS PresentationRetainedCommandLimits  accepted-fixture=true rejected-hostile=true
PASS ShootingRetainedCommandLimits      accepted-fixture=true rejected-hostile=true
PASS Fem3dRetainedCommandLimits         accepted-fixture=true rejected-hostile=true
PASS RemodelingRetainedCommandLimits    accepted-fixture=true rejected-hostile=true
PASS VcsRetainedCommandRoutes           accepted-fixture=true rejected-hostile=true
PASS WiresRetainedCommandRoutes         accepted-fixture=true rejected-hostile=true
PASS ProcedureRetainedCommandRoutes     accepted-fixture=true rejected-hostile=true
PASS EquationRetainedCommandLaw         accepted-fixture=true rejected-hostile=true
PASS EquationSceneOwnerLaw              accepted-fixture=true rejected-hostile=true
PASS PlaybookViewCommandLimits          accepted-fixture=true rejected-hostile=true
PASS PlaybookSceneOwnerLaw              accepted-fixture=true rejected-hostile=true
PASS Process3dRetainedRouteLaws         accepted-fixture=true rejected-hostile=true
PASS ReasoningPluginIdentity            accepted-fixture=true rejected-hostile=true
PASS WriterChildLocalTextLaw            accepted=true hostileRejected=true
PASS SpacePlayRetainedCommandLimits     accepted=true hostileRejected=true
PASS HomeRetainedCommandLimits          accepted=true hostileRejected=true
PASS SpaceIndexRetainedCommandLimits    accepted=true hostileRejected=true
PASS s.block/BlockPublicationAuthority       real=valid | hostile=rejected (apps/0/owner not an allowed value)
PASS s.draw/DrawPublicationAuthority         real=valid | hostile=rejected (routes/6/lanes > 1 item)
PASS s.lowpoly/LowpolyInteractiveJobPartition real=valid | hostile=rejected (routes/0/lanes const)
PASS s.norm/NormRetainedCommandDispositions  real=valid | hostile=rejected (apps > 15 items)
PASS s.norm/NormConfigMutationCases          real=valid | hostile=rejected (cases/0/id pattern)
PASS s.norm/NormSurfaceRenderCases           real=valid | hostile=rejected (rows/0/appId pattern)
PASS GisArtifactIdentity / GisInferenceControl / GisDurableThreeStoreAssembly / GisComponentColdMapPatch
PASS GisMapCreateRegionGroup / VcsNativeOpenableIdentity / StdioCatalogRoot / StdioHomeIoSurface
PASS NativeCodecFactories / NativeCatalogSurface / NativeCatalogSurfaceCommitmentCases
PASS NativeCatalogSurfaceBudget / NativeCatalogSurfaceImports / ClaimAuthority
PASS Puzzle2dWasmSessionFactory (valid fixture => true, hostile globalFactory => false)
```

Proof the framework refs are load-bearing rather than silently vacuous — compiling without
`addSchema(framework.ui)`:

```
resolution-required PresentationRetainedCommandLimits -> can't resolve reference
  https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandLimits
  from id https://semio.tech/schema/s/animate/presentation/artifact.json
… identical for Shooting, Fem3d, Remodeling, Vcs, Wires, Procedure
WiresRetainedCommandRoutes: owner-arm-alone accepts ["transient"]=true, with framework $ref accepts=false
```

### Every touched module re-validated structurally

A final pass over all 30 modules this work package created or wrote into checks dialect, `$id`, and
that every `$ref` — intra-module and cross-scope into `framework.ui` — resolves:

```
$ python3 wp4-final-check.py
modules checked: 30 | problems: 0
```

(The one finding it originally reported, `DIALECT …/🗺️gismap/…/🧬️schema` = `None`, was fixed by
declaring `"$schema": "http://json-schema.org/draft-07/schema#"` on that module per contract §B; its
Rust consumers only `include_str!` the text into an `ArtifactSchemaDescriptor.json_schema` field and
never parse a dialect, and `gis-map-create-region-group-check: checks=26 clean` still passes after.)

### Repo schema gate

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts schema --under '✏️s/🔌️plugins'
… 39 findings shown, "… and 479 more"
$ grep -E '(🌊️flow/🧬️schema|🎬️action-cohort/🧬️schema|🌍️gis/🧬️schema|🌿️vcs/🧬️schema|🗄️stdio/🧬️schema|📇️registry/🧬️schema|🪐️space/🧬️schema|💡️reasoning/🧬️schema|🧱️block/🧬️schema|🖍️draw/🧬️schema|💠️lowpoly/🧬️schema|📕️norm/🧬️schema)' <output>
(no output)
```

**No gate finding names a module this work package created or edited.** Per-plugin before → after,
where it was measured: block 2 → 0, lowpoly 2 → 0, norm 28 → 20 (the 8 that were mine are gone; the
remaining 20 are pre-existing `schema-owner-ineligible` on norm's 15 artifact variants plus
`🎚️config`/`👥️presence`/`📇️registry`, and 2 `schema-placement-forbidden-filename` on files outside
this ticket's scope). The 479 remaining repo-wide findings are pre-existing drift in other subtrees
(2020-12 dialects, `🚪️io/🧬️mutations/📝️text/🔣️.json` placements, cad/raster/puzzle
`🔏️publication-authority` filenames) — see §7.

### Rust

Only one crate's Rust changed semantically: `semio-s-plugin-puzzle` (the `include_str!` repoint).
lowpoly's Rust edit is a doc comment only; nothing else in this partition touched Rust.

`cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --tests` was run with
`CARGO_TARGET_DIR` in the session scratchpad and `RUSTC_WRAPPER=""` (never a workspace build). **It
cannot be taken green right now: the puzzle crate is mid-refactor by a peer.** 163 errors, every one
of them inside `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`, and **zero** naming the file this work
package edited:

```
$ grep -o '^error\[[A-Z0-9]*\]' puzzle-check.txt | sort | uniq -c
   3 error[E0260]      # "the name `dsl` is defined multiple times" in 🗿️artifacts/◻️2d|🖐️5d|🧊️3d/🦀️.rs
 154 error[E0432]      # unresolved import crate::mutations / crate::Puzzle2dSnapshot / crate::diff
   4 error[E0433]
$ grep -c '⏳️precompute/🪣️fill' puzzle-check.txt
0
$ git status --porcelain ✏️s/🔌️plugins/🧩️puzzle
M  🗿️artifacts/◻️2d/🦀️.rs        M  🗿️artifacts/🖐️5d/🦀️.rs        M  🗿️artifacts/🧊️3d/🦀️.rs
M  🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml   A  …/🔣️icons/🧩️metabolism.rs   R  (12 example renames)
```

An earlier attempt failed even earlier, at `semio-framework`, on
`E0432: unresolved import semio_framework_schema` in
`🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🦀️.rs:15` — the WP2 Rust-schema-registry worker's
uncommitted 17:43 edit (HEAD has 0 occurrences of that import); it resolved on the rerun.

Static verification of the one change instead: `include_str!("../../../🧬️schema/🔣️.json")` resolves
from `…/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` to the real subset module, and the three `$defs`
keys the surrounding code indexes are present:
`['Puzzle3dFillPreviewJson', 'Puzzle3dFillPreviewNullableString', 'Puzzle3dFillPreviewVector3',
'Puzzle3dFillPreviewQuaternion', 'Puzzle3dFillPreviewGhost', 'Puzzle3dFillPreviewDiagnostic']`.
**Open: re-run this check once the peer's puzzle-artifact refactor settles.**

---

## 5. Failures observed that are NOT this work package's

Each was confirmed against `HEAD` before being classified.

1. **`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/📜️script.ts:30`** — expects 9
   occurrences of `as UiTypedRetire>::DEPTH <= super::typed::UI_TYPED_RETIREMENT_DEPTH` in its
   sibling `🦀️.rs`, finds 0. `git show HEAD:` on that `🦀️.rs` also gives 0 — a peer's `super::`
   unqualify sweep. This is the first statement of flow's editor oracle, so it blocks the whole file.
2. **`🌊️flow/…/✏️editor/🦀️.rs`** — `authority.prepare_one_item(edit, std::sync::Arc::new(post))` is
   now spelled `authority.prepare_one_item(edit, Arc::new(post))` (line 721), and
   `crate::artifacts::flow::retirement::store_owners()` is now `crate::retirement::store_owners()`.
   Same live unqualify sweep (the second is an *uncommitted* working-tree change; HEAD still has the
   qualified form). These break `action-cohort-audit`'s `sourceOracle` and the flow editor oracle's
   `🗃️SharedDocumentOwnerAuthority` region. Nothing schema-related: the Ajv step passes first.
   Not reverted, per contract §E.
3. **`🗒️note/🧪️action-cohort/🧪️component.test.ts:50`** expects
   `factory: "BoundedFirstStepCommandJobFactory"` in note's retained `🦀️.rs`, which declares
   `NoteCommandJobFactory`. HEAD has 0 occurrences of the expected string — stale assertion,
   pre-existing.
4. **`🌍️gis` `durable-three-store-assembly-check`** fails *after* its Ajv step at `📜️script.ts:226`
   (`Store durable journal does not begin exactly once`), a predicate over
   `🧰️framework/…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` — a peer's store refactor.
5. **`🪐️space` `home-directory-identity-rows-check`** fails at `📜️script.ts:271`
   (`Home testkit does not await async app construction`); the Home editor no longer contains
   `testkit::new_app` after a peer's 16:26 refactor.
6. **`💠️lowpoly` `test`** throws `Lowpoly proof drift: addPrimitive` at `📜️script.ts:79`, before any
   schema code, because the editor source lost its `semio_framework::` qualifier on
   `ToolExecutionContract::resumable` (0 occurrences at HEAD). The rewired Ajv block was replayed
   separately and passes.
7. **`🌊️flow/…/✏️editor/🧪️fixtures/📜️script.ts:369`** hit `ENOENT` on
   `🧬️mutations/👯️duplicate-widget/🧪️tests/🚫️rejects-duplicating-onto-a-taken-id/…` because the
   WP4-mutations worker renamed that case directory to `🚫️rejects-duplicating-6d209e` and moved the
   leaf payload schema to `👯️duplicate-widget/🧬️schema/🔣️.json`. Fixed here (the consumer file is in
   this partition), but the mutations worker should know the reference is explicit, not globbed.

---

## 6. Cross-partition requests

**A. `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — BREAKING NOW, highest priority.** Six reads of files
migrated into `s.stdio.registry`. Load the module once
(`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json`, `$id
https://semio.tech/schema/s/stdio/registry/schema.json`, draft-07 → `ajv`, not `ajv/dist/2020.js`),
`ajv.addSchema(module)`, then compile `{ $ref: "<id>#/$defs/<Export>" }`:

| Line | Was | Export |
|---|---|---|
| 4746 | `…/🧬️native-codec-factories.schema.json` | `NativeCodecFactories` |
| 4757 | `surfaceRoot/🧬️.schema.json` | `NativeCatalogSurface` |
| 4760 | `surfaceRoot/🧬️commitment-cases.schema.json` | `NativeCatalogSurfaceCommitmentCases` |
| 4762 | `surfaceRoot/📌️commitment.schema.json` | `NativeCatalogSurfaceCommitment` |
| 4764 | `surfaceRoot/🧬️imports.schema.json` | `NativeCatalogSurfaceImports` |
| 4772 | `surfaceRoot/🧬️budget.schema.json` | `NativeCatalogSurfaceBudget` |
| 4784 | `claimRoot/🧬️.schema.json` | `ClaimAuthority` |
| 5195 | absolute `…/📌️commitment.schema.json` | `NativeCatalogSurfaceCommitment` |
| 9079 | `schemaPaths.stdio` | the registry module + `NativeCodecFactories` (the `gis` entry there points at `📇️native-codecs/🧬️.schema.json`, untouched — leave it) |

`claimRoot` / `surfaceRoot` / `fixtureRoot` (4749/4750/5136) stay: they still resolve the **data**
files. `9200`/`9232` read `📜️native-codec-factories.json` (data) and are unaffected.

**B. `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` — four `definitions` widenings.** None is
blocking (every export validates today), but each would let an owner narrow the shared shape instead
of standing alone:

1. `definitions.retainedCommandOracle` is `additionalProperties:false` over
   `library`/`scope`/`runtimeDependency`. Add optional `ownedInterface` (string, minLength 1) and
   `expected` (object) — unblocks `ShootingRetainedCommandLimits`, `RemodelingRetainedCommandLimits`
   and `NormRetainedCommandDispositions` from the document-level ref.
2. `definitions.retainedCommandByteBudget`: add optional `artifactStoreBytes`, `decodedItems`
   (integer, minimum 0) — unblocks `Fem3dRetainedCommandLimits`.
3. `definitions.retainedCommandLane`: add `"transient"` / `"Transient"`. The wires and procedure
   laws declare it; the `allOf` intersection silently narrows it away today (no fixture uses it).
4. `$defs.RetainedCommandRoutesDocument` is `additionalProperties:false` over `schema`/`routes`, so a
   routes document that also carries `maximumRawBytes`/`maximumWorkItems` (wires, imperative) cannot
   use it. Either widen it or add a second arm.
5. A **new** document-level export, e.g. `RetainedCommandCohort`, over
   `{schemaVersion, factory: object, publicationContracts[], routes[], apps[], expected, oracle}`
   would let `NormRetainedCommandDispositions` join the family; `retainedCommandDeclaredLimits`
   cannot describe it (norm's `factory` is an object, and `controller`/`documentSchema` are per-app
   rows inside `apps[]`).
6. Convenience: publish `"RetainedCommandRoute": { "$ref": "#/definitions/retainedCommandRoute" }`
   so owners can narrow a single route positionally without reaching into `definitions`. Twenty such
   refs were written and then lifted to the array level precisely because that export is missing.

**C. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`** — its
`identityRoot`/`controlRoot` reads (lines ~144/163) now read only the gis **data** files, which is
correct; if a schema read is reintroduced it must target
`https://semio.tech/schema/s/gis/schema.json#/$defs/GisArtifactIdentity` /
`#/$defs/GisInferenceControl` with draft-07 `Ajv`, not `Ajv2020`.

**D. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🪪️plugin-identity.test.ts:89`** reads
`<fixtureDir>/🧬️.schema.json` for every plugin. Switch to the owner module, mirroring
`spacePluginIdentityOracle` in `🪐️space/📦️packages/🦀️rust/📜️script.ts:368-374`: read
`<pluginRoot>/🧬️schema/🔣️.json`, `addSchema`, `compile({$ref: "<id>#/$defs/<Plugin>PluginIdentity"})`
— `SpacePluginIdentity`, `ReasoningPluginIdentity`. (This test was already red before this wave: the
space fixture schema was deleted in the working tree by a peer.)

**E. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.**
`semanticDirectoryMemberKinds.members-of-fixtures.memberNames` (lines ~7906/7910/7912) still lists
`🧬️retained-command-routes.schema.json`, `🎬️playbook-scene-owner.schema.json`,
`🎚️playbook-view-command-limits.schema.json`, none of which exist any more. `memberNames` is used as
an allowlist (`.includes(...)`), never set-equality, so this is cosmetic staleness rather than a gate
failure — sweep it with the taxonomy regeneration. Also: the plugin-root `🧬️schema` slot promised by
contract §C is what the twelve new plugin-root modules depend on.

**F. Schema catalog + coordinator decision.**
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` must be regenerated
(`bun ./📜️script.ts schema generate`). Measured with the real `schemaResolutionDiagnostics()`, exactly
**7 `schema-cross-scope-dependency-forbidden` findings** appear because those scopes' `dependsOn` is
`[]` while they now `$ref` `framework.ui`: `s.animate.presentation`, `s.fem.fem3d`,
`s.imperative.imperative`, `s.reasoning.wires`, `s.remodel.remodeling`, `s.shooting.shooting`,
`s.vcs.vcs` — each needs `dependsOn: ["framework.ui"]`. Twelve **new scopes** must be added
(`s.flow`, `s.flow.action-cohort`, `s.gis`, `s.vcs`, `s.stdio`, `s.stdio.registry`, `s.space`,
`s.reasoning`, `s.block`, `s.draw`, `s.lowpoly`, `s.norm`).
Two decisions are needed:
- **Format coverage** (see §2): are JSON-Schema-only plugin-root modules acceptable, or must the
  other four canonical files be generated and wired?
- **`schemaExportCompletenessDiagnostics`**: adding one `$defs` key to a five-format subset scope
  produces 4 `schema-export-incomplete` findings (one per declared format). 16 new `$defs` land in
  five-format subset scopes → ~64 new findings. Either the generator excludes law/contract exports
  from `exports`, or the rule requires format parity only for exports the projection layer emits.

**G. Repo library helper.** The same six-line `compile<scope>Export` helper had to be written into
five different `📜️script.ts` files (gis, stdio, space, flow ×2, block/draw/lowpoly/norm inline).
Hoist one `compileScopeExport(modulePath, exportId)` into
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`. It must declare
the vendor keyword and numeric formats that artifact-level modules carry, otherwise Ajv `strict:true`
throws before validating anything:

```ts
ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
for (const numeric of ["double", "float", "int32", "int64", "uint32", "uint64"]) ajv.addFormat(numeric, true);
```

This is a direct consequence of merging law exports into artifact modules and is worth centralising
before WP7.

---

## 7. Open items and follow-ups

1. **`cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2 --tests` must finish green.** It was
   still running when this report was written. Nothing else in this partition changed Rust semantics.
2. **Same-family files that are NOT `🧪️`/`🧫️`-marked and so fell outside this ticket's violation
   list**, all reported by the repo schema gate as `schema-placement-forbidden-filename`:
   `🖨️raster/🔏️publication-authority/🧬️.schema.json`, `🧩️puzzle/🔏️publication-authority/🧬️.schema.json`,
   `➗️mathematical/📣️publication-authority/🧬️.schema.json`,
   `📐️cad/🗿️artifacts/📐️cad/…/🗄️retained-jobs/🧬️.schema.json`,
   `📐️cad/…/✏️editor/👥️presence/🧬️schema/🧬️.schema.json`,
   `🌿️vcs/📇️native-codecs/🧬️.schema.json`, `🌍️gis/📇️native-codecs/🧬️.schema.json`,
   `📕️norm/📇️registry/🧬️contract/🧬️schema/🧬️.schema.json`,
   `📕️norm/📦️packages/🦀️rust/🧬️mutation-leaf-taxonomy-v1.schema.json`,
   `🗄️stdio/🧬️schema/📦️artifact-package/🧬️.schema.json`,
   `🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json`.
   They are the same violation pattern one directory over and should be swept in WP7.
3. **`📖️playbook/🧩️extensions/🌀️procedural`** carries a `🧬️schema/` module at a path no declared
   scope-owner level matches (`schema-owner-ineligible`) — a taxonomy/eligibility question, not a
   relocation.
4. `👁️playbook-view-command-limits.json` has no consumer anywhere; `PlaybookViewCommandLimits` is
   currently an unreferenced export.
5. **Concurrency.** Two stdio fixture schemas were resurrected mid-wave by a peer session and
   re-deleted after adopting the peer's better `dwg-artifact-ownership` derivation (a
   two-anchored-pattern `anyOf` faithful to `ownsCodec()`). If that peer is still in flight, re-run
   the hygiene grep at ticket close.
