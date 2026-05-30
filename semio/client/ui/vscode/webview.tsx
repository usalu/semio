// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// VS Code webview: mounts sketchpad {@link Platform} via {@link mountPlatform}; kit host wires registry separately.

// #endregion 🧲Header

// #region 🛎️Entrypoint

// #region 🔌Adapters
import { mountPlatform } from "@framework/platform/renderer/react";
import { ensureSketchpadPlatform, setSketchpadKitRegistryBridge } from "@semio/sketchpad";
import { getKitRegistryBridge as getSemioReactKitRegistryBridge } from "@semio/react";
// #endregion 🔌Adapters

declare global {
  interface Window {
    __SEMIO_KIT_JSON__?: string;
    __SEMIO_VSCODE_API__?: { postMessage(message: unknown): void };
    __SEMIO_ON_EXTERNAL_UPDATE__?: (content: string) => void;
  }
}

void (async () => {
  const reg = getSemioReactKitRegistryBridge();
  if (reg) {
    setSketchpadKitRegistryBridge(reg as Parameters<typeof setSketchpadKitRegistryBridge>[0]);
  }
  await mountPlatform(ensureSketchpadPlatform);
})().catch((err) => {
  console.error("[semio.vscode.webview]", err);
});

// #endregion 🛎️Entrypoint
