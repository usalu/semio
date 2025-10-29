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
    { key: "details", icon: Info, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.details.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.details.show.hotkey") },
    { key: "chat", icon: MessageCircle, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.chat.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.chat.show.hotkey") },
    { key: "settings", icon: Settings, tooltip: { labelKey: "semio.sketchpad.navbar.panelToggle.settings.show" }, hotkey: t("semio.sketchpad.navbar.panelToggle.settings.show.hotkey") },
  ],
  matchesPath: (pathParts) => pathParts.length === 0 || (pathParts.length === 1 && pathParts[0] === "kits"),
  order: 0,
};
