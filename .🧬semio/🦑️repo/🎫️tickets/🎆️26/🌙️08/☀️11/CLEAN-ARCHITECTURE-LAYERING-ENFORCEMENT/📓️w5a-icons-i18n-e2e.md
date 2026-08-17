# w5a — icon concepts, I18n `studio` label slot, studio e2e split

Scope: genericize three framework-side literals that leak s-plugin identity.
Only the four assigned files were edited. No git-state-modifying commands were run.

## 1. Icon concepts split (`app.*` / `window.*` → plugin-domain, rest stays generic)

Files:
- `🧰️framework/🔨️modules/🖼️assets/🎯️concepts/🟦️component.ts`
- `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🟦️icon_concepts.ts` (ui mirror, kept in lockstep)

Both files previously exported a single `ICON_CONCEPT_ASSIGNMENTS` map mixing:
- domain-neutral framework concepts (`chrome.*`, `graph.*`, `projection.*`, `tool.*`, `utility.*`)
- per-plugin app/window identities (`app.<pluginId>`, `window.<pluginId>.<sub>` — e.g. `app.cad`,
  `app.s`, `window.trinity.lhs`, `window.trinity.rhs`)

Split criterion used: any key literally naming a specific installed plugin (`app.*` prefix, or
`window.<pluginId>.*` prefix scoped to one app's own window) moved out; everything else (chrome
panels/generic graph shapes/projection kinds/generic tools/generic utilities — none of which name a
specific plugin) stayed.

Result in both files:
- `ICON_CONCEPT_ASSIGNMENTS` (kept, region `FrameworkIconConcepts`) — 38 entries: `chrome.*`
  (12), `graph.*` (6), `projection.*` (15), `tool.*` (2), `utility.*` (4).
- `PLUGIN_DOMAIN_ICON_CONCEPTS` (new, region `PluginDomainIconConcepts`) — 47 entries: all 30
  `app.*` entries + all 17 `window.*` entries, moved verbatim (no icon id or mapping changed).
  Exported type `PluginDomainIconConceptId = keyof typeof PLUGIN_DOMAIN_ICON_CONCEPTS`.
  Docstring on the const: `TODO(follow-up): should be plugin-declared metadata ... not hardcoded
  here`, per the task's fallback instruction (a real plugin-declared-metadata mechanism was not
  something a hook-in target existed for in this pass, so the isolated-constant fallback was used).
