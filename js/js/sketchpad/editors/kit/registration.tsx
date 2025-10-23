// #region Header

// registration.tsx

// 2025 Ueli Saluz

// #endregion

import { Info, MessageCircle, Settings } from "lucide-react";
import { KitScopeProvider } from "../../kits/store";
import { editorRegistry } from "../registry";
import KitEditor from "./Editor";

editorRegistry.register({
  id: "kit",
  component: KitEditor,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
  ],
  getPanels: (t) => [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  matchesPath: (pathParts) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 2 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]);
  },
  order: 10,
});
