// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Specs: VS Code webview entry boots {@link Sketchpad} with kit registry wiring from `@semio/react` (no pre-instantiated host store in sketchpad).
// Persistence follows the backbone shell: factory builds Json kit store after {@link KitStoreProvider} mounts; external updates go through the active registry row.

// Webview app for the semio VS Code kit editor.

// #endregion 🧲Header

// #region 🛎️Entrypoint

import {
  asKitInstance,
  createVscodeWebviewSketchpadFileKitStoreFactory,
  getKitRegistryBridge,
  Kit,
  KitFullDtoSchema,
} from "@semio/react/host";
import { Sketchpad, appRegistry, designConfig, docsConfig, feedbackConfig, homeConfig, kitConfig, qualityConfig, typeConfig } from "@semio/sketchpad";
import { createRoot } from "react-dom/client";

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
  if (!vscodeApi) {
    throw new Error("VS Code API not available on window.");
  }

  const raw = window.__SEMIO_KIT_JSON__;
  const initialState =
    raw != null
      ? {
          kits: [
            {
              kit: asKitInstance(Kit.fromPlain(KitFullDtoSchema.parse(typeof raw === "string" ? JSON.parse(raw) : raw))),
              kind: "file" as const,
              source: { kind: "file" as const, path: "vscode-webview" },
            },
          ],
        }
      : undefined;

  window.__SEMIO_ON_EXTERNAL_UPDATE__ = (content: string) => {
    try {
      const reg = getKitRegistryBridge();
      const kid = reg?.list()?.[0];
      if (!kid) return;
      const st = reg.get(kid)?.store as { applyExternalUpdate?: (k: unknown) => void } | undefined;
      st?.applyExternalUpdate?.(KitFullDtoSchema.parse(JSON.parse(content)));
    } catch {
      /* ignore parse errors */
    }
  };

  const fileKitStoreFactory = createVscodeWebviewSketchpadFileKitStoreFactory(vscodeApi);

  createRoot(document.getElementById("root")!).render(
    <div className="h-screen w-screen">
      <Sketchpad initialState={initialState} fileKitStoreFactory={fileKitStoreFactory} embedded />
    </div>,
  );
}

void boot();

// #endregion 🛎️Entrypoint
