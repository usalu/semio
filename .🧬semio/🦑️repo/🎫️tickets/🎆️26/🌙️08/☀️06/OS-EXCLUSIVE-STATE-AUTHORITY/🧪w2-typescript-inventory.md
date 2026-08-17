# W2 TypeScript state inventory

Generated: 2026-08-06 (Wave 2 TS)

## 🧰️framework/🔨️modules
🧰️framework/🔨️modules/🧩core/🟦️component.ts:1573:        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
🧰️framework/🔨️modules/🧩core/🟦️component.ts:1580:        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
🧰️framework/🔨️modules/🧩core/🟦️component.ts:1587:        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:1702: * (not a `localStorage` default) since two shells on one page must never read/write each other's
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:1821:/** @emoji 🧵️ localStorage key for WASM compute worker thread count. */
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:3485:/** @emoji 🗣️ React hook giving TS-native products (no Rust `AppDefinition`) read/write access to the shared `ui.chrome.terminology` contract — the same localStorage key the shell's Settings terminology dropdown drives — without depending on `os-shell` state or any Rust type. */
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:3579:      order: ["localStorage", "querystring", "navigator", "htmlTag"],
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts:527:export const PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = `(function(){var d=document.documentElement,m=window.matchMedia("(prefers-color-scheme: dark)");var stored=null;try{stored=localStorage.getItem("ui.chrome.appearance")}catch(e){}var dark=stored==="dark"||(stored!=="light"&&m.matches);d.classList.toggle("dark",dark);d.dataset.uiAppearance=dark?"dark":"light";d.style.colorScheme=dark?"dark":"light";if(document.body){document.body.style.colorScheme=dark?"dark":"light";document.body.style.backgroundColor=dark?"#001117":"#f7f3e3";document.body.style.color=dark?"#f7f3e3":"#001117";}})();`;
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts:533:export const PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = `(function(){try{var raw=localStorage.getItem("ui.chrome.theme.snapshot");if(!raw)return;var t=JSON.parse(raw);if(!t||!t.colors)return;var d=document.documentElement;var dark=d.classList.contains("dark");for(var k in t.colors){d.style.setProperty("--color-"+k.replace(/_/g,"-"),t.colors[k])}if(t.spacing)for(var s in t.spacing){d.style.setProperty("--spacing-"+s.replace(/_/g,"-"),t.spacing[s])}d.dataset.uiTheme=t.id;var appearance=t.appearances&&t.appearances[dark?"dark":"light"];var chrome=appearance&&appearance.chrome;function resolveSimple(ref){return ref&&ref.token&&t.colors[ref.token]?t.colors[ref.token]:undefined}var base=chrome&&resolveSimple(chrome.base);var fg=chrome&&resolveSimple(chrome.foreground);if(document.body){if(base)document.body.style.backgroundColor=base;if(fg)document.body.style.color=fg}}catch(e){}})();`;

## 🧰️framework/📦️packages/🟦️typescript

## ✏️s

## module-level let (sample)
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:537:let referencePdfWorkerReady: Promise<void> | null = null;
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:1393:let elementsSurfaceChromeLeaseSeq = 0;
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:1399:let elementsSurfaceChromeDomBindings: ReturnType<typeof createDOMEventBinding> | null = null;
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:1400:let elementsSurfaceChromeSystemListenersInstalled = false;
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:6351:let activePanelTreeUnitDragSession: PanelTreeUnitDragSession | null = null;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬Scene/🟦️component.tsx:60:let selectableCursorUsageCount = 0;
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts:1146:let openFreeMapTileTemplate: string | null = null;
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts:1147:let openFreeMapTileTemplateAt = 0;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵Tree/🟦️component.tsx:875:let activeCatalogueDragPayload: string | null = null;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨Canvas/🟦️component.tsx:94:let activeWindowTemplateDragSession: WindowTemplateDragSession | null = null;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🐚️ShellScope/🟦️component.tsx:71:let shellScopeAutoIdSeq = 0;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🐚️ShellScope/🟦️component.tsx:129:let activeShellRootValue: HTMLElement | null = null;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🐚️ShellScope/🟦️component.tsx:131:let shellActivityListenersInstalled = false;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🌈️Surface/🟦️component.tsx:131:let surfaceActiveRoot: HTMLElement | null = null;
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🌈️Surface/🟦️component.tsx:133:let surfaceActiveListenersInstalled = false;
✏️s/🔌️plugins/📐️cad/🔨️modules/🏃️runtime/🟦️component.ts:25:let shippedModelDefinitionAssetsCache: ModelDefinitionAssetModules | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🏃️runtime/🟦️component.ts:56:let cadModulesBootstrapped = false;
✏️s/🔌️plugins/📐️cad/🔨️modules/📺️renderer/🟦️component.tsx:2000:let spatialSceneColorCache: SpatialSceneColorPalette | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:37:let modelDefinitionAssetModules: ModelDefinitionAssetModules = emptyModelDefinitionAssetModules();
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:40:let modelDefinitionFolderIdMapCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:41:let typologyOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:42:let actionOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:43:let interactionOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:44:let attributeOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:45:let propertyOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:46:let statOwnerByIdCache: ReadonlyMap<string, string> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:47:let defaultModelDefinitionIdCache: string | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:49:let typologyStyleCache: ReadonlyMap<string, ResolvedTypologyStyle> | null = null;
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:78:let interactionCompileCacheClear: () => void = () => {};
✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts:3630:let typologyConstructKitByInteractionCache: ReadonlyMap<string, TypologyConstructKit> | null = null;
