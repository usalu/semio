// #region Header

// config.ts

// 2025 Ueli Saluz

// #endregion

import { BarChart3, Box, Hammer, Info, Layers, MessageCircle, Settings, Wrench } from "lucide-react";
import { KitScopeProvider, TypeScopeProvider } from "../../kits/store";
import { AppConfig } from "../registry";
import TypeApp from "./App";

export const config: AppConfig = {
  id: "type",
  component: TypeApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "types/:type",
      paramName: "type",
      scopeProvider: TypeScopeProvider,
    },
  ],
  getPanels: (t) => [
    { key: "workbench", icon: Box, tooltip: t("semio.panels.workbench"), hotkey: "⌘J" },
    { key: "tools", icon: Wrench, tooltip: t("semio.panels.tools"), hotkey: "⌘U" },
    { key: "toolbar", icon: Hammer, tooltip: t("semio.panels.toolbar"), hotkey: "⌘K" },
    { key: "hud", icon: Layers, tooltip: t("semio.panels.hud"), hotkey: "⌘H" },
    { key: "stats", icon: BarChart3, tooltip: t("semio.panels.stats"), hotkey: "⌘I" },
    { key: "details", icon: Info, tooltip: t("semio.panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("semio.panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("semio.panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "types" && isUuidPattern(pathParts[3]);
  },
  order: 30,
};
