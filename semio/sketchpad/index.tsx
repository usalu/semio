// #region 🔖Header
// [👤semio🖱️sketchpad💻index](repo://p/u/semio/b/u/sketchpad/f/index.tsx)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the standalone sketchpad web application.

// #endregion 🔖Header

// #region 🔖Entrypoint
// [👤semio🖱️sketchpad💻index🔖entrypoint](repo://p/u/semio/b/u/sketchpad/f/index.tsx/s/Entrypoint)
// Sketchpad application entrypoint registering apps and rendering the root.
// Entrypoint MUST register all app configs before rendering the Sketchpad component.

import { createRoot } from "react-dom/client";
import { createIndexeddbPersistenceFactory, createJsonFileKitStore } from "../studio/studio";
import type { KitJsonFileAdapter } from "../studio/studio";
import { Sketchpad, designConfig, docsConfig, feedbackConfig, homeConfig, kitConfig, qualityConfig, typeConfig } from "./index";
import "./globals.css";

import { appRegistry } from "./sketchpad/Sketchpad";

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(qualityConfig);
appRegistry.register(typeConfig);

// #region 🔖VscodeAdapter
// VS Code webview adapter for JsonFileKitStore. Bridges file I/O via postMessage.
const isVscodeWebview = typeof (window as any).__SEMIO_VSCODE_API__ !== "undefined";

function createVscodeAdapter(): KitJsonFileAdapter {
  const vscodeApi = (window as any).__SEMIO_VSCODE_API__;
  return {
    read: async () => (window as any).__SEMIO_KIT_JSON__ ?? null,
    write: async (json: string) => {
      vscodeApi.postMessage({ kind: "kit.save", content: json });
    },
  };
}
// #endregion 🔖VscodeAdapter

async function boot() {
  let kitStore = undefined;
  if (isVscodeWebview) {
    const adapter = createVscodeAdapter();
    kitStore = await createJsonFileKitStore(adapter);
    // Listen for external updates from the VS Code extension host.
    (window as any).__SEMIO_ON_EXTERNAL_UPDATE__ = (json: string) => {
      try {
        const parsed = JSON.parse(json);
        const { KitSchema } = require("@semio/js/semio");
        const kit = KitSchema.parse(parsed);
        kitStore!.applyExternalUpdate(kit);
      } catch {
        /* ignore parse errors */
      }
    };
    // Auto-save on changes for VS Code integration.
    kitStore.subscribe(() => {
      const snapshot = kitStore!.getSnapshot();
      if (snapshot.sync.dirty) {
        kitStore!.save();
      }
    });
  }

  const indexeddbPersistenceFactory = isVscodeWebview ? undefined : createIndexeddbPersistenceFactory();

  createRoot(document.getElementById("root")!).render(
    <div className="h-screen w-screen">
      <Sketchpad persistenceFactory={indexeddbPersistenceFactory} kitStore={kitStore} />
    </div>,
  );
}

boot();
// #endregion 🔖Entrypoint
