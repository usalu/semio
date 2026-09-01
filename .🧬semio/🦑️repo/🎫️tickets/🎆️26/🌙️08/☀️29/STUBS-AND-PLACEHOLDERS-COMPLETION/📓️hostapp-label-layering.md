# hostApp Search-Category Label — Investigation, Decision, Plan

## Task
Both localization dictionaries in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
(German ~2320, English ~3120) hardcoded `hostApp: { label: { normal: "Space", beginner: "Space" } }`
with a `TODO(follow-up)` saying it should come from the host plugin's own manifest label instead of
being framework-side literal. Goal: fix it, or replace the TODO with a definitive comment plus a
precise plan if it can't be fixed within this ticket's scope.

## Where `hostApp` is used (traced, not assumed)
- **Consumer of the dictionary key** (the only two call sites in the whole repo):
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:6448,6455`
  — `category: shellLabel("ui.search.category.hostApp")` on the "undo"/"redo" rows of the command
  palette, shown only when `hostMode && panel` (hostMode = the shell is running inside a host plugin,
  e.g. the `s`/space plugin).
- **Type slot**: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx:111`
  (`UiTranslationSchema.ui.search.category.hostApp: UiLabelValue`).
- **The `hostApp` *variable*** (distinct name, same file, `ShellHost/🟦️component.tsx:1132`):
  `const hostApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id ===
  hostConfig?.hostAppId), [hostPlugin, hostConfig])` — this is the actual `AppDefinition` object for
  the plugin's own "host" app, looked up from the already-loaded WASM manifest.
- **Where `hostConfig` comes from**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:244-245`
  parses `[package.metadata.semio].host = { landing = "home", shell = "studio" }` straight out of
  Cargo.toml with a regex and embeds `landingAppId`/`hostAppId` **verbatim as those raw alias
  strings** into the generated `PLUGIN_CATALOG` (`glue.ts` test at line 675 confirms:
  `resolvePluginHostConfig(...)` returns `{ hostAppId: "studio" }` literally, no resolution).

## (a) How the host plugin/app is identified at runtime
Two different things share the name "studio" and get conflated:
1. The **Cargo.toml alias** `"studio"` (a human-readable nickname the plugin author picked).
2. The **real, canonical `AppDefinition.id`**, which Rust always derives from the artifact dialect —
   `surface_app_id()` in `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3018` formats it as
   `<artifact_kind>@<standard>/<subset>#<role>`. For the space plugin's studio app this is the
   constant `S_PLAY_APP_ID = "s.space.studio@1/*#editor"`
   (`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs:52`), never the bare string `"studio"`.

`hostApp`'s lookup (`app.id === hostConfig?.hostAppId`) compares (2) against (1) — `"s.space.studio@1/*#editor"
=== "studio"` — which is **never true**. This is not a new finding: `ShellHost/🟦️component.tsx:4112-4121`
(a prior session's "w4-h root-cause fix #2" comment) already documents this exact bug in detail,
independently confirms `hostApp` is "consequently always `undefined` today", and explicitly calls it "a
separate, pre-existing, wider bug this lane's lease does not cover fixing." I re-derived the same
conclusion independently by reading `surface_app_id`, `S_PLAY_APP_ID`, and the registry parser before
finding that comment — it checks out.

The same raw-alias-vs-canonical-id mistake also affects roughly a dozen other comparisons in the same
file (`session.app.id === hostAppId` at lines 4143, 5254, 5987, 6243, 6314, 6707, and the analogous
`landingAppId` comparisons at 3213, 5681, 6341) — `landingApp` only "works" today because it has an
unrelated `?? manifest.apps[0]` fallback that happens to land on the correct app since "home" is always
the plugin's first-registered app; `hostApp` has no such fallback and is simply broken.

## (b) Does the manifest already carry a localized label the framework can read?
**Yes, exactly this one.** The studio app's own Rust builder call already declares:
```rust
// ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs:869
let builder = App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space")).document(...)
```
`AppDefinition.label: LocalizedLabel` (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3034` region,
field doc: "The app's own display name … manifest-level, locale×terminology-checked") is a real,
already-shipped field. The hardcoded TS string is a **byte-for-byte duplicate** of this manifest value.

