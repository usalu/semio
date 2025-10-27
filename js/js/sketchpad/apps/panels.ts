// #region Header

// panels.ts

// 2025 Ueli Saluz

// #endregion

import { ReactNode } from "react";

export interface PanelConfig {
  id: string;
  key: "workbench" | "details" | "settings" | "tools" | "hud" | "stats" | "toolbar" | "chat";
  label: string;
  order?: number;
  defaultOpen?: boolean;
  content: ReactNode | (() => ReactNode);
}

export interface AppPanels {
  panels: PanelConfig[];
}

export async function loadAppPanels(appId: string): Promise<PanelConfig[]> {
  try {
    const module = await import(`./${appId}/panels.ts`);
    if (module && module.panels) {
      return module.panels;
    }
  } catch (e) {
  }
  return [];
}
