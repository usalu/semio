// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// VS Code webview: mounts sketchpad {@link Platform} and attaches the injected kit JSON.

// #endregion 🧲Header

// #region 🛎️Entrypoint

// #region 🔌Adapters
import { mountPlatform } from "@framework/platform/renderer/react";
import { Kit, KitFullDtoSchema, asKitInstance } from "@compose/react";
import { attachSketchpadKit, ensureSketchpadPlatform, InMemoryComposeKitStore } from "../../lib/sketchpad/js";
// #endregion 🔌Adapters

declare global {
  interface Window {
    __COMPOSE_KIT_JSON__?: string;
    __COMPOSE_VSCODE_API__?: { postMessage(message: unknown): void };
    __COMPOSE_ON_EXTERNAL_UPDATE__?: (content: string) => void;
  }
}

void (async () => {
  const platform = await ensureSketchpadPlatform();
  const raw = window.__COMPOSE_KIT_JSON__;
  if (raw != null) {
    const parsed = typeof raw === "string" ? JSON.parse(raw) : raw;
    const kit = asKitInstance(Kit.fromPlain(KitFullDtoSchema.parse(parsed)));
    attachSketchpadKit(kit.id, new InMemoryComposeKitStore(kit), { kind: "file", navigate: true });
  }
  await mountPlatform(() => Promise.resolve(platform));
})().catch((err) => {
  console.error("[compose.vscode.webview]", err);
});

// #endregion 🛎️Entrypoint