## (c) How this dictionary is consumed, and the correct read pattern
`ShellHelpers/🟦️component.tsx` already has the exact resolver needed —
`resolveManifestLabel(label: unknown, terminology, locale): string` (line 1995) — and an established
precedent for reading an app's own label instead of a static `ui.*` string:
`appWindowLabel()` (line 1027) falls back to `resolveManifestLabel(app.label, terminology,
locale).trim()`, and `resolveAppBreadcrumb`/`resolveArtifactByAppId` (lines 1010-1023) already resolve
breadcrumbs from `manifest.apps.find(...)` rather than a framework-side dictionary. The correct fix for
`ui.search.category.hostApp` is exactly this shape:
`resolveManifestLabel(hostApp?.label as LocalizedLabel | string, uiTerminology, uiLocale)` in place of
`shellLabel("ui.search.category.hostApp")` at `ShellHost/🟦️component.tsx:6448,6455`, then delete the
dictionary entries (both languages, `📦️index.tsx`) and the `hostApp` slot from `UiTranslationSchema`
(`📚️I18n/🟦️component.tsx:111`).

## Precedent check: is `hostApp` a no-consumer roster like `PLUGIN_DOMAIN_ICON_CONCEPTS`?
No — checked before touching anything (`grep -rn "ui.search.category.hostApp"` across the repo,
excluding the ticket tree): the key has exactly two real consumers, both in `ShellHost/🟦️component.tsx`
(the undo/redo palette rows). Unlike `PLUGIN_DOMAIN_ICON_CONCEPTS`, it cannot simply be deleted — the
`UiTranslationSchema` type requires it and the palette rows render it live today. It also renders
correctly today (`hostMode` is not gated by the broken `hostApp` lookup — it's `hostConfig !==
undefined`), so a change here has a real, currently-working user-visible behavior to preserve.

## Decision: stop short of a half-migration
Wiring the dictionary read to `hostApp?.label` **today** would make the category header render as an
**empty string**, not "Space" — because `hostApp` is always `undefined` (see (a)). That is a regression,
not a fix, and it would ship broken text in a UI-facing label. The instructions for this task are
explicit about this exact situation: if the correct fix needs a new manifest field plumbed through the
plugin registry and the WASM boundary, stop rather than half-migrate. That is precisely what's needed
here — the manifest currently has no field that lets TypeScript tell "this is the plugin's host app"
apart from "this is some other app in the same plugin" without re-deriving the dialect-based id (which
is domain-specific and not something the framework should special-case).

**No code was left broken and nothing was removed.** The literal `"Space"` stays (it is truthful — it
matches the manifest exactly) and both TODO comments were replaced with definitive comments (not
"TODO(follow-up)") stating exactly why this is deferred and pointing here.

## Implementation plan for a future session
1. **Rust — new field**: add `pub host_role: Option<HostRole>` to `AppDefinition`
   (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3034` region), with a small
   `pub enum HostRole { Landing, Host }` (`#[serde(rename_all = "camelCase")]`,
   `#[serde(default, skip_serializing_if = "Option::is_none")]` on the field so every other app's
   manifest is untouched).
