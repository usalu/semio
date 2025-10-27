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

export interface EditorPanels {
  panels: PanelConfig[];
}

export async function loadEditorPanels(editorId: string): Promise<PanelConfig[]> {
  try {
    const module = await import(`./${editorId}/panels.ts`);
    if (module && module.panels) {
      return module.panels;
    }
  } catch (e) {
  }
  return [];
}
