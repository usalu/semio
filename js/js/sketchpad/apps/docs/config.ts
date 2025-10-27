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
        labelKey: "panels.workbench",
        manualPath: "/docs/manuals/sketchpad#workbench",
      },
      hotkey: "⌘1",
    },
    {
      key: "details",
      icon: Info,
      tooltip: {
        labelKey: "panels.details",
        manualPath: "/docs/manuals/sketchpad#details",
      },
      hotkey: "⌘2",
    },
    {
      key: "settings",
      icon: Settings,
      tooltip: {
        labelKey: "panels.settings",
        manualPath: "/docs/manuals/sketchpad#settings",
      },
      hotkey: "⌘,",
    },
  ],
  matchesPath: (pathParts) => pathParts.length > 0 && pathParts[0] === "docs",
  order: 5,
};