2. **Rust — builder plumbing**: add `host_role: Option<HostRole>` to `AppBuilder`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4749` struct, default `None`) and a
   `.host_role(HostRole::...)` setter (~line 4780 `impl AppBuilder` block); forward it from
   `EditorBuilder`/`ViewerBuilder` the same way `.document(...)` is forwarded (~lines 26765-26810), and
   copy it through in each builder's `build_definition()`.
3. **Rust — the two call sites that actually need it** (both in `✏️s/🔌️plugins/🪐️space`):
   - `create_home_app()` (`🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:490`):
     add `.host_role(HostRole::Landing)` to the `Editor::builder(...)` chain.
   - `create_space_app()` (`⚙️engine/🪐️space/🦀️component.rs:869`): add `.host_role(HostRole::Host)` to
     the `App::builder(S_PLAY_APP_ID, ...)` chain.
   No other plugin in the repo declares `[package.metadata.semio].host`, so no other call site needs
   this today — but any future host plugin would set it the same way.
4. **Codegen**: regenerate the owned-schema TS mirror (`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts`,
   produced by the root owned-schema exporter, `📜️script.ts`) so `GeneratedAppDefinition` picks up
   `hostRole?: "landing" | "host"`. Confirm `AppDefinition`'s `Omit<GeneratedAppDefinition, ...>` list
   in `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:1127` does not need to widen/narrow the new field.
5. **TypeScript — fix the actual lookup**: in `ShellHost/🟦️component.tsx:1132-1133`, change:
   ```ts
   const hostApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.hostRole === "host"), [hostPlugin]);
   const landingApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.hostRole === "landing") ?? hostPlugin?.manifest.apps[0], [hostPlugin]);
   ```
   (dropping the broken `app.id === hostConfig?.hostAppId/landingAppId` comparisons entirely — `hostConfig`
   is still needed elsewhere for `pluginId`/`hostMode`, just not for this id match). This alone also
   fixes `hostControllerId`/`hostCatalogueTabId` (lines 1136, 1138) and the `isStudio` dispatch check
   (line 4133), which are silently broken today for the same root-cause reason.
6. **TypeScript — the label itself**: at `ShellHost/🟦️component.tsx:6448,6455`, replace
   `shellLabel("ui.search.category.hostApp")` with
   `resolveManifestLabel(hostApp?.label as LocalizedLabel | string, uiTerminology, uiLocale)`.
7. **Delete the now-dead dictionary slot**: remove `hostApp: { label: {...} }` from both the German and
   English trees in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
   (~2320/~3120 after this session's comment edits shifted line numbers by ~15) and the `hostApp`
   field from `UiTranslationSchema.ui.search.category` in
   `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx:111`.
8. **Do NOT** fix the other ~9 `session.app.id === hostAppId/landingAppId` raw-string comparisons
   (step 5's fix does not change their behavior — they still compare against the raw Cargo alias, still
   always false) unless that is explicitly in scope for whatever ticket picks this up next; scope creep
   there is exactly what this ticket's instructions told this session to avoid. If a future session does
   want to fix those too, the same `hostApp?.id`/`landingApp?.id` objects resolved in step 5 are the
   right replacement for the `hostAppId`/`landingAppId` primitives at every one of the sites listed in
   the "Where `hostApp` is used" section above.
9. **Tests**: check `ShellHost`/`ShellHelpers` test files (and `glue.ts`'s
   `SYNTHETIC_PLUGIN_CATALOG` — that one only exercises `resolvePluginHostConfig`, unaffected) for any
   fixture manifests that will need a `hostRole` value added to keep host-mode tests meaningful, and any
   I18n schema snapshot/shape test that enumerates `ui.search.category.*` keys.

## Changes made this session
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` (German ~2320,
  English ~3135 after edit): replaced the `TODO(follow-up)` comment above `hostApp: { label: {...} }`
  with a definitive comment (evidence + exact blocking bug + exact fix plan pointer). The `"Space"`
  literal itself is unchanged.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx:108-112`: replaced the stale
  "see ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT" doc comment on `UiTranslationSchema...hostApp`
  with a comment pointing at this file for the real reason and the exact required fix.
- No runtime/type behavior changed. Nothing removed.

## Verification
- `bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler
  --esModuleInterop --skipLibCheck` on both touched files: only the known, pre-existing
  `import.meta.env`/`import.meta.glob` errors from `🎨️styling/📦️packages/🟦️typescript/📦️index.ts`
  (an artifact of this ad-hoc invocation missing Vite types, explicitly called out as not-my-bug in
  the ticket brief) plus pre-existing unrelated errors from concurrent work elsewhere in the repo (not
  introduced by this change — see raw log in this file's sibling ticket folder if needed). No new
  errors attributable to either edited file (both edits are comment-only, no code/type surface
  changed).
- Both label sites still literally render `"Space"`/`"Space"` for English and `"Space"`/`"Space"` for
  German (unchanged), matching the host plugin's own manifest value exactly — verified by inspection
  (dictionary value untouched) rather than a UI run, since no runtime/behavioral change was made.
- No dedicated test exists for this specific dictionary key (`grep -rn "ui.search.category.hostApp"`
  found only the two `ShellHost` consumer sites, no test file references it by string), so there was no
  test to run for this key specifically.
