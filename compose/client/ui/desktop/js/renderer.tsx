// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// Electron renderer: configures kit factories then mounts sketchpad via {@link mountPlatform}.

// #endregion 🧲Header

// #region ⛩️Renderer

// #region 🔌Adapters
import { mountPlatform } from "@semio-tech/framework-platform-renderer-react";
import { configureSketchpadKitFactories, createComposeKitStoreFromJsStore, ensureSketchpadPlatform, importKit, InMemoryComposeKitStore, type ComposeKitStore } from "@semio-tech/compose-sketchpad";
import { createRoot } from "react-dom/client";
// #endregion 🔌Adapters

import "./globals.css";

declare global {
  interface Window {
    __COMPOSE_E2E_KIT_FOLDER__?: string;
    __COMPOSE_E2E_KIT_FILE__?: string;
    os: {
      getUserId(): Promise<string>;
    };
    kitFolder: {
      selectFolder(): Promise<string | null>;
      readKit(folderPath: string): Promise<ArrayBuffer | null>;
      addRecentFolder(folderPath: string): Promise<void>;
    };
    kitFile: {
      selectFile(): Promise<string | null>;
      readJson(filePath: string): Promise<string | null>;
    };
  }
}

configureSketchpadKitFactories({
  folder: async (): Promise<ComposeKitStore> => {
    const e2e = typeof window !== "undefined" ? window.__COMPOSE_E2E_KIT_FOLDER__ : undefined;
    const folder = e2e && e2e.length > 0 ? e2e : await window.kitFolder.selectFolder();
    if (!folder) throw new Error("No folder selected for kit storage");
    await window.kitFolder.addRecentFolder(folder);
    const bytes = await window.kitFolder.readKit(folder);
    if (!bytes) throw new Error(`Could not read kit from folder: ${folder}`);
    const { kit, session } = await importKit(bytes);
    const jsStore = (await session.stores())[0];
    if (jsStore) {
      return createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
    }
    return new InMemoryComposeKitStore(kit);
  },
  file: async (): Promise<ComposeKitStore> => {
    const e2e = typeof window !== "undefined" ? window.__COMPOSE_E2E_KIT_FILE__ : undefined;
    const filePath = e2e && e2e.length > 0 ? e2e : await window.kitFile.selectFile();
    if (!filePath) throw new Error("No file selected for kit storage");
    const json = await window.kitFile.readJson(filePath);
    if (json == null) throw new Error(`Could not read kit file: ${filePath}`);
    const { kit, session } = await importKit(new Blob([json], { type: "application/json" }));
    const jsStore = (await session.stores())[0];
    if (jsStore) {
      return createComposeKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
    }
    return new InMemoryComposeKitStore(kit);
  },
});

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Renderer root element '#root' is missing.");
}

createRoot(rootElement).render(
  <div className="flex h-screen w-screen items-center justify-center ui-surface text-white" data-level="base">
    Loading sketchpad…
  </div>,
);

void (async () => {
  try {
    await window.os.getUserId();
  } catch (error) {
    console.error("Failed to get user ID:", error);
  }
  try {
    await mountPlatform(ensureSketchpadPlatform);
  } catch (error) {
    console.error("[compose.desktop] sketchpad mount failed:", error);
  }
})();

// #endregion ⛩️Renderer
