# G3 — proven-eight + space re-verification: rename-drift audit (static)

Scope: peer's repo-wide taxonomy rename under `✏️s/🔌️plugins/**`, `🎚️options` → `☑️options` and `⚙️config` → `🎚️config`. Read-only, static (grep/find), no builds. `git status --short | grep -c '^R'` = **158** renames, all `R  ` (staged, worktree clean at those paths).

## 1. Inventory — directory counts per plugin

Repo-wide directory counts (live tree only, excludes `.🎫️tickets` snapshots and `⚡️cache`):

| name | count |
|---|---|
| `🎚️options` (old) | **1** |
| `☑️options` (new) | 353 |
| `⚙️config` (old) | **0** |
| `🎚️config` (new) | 946 |

**The `⚙️config` → `🎚️config` rename is 100% complete** — zero old-emoji `⚙️config` directories remain anywhere under `✏️s/🔌️plugins`.

**The `🎚️options` → `☑️options` rename has exactly one straggler**, in the `🪵️sourcing` plugin:

- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options` — not renamed. Siblings in that directory: `🎬️actions`, `👥️presence`, `📦️packages`, `🦀️.rs`, `🧪️tests`, `🪛️utilities` — **no `🎚️config` sibling present**, so this straggler does not currently trigger a same-parent emoji collision (see §3).

Per-plugin breakdown (only plugins with any options/config dirs shown; all 34 plugin roots under `✏️s/🔌️plugins` have at least one):

| plugin | old `🎚️options` | new `☑️options` | old `⚙️config` | new `🎚️config` |
|---|---|---|---|---|
| all 34 plugins except 🪵️sourcing | 0 | (varies, 2–176) | 0 | (varies, 4–528) |
| 🪵️sourcing | **1** | 5 | 0 | 9 |

Verdict: **33/34 plugins fully renamed; 1/34 (`🪵️sourcing`) half-renamed (one leftover directory); 0/34 untouched.**

## 2. Dangling references to old path segments (file:line)

Grouped by plugin. Severity noted per item.

### 🌍️gis — HIGH: broken TS imports (compile-breaking)

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🟦️.ts` (27-line file) — the sibling directory was renamed to `☑️options` but the barrel file still imports from the old path:
```
23:export * as vectorStyleOption from "./🎚️options/🎨️vector-style/🟦️";
24:export * as layersOption from "./🎚️options/👁️layers/🟦️";
25:export * as layerWeightsOption from "./🎚️options/📏️layer-weights/🟦️";
26:export * as lodModeOption from "./🎚️options/🔽️lod-mode/🟦️";
27:export * as renderModeOption from "./🎚️options/🖼️render-mode/🟦️";
```
Actual directory on disk: `.../🗺️map/☑️options/{🎨️vector-style,👁️layers,📏️layer-weights,🔽️lod-mode,🖼️render-mode}`. **Fix: replace `🎚️options` → `☑️options` in these 5 import specifiers.** This is the only genuinely load-bearing break found in the whole audit.

### 🌍️gis — MEDIUM: stale self-describing `owner` path literals in mutation descriptors

6 mutation-descriptor JSONs under the now-renamed `🎚️config` directory still carry the *old* `⚙️config` segment inside their own `owner` string (a self-referential path baked in at descriptor-generation time, now inconsistent with the directory it lives in):

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/.../🗺️map/🎚️config/🧬️schema/🧬️mutations/👁️set-layer-visibility/🔣️.json:3`
- `.../🗺️map/🎚️config/🧬️schema/🧬️mutations/🎨️set-vector-style/🔣️.json:3`
- `.../🗺️map/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json:3`
- `.../🗺️map/🎚️config/🧬️schema/🧬️mutations/🖼️set-render-mode/🔣️.json:3`
- `.../🗺️map/🎚️config/🧬️schema/🧬️mutations/📏️set-layer-stroke-scale/🔣️.json:3`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/.../🏔️terrain/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json:1`

Each has `"owner": ".../🪟️windows/🗺️map (or 🏔️terrain)/⚙️config/🧬️schema/🧬️mutations/<name>"` — the literal string still says `⚙️config`, the real directory is `🎚️config`. **Fix: replace `⚙️config` → `🎚️config` in each `owner` value.** If any registry/catalog check cross-validates `owner` against actual filesystem path (this repo has such gates elsewhere — see `project-artifact-name-registry-gate` pattern), these 6 files will fail it.

