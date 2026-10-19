# Result lay0: Dependency-Cruiser Policy Fixes and Plugin Cycle Analysis

Executor `lay0`. Source audit: `audit-layering.md` sections 1, 5, 6. No cargo, no `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths. No git-modifying command.

## 1. Changes (file list)

All paths are under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

| file | change |
|---|---|
| `🔣️taxonomy.json` | schema-first authority: new `bannedNameStemExemptions.subsetDirectories: ["base"]` (+ `_bannedNameStemExemptionsComment`) beside `bannedNameStems` |
| `🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` | policy builder: `no-core-path` rewritten (exact segment semantics, subset exemption, builtin/unresolved exemption, segment-anchored `node_modules`/`target`/`pkg` exemption); new `installedPackagePath()` makes the renderer allow-list match bun-store realpaths; `bun:test` allowed; builtin list ignores host-specific `bun:*` names; `assertFocusedBoundarySemantics` gained store-path, `bun:test`, `bun:sqlite` witnesses |
| `🕸️dependencies/🧭️direction/🏗️construction/🟦️.ts` | `DependencyPolicyConstructionInputs` taxonomy type: `bannedNameStems`, `bannedNameStemExemptions`, `subsetsDirName` required (the old `|| ["core"]` fallback is gone) |
| `🔍️discovery/📑️taxonomy-document/🟦️.ts` | `Taxonomy.bannedNameStemExemptions` |
| `⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts` | `policyBannedNameStemBreaches` (filesystem law) reads the same exemption, so the two enforcers agree |
| `🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🛡️pattern-safety/🟦️.ts` | taxonomy cast type extended |
| `🕸️dependencies/🧭️direction/🚀️bootstrap/🧪️tests/🟦️.ts` | freshness test reads `to.path` as string or array |
| new `🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🟦️.ts` + `🧫️fixtures/🔣️.json` | language-agnostic corpus (35 cases + filesystem-law tree), checked against our direct regex evaluation AND dependency-cruiser's own `validateDependency`; plus a test that the complete actual policy passes dependency-cruiser's `assertRuleSetValid` (incl. its safe-regex check) |

Ticket tools (inputs, kept): `lay0-tools/{aggregate,replay,cycles,law-run}.ts`, `lay0-tools/plugin-cycles.json` (the 92 plugin cycles with edge kinds, for the follow-up executor).

## 2. Decisions

### `base` is a legitimate taxonomy name (no rename)
- `taxonomy.subsetDirectoryOverrides` declares `base -> 🧱️base` for gif 89a, svg 1.1, xml 1.0, docx ecma-376 and step ap214; the default `subsetSlugPattern` admits it everywhere else (json rfc8259, zip 2.0, pdf 1.4/1.7, ifc 2x3, pptx, xlsx, `✉️base` in semio v1). 18 standards carry a `🪆️subsets/🧱️base`. Renaming would touch hundreds of files and contradict the declared vocabulary.
- The old rule `(^|/)([^/]*?)(stems)(/|$)` matched any segment that merely ENDS with a stem (`graph-core`, `database`, `set-relationship-base`), which also contradicted the filesystem law `policyBannedNameStemBreaches` (exact emoji-stripped, extension-stripped stem). `no-core-path` now mirrors the law exactly: a segment is a breach only when its emoji-stripped, extension-stripped name equals a stem.
- `base` is exempt ONLY as the direct child directory of a `🪆️subsets` owner (`bannedNameStemExemptions.subsetDirectories` ∩ `bannedNameStems`, validated at policy build). Deeper `base` segments, `base.ts` files, `core` below `🧱️base` and every other owner still breach (tested). The filesystem law had the same blind spot: it flagged 33 `base` directories; with the shared exemption it drops from 55 to 36 breaches (`base` 33 -> 14). The remaining 14 are non-subset-child `base` names (5 `✳️any/✏️editor/🧱️base`, 4 mirrored `…/📖️pdf/🔖️1.4/🧱️base` io dirs under writer/note, `gltf/…/🌱️metabolism/🏙️base`, the energy test case `🏛️mutate-energy-model-1/🧱️base`, 3 not itemized): open naming decisions, not policy.

### Config false positives
- Node builtins `util`/`zlib` (115 edges): dependency-cruiser types them `core`; `no-core-path` has `dependencyTypesNot: ["core","unknown"]`. `unknown` also covers unresolvable specifiers such as `@dnd-kit/core` (4 edges), `bun:test`, `bun:sqlite`.
- `renderer-hosts-only-ui`: `node_modules` is a symlink into `.cache`, so react/vite/vitest resolve to `<cache>/node_modules/.bun/<name>@<ver>/node_modules/<name>/…`. `installedPackagePath(names)` matches both `node_modules/<name>/` and the store layout, independent of the store hash. `^bun:test$` allowed like vitest. The pattern avoids an optional group around a repetition because dependency-cruiser rejects such rules (`has an unsafe regular expression. Bailing out.`, my first attempt failed the CLI at load time; now guarded by the `assertRuleSetValid` test).
- `no-core-path` exemption `node_modules|target|pkg` was an unanchored substring (any path containing `target`, e.g. `🎯️targets/`, was exempt). It is now segment-anchored.
- Host dependence: under Bun, `node:module.builtinModules` contains `bun:*`; those are filtered so the policy is identical under Node and Bun.

## 3. Verification (exact commands, outcomes)

Working directory `/Users/ueli/Documents/semio`, `SEMIO_TEST_ARTIFACT_DIR` pointed into the ticket `🗑️generated` (deleted afterwards).
- `bun test ./<…>/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🟦️.ts ./<…>/🚀️bootstrap/🧪️tests/🟦️.ts` -> 23 pass, 0 fail, 177 expects (4 new tests + 19 bootstrap tests; the whole-policy `assertRuleSetValid` test takes about 50 s under load, timeout set to 600 s).
- `bun test <…>/🏗️construction/🧪️tests/🟦️.ts` -> cannot run: `Cannot find package 'safe-regex'` imported by `🛡️pattern-safety/🟦️.ts`. Pre-existing (the package is absent from the installed `node_modules`; dependency edits are frozen). The cross-plugin matrix and pattern-safety tests are therefore unverified by me; my change does not touch those rules.
- Full cruise, the exact audit command: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json --output-to <ticket>/🗑️generated/lay0-run/depcruise.json` -> exit 0, 1194 s, 19,827 cruised modules, 60,778 edges (report deleted after aggregation).
- That cruise ran one rule tweak before the last one (exempt `unknown` dependency type). The remaining delta was verified by replaying all 60,778 edges of the saved report through dependency-cruiser's `validateDependency` with the final `no-core-path` and `renderer-hosts-only-ui` rules: no-core-path 23 (not 26), renderer 212 edges (211 violations, same).
- Law re-run: `bun lay0-tools/law-run.ts` -> 36 breaches (was 55).

