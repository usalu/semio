// #region Header

// config.ts

// 2025 Ueli Saluz

// #endregion

import { Info, MessageCircle, Settings } from "lucide-react";
import { KitScopeProvider } from "../../kits/store";
import { AppConfig } from "../registry";
import KitApp from "./App";

export const config: AppConfig = {
  id: "kit",
  component: KitApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
  ],
  getPanels: (t) => [
    { key: "details", icon: Info, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.details.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.details.show.hotkey") },
    { key: "chat", icon: MessageCircle, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.chat.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.chat.show.hotkey") },
    { key: "settings", icon: Settings, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.settings.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.settings.show.hotkey") },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 2 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]);
  },
  order: 10,
};
