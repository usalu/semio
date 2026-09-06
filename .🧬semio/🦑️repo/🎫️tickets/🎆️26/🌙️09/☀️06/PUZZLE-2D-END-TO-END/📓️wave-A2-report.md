# Wave A2 — schema + hygiene (2026-09-06)

All five tasks landed and were verified by running the real gates. No cargo was run (main session owns it).

## 1. `🧬️mutations/🔣️.json` — real discriminated union

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`

| | before | after |
|---|---|---|
| shape | byte-copy of `📸️snapshot/🔣️.json` (`{schema, camera, nodes, edges, meta}`) with only the `title` changed | `oneOf` of 26 `$ref`s in `Puzzle2dMutation` declaration order |
| `$schema` | absent | `https://json-schema.org/draft/2020-12/schema` (architect `ProgramMutation` pattern) |
| `$defs` | 13 snapshot payload types | 26 mutation branches + 13 shared payload types |
| discriminator | none | `"mutation": {"const": "<camelCase>"}` on every branch, `additionalProperties: false` |

Handcrafted (no codegen). Each branch mirrors that mutation's leaf `<slug>/🧬️.schema.json` field-for-field, with three
deliberate corrections where the leaf disagrees with the committed wire format (see §1.2). Shared nested types
(`Puzzle2dNode`, `Puzzle2dHandle`, `Puzzle2dKindCatalogs`, the four catalog kinds, …) are `$ref`s into `$defs` rather than
re-inlined per branch, matching the sibling snapshot aggregate's own style.

### 1.1 Validation — third-party `jsonschema`, all green

Venv + script live under `🗑️generated/a2/` (re-runnable):

```
uv venv   ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated/a2/venv"
VIRTUAL_ENV=…/🗑️generated/a2/venv uv pip install jsonschema      # jsonschema 4.26.0
…/🗑️generated/a2/venv/bin/python …/🗑️generated/a2/validate_mutation_schema.py
```