### 🪵️sourcing — MEDIUM: same owner-literal staleness, plus the leftover directory itself

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/.../🪟️windows/🔢️grid/🎚️config/🦀️.rs:81` — `owner: "...🔢️grid/⚙️config"` (dir is `🎚️config`, literal still says `⚙️config`).
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options` (directory itself) — **not renamed**; the one remaining `🎚️options` on disk. Fix: `git mv` (or peer's normal rename tool) this directory to `☑️options`, then re-check its contents for any self-describing owner literals.

### Doc-comment-only mentions (LOW — non-functional, rustdoc prose citing the old facet name as an example; harmless but stale, worth a pass)

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5` and `:53`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️.rs:51`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️main/🦀️.rs:21`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️.rs:18`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs:59`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5` and `:120`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs:39`
- `.../🗺️map/🟦️.ts:6` (`* (\`⚙️config/🦀️.rs\`), owned by each concrete Map window instance.`)

None of these affect compilation; fix opportunistically by search-replace `🎚️options`→`☑️options`/`⚙️config`→`🎚️config` inside doc comments only (do not touch code).

### Framework-level (not inside a plugin dir, but governs plugin-tree admission) — HIGH: schema/fixture inconsistency

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🔣️.json:22`:
```json
"windowFacet": { "enum": ["🎚️config", "🎚️options", "🎬️actions", "👥️presence", "🪛️utilities", "🫧️transient"] }
```
This is the **Independent Empty-Facet Fixture Owner Oracle** schema (scope: plugins `🔋️energy`, `🗄️stdio`, `🪐️space`). It still lists the *retired* `🎚️options` as the legal windowFacet spelling — `☑️options` (the renamed one) is not in the enum at all. Its own fixture file, already updated for the new shape —
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/☑️options.json` — expects the *opposite*:
- case `"artifact-window"` (sourcePath ending `.../🪟️main/☑️options/📌️.empty.md`) expects `ownerForm: "artifact-window"` (should validate as legal) — **but the schema's enum doesn't contain `☑️options`, so this case cannot currently pass.**
- case `"old-generic-options"` (sourcePath ending `.../🪟️main/🎚️options/📌️.empty.md`) expects `ownerForm: null` (should be rejected as a retired shape) — **but the schema's enum still contains `🎚️options`, so a naive enum check would accept it instead of rejecting it.**

Both wired into a live test: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts` and registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`. **Fix: line 22, replace `"🎚️options"` with `"☑️options"` in the `windowFacet` enum.** (`surfaceFacet`/`modeFacet` at lines 20–21 never had an `options` member and need no change.)

### Framework-level — LOW/latent: ambiguous taxonomy kind overlap

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:
- line 3214–3218: dedicated `"options"` kind, `"emoji": "☑️"`, `"slugPattern": "^options$"` (the new target kind).
- line 4564–4568: `"configuration"` kind, `"emoji": "🎚️"`, `"slugPattern": "^(config|options)$"` — **still admits a bare `options` slug into the `configuration`/🎚️ kind**, i.e. two taxonomy kinds simultaneously match a directory literally named `options`. This was presumably how the *old* `🎚️options` shape got its emoji before the dedicated `options` kind existed. Now that the dedicated kind exists and the directory rename is essentially complete, the `|options` alternative in the `configuration` slugPattern is dead weight that keeps the ambiguity alive for the one remaining straggler (and for any future regression). **Fix: once `🪵️sourcing`'s straggler is renamed, narrow line 4566's `slugPattern` to `"^config$"`.**

Checked and found **clean** (no dangling old-segment references): Cargo.toml `path`/`[[test]]` entries under any plugin, `#[path = "…"]` attributes, `include_str!`/`include_bytes!` (repo-wide grep over `✏️s`), `.vscode/launch.json` (4900 lines, 0 hits), vite configs (0 hits), any `📜️script.ts` (0 hits), `🗺️catalog.json` (live deployment catalog, 0 hits). Framework `📋️project.json` and other JSON hits found by broad grep (`⚙️config-graph.json`, `⚙️config.mjs`, `⚙️config.toml`, a generic `FileKind` vocabulary comment) are **false positives** — ordinary files/words containing "config", unrelated to the plugin directory rename.

## 3. Collisions (taxonomy admission: `pathEmojiPolicy.identity = "single-emoji-grapheme"`, `siblingNamespace = "files-and-directories"`)

Checked programmatically: for every `🎚️config` directory and every `☑️options` directory under `✏️s/🔌️plugins`, listed siblings and counted how many other entries in the same parent start with the same emoji grapheme (`🎚️` or `☑️` respectively).

**Result: zero collisions found on disk.** No parent directory currently contains two siblings both keyed `🎚️` or both keyed `☑️`. The one surviving `🎚️options` straggler (`🪵️sourcing/🧩️extensions/🪟️windows/🎚️options`) has no `🎚️config` sibling in that same parent, so it does not trigger a live collision either — it's a straggler, not a collision.

The closest thing to a collision is **schema-level, not filesystem-level**: the `windowFacet` enum in §2's framework schema literally lists `🎚️config` and `🎚️options` side by side as if both were legal siblings sharing the `🎚️` identity — which the taxonomy's own `pathEmojiPolicy` would reject if two real directories ever matched both. Fixing that enum (§2) also removes this latent conflict.

## 4. Boot recipes / probes referencing moved paths

**This ticket's own scripts** (`📜️*-activate.sh`, `📜️*-serve.sh`, `🐍️*-probe.mjs`/`.ts`/`.py` under this ticket folder): grepped all for `🎚️options`/`⚙️config` — **zero hits in scripts**. The only 2 hits in the whole ticket folder are prose in `.md` reports (`📓️b1a-dormant-plugin-boots.md:226` narrates the rename landing mid-slice; `📓️status.md` is this slice's own assignment line) — not stale paths, just narrative.

