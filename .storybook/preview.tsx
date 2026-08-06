// #region 🧲️Header
// 💻️ .storybook/preview.ts
// Specs: Reuse the shared UI appearance and level decorators for the root monorepo Storybook.
// Summary: Defines global Storybook preview parameters; loads CSS stacks only when any scope is active; provides the decorator toolkit (level, appearance, locale, terminology, theme, renderer-port swap, wasm gate).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Preview } from "@storybook/react-vite";

import { DEFAULT_UI_DRIVER, COMPACT_UI_DRIVER, resolveUiDriver } from "@semio-tech/ui-react";
import { scopeActive } from "./scopes.ts";

declare const __STORYBOOK_ACTIVE_SCOPES__: string[];

//#region 🔖️ScopeStyles
if (__STORYBOOK_ACTIVE_SCOPES__.length > 0) {
  await import("./globals.css");
}
/** @emoji 🎯️ True when `prefix` is (a prefix of) an active `STORYBOOK_SCOPE` — for stories/decorators that gate behavior by scope. */
export function storybookScopeActive(prefix: string): boolean {
  return scopeActive(__STORYBOOK_ACTIVE_SCOPES__, prefix);
}
//#endregion 🔖️ScopeStyles

enum Appearance {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

enum Level {
  BASE = "base",
  WINDOW = "window",
  PANE = "pane",
  PANEL = "panel",
  DIALOG = "dialog",
  MENU = "menu",
}

enum Device {
  DESKTOP = "desktop",
  TABLET = "tablet",
  MOBILE = "mobile",
}

enum Locale {
  EN = "en",
  DE = "de",
}

enum Terminology {
  NATIVE = "native",
  REUSE = "reuse",
}

enum IconRenderer {
  WEBGL = "webgl",
  SVG = "svg",
}

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  globalTypes: {
    appearance: {
      description: "Global appearance for components",
      toolbar: {
        title: "Appearance",
        icon: "circlehollow",
        items: [
          { value: Appearance.SYSTEM, title: "System", icon: "browser" },
          { value: Appearance.LIGHT, title: "Light", icon: "sun" },
          { value: Appearance.DARK, title: "Dark", icon: "moon" },
        ],
        dynamicTitle: true,
      },
    },
    level: {
      description: "UI level for components",
      toolbar: {
        title: "Level",
        icon: "component",
        items: [
          { value: Level.BASE, title: "Base" },
          { value: Level.WINDOW, title: "Window" },
          { value: Level.PANE, title: "Pane" },
          { value: Level.PANEL, title: "Panel" },
          { value: Level.DIALOG, title: "Dialog" },
          { value: Level.MENU, title: "Menu" },
        ],
        dynamicTitle: true,
      },
    },
    device: {
      description: "Layout density / shell device (Elements + sketchpad parity)",
      toolbar: {
        title: "Device",
        icon: "mobile",
        items: [
          { value: Device.DESKTOP, title: "Desktop" },
          { value: Device.TABLET, title: "Tablet" },
          { value: Device.MOBILE, title: "Mobile" },
        ],
        dynamicTitle: true,
      },
    },
    driver: {
      description: "UI presentation driver (Elements driver provider)",
      toolbar: {
        title: "Driver",
        icon: "user",
        items: [
          { value: DEFAULT_UI_DRIVER.id, title: DEFAULT_UI_DRIVER.label },
          { value: COMPACT_UI_DRIVER.id, title: COMPACT_UI_DRIVER.label },
        ],
        dynamicTitle: true,
      },
    },
    locale: {
      description: "i18n locale (react-i18next)",
      toolbar: {
        title: "Locale",
        icon: "globe",
        items: [
          { value: Locale.EN, title: "English" },
          { value: Locale.DE, title: "Deutsch" },
        ],
        dynamicTitle: true,
      },
    },
    terminology: {
      description: "Chrome terminology variant (native / reuse)",
      toolbar: {
        title: "Terminology",
        icon: "book",
        items: [
          { value: Terminology.NATIVE, title: "Native" },
          { value: Terminology.REUSE, title: "Reuse" },
        ],
        dynamicTitle: true,
      },
    },
    theme: {
      description: "Active UiTheme (only affects stories rendered under withAppearance)",
      toolbar: {
        title: "Theme",
        icon: "paintbrush",
        items: [], // populated lazily in withTheme — builtinUiThemes() needs the ui-styling module graph, avoided at globalTypes-declaration time
        dynamicTitle: true,
      },
    },
    iconRenderer: {
      description: "IconRenderRequest.format for IconRenderHost-family stories — read directly from context.globals.iconRenderer, not applied by a decorator",
      toolbar: {
        title: "Icon Format",
        icon: "image",
        items: [
          { value: IconRenderer.WEBGL, title: "WebGL" },
          { value: IconRenderer.SVG, title: "SVG" },
        ],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: {
    appearance: Appearance.SYSTEM,
    level: Level.BASE,
    device: Device.DESKTOP,
    driver: DEFAULT_UI_DRIVER.id,
    locale: Locale.EN,
    terminology: Terminology.NATIVE,
    iconRenderer: IconRenderer.WEBGL,
  },
  // Storybook composes decorators left-to-right with the LAST entry outermost, so this list reads
  // innermost → outermost: appearance/level closest to the story, async wasm gate wrapping everything.
  decorators: [withAppearance, withLevel, withTerminology, withLocale, withTheme, withRenderer, withWasm],
  tags: ["autodocs"],
};

export default preview;

//#region 🔖️withLevel
import { type Level as UiLevel, LevelProvider, surfaceClass } from "@semio-tech/ui-react";
import type { Decorator } from "@storybook/react-vite";
import React from "react";

// #region 🧩️LevelWrapper
/** Border + min width used when a story sets `level` via args. */
export const LevelWrapper: React.FC<{ level: UiLevel; children: React.ReactNode }> = ({ level, children }) => {
  return (
    <div className={`p-4 ${surfaceClass} border min-w-[200px]`} data-level={level}>
      <LevelProvider level={level}>{children}</LevelProvider>
    </div>
  );
};
// #endregion 🧩️LevelWrapper

// #region 🧩️WithLevel
export const withLevel: Decorator = (Story, context) => {
  const argLevel = context.args?.level as UiLevel | undefined;
  if (argLevel) {
    return (
      <LevelWrapper level={argLevel}>
        <Story />
      </LevelWrapper>
    );
  }
  const level = context.globals.level as UiLevel;
  return (
    <LevelProvider level={level}>
      <div className={`p-4 ${surfaceClass}`} data-level={level}>
        <Story />
      </div>
    </LevelProvider>
  );
};
// #endregion 🧩️WithLevel
//#endregion 🔖️withLevel

//#region 🔖️withAppearance
import { TextSelectionContextMenuHost, useElementsSurfaceChrome, type ElementsSurfaceDevice, type ElementsSurfaceAppearance } from "@semio-tech/ui-react";

// #region 🌈️StorySurfaceHost
const StorySurfaceHost: React.FC<{
  children: React.ReactNode;
  globals: { appearance?: string; device?: string; driver?: string };
}> = ({ children, globals }) => {
  const appearance = (globals.appearance as ElementsSurfaceAppearance | undefined) ?? "system";
  const device = (globals.device as ElementsSurfaceDevice | undefined) ?? "desktop";
  const driver = resolveUiDriver(globals.driver ?? DEFAULT_UI_DRIVER.id, {});
  useElementsSurfaceChrome({ appearance, device, driver });
  return (
    <>
      {children}
      <TextSelectionContextMenuHost />
    </>
  );
};
// #endregion 🌈️StorySurfaceHost

// #region 🌈️WithAppearance
export const withAppearance: Decorator = (Story, context) => (
  <StorySurfaceHost globals={context.globals as { appearance?: string; device?: string; driver?: string }}>
    <Story />
  </StorySurfaceHost>
);
// #endregion 🌈️WithAppearance
//#endregion 🔖️withAppearance

//#region 🔖️withLocale
import { setUiLocale, writeStoredUiChromeLocale, type UiLocale } from "@semio-tech/ui-react";
import { createBrowserStoragePort } from "@semio-tech/framework-core";

const LocaleHost: React.FC<{ children: React.ReactNode; locale: UiLocale }> = ({ children, locale }) => {
  React.useEffect(() => {
    writeStoredUiChromeLocale(createBrowserStoragePort(), locale);
    void setUiLocale(locale);
  }, [locale]);
  return <>{children}</>;
};

/** @emoji 🌐️ Drives the story's `changeLanguage` from the `locale` toolbar so `t(...)`-consuming components re-render translated. */
export const withLocale: Decorator = (Story, context) => (
  <LocaleHost locale={(context.globals.locale as UiLocale | undefined) ?? "en"}>
    <Story />
  </LocaleHost>
);
//#endregion 🔖️withLocale

//#region 🔖️withTerminology
import { writeStoredUiChromeTerminology, type UiChromeTerminologyId } from "@semio-tech/ui-react";

const TerminologyHost: React.FC<{ children: React.ReactNode; terminology: UiChromeTerminologyId }> = ({ children, terminology }) => {
  React.useEffect(() => {
    writeStoredUiChromeTerminology(createBrowserStoragePort(), terminology);
  }, [terminology]);
  return <>{children}</>;
};

/** @emoji 📚️ Persists the `terminology` toolbar choice (native / reuse) before render, mirroring `useUiTerminology`'s storage-event contract. */
export const withTerminology: Decorator = (Story, context) => (
  <TerminologyHost terminology={(context.globals.terminology as UiChromeTerminologyId | undefined) ?? "native"}>
    <Story />
  </TerminologyHost>
);
//#endregion 🔖️withTerminology

//#region 🔖️withTheme
import { builtinUiThemes, setActiveUiTheme } from "@semio-tech/ui-react";

const ThemeHost: React.FC<{ children: React.ReactNode; themeId: string }> = ({ children, themeId }) => {
  React.useEffect(() => {
    const theme = builtinUiThemes().find((t) => t.id === themeId);
    if (theme) setActiveUiTheme(theme);
  }, [themeId]);
  return <>{children}</>;
};

/** @emoji 🎨️ Applies the `theme` toolbar selection via `setActiveUiTheme`; a no-operation until the toolbar's `items` are populated (see `populateThemeToolbarItems` below) since `initialGlobals.theme` is unset by default. */
export const withTheme: Decorator = (Story, context) => {
  const themeId = context.globals.theme as string | undefined;
  if (!themeId) return <Story />;
  return (
    <ThemeHost themeId={themeId}>
      <Story />
    </ThemeHost>
  );
};

/** @emoji 🎨️ Populates the `theme` toolbar's `items` from `builtinUiThemes()` — deferred to module-init time (not the `globalTypes` literal above) since it needs the ui-styling module graph loaded. */
if (preview.globalTypes?.theme?.toolbar) {
  preview.globalTypes.theme.toolbar.items = builtinUiThemes().map((t) => ({ value: t.id, title: t.name ?? t.id }));
}
//#endregion 🔖️withTheme

//#region 🔖️withRenderer
import { configureHostPorts, type HostPortOverrides } from "@semio-tech/ui-react";

/** @emoji 🔌️ A story requests an alternate host-port adapter (stub renderer, test double, …) by setting
 * `parameters.hostPortOverrides` to a {@link HostPortOverrides} object (or a thunk returning one, for
 * overrides that need `context.globals`, e.g. the `iconRenderer` toggle). Applied via `configureHostPorts`
 * before render and restored to whatever was installed beforehand on cleanup, since ports are page-global.
 * Most stories set nothing here and render with the library's real default adapters. */
export const withRenderer: Decorator = (Story, context) => {
  const overridesParam = context.parameters.hostPortOverrides as HostPortOverrides | ((context: typeof context) => HostPortOverrides) | undefined;
  if (!overridesParam) return <Story />;
  const overrides = typeof overridesParam === "function" ? overridesParam(context) : overridesParam;
  React.useLayoutEffect(() => {
    return configureHostPorts(overrides);
  }, [JSON.stringify(Object.keys(overrides))]);
  return <Story />;
};
//#endregion 🔖️withRenderer

//#region 🔖️withWasm
/** @emoji 🧱️ Single-flight dynamic-import loader registry for scope wasm graphs. Dynamic imports
 * code-split per loader, so a scoped Storybook boot never pulls in another scope's wasm graph until a
 * story actually requests it via `parameters.wasm`. */

// #region 🔌️FrameworkHostsWasmLoaders
/** 🧵️ Wraps a `framework/os/renderer/js/react/index.tsx`-style wasm-bindgen module import (bare workspace
 * specifier, resolved by bun-symlinked `node_modules/@semio-tech/*` — no scope alias needed) into a
 * single-flight `WASM_LOADERS` entry; `mod.default()` is itself idempotent once the wasm instance is
 * live, but the cache also collapses concurrent first-call races and clears on failure so a later
 * story render can retry. */
const frameworkHostsWasmInitPromises = new Map<string, Promise<void>>();
function singleFlightWasmInit(id: string, load: () => Promise<{ readonly default: (input?: unknown) => Promise<unknown> }>): () => Promise<void> {
  return () => {
    let promise = frameworkHostsWasmInitPromises.get(id);
    if (!promise) {
      promise = load()
        .then(async (mod) => {
          await mod.default();
        })
        .catch((error) => {
          frameworkHostsWasmInitPromises.delete(id);
          throw error;
        });
      frameworkHostsWasmInitPromises.set(id, promise);
    }
    return promise;
  };
}
// #endregion 🔌️FrameworkHostsWasmLoaders

const WASM_LOADERS: Record<string, () => Promise<void>> = {
  "node-graph": singleFlightWasmInit("node-graph", () => import("@semio-tech/framework-surface-node-graph-rs/pkg/framework_surface_node_graph.js")),
  editor: singleFlightWasmInit("editor", () => import("@semio-tech/framework-editor-rs/pkg/framework_editor.js")),
  "paint-2d": singleFlightWasmInit("paint-2d", () => import("@semio-tech/framework-surface-paint-rs/pkg/framework_surface_paint.js")),
  "tiled-map": singleFlightWasmInit("tiled-map", () => import("@semio-tech/framework-surface-tiled-map-rs/pkg/framework_surface_tiled_map.js")),
  terrain: singleFlightWasmInit("terrain", () => import("@semio-tech/framework-surface-terrain-rs/pkg/framework_surface_terrain.js")),
  flow: singleFlightWasmInit("flow", () => import("@semio-tech/flow-core/flow_core.js")),
};

type WasmGateState = "idle" | "loading" | "ready" | "error";

const WasmGateHost: React.FC<{ children: React.ReactNode; ids: string[] }> = ({ children, ids }) => {
  const [state, setState] = React.useState<WasmGateState>("idle");
  const [message, setMessage] = React.useState("");
  React.useEffect(() => {
    let cancelled = false;
    setState("loading");
    Promise.all(ids.map((id) => WASM_LOADERS[id]?.() ?? Promise.reject(new Error(`no wasm loader registered for ${JSON.stringify(id)}`))))
      .then(() => !cancelled && setState("ready"))
      .catch((error) => {
        if (cancelled) return;
        setMessage(error instanceof Error ? error.message : String(error));
        setState("error");
      });
    return () => {
      cancelled = true;
    };
  }, [ids.join(",")]);
  if (state === "error") return <div className="p-4 text-sm text-red-600">wasm load failed: {message}</div>;
  if (state !== "ready") return <div className="p-4 text-sm opacity-60">loading wasm…</div>;
  return <>{children}</>;
};

/** @emoji 🧱️ Gates a story behind `parameters.wasm: string[]` loader ids until every referenced wasm module resolves. */
export const withWasm: Decorator = (Story, context) => {
  const ids = (context.parameters.wasm as string[] | undefined) ?? [];
  if (ids.length === 0) return <Story />;
  return (
    <WasmGateHost ids={ids}>
      <Story />
    </WasmGateHost>
  );
};
//#endregion 🔖️withWasm
