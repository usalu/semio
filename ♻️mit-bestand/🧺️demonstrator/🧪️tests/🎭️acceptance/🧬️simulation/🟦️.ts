import type { Page } from "@playwright/test";

/** 🆕️ Requires a real distinct group before targeting a restarted tool run. */
export function isNewToolRunGroup(previous: string, candidate: string): boolean {
  return /^framework\.toolRun\.[0-9]+$/.test(candidate) && candidate !== previous;
}

/** 🪪️ Reads the newest run from the status keys displayed in the Tool Runs panel. */
export async function latestToolRunGroup(page: Page, paneId: string): Promise<string> {
  const statuses = page.locator(`[data-shell-id="${paneId}"] [data-ui-node-key^="framework.toolRun."][data-ui-node-key$=".status"]`);
  return statuses.evaluateAll((elements) => {
    let latest = 0n;
    for (const element of elements) {
      const match = /^framework\.toolRun\.(\d+)\.status$/.exec(element.getAttribute("data-ui-node-key") ?? "");
      if (match && BigInt(match[1]!) > latest) latest = BigInt(match[1]!);
    }
    return latest ? `framework.toolRun.${latest}` : "";
  });
}
