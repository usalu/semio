---
name: framework icon interface
overview: Replace direct lucide-react usage and inconsistent ad-hoc icon styling across the framework with a single vendored-SVG icon asset package (`@ui/assets`, with js/net/py codegen) plus a library-agnostic `Icon` interface in `@ui/react`. The framework picks icons for all static chrome (Workbench, Details, navbar, panels, footer, toolbar); consumers registering new tabs/tools must supply an icon asset (svg/url) or reference a built-in name.
todos: []
isProject: false
---

## Context (current state)

- The **platform** and **playground** React renderers import `lucide-react` directly; the **presentation** renderer uses Unicode glyphs (`↺`, `⤢`). See [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx) (lucide import block ~118-143, `PANEL_KIND_LUCIDE`, `registerTabIcon`, `registerElementIcon`, `resolveTabIconNode`) and [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) (duplicate `shellTabIcons`/`registerTabIcon`).
- Core layers ([framework/core/index.ts](framework/core/index.ts)) carry opaque `iconId` strings (`ToolItem.iconId`, `SideTabSpec.iconId`, `FooterItem.iconId`); two **duplicate registries** resolve them in renderers, with inconsistent sizing (`size-tiny`, `size-small`, `size={16}`, `size-10`).
- [ui/react/index.tsx](ui/react/index.tsx) imports lucide directly (`components.json` `iconLibrary: "lucide"`) and types props as `LucideIcon` (`ContextMenuItem.icon`, `Card.icon`, VFS map, `IconSelector`). [semio/assets/index.ts](semio/assets/index.ts) re-exports ~80 lucide symbols under semio names.
- [ui/assets](ui/assets) is today only a static-asset folder (cursors/fonts/lists) served by `uiAssetsVitePlugin` in [ui/styling/vite-elements-assets.ts](ui/styling/vite-elements-assets.ts); it has no `package.json`/`project.json`/`script.ts`/`index.ts`.

## Decisions (confirmed)

