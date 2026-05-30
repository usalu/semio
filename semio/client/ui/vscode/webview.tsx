// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// VS Code webview: mounts sketchpad {@link Platform} and attaches the injected kit JSON.

// #endregion 🧲Header

// #region 🛎️Entrypoint

// #region 🔌Adapters
import { mountPlatform } from "@framework/platform/renderer/react";
import { Kit, KitFullDtoSchema, asKitInstance } from "@semio/react";
import { attachSketchpadKit, ensureSketchpadPlatform, InMemorySemioKitStore } from "../../lib/sketchpad/js";
// #endregion 🔌Adapters

declare global {
  interface Window {
    __SEMIO_KIT_JSON__?: string;
    __SEMIO_VSCODE_API__?: { postMessage(message: unknown): void };
    __SEMIO_ON_EXTERNAL_UPDATE__?: (content: string) => void;
  }
}

void (async () => {
  const platform = await ensureSketchpadPlatform();
  const raw = window.__SEMIO_KIT_JSON__;
  if (raw != null) {
    const parsed = typeof raw === "string" ? JSON.parse(raw) : raw;
    const kit = asKitInstance(Kit.fromPlain(KitFullDtoSchema.parse(parsed)));
    attachSketchpadKit(kit.id, new InMemorySemioKitStore(kit), { kind: "file", navigate: true });
  }
  await mountPlatform(() => Promise.resolve(platform));
})().catch((err) => {
  console.error("[semio.vscode.webview]", err);
});

// #endregion 🛎️Entrypoint
