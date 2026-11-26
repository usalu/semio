import type { Locator, Page } from "@playwright/test";

/** 🎛️ Finds an authored control in its panel, including controls nested in a virtual row. */
export function panelControl(page: Page, paneId: string, panelId: string, key: string): Locator {
  return page.locator(`[data-shell-id="${paneId}"] [id^="panel:${panelId}/"][id$="/${key}"]`);
}

/** 🗺️ Reads the numeric count from a localized GIS inspector row. */
export async function gisInspectorCount(page: Page, kind: "positions" | "routes" | "regions"): Promise<number> {
  const row = panelControl(page, "verfolgen", "framework.panel.inspection", `gis2d-play-inspector.${kind}-count`);
  const text = await row.innerText();
  const match = text.trim().match(/(?:^|\s)(\d+)$/u);
  if (!match) throw new Error(`Missing ${kind} count in inspector row: ${text}`);
  return Number(match[1]);
}