- Vendor SVGs (initial set copied from lucide), with a README notice listing vendored icons + lucide (ISC) attribution.
- New general UI asset package lives at `ui/assets`; the JS `Icon` interface lives in `@ui/react`.
- `ui/assets/script.ts` codegen emits bindings for **JS/TS, .NET (C#), and Python** from the SVG source of truth.
- Remove direct lucide from **framework + `@ui/react` + `@semio/assets`** (full migration).

## Architecture

```mermaid
flowchart TD
  svg["ui/assets/icons/*.svg (vendored)"] --> script["ui/assets/script.ts generate"]
  script --> js["generated/icons.ts (IconName union + svg strings)"]
  script --> net["generated/Icons.cs"]
  script --> py["generated/icons.py"]
  js --> uiassets["@ui/assets index.ts"]
  uiassets --> icon["@ui/react Icon primitive + IconName"]
  icon --> fw["framework renderers: built-in chrome role -> IconName"]
  icon --> semio["@semio/assets semantic re-exports"]
  fw --> consumer["consumer tab/tool registration: IconSource (name | svg | url | node)"]
```



## 1. New `@ui/assets` package (source of truth + codegen)

- Add [ui/assets/package.json](ui/assets/package.json) (`@ui/assets`, library, no lucide dep), [ui/assets/project.json](ui/assets/project.json) with `build`/`dev` targets calling `bun ./script.ts generate ...` (mirror [semio/assets/logo/project.json](semio/assets/logo/project.json)), and a single [ui/assets/script.ts](ui/assets/script.ts) using `BundleScript`/`ScriptRouter`/`runBundleScriptMain` from `repo/lib/js/src/index.ts` (mirror [semio/assets/logo/script.ts](semio/assets/logo/script.ts)).
- `script.ts generate {js|net|py|all}` reads `ui/assets/icons/*.svg`, normalizes (strip fixed width/height, force `stroke="currentColor"`/`fill` conventions), and writes into `ui/assets/icons/generated/`:
  - `icons.ts`: `export const ICONS = { ... } as const; export type IconName = keyof typeof ICONS;`
  - `Icons.cs`: static class with name constants + `IReadOnlyDictionary<string,string>`.
  - `icons.py`: `ICONS: dict[str,str]` + `IconName` literal.
- Add [ui/assets/index.ts](ui/assets/index.ts) barrel re-exporting `ICONS`/`IconName` from generated JS.
- Vendor SVGs into `ui/assets/icons/*.svg` for the union of icon names currently used (chrome roles + semio-named set). Add `ui/assets/README.md` listing every vendored-from-lucide icon and the lucide ISC attribution.
- Register `@ui/assets` in root [package.json](package.json) `workspaces` and add a `📦build👤ui🏪assets` entry in [.vscode/launch.json](.vscode/launch.json) following the existing `4_build` group ordering/naming.

## 2. `Icon` interface in `@ui/react` (library-agnostic primitive)

In [ui/react/index.tsx](ui/react/index.tsx), add a `#region 🔖Icon`:

- `export type { IconName } from "@ui/assets"`.
- `export type IconSource = IconName | { name: IconName } | { svg: string } | { url: string } | { node: React.ReactNode }` — the abstraction by which registrants "provide an asset or link to an existing one".
- `export interface IconProps { icon: IconSource; size?: number | "tiny" | "small" | "base" | "large"; className?; title? }` and `export function Icon(props)` rendering vendored inline SVG via a sized wrapper using `currentColor` and the existing size tokens. No lucide.
- Migrate internal lucide usages to `Icon`/`IconName`: remove the lucide import block, switch `ContextMenuItem.icon`, `Card.icon`, `renderContextMenuIcon`, `IconSelector`, and the VFS `VIRTUAL_FILE_SYSTEM_ICON_BY_ID`/`virtualFileSystemKindIcon` map to `IconName`/`IconSource`. Keep `Cursor`/`Spinner` (already custom SVG).
- Remove `lucide-react` from [ui/react/package.json](ui/react/package.json) and set `components.json` `iconLibrary` accordingly.

## 3. Framework: framework-chosen static icons + unified registry

- In [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx): delete the lucide import block; replace `PANEL_KIND_LUCIDE` with `PANEL_KIND_ICON: Record<PanelKind, IconName>` (framework owns this mapping). Replace all inline lucide usages (navbar back/forward/up, search/find, footer minimize, toolbar category icons, window measure check) with `<Icon name=... size=...>` at consistent sizes. Unify the icon registry into one `registerIcon(iconId, IconSource)` / `resolveIcon(iconId)` mechanism (replacing `registerTabIcon`+`registerElementIcon`).
- In [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx): remove the duplicate `shellTabIcons`/`registerTabIcon`/lucide imports and consume the shared registry + `Icon`. Panel toggle icon strategy made consistent with platform (use first tab icon, falling back to framework default).
- In [framework/product/presentation/renderer/react/index.tsx](framework/product/presentation/renderer/react/index.tsx): replace `↺`/`⤢` glyphs with `<Icon name="reset">` / `<Icon name="maximize">`.
- Built-in chrome `iconId`s (e.g. `workbench`, `details`, `settings`, `chat`, `windows`, `overview`) resolve to framework defaults; consumer-registered tabs/tools/footer items must supply an `IconSource` via the registration API / `SidePanelTabConfig.icon` / `augmentPanelTabs`, otherwise a visible missing-icon placeholder is shown.
- Sizing: route every framework icon through `Icon` size tokens; drop ad-hoc `size-tiny`/`size-small`/`size={16}`/`size-10`.
- Remove `lucide-react` from [framework/product/platform/renderer/react/package.json](framework/product/platform/renderer/react/package.json) and [framework/product/playground/renderer/react/package.json](framework/product/playground/renderer/react/package.json).

## 4. `@semio/assets` migration

- In [semio/assets/index.ts](semio/assets/index.ts): replace the lucide re-export block with semio-named exports backed by `@ui/react` `Icon` / `@ui/assets` `IconName` (keep names like `WorkbenchIcon`, `DetailsIcon`, etc.). Drop `lucide-react` from [semio/assets/package.json](semio/assets/package.json). Repurpose/zero the `@semio/icons` placeholder ([semio/assets/icons/project.json](semio/assets/icons/project.json)) to delegate to `@ui/assets` or note it as superseded.

## 5. Tests (extend existing only)

- Update the inline vitest blocks in the renderer `index.tsx` files (they currently assert lucide CSS classes) to assert vendored-svg/`data-icon` output.
- Extend `@ui/react` tests for the new `Icon` primitive and add inline tests in `ui/assets/script.ts` for SVG normalization + generated-name stability.

## 6. Remaining direct lucide consumers (same ticket, after core waves)

- [puzzle/2d/react](puzzle/2d/react), [puzzle/3d/react](puzzle/3d/react), [puzzle/5d/react](puzzle/5d/react), and [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx) also import lucide directly; migrate them to `@ui/react` `Icon` / `@semio/assets`. Storybook stories under `.storybook/stories/ui/*` that import lucide are updated to use `Icon`.

## Notes

- All work happens inside a repo MCP ticket (open first, associate with the most appropriate goal from `repo://goals`, close with summary at the end). Code is added via regions/subregions; no new script/test/example files beyond the package's single `script.ts`.

[{"id": "ticket", "content": "Open repo MCP ticket, read repo://goals and associate with the best goal."}, {"id": "assets-pkg", "content": "Create @ui/assets package: package.json, project.json, script.ts (js/net/py codegen), index.ts; vendor lucide SVGs into ui/assets/icons/*.svg; add README notice; register in root workspaces + launch.json."}, {"id": "icon-primitive", "content": "Add library-agnostic Icon primitive + IconName/IconSource to @ui/react; migrate @ui/react internals off lucide; drop lucide dep."}, {"id": "framework-chrome", "content": "Framework renderers: replace lucide/glyphs with Icon, framework-owned chrome icon mapping, unified single icon registry, consistent sizing; drop lucide deps."}, {"id": "semio-assets", "content": "Migrate @semio/assets semantic re-exports onto @ui/assets/@ui/react; drop lucide dep."}, {"id": "tests", "content": "Extend existing vitest blocks (renderers, @ui/react, ui/assets script) to cover new icon rendering and codegen."}, {"id": "remaining-lucide", "content": "Migrate remaining direct lucide consumers (puzzle 2d/3d/5d react, cad play, storybook stories)."}, {"id": "close", "content": "Run builds/tests, then close ticket with summary and file list."}]
