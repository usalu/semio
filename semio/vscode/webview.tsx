// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Specs: VS Code webview entry point that boots Sketchpad with a JsonFileKitStore.
// Reads initial kit JSON from window.__SEMIO_KIT_JSON__ injected by the extension host.
// Creates a KitJsonFileAdapter that bridges save/reload to the extension via postMessage.
// External file changes arrive as kit.externalUpdate messages and are applied via applyExternalUpdate.

// Webview app for the semio VS Code kit editor.

// #endregion 🧲Header

// #region 🛎️Entrypoint
// Webview entrypoint MUST mount Sketchpad with a JsonFileKitStore backed by VS Code messaging.

import { Sketchpad, appRegistry, designConfig, docsConfig, feedbackConfig, homeConfig, kitConfig, qualityConfig, typeConfig } from "@semio/sketchpad";
import type { SketchpadKitStoreFactory } from "@semio/sketchpad";
import { createJsonFileKitStore, type KitJsonFileAdapter } from "@semio/studio";
import { createRoot } from "react-dom/client";

// Declare globals injected by the extension host.
declare global {
  interface Window {
    __SEMIO_KIT_JSON__?: string;
    __SEMIO_VSCODE_API__?: { postMessage(message: unknown): void };
    __SEMIO_ON_EXTERNAL_UPDATE__?: (content: string) => void;
  }
}

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

async function boot() {
  const vscodeApi = window.__SEMIO_VSCODE_API__;
  const initialJson = window.__SEMIO_KIT_JSON__ ?? null;

  // 🔄Create a file adapter that bridges to the VS Code extension host.
  const adapter: KitJsonFileAdapter = {
    async read(): Promise<string | null> {
      return initialJson;
    },
    async write(json: string): Promise<void> {
      vscodeApi?.postMessage({ kind: "kit.save", content: json });
    },
  };

  const store = await createJsonFileKitStore(adapter);

  // Listen for external file changes from the extension host.
  window.__SEMIO_ON_EXTERNAL_UPDATE__ = (content: string) => {
    try {
      const kit = JSON.parse(content);
      store.applyExternalUpdate(kit);
    } catch {
      // Ignore parse errors from external updates.
    }
  };

  // ♻️Auto-save on changes with debounce.
  let saveTimer: ReturnType<typeof setTimeout> | undefined;
  store.subscribe(() => {
    const snapshot = store.getSnapshot();
    if (snapshot.sync.dirty) {
      if (saveTimer) clearTimeout(saveTimer);
      saveTimer = setTimeout(() => {
        store.save();
      }, 500);
    }
  });

  // 🏭File kit store factory for creating new kits as JSON files via VS Code messaging.
  const fileKitStoreFactory: SketchpadKitStoreFactory = async (kit) => {
    const kitAdapter: KitJsonFileAdapter = {
      async read(): Promise<string | null> {
        return JSON.stringify(kit);
      },
      async write(json: string): Promise<void> {
        vscodeApi?.postMessage({ kind: "kit.save", content: json });
      },
    };
    return createJsonFileKitStore(kitAdapter);
  };

  createRoot(document.getElementById("root")!).render(
    <div className="h-screen w-screen">
      <Sketchpad kitStore={store} fileKitStoreFactory={fileKitStoreFactory} embedded />
    </div>,
  );
}

boot();

// #endregion 🛎️Entrypoint
