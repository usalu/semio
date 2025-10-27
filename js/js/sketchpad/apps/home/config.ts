// #region Header

// config.ts

// 2025 Ueli Saluz

// #endregion

import { Info, MessageCircle, Settings } from "lucide-react";
import { AppConfig } from "../registry";
import Home from "./App";

export const config: AppConfig = {
  id: "home",
  component: Home,
  routeSegments: [],
  additionalPaths: ["kits"],
  getPanels: (t) => [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => pathParts.length === 0 || (pathParts.length === 1 && pathParts[0] === "kits"),
  order: 0,
};
