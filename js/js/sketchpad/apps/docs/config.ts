// #region Header

// config.ts

// 2025 Ueli Saluz

// #endregion

import { BookOpen, Info, Settings } from "lucide-react";
import { AppConfig } from "../registry";
import DocsApp from "./App";

export const config: AppConfig = {
  id: "docs",
  component: DocsApp,
  routeSegments: [{ path: "docs" }, { path: "*" }],
  getPanels: (t) => [
    {
      key: "workbench",
      icon: BookOpen,
      tooltip: {
        labelKey: "semio.sketchpad.navbar.panelToggle.workbench.show",
        manualPath: "/docs/manuals/sketchpad#workbench",
      },
      hotkey: t("semio.sketchpad.navbar.panelToggle.workbench.show.hotkey"),
    },
    {
      key: "details",
      icon: Info,
      tooltip: {
        labelKey: "semio.sketchpad.navbar.panelToggle.details.show",
        manualPath: "/docs/manuals/sketchpad#details",
      },
      hotkey: t("semio.sketchpad.navbar.panelToggle.details.show.hotkey"),
    },
    {
      key: "settings",
      icon: Settings,
      tooltip: {
        labelKey: "semio.sketchpad.navbar.panelToggle.settings.show",
        manualPath: "/docs/manuals/sketchpad#settings",
      },
      hotkey: t("semio.sketchpad.navbar.panelToggle.settings.show.hotkey"),
    },
  ],
  matchesPath: (pathParts) => pathParts.length > 0 && pathParts[0] === "docs",
  order: 5,
};