Last run (after wave A1's new vectors landed):

```
1. union is a valid draft 2020-12 schema: OK
2./3. 75 committed mutation payloads across 26 leaves: OK      ← every payload matches EXACTLY ONE oneOf branch
4. 5 hostile discriminator/shape inputs rejected: OK           ← wrong / unknown / kebab-case / missing discriminator, stray property
5. 26 oneOf branches vs 26 mutation leaves: OK
ALL CHECKS PASSED
```

An earlier run at 26 payloads also passed; the 75-payload run is the live cross-check against A1's real-world and
`🚫️`-rejected vectors, which all validate too.

### 1.2 Drift the union had to correct — **for wave A1** (leaf `🧬️.schema.json` files are A1's region)

Measured with `jsonschema` Draft7 against the leaves themselves: **75 of 75 committed `🦠️mutation/🔣️.json` payloads FAIL
their own leaf schema.** Three independent causes:

1. **No `mutation` discriminator.** Every leaf has `additionalProperties: false` and no `mutation` property, so every
   payload is rejected with `Additional properties are not allowed ('mutation' was unexpected)`. Affects 24 leaves.
2. **Anchor enum casing.** `⚓change-node-anchor` and `🌱create-node` declare `enum: ["Fixed","Derived"]`; the payloads,
   `📸️snapshot/🔣️.json`, `📸️snapshot/🟦️.ts` and the DSL all use `"fixed"`/`"derived"`. The union uses lowercase.
3. **`isAbstract` vs `abstract`.** `📚replace-kind-catalogs` declares `isAbstract`; `◻️2d/🦀️.rs:346-348` renames the field
   to `abstract` on the wire (`#[value(default, rename = "abstract")]`, asserted at `◻️2d/🦀️.rs:728`) and both the
   snapshot JSON Schema and TS mirror use `abstract`. The union uses `abstract`.

Plus a fourth, softer one: leaves type optional fields as non-nullable (`"newRadius": {"type":"number"}`) while payloads
write explicit `null` and `🧬️mutations/🟦️.ts` types them `T | null`. The union types every `Option<T>` field as
`{"type": ["…","null"]}` **and** leaves it out of `required`, so both the explicit-null carrier and the omitting
DSL/pack carriers validate.

### 1.3 Stale prose still claiming the aggregate is snapshot-shaped — NOT in my region

Three places still say `🧬️mutations/🔣️.json` "is not a mutation schema at all … a copy of the SNAPSHOT schema". All were
true this morning and are now false:

- `…/✳️any/🔮️oracle/🔣️.json` → `_comment` (line 4) and `oracles[0].rationale` ("A FINDING THE REFERENCE MADE WHILE BEING
  WRITTEN: …") — **wave C1's region**.
- `…/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py:19` — **wave A1's region**.
- `…/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🥒️.feature` (~lines 29-31, same verbatim paragraph) — **wave A1's region**.

Consumers of the aggregate: only `…/✳️any/🧬️schema/🦀️.rs:156` `include_str!`s it into
`puzzle2d_artifact_schema_descriptor().mutations.json_schema` as an opaque `&'static str` — no parsing, no shape gate,
so the change is compile-safe. Nothing else in the repo reads the file.

Not touched (out of region, same bug): `🧊️3d` and `🖐️5d` `🧬️mutations/🔣️.json` are still snapshot-shaped copies
(`Puzzle3dMutation` / `Puzzle5dMutation`, `oneOf` length 0).

## 2. Artifact-root manifest hygiene

### 2.1 `◻️2d/🔣️.json` — DELETED

Provenance settled with `git log --follow`: it was `🛂️manifest.jsonconcrete-forest.manifest.json` →
(2026-08-13) `🌲️manifest.jsonconcrete-forest.manifest.json` → (2026-09-02, commit `e5465a2c1c`) `🔣️.json`. That last
rename dropped the `*manifest.json` suffix the graph module discovers by, and the current graph output catalog
(`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json`, written 2026-09-05) declares 9 manifest ids with
**no `concrete-forest`** — so it has not been an admitted manifest since at least then.

Evidence it was dead:
- Content was puzzle**3d** data in a 2d artifact: `presentation.meshUrl: "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"`,
  `edgeKinds: [puzzle3d.attraction.link]`.
- Discovery is by filename suffix only (`name.ends_with("manifest.json")` in
  `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs` and `📜️script.ts:118`, oracle-tested against
  `picomatch("*manifest.json")` in `🔏️path-emoji-statutes`), so `🔣️.json` was invisible to it.
- No `include_str!`, no `taxonomy.json` entry, no registry scan of `🗿️artifacts/*/🔣️.json` (the registry's
  `descriptorRelativePath` is the *owner-root* `../../🔣️.json`, a different file).
- **Not a convention**: `🧊️3d` and `🖐️5d` have no artifact-root `🔣️.json` at all. (`🔱️trinity/♻️rewriting/🔣️.json`
  does — an `id: rewriting-rhs` manifest, invisible to graph discovery for the same reason. Same class of debris, that
  plugin's own problem.)

### 2.2 `🛂️manifest.jsondefault.manifest.json` → `🛂️manifest.json` — RENAMED

Rename corruption confirmed: the original layout was a `🛂️manifest/` **directory** holding `default.manifest.json`; the
flattening sweep concatenated `🛂️manifest.json` + `default.manifest.json`. The kind-only basename per the taxonomy's own
graph-manifest oracle (`*manifest.json`) and per the plugins that already use it correctly
(`🖍️draw`, `💡️reasoning/🔌️wires`, `✒️writer`, `📏️layout`) is `🛂️manifest.json`.

- `mv`'d in place; content unchanged (`id: puzzle2d-default`, the empty default manifest).
- Its one consumer updated: `…/✳️any/✏️editor/⚙️engine/🎲️board-host/🦀️.rs:348`
  `include_str!("../../../../../../../🛂️manifest.json")` — 7 levels up still lands on `◻️2d/`, verified on disk;
  `rustfmt --check` on that file exits 0.
- No taxonomy/registry entry names either filename. The `🔏️path-emoji-statutes` fixture row
  `{ "name": "🛂️manifest.jsondefault.manifest.json", "expected": true }` is a *predicate* vector for the
  `*manifest.json` glob, not a claim the file exists — left alone, and `bun nx run @semio-tech/repo-lib:test-path-emoji-statutes`
  still passes (**38 pass / 0 fail**).
- **Gate run**: `bun ./📜️script.ts check-generated` in `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust` →
  discovers `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest.json`, `3 pass / 0 fail`,
  `9 generated manifests are fresh`. The rename is transparent to the generated graph registry (it keys on `doc.id`).

Still corrupted, out of region: `🧊️3d/🛂️manifest.jsondefault.manifest.json`,
`🖐️5d/🛂️manifest.jsondefault.manifest.json`, `🔱️trinity/🔌️jack/🛂️manifest.jsonnakagin.manifest.json`,
`🔱️trinity/♻️rewriting/🛂️manifest.jsonrewrite-lhs.manifest.json`.

## 3. Playground example catalog — fixed at the generator

**Root cause** (not a copy from 3d/5d rows): `discoverExamplesForPlayground` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` ignored its `_pluginId`/`_variant` arguments and
called `registryExampleCatalog(repoRoot, cratePath, …)`, which walks **every artifact of the whole plugin root** plus every
`👁️viewer`/`✏️editor` surface's `📚️examples/`. One crate serves puzzle2d/3d/5d, so all three variants got the union
(`🌙️capsule-dream` is 5d-only; `🎬️demo-session` is an editor cmd-replay fixture). The function's own docstring described a
variant-suffix fallback that no longer existed in the code.

**Fix**: the owner descriptor is each app's own declaration of its examples, and it is already a declared registry input
(`taxonomy.generatorContracts["plugin-registry"].inputDiscovery.descriptorRelativePath = "../../🔣️.json"`, read by the
existing `readDescriptorJson`). `generatePlaygroundRegistry` now reads it once per crate and
`declaredExampleIdsForPlayground(descriptor, playground.app)` narrows the membership scan to the ids that app declares
(all rows when the playground names no app). A crate with no descriptor, or an app that declares no examples, keeps the
whole-crate scan unchanged. Slug → id is the `exampleSlugPattern` tail (everything after the emoji identity's `U+FE0F`),
via the new `EXAMPLE_SLUG_IDENTITY_SEPARATOR` constant. No taxonomy change, no new input class.

**Regenerated** with `bun ./📜️script.ts generate` (59 plugin crates, 60 playgrounds, 45 framework packages);
`check-generated` → `plugin registry generated catalog and launch bytes are fresh.`

`🎮️playgrounds.ts:66` now reads exactly:

```
{ variant: "puzzle2d", … app: "s.puzzle.puzzle2d@1/*#editor", … examples: ["🌲️concrete-forest","🏗️nakagin-capsule-tower"], … }
```

**16 rows changed in total — all corrections, none a regression, none reverted:**

| rows | before → after | why |
|---|---|---|
| `puzzle2d` | `["🌙️capsule-dream","🌲️concrete-forest","🎬️demo-session","🏗️nakagin-capsule-tower"]` → `["🌲️concrete-forest","🏗️nakagin-capsule-tower"]` | descriptor declares exactly these two for `s.puzzle.puzzle2d@1/*#editor` |
| `puzzle3d` | same 4 → `["🌲️concrete-forest","🏗️nakagin-capsule-tower"]` | 3d declares two |
| `puzzle5d` | same 4 → `["🌙️capsule-dream","🌲️concrete-forest","🏗️nakagin-capsule-tower"]` | 5d declares three |
| `animate`, `dag`, `draw`, `fem2d`, `fem3d`, `forms`, `mathematical`, `note`, `reasoning-wires`, `s`, `sequence`, `sourcing`, `vcs`, `writer` (13 plugins) | `["🎬️demo","🎬️demo-session"]` → `["🎬️demo"]` | each descriptor declares only `demo`; `🎬️demo-session` is a recorded-command test fixture under `✏️editor/📚️examples`, never an app example |

Plugins whose descriptor declares no examples (`architect`, `block`, `cad`, `demonstrator`, `flow`, `gis`, `imperative`,
`layout`, `lowpoly`, `norm`, `process`, `procedural`, `raster`, `remodel`, `shooting`, `energy`, …) are byte-identical —
the whole-crate scan still stands for them. Note the `examples` field is currently **not consumed** by any runtime code
(only emitted, plus the brand-donor fallback at `📜️script.ts:517`), so the practical blast radius is documentary.

## 4. Storybook

`.storybook/stories/puzzle/2d/Board.stories.tsx` — 2 imports of the non-existent ASCII path
`../../../../framework/product/os/module/renderer/js/react/index.tsx` → the real
`../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️`
(extensionless, matching the working `stories/puzzle/3d/World.stories.tsx` and `stories/remodel/*.stories.tsx`).

`.storybook/stories/puzzle/2d/Fixtures.stories.tsx` was worse — **five** dead references, all fixed:

| before | after |
|---|---|
| 2 × ASCII renderer path | real renderer path (as above) |
| `…/◻️2d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio?raw` | `…/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio?raw` |
| `…/◻️2d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio?raw` | `…/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio?raw` |
| `import("@semio-tech/puzzle-2d-rs/pkg/puzzle_2d.js")` (package does not exist) | `import("../../../../✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js")` — the real wasm-pack output (`@semio-tech/puzzle-wasm`), which does export `puzzle2dParseDslJson`; same form `World.stories.tsx` uses |
| `import capsuleDreamFixtureDsl from "…/.🦑️repo/🎫️tickets/…/🌙️capsule-dream-out/🗣️dream.2d.dsl.semio?raw"` | **removed**, together with the `CapsuleDream` story it fed — the file does not exist anywhere on disk (no `.🦑️repo` root any more, and no 2d capsule-dream fixture was ever produced). No Playwright spec referenced it (`.storybook/puzzle-3d-5d-infinite.spec.ts` only loads `🧩️puzzle🩻️2d-fixtures--nakagin-capsule-tower` and `--concrete-forest`, both intact). |

Stale prose paths in both files' headers/docstrings updated to the real ones.

`.storybook/scopes.ts` — puzzle/2d `sourceRoots`: `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻️2d` → `…/🗿️artifacts/◻️2d`.

**Verified**: a resolver pass over every relative specifier in both story files plus the puzzle scope roots →
`ALL RESOLVE` (7 imports + 1 source root). No Storybook build was run.

Left for someone else (out of my region, same class):
- `.storybook/scopes.ts` puzzle/**3d** and **5d** `sourceRoots` still point at `🎛️apps/🧊️3d` / `🎛️apps/🖐️5d` (nonexistent).
- `.storybook/scopes.ts:20` `import type { PlaygroundAssetSpec } from "…/🤖️generated/🟦️playgrounds"` — the generated file
  is `🎮️playgrounds.ts`; type-only so it is erased at bundle time, but `tsc` cannot resolve it.

## 5. `.vscode/launch.json` — **it is generated; the seed is the source**

`launch.json` is regenerated from `.vscode/🧩️launch.seed.jsonc` by
`@semio-tech/plugin-registry:generate` (`📜️script.ts:1919-1924`) and its freshness is gated by `check-generated`, which
the root `verify` gate runs. So the three entries were authored in the **seed**, not in `launch.json`.

Added (following the exact sibling conventions):

| name | command | placement |
|---|---|---|
| `🧪️test🧩️puzzle📚️examples` | `bun nx run @semio-tech/puzzle-js:test` | end of the `🧪️test<plugin>📚️examples` family (after `🧪️test🏺️remodel📚️examples`), no `presentation` block — exactly like `🧪️test🖍️draw📚️examples` / `🧪️test🧱️block📚️examples` |
| `🖨️describe🧩️puzzle` | `bun nx run @semio-tech/puzzle-plugin:describe` | next to `📦️build🧩️puzzle🌉️board`, `{"group": "4_build", "order": 122.1}` |
| `🧪️lint🧩️puzzle🧬️fixtures` | `bun nx run @semio-tech/puzzle-plugin:fixtures-lint` | same cluster, `order: 122.2` |

No `describe`/`fixtures-lint` launch family existed anywhere in the repo, so those two follow puzzle's own local cluster
(`📦️build🧩️puzzle🌉️board` order 122, `📦️audit🧩️puzzle🛂️publication-authority`) and the sub-order style used by the
`🛠️dev🧩️puzzle…` entries (220 / 220.1 / 220.2). CLAUDE.md requires every executable command to be registered there.

Diff after regeneration: **786 → 789 configurations, added exactly those 3, removed 0.**

### ⚠️ Peer work rescued — for the main session / the ENERGY ticket

`check-generated` was **already failing on `.vscode/launch.json` before I touched anything**: the ENERGY session had added
8 entries directly to the *generated* `launch.json` instead of the seed, so my first `generate` deleted them
(`🔮️oracle🔋️energy⚙️setup`, `…🔍️status`, `…🏛️native`, `…▶️run`, `…🧫️emit`, `🧪️test🔋️energy🔮️oracle`,
`🏛️bestest🔋️energy♻️regenerate`, `🏛️bestest🔋️energy🧪️laws`). **I restored all 8 verbatim into
`.vscode/🧩️launch.seed.jsonc`** (same position, right after `🔋️ Energy JS Tests`) and regenerated, so they are back in
`launch.json` and are now durable. The ENERGY session should author future entries in the seed, not in `launch.json`.

## 6. Gates run (all by me, none inferred)

| gate | result |
|---|---|
| `jsonschema` union validation (75 payloads, 5 hostile inputs) | **ALL CHECKS PASSED** |
| `@semio-tech/plugin-registry:check-generated` | `generated catalog and launch bytes are fresh` |
| `framework-graph check-generated` | `3 pass / 0 fail`, `9 generated manifests are fresh` |
| `@semio-tech/repo-lib:test-path-emoji-statutes` | `38 pass / 0 fail` |
| `rustfmt --check` on `🎲️board-host/🦀️.rs` | exit 0 |
| relative-import resolver pass over the two 2d story files + puzzle scope roots | `ALL RESOLVE` |
| `.vscode/launch.json` JSON parse + entry diff | 789 configs, +3 / −0 |

### Pre-existing failures I did NOT cause (attributed, not guessed)

`@semio-tech/plugin-registry:test` → `🚀️launch.test.ts` 2 failed / 18 passed (the other two test files pass):

1. *"exposes every owned generator preview exactly once in contract order"* — `expected [ …(16) ] to deeply equal [ …(15) ]`.
   Caused by an **uncommitted peer edit to `🔣️taxonomy.json`** (`git diff` shows a newly added
   `plugin-publication-authority` collection and a changed wgpu `catalogSha256`), not by anything in this wave.
2. *"keeps generated native, root preflight, and MCP runtime profiles identical without debug"* —
   `expected '#!/usr/bin/env bun…' to contain '"wasm32-wasip2", "wasm-dev"'`, i.e. a peer's in-flight rewrite of
   `🧰️framework/🔨️modules/🛂️manifest/📦️packages/🦀️rust/📜️script.ts`.

Neither test reads examples, playgrounds, or the launch entries added here. Note also that the target's own 15 s budget
kills the suite before it finishes on this loaded host (`[budget] … exceeded 15000ms — killed`); the numbers above come
from running vitest directly with a 120 s timeout.

## 7. Files changed

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json` — rewritten
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🔣️.json` — **deleted**
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🛂️manifest.jsondefault.manifest.json` → `…/◻️2d/🛂️manifest.json` — **renamed**
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🦀️.rs` — `include_str!` at line 348
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — `EXAMPLE_SLUG_IDENTITY_SEPARATOR`, `declaredExampleIdsForPlayground`, `discoverExamplesForPlayground`, its call site in `generatePlaygroundRegistry`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/*` — regenerated (gitignored)
- `.vscode/🧩️launch.seed.jsonc` — 3 puzzle entries added, 8 ENERGY entries restored
- `.vscode/launch.json` — regenerated
- `.storybook/stories/puzzle/2d/Board.stories.tsx`, `.storybook/stories/puzzle/2d/Fixtures.stories.tsx`, `.storybook/scopes.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated/a2/validate_mutation_schema.py` + `venv/`
