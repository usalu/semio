# W1 — Registry Genericization Progress

Started: 2026-08-11

## Task
De-hardcode plugin registry from "✏️s" and "studio" literals, data-driven from taxonomy + crate metadata.

## Log
- Read registry script.ts (1540 lines) and taxonomy.json fully first.

### taxonomy.json
- Added top-level `"pluginAreas": ["✏️s/🔌️plugins"]` array (placed right before `"areas"`).

### Ownership gap: `pluginAreas` not on the shared `Taxonomy` TS type
- `🦑️repo/📚️library/🔍️discovery/🟦️component.ts` (NOT in my file ownership) declares the `Taxonomy`
  interface used by `loadTaxonomy()`. It does not have a `pluginAreas` field.
- Cannot touch that file (explicit ownership boundary + CRITICAL OPERATIONAL RULES). Worked around it
  in registry script.ts with a local type augmentation + cast:
  `type TaxonomyWithPluginAreas = typeof TAXONOMY & { readonly pluginAreas: readonly string[] };`
  then `(TAXONOMY as TaxonomyWithPluginAreas).pluginAreas`, validated non-empty + every entry present
  in `TAXONOMY.areas` at module load (throws otherwise).
- Precedent found while investigating: `🔍️discovery/🟦️component.ts` itself already has the mirror-image
  problem — `taxonomy.json`'s `standardDirPrefix` field is used at component.ts:349 but is NOT on the
  `Taxonomy` interface either (confirmed via ad-hoc `tsc` pass), so this kind of JSON/type drift is a
  known, tolerated pattern in this codebase, not something I introduced.
- Whoever owns `🔍️discovery/🟦️component.ts` should add `readonly pluginAreas: readonly string[];` to
  the `Taxonomy` interface and I can then drop the local cast in registry script.ts.

### registry script.ts — deleted dead code (per taxonomy area "✏️s/🔌️plugins" = "clean")
Verified each had no live caller (or updated the caller) before deleting:
- `areaAdmitsLegacyShape` — its only remaining live caller (CheckScript's taxonomy-findings warn/fail
  branch) was inlined to `PLUGIN_AREAS_STATE === "legacy" || PLUGIN_AREAS_STATE === "mixed"`.
- `legacyRustManifestIn`
- `findLegacyComponentManifests` — `findPluginCargoFiles` simplified to just the shared discovery
  contract (no more union with a legacy-manifest scan).
- `dedupeInFlightPlaygroundEntries` — call site in `generatePlaygroundRegistry` removed; raw playground
  array is used directly now.
- `validateConstitutionalCrates`
- `CONSTITUTIONAL_SLOTS`
- `CONSTITUTIONAL_SLOT_DIRNAME`
- Also removed now-dead `RUST_MANIFEST_FILENAME` and `WALK_SKIP_DIRS` constants (only consumers were
  the deleted functions).
- `migratedPluginIds` var in `CheckScript` removed (was only threaded into
  `validateConstitutionalCrates`); `newContractPluginRoots` still used for the taxonomy tree audit.

### registry script.ts — PLUGINS_AREA → PLUGIN_AREAS (array, membership test)
- New `PLUGIN_AREAS: readonly string[]` (from taxonomy, validated), replacing the `PLUGINS_AREA`
  literal at every live call site:
  - `discoverExamplesForPlayground` — now finds the matching area via `.find()` over the array instead
    of splitting one hardcoded literal.
  - `findNewContractPluginRoots` — `dirname(pkg.ownerRel) === PLUGINS_AREA` →
    `PLUGIN_AREAS.includes(dirname(pkg.ownerRel))`.
  - `CheckScript`'s taxonomy-findings log lines — now print `PLUGIN_AREAS.join(", ")`.
- `PLUGINS_AREA_STATE` → `PLUGIN_AREAS_STATE`, now merged across every declared plugin area via a new
  `mergeAreaStates()` helper (most-permissive-wins: legacy > mixed > clean), so a second, still-legacy
  plugin area added to the array later can't be silently masked by a sibling that's already clean.
  Currently reduces to the single "✏️s/🔌️plugins" → "clean" state.

