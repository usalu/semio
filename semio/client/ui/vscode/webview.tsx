// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// VS Code webview: mounts sketchpad {@link Platform} via {@link mountPlatform}; kit stores register on {@link SketchpadShellController}.

// #endregion 🧲Header

// #region 🛎️Entrypoint

// #region 🔌Adapters
import type { Kit } from "@semio/js";
import { mountPlatform } from "@framework/platform/renderer/react";
import { ensureSketchpadPlatform, getSketchpadShellController, InMemorySemioKitStore } from "@semio/sketchpad";
// #endregion 🔌Adapters

declare global {
  interface Window {
    __SEMIO_KIT_JSON__?: string;
    __SEMIO_VSCODE_API__?: { postMessage(message: unknown): void };
    __SEMIO_ON_EXTERNAL_UPDATE__?: (content: string) => void;
  }
}

void (async () => {
  await mountPlatform(ensureSketchpadPlatform);
  const controller = getSketchpadShellController();
  if (controller && typeof window !== "undefined" && window.__SEMIO_KIT_JSON__) {
    try {
      const kit = JSON.parse(window.__SEMIO_KIT_JSON__) as Kit;
      if (kit?.id) {
        controller.registerKitStore(kit.id, new InMemorySemioKitStore(kit));
      }
    } catch (err) {
      console.error("[semio.vscode.webview] kit json", err);
    }
  }
  if (typeof window !== "undefined" && window.__SEMIO_ON_EXTERNAL_UPDATE__) {
    const prior = window.__SEMIO_ON_EXTERNAL_UPDATE__;
    window.__SEMIO_ON_EXTERNAL_UPDATE__ = (content: string) => {
      prior(content);
      const ctrl = getSketchpadShellController();
      if (!ctrl) return;
      try {
        const kit = JSON.parse(content) as Kit;
        if (kit?.id) {
          const existing = ctrl.getKitStore(kit.id);
          if (existing) {
            existing.replaceKit(kit);
          } else {
            ctrl.registerKitStore(kit.id, new InMemorySemioKitStore(kit));
          }
        }
      } catch (err) {
        console.error("[semio.vscode.webview] external kit update", err);
      }
    };
  }
})().catch((err) => {
  console.error("[semio.vscode.webview]", err);
});

// #endregion 🛎️Entrypoint