## 4. New totals per rule

The tree is edited concurrently, so unchanged rules drift by a few (ncpr +45).

| rule | audit (snapshot 19:17) | this run | change cause |
|---|---:|---:|---|
| no-cross-package-relative | 6957 | 7002 | drift |
| no-core-path | 1179 | **26** (**23** with the final tweak) | policy, see below |
| plugins-framework-sdk-only | 735 | 734 | drift |
| no-circular | 688 | 688 | none (os 346, actor 50, repo 35, cad 32, process 29, stdio 27, kernel 24, ui 22, ts packages 20, …) |
| no-cross-technology | 620 | 620 | none |
| ui-no-framework-packages | 285 | 287 | drift |
| renderer-hosts-only-ui | 433 | **211** | policy |
| no-state-outside-os | 112 | 112 | none |
| plugin-no-extension-or-artifact | 15 | 15 | none |
| framework-no-implementation | 1 | 1 | none |
| **total** | **11,025** | **9,696** (**9,693** final) | |

`no-core-path` edge-level attribution (1205 old-rule edges in this tree): 891 `🪆️subsets/🧱️base` (audit said 866), 177 compound or prefixed names (`graph-core`, `flow_core`-style), 115 Node builtins, 4 unresolvable specifiers, 18 genuine stems still hit, plus 5 file-name hits newly visible (`lib.js`, `util.js`, `shared.ts` style) = 23 edges. The 23 are real grab-bag names, 11 distinct targets: `plugins/space/🫀️core/documents` (3 files), `framework/modules/value/…/🔗️shared/{factory,sealed}`, `framework/modules/ui/styling/tests/suite/🧰️shared`, `products/os/modules/flow/🫀️core/bindings/**`, `products/os/testing/jcoprobe/…/preview2-shim/common.js`, `products/repo/…/caching/tests/fixtures/import-edges/b/lib.js`. Core 14, shared 7, common 4, lib 1 -> these are for the owners of those areas (flow `🫀️core` and space `🫀️core` need renames; the jco shim and fixtures need an exemption or generated-file classification).

`renderer-hosts-only-ui`: 218 store-resolved react/react-dom/vite/vitest edges and the `bun:test` imports are gone. The 211 left are real: framework-internal imports 156 (actor 64, os 55, manifest 9, repo 7, value 6, kernel 6, process 3, surface 3, other 3), 49 third-party packages that the allow-list does not admit (three 15, playwright 11, sharp 6, fast-deep-equal 4, fast-json-patch 3, typescript 3, 7 single-use packages), `katex` 1, `bun:sqlite` 2. Whether `three`/`playwright`/`sharp` belong in a renderer is an owner decision, not a config bug.