- `assertUniqueIconConceptAssignments()` untouched in behavior — its default parameter still binds
  to `ICON_CONCEPT_ASSIGNMENTS` only (the framework-generic map), so existing callers (react
  package's `assertUniqueIconConceptAssignments()` test at
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:12415`,
  outside my ownership, not touched) keep working unmodified. `PLUGIN_DOMAIN_ICON_CONCEPTS` is not
  currently passed through this assertion anywhere — a natural addition for the follow-up wave once
  it becomes plugin-declared.

No `ICON_CONCEPT_ASSIGNMENTS`-adjacent plugin-keyed literal was found inside the react
`📦️index.tsx` (searched for `"app.` / `"window.` prefixed object keys and for
`iconConcept`/`IconConcept` identifiers) — the task step 1 scope for that file did not apply; the
only relevant edit needed there was the `studio` label rename (see §3).

Known consumer consequence (not fixed, outside ownership): `IconConceptId` shrank from 96 keys to
38. The only place using it is the type declaration site itself and its package barrel re-export
(`🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📦️index.ts:20`, not in scope — does not
yet re-export the new `PLUGIN_DOMAIN_ICON_CONCEPTS`/`PluginDomainIconConceptId`). The Storybook
consumer `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️story.tsx:93` iterates
`Object.entries(ICON_CONCEPT_ASSIGNMENTS)` to render every icon — after this split it will render
only the 38 generic entries, not the 47 plugin ones, until that barrel is updated to also export
and render `PLUGIN_DOMAIN_ICON_CONCEPTS`. This is a visible-but-non-breaking (compiles fine)
consequence; flagging for the follow-up wave that wires the barrel export.

`🧰️framework/🔨️modules/🖱️ui/🖼️assets/🟦️icon_concepts.ts` was confirmed to be presently unreferenced
by any importer anywhere in the repo (only appears in old ticket artifact/rename-map JSON, not in
live source) — it is a dormant mirror, so editing it carried no live-consumer risk. Its
`import type { IconName } from "./📦️index.ts"` was already broken before this change (no
`📦️index.ts` file exists in that directory) — pre-existing, unrelated to this edit, left as-is.

## 2. I18n `studio` → `hostApp` rename

File: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx`

`UiTranslationSchema.ui.search.category.studio: UiLabelValue` renamed to `hostApp: UiLabelValue`
(line ~111), with a docstring explaining the rename and pointing at this ticket.

Consumers of `ui.search.category.studio` found repo-wide (grepped for the literal key string):

1. **`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`** — in
   my ownership, repointed. See §3.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`**
   — **NOT in my ownership.** Two call sites still pass the literal string
   `"ui.search.category.studio"` to `shellLabel(...)`:
   - line 4729: `category: shellLabel("ui.search.category.studio"),` (for the `studio.undo` palette item)
   - line 4736: `category: shellLabel("ui.search.category.studio"),` (for the `studio.redo` palette item)

   Since `shellLabel` is typed against the `UiTranslationKey` union derived from
   `UiTranslationSchema`, and that union no longer contains `ui.search.category.studio` (only
   `ui.search.category.hostApp`), **this file will now fail to typecheck** — a real, predictable
   consequence of the rename, not a pre-existing/unrelated break. Left untouched per the operational
   rule restricting edits to assigned files only; documented here instead per the task's own
   fallback for out-of-ownership consumers (analogous to step 4's e2e-split instruction). **Fast
   follow-up needed**: change both occurrences of the string literal
   `"ui.search.category.studio"` to `"ui.search.category.hostApp"` at those two lines.
3. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` — false
   positive from the initial broad grep (matched on the unrelated pattern `category: {`, not on the
   `studio` key); this file has no reference to the i18n `studio`/`hostApp` slot.
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/dist/assets/📦️index-D5Pw6xfV.js` — built/bundled
   output, not source; ignored (regenerates from source on next build).

## 3. React `📦️index.tsx` — `studio` literal repointed

File: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`

Two locale bundles define the schema's `search.category` block — German (line ~1984) and English
(line ~2712). Both had a `studio: { label: { normal: "Space", beginner: "Space" } }` entry (German
label was also literally `"Space"`, not translated). Both renamed to `hostApp`, key only:

```
hostApp: { label: { normal: "Space", beginner: "Space" } },
```

with a `TODO(follow-up)` comment above each noting the *value* `"Space"` is still a hardcoded
literal that should come from the host plugin's own manifest label rather than being baked into the
framework's i18n bundle — reconsidering that value's source was judged a bigger change than fits
this pass (would need a manifest-label lookup threaded into i18n bundle construction), so only the
key was repointed as instructed.

No other `ICON_CONCEPT_ASSIGNMENTS`-adjacent plugin-keyed literal existed in this file (see §1).

## 4. Studio e2e split — located, not moved (out of ownership)

Searched `os/dev`'s `script.ts` for the studio-specific Playwright e2e scenario. Found it:

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
**Region**: `//#region 🔖️SpaceE2eVerify` … `//#endregion 🔖️SpaceE2eVerify`, lines **1523–1696**.

Docstring at line 1524: "Playwright end-to-end workflow verification for the `s` studio shell
(folded in from the former
`.🦑️repo/🎫️tickets/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`)."

Contents of the region: `spaceE2eAssert`, `isIgnorableStudioE2ePageError`,
`waitForStudioE2eCondition`, `openStudioE2e`, and the `runStudioE2eVerify` orchestrator (asserts on
"Demo Studio"/"New Studio" home shell text, `/spaces/` URIs, `data-row-id="studio:default"`, studio
window rendering, and studio command-palette entries) — driven by `TestScript`/`VerifyScript`
classes immediately around it (`class TestScript` ends at line 1521, `class VerifyScript` begins at
line 1698).

This file is **not** in my file ownership for this ticket (`os/dev`'s `script.ts` is huge and
shared, exactly as anticipated in the task brief). Relocating this region to the space plugin is
feasible in principle — it's a self-contained, clearly `#region`-delimited block with no bleed into
surrounding `TestScript`/`VerifyScript` code apart from being invoked from them — but the actual
move is deferred to a follow-up wave scoped to that file.

## Verify: scoped `tsc --noEmit`

Wrote temp configs (kept in this ticket folder):
- `w5a-tsconfig.jsonc` — extends the react target's own `tsconfig.json` (for
  `allowImportingTsExtensions`/paths), includes all four touched files.
- `w5a-category-tsconfig.jsonc` + `w5a-category-check.ts` (scratchpad) — isolated structural check:
  assigns a literal `{ panels, windows, catalogue, hostApp, navigation }` object to
  `UiTranslationSchema["ui"]["search"]["category"]` to confirm the `studio→hostApp` rename is
  self-consistent between the schema and its two locale-bundle usages, independent of the rest of
  the 20k-line react file's pre-existing errors.

Ran `bunx tsc --noEmit -p <config>` for both. Output for the full four-file run:
235 pre-existing errors (missing ambient types like `UiPresence`/`NamedLayout`/`UiMenuRef` that only
resolve inside the full monorepo project graph, not this narrow `include` list; plus one genuinely
pre-existing `surfaceContextMenu.architecture` schema/literal mismatch in `📦️index.tsx` at lines
2507 and 3236 that predates this change and is unrelated to `studio`/`hostApp` or icon concepts).
**Zero** of the 235 lines reference `studio`, `hostApp`, `category`, `ICON_CONCEPT_ASSIGNMENTS`, or
`PLUGIN_DOMAIN_ICON_CONCEPTS`, and the isolated category-only check produced the identical
pre-existing-noise-only result with no error on the check file itself — confirming the renames
introduced zero new type errors within scope.

The one real new error caused by this change — `ShellHost/🟦️component.tsx` lines 4729/4736 still
passing the now-nonexistent `"ui.search.category.studio"` key — lives in a file outside this
ticket's file ownership and is documented in §2 point 2 above rather than fixed here.

## Files touched

- `🧰️framework/🔨️modules/🖼️assets/🎯️concepts/🟦️component.ts` (edited)
- `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🟦️icon_concepts.ts` (edited)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx` (edited)
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` (edited,
  surgically — only the two `search.category.studio→hostApp` sites; no other region touched)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w5a-tsconfig.jsonc` (new, scratch)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/w5a-category-tsconfig.jsonc` (new, scratch)

Not touched (documented only): `ShellHost/🟦️component.tsx` (§2), `os/dev`'s `📜️script.ts` (§4),
`🖼️assets/📦️packages/🟦️typescript/📦️index.ts` barrel and `🔣️Icons/🧪️story.tsx` (§1 consequence).