### registry script.ts / Cargo.toml — host key rename studio → shell
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`: `host = { landing = "home", studio = "studio" }`
  → `host = { landing = "home", shell = "studio" }` (key renamed, value "studio" — the plugin's own app
  id — left untouched, per instructions).
- registry script.ts's `hostBlock` regex: `/studio\s*=\s*"([^"]+)"/` → `/shell\s*=\s*"([^"]+)"/`.

### registry script.ts — renames
- `isStudioPluginFilter` → `isHostPluginFilter` (all call sites + export).
- `PlaygroundSession.studioMode` → `hostMode` (type, `buildPlaygroundSession`, `emitSessionTypeScript`
  emitter incl. the emitted TS type/const text, `validatePlaygroundSessions`).

### registry script.ts — new resolver + DEFAULT_HOST_VARIANT emission
- Added `export function resolveDefaultHostVariant(repoRoot)`: finds the single plugin crate whose
  entry has `.host !== undefined` (throws if 0 or >1), resolves its playground variant. Verified it
  emits `"s"` (matches the previous hardcoded literal) via `bun nx run @semio-tech/plugin-registry:generate`.
- `emitPlaygroundsTypeScript` now takes `defaultHostVariant` and emits
  `export const DEFAULT_HOST_VARIANT = "...";` into generated `🟦️playgrounds.ts`.
- `emitRustHosts` now takes `defaultHostVariant` and emits
  `pub const DEFAULT_HOST_VARIANT: &str = "...";` into generated `🦀️hosts.rs`.
- `renderCatalogFiles` computes `defaultHostVariant` once via `resolveDefaultHostVariant(repoRoot)` and
  threads it to both emitters.
- BUG CAUGHT + FIXED: first attempt at the `emitPlaygroundsTypeScript` doc-comment used unescaped
  backticks inside the JS template literal, which truncated the generated file at parse time (`bun`
  threw `Expected ";" but found "resolveDefaultHostVariant"`). Fixed by escaping (`\``), verified by
  rerunning `generate`.

### registry script.ts — validatePlaygroundSessions data-driven (task item 7)
- `studioVariant` (local var name kept, task's own wording) ← `resolveDefaultHostVariant(repoRoot)`
  instead of hardcoded `"s"`.
- `standaloneVariant` ← lowest-sorted playground variant whose plugin does NOT declare `.host`, instead
  of hardcoded `"playbook"`.

### vite.config.ts (full ownership: isStudioPluginFilter import + studioMode threading + 2x `?? "s"`)
- Import `isHostPluginFilter` (renamed) + `DEFAULT_HOST_VARIANT` from generated playgrounds.ts.
- `plugin = ... ?? "s"` → `... ?? DEFAULT_HOST_VARIANT`.
- `VITE_SEMIO_PLUGIN` define `?? "s"` → `?? DEFAULT_HOST_VARIANT`.
- Both `isStudioPluginFilter(plugin)` call sites → `isHostPluginFilter(plugin)`.

### os/dev script.ts (LITERAL-ONLY ownership: `?? "s"` / `|| "s"` + PLUGIN_HOST_MODE_SYMBOLS string)
- Added import of `DEFAULT_HOST_VARIANT` from generated playgrounds.ts (new import line — required to
  use it, otherwise in scope of "the `?? "s"` literals").
- Replaced all 7 occurrences of `?? "s"` / `|| "s"` default-variant literals with
  `?? DEFAULT_HOST_VARIANT` / `|| DEFAULT_HOST_VARIANT` (lines ~811, 881, 1257, 1350, 2378, 2390, 2417
  pre-edit numbering).
- `PLUGIN_HOST_MODE_SYMBOLS` array: `"studioMode"` → `"hostMode"`.
- DID NOT touch the `isStudioPluginFilter` import (line 39) or its 3 call sites (lines ~102, 826, 853)
  — explicitly out of my ownership scope for this file (only literal defaults + the
  PLUGIN_HOST_MODE_SYMBOLS string were granted). **This import is now broken** — registry script.ts no
  longer exports `isStudioPluginFilter` (renamed to `isHostPluginFilter`). Whoever owns the rest of
  dev/script.ts must rename this import + its 3 call sites to `isHostPluginFilter`. Flagged prominently
  in final report.

### kernel 🟦️component.ts (only: expandPluginRegistry's studioMode param + its doc comment)
- `expandPluginRegistry(plugins, primaryPluginId, studioMode = false)` → `... hostMode = false`; body
  `if (studioMode || ...)` → `if (hostMode || ...)`.
- Doc comment above it: "Studio mode, or the absence of a primary id, ..." → generic "Host mode (a
  launch that hosts every plugin at once, e.g. a shell/studio session), or the absence of a primary
  id, ...".
- Did NOT touch the unrelated local `studioMode` var in `resolvePlaygroundBoot` (line ~1118/1128,
  passed positionally to `expandPluginRegistry` — different variable, explicitly out of scope) or other
  "studio" mentions elsewhere in the file (line ~276 unrelated UI doc comment, line ~1134 doc comment
  describing the real "s" plugin's actual home/studio app-id pair — value stays per task item 4).

### Verification
- `bun nx run @semio-tech/plugin-registry:generate` — SUCCEEDED (after fixing the backtick-escaping
  bug above). 59 plugin crates, 58 playgrounds, 18 framework packages.
- `bun nx run @semio-tech/plugin-registry:check` — FAILS, but with the exact same pre-existing
  "plugin taxonomy tree violations (area(s) "✏️s/🔌️plugins" is "clean")" already documented in this
  ticket's own `📸️baseline.md`/`📸️baseline-verify-gate.txt` captured before this wave started.
  Confirmed byte-identical violation count: `grep -c "does not exist on disk\|is not declared by any\|is missing"`
  = 5349 in both the baseline log and my post-change run. My playground-registry/session validation
  logic (the part I actually changed) produced ZERO new violations — none of
  "playground validation errors", "resolved unexpectedly", "does not distinguish",
  "expected exactly one plugin crate", or "catalog is stale" appear anywhere in my check output.
- grep-gate: zero remaining `isStudioPluginFilter`/`studioMode`/`PLUGINS_AREA =` in registry script.ts
  and vite.config.ts (both fully owned). kernel component.ts's one remaining `studioMode` occurrence
  (line ~1128) is the explicitly out-of-scope local var, confirmed correct to leave.
- Did NOT run cargo check (Cargo.toml key rename is TOML metadata only, read by the TS regex parser,
  not by rustc — no Rust code references `package.metadata.semio.host.studio` as a Rust identifier).
- Did NOT run `git commit`/`ticket_close` (not authorized to use MCP repo tools per this session's
  rules).