Note: the `verify` gate (`dependencyDirectionStrictRules`) selects a fixed rule-name set that does not include `no-core-path`, `no-circular` or `renderer-hosts-only-ui`, so these fixes change the audit config and the filesystem law, not what `verify` blocks.

## 5. Plugin cycles (92 now; the audit said 89, 92 in its own table)

### The premise is wrong: there is no plugin to plugin cycle
- `cross-plugin = 0`. All 92 cycles lie inside ONE plugin, and the 1,260 `no-cross-plugin-<a>-to-<b>` rules (36 plugins) plus `s-modules-no-plugins` have 0 violations. A cross-plugin shared-contract package would remove none of the 92.
- Distribution: cad 32, stdio 27, forms 11, draw 11, architect 7, energy 3, puzzle 1. (Outside plugins: `s/dev/cad` 4, `s/modules/spatial-kernel` 2.)
- Shape: 56 two-file cycles, 23 triangles, 13 larger (up to 12 files, all in cad `extensions/*/tests`). 54 cycles have every member under one hub directory; 30 involve a test file.
- Edge kinds (the cruiser counts `type-only` and dynamic imports because `tsPreCompilationDeps: true`): 21 cycles are purely static value edges (architect `schema/diff` 7, draw `scene/prepare|trace` 4, energy `sqlite/snapshot` 3, stdio pdf/gltf `snapshot` 5, stdio `native-json`, `copy`), 48 contain a dynamic `import()` edge, 45 contain a type-only edge, 71 contain at least one of the two (the runtime graph is already acyclic there).
- Cause: a split wave left the PARENT module as both the contract owner and the aggregator. Example `stdio/…/step/…/ap214/…/6cc6/generator/📜️script.ts` defines `Kernel`, `Recipe`, `call` and aggregates children with `await import("./⚙️mechanical/📜️script.ts")`; each child does `import { type Kernel, type Recipe, call } from "../📜️script.ts"`.

### Proposed design (not implemented): lowest-common-owner contract leaf
Taxonomy already states the law: "Shared code belongs at the lowest common owner" (`_cleanMechanismComment`).
1. For every hub (parent module with children that import back): move the symbols the children need (types, constants, tiny helpers like `call`) into ONE contract leaf owned by the hub, a sibling directory of the children (for example `generator/🧬️contract/🟦️.ts` or the hub's schema facet when the symbols are schema types). Parent and children import the leaf; the leaf imports neither. Direction after the change: `children -> leaf <- parent`, `parent -> children` (aggregation).
2. Type-only cycles (45): same extraction, but they are mechanical: `import type` targets move to the leaf; no behavior change.
3. Dynamic aggregation cycles (48): once the contract is in the leaf the dynamic `import()` is no longer part of a cycle; keep it (lazy registry) or replace it with a static registry list owned by the hub.
4. Test-involving cycles (30, cad `*/tests/semio-tech-cad-js-*` and `extensions/*`): tests import the module under test and the module registers/loads its tests dynamically; move the registration into the test owner, or make the module not know its tests.
5. Hubs by size, in this order: stdio generators 18 (`semio/{mesh,brep,document}/generator`, `step/…/generator`), cad `editor/engine/{artifact,runtime,actions,renderer}` 15 + extensions 8, architect `schema/diff` 7, forms `schema/{mutations,validation,value,definition,response}` 11, draw `schema/{scene,fill}` 11, energy `io/sqlite/snapshot` 3, the 5 stdio snapshot pairs, puzzle 1.
6. Gate: keep `no-circular` strict (no type-only or dynamic exemption); add a per-plugin cycle count to the audit so the fleet can watch it fall to 0. The 21 static cycles are the only ones that can fail at runtime, so do those first.

When a real cross-plugin need appears, the sanctioned shared substrate is `✏️s/🔨️modules/<name>` (modules may not import plugins, plugins may import modules) and, toward the framework, the neutral SDK `@semio-tech/framework`; no new `shared`/`common` package, which `bannedNameStems` forbids anyway.

## 6. Still open
- Run the pattern-safety/matrix test file once `safe-regex` is installed (deps owner).
- 23 genuine `no-core-path` edges (11 targets) and the 14 non-subset `base` directories (naming decisions for flow, space, value, ui styling tests, jcoprobe shim, one fixture, writer/note pdf io mirrors, editor `🧱️base`, gltf `🏙️base`, one energy test case).
- 211 renderer imports and the 92 intra-plugin cycles are code work (audit WP9, WP3 corrected as above).
- `audit-layering.md` section 5 and WP3 text should be amended: builtin, store and `base` items are resolved; WP3 is intra-plugin parent/child cycles, not plugin contracts.