**Proven-eight plugin tickets** — located and grepped each for `🎚️options`/`⚙️config`:

| plugin | ticket | hits in scripts/probes |
|---|---|---|
| raster | `🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END` | 0 (1 prose hit in `📓️explore-raster-history-and-prior-tickets.md:13`, describing an *unrelated, earlier* Sep-1→3 rename sweep — not this one) |
| forms | `🎆️26/🌙️09/☀️16/FORMS-PLUGIN-END-TO-END` | 0 |
| note | `🎆️26/🌙️09/☀️17/NOTE-PLUGIN-END-TO-END` | 0 |
| fem | `🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END` | 0 |
| energy | `🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END` | 0 |
| layout | `🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END` | 0 |
| remodel | `🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END` | 0 |
| draw | `🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END` | 0 |

**Verdict: no boot recipe or probe in this ticket or the proven-eight tickets hardcodes a path through the renamed segments.** They're clean with respect to this rename. (`🪐️space`'s own recipe is owned by C1b's cold `dev s` boot, not assigned to this slice or the proven-eight; not audited here beyond the empty-facet schema/fixture check in §2, which does name `🪐️space` in its plugin enum.)

## Mechanical fix list (in priority order)

1. **`✏️s/🔌️plugins/🌍️gis/.../🗺️map/🟦️.ts:23-27`** — change `"./🎚️options/…"` → `"./☑️options/…"` in all 5 export lines. (Compile-breaking; highest priority.)
2. **`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🔣️.json:22`** — change `"🎚️options"` → `"☑️options"` in the `windowFacet` enum. (Makes the schema agree with its own already-updated fixture; a live test consumes both.)
3. **Rename the straggler**: `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🎚️options` → `☑️options` (and re-grep its subtree afterward for any owner-literal staleness, same pattern as item 4).
4. **6 `owner` literals** in gis mutation descriptors (`⚙️config`→`🎚️config`): the 5 gismap JSONs + 1 gisterrain JSON listed in §2, plus **1 Rust literal** at `🪵️sourcing/.../🔢️grid/🎚️config/🦀️.rs:81`.
5. **Doc-comment cleanup** (non-blocking): the 8 rustdoc/TSdoc lines listed in §2 under "Doc-comment-only mentions" — cosmetic, batch with item 1/4 if touching those files anyway.
6. **`🔣️taxonomy.json:4566`** — once item 3 lands, narrow `"slugPattern": "^(config|options)$"` to `"^config$"` on the `configuration` kind to retire the now-dead `options` alternative and remove the latent double-kind-match on a bare `options` slug.

## Honest gaps

- Static/grep-only per slice rules: no cargo check, no vitest, no nx run was executed to *confirm* item 1's TS break actually fails a build, or that item 2's schema change is what the live test (`🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts`) actually asserts byte-for-byte — this report identifies the textual inconsistency; a worker with build permission should confirm at runtime.
- Did not search generated/cache trees (`⚡️cache`, `🗑️generated`, `🤖️generated`) or old ticket snapshots (`.🎫️tickets/🎆️26/🌙️08/...`) for old-segment strings — those are stale-by-design or historical, not live blast radius.
- Did not audit the non-plugin `✏️s/🔨️modules` shared-module trees or `🧰️framework` product code paths beyond the two framework files in §2 that directly gate plugin-tree admission (taxonomy.json, the empty-facet-authority schema/fixture pair) — a full framework-wide sweep was out of scope for "proven-eight + space re-verification."
