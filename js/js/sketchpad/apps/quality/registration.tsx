// #region Header

// registration.tsx

// 2025 Ueli Saluz

// #endregion

import { Box, Info, MessageCircle, Settings } from "lucide-react";
import { KitScopeProvider, QualityScopeProvider } from "../../kits/store";
import { appRegistry } from "../registry";
import QualityApp from "./App";

appRegistry.register({
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
    { key: "workbench", icon: Box, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "qualities" && isUuidPattern(pathParts[3]);
  },
  order: 40,
});
