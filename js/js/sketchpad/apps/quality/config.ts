// #region Header

// config.ts

// 2025 Ueli Saluz

// #endregion

import { BarChart3, Box, Hammer, Info, Layers, MessageCircle, Settings, Wrench } from "lucide-react";
import { KitScopeProvider, QualityScopeProvider } from "../../kits/store";
import { AppConfig } from "../registry";
import QualityApp from "./App";

export const config: AppConfig = {
  id: "quality",
  component: QualityApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "qualities/:quality",
      paramName: "quality",
      scopeProvider: QualityScopeProvider,
    },
  ],
  getPanels: (t) => [
    { key: "workbench", icon: Box, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.workbench.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.workbench.show.hotkey") },
    { key: "tools", icon: Wrench, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.tools.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.tools.show.hotkey") },
    { key: "toolbar", icon: Hammer, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.toolbar.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.toolbar.show.hotkey") },
    { key: "hud", icon: Layers, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.hud.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.hud.show.hotkey") },
    { key: "stats", icon: BarChart3, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.stats.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.stats.show.hotkey") },
    { key: "details", icon: Info, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.details.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.details.show.hotkey") },
    { key: "chat", icon: MessageCircle, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.chat.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.chat.show.hotkey") },
    { key: "settings", icon: Settings, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.settings.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.settings.show.hotkey") },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "qualities" && isUuidPattern(pathParts[3]);
  },
  order: 40,
};
