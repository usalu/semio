// #region Header

// registration.tsx

// 2025 Ueli Saluz

// #endregion

import { MessageCircle, Settings } from "lucide-react";
import { editorRegistry } from "../registry";
import Home from "./Editor";

editorRegistry.register({
  id: "home",
  component: Home,
  routeSegments: [],
  additionalPaths: ["kits"],
  getPanels: (t) => [
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => pathParts.length === 0 || (pathParts.length === 1 && pathParts[0] === "kits"),
  order: 0,
});
