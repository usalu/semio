import type { BrowserContext } from "@playwright/test";
import { readGeneratedCatalogProjection } from "@semio-tech/plugin-registry/catalog-view";
import { buildPlaygroundSession } from "@semio-tech/plugin-registry/playground/session";
import { repoPlaywrightBrowsersPath } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoPlaywrightBrowsersPath(process.cwd());

export const { chromium, expect } = await import("@playwright/test");

export const processPluginIds = ["process", "process-extension-concrete", "process-extension-metal", "process-extension-robotic", "process-extension-wood"] as const;

export function observeProcessPluginDelivery(context: BrowserContext, target: URL): () => string[] {
  const projection = readGeneratedCatalogProjection("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
  const { plugins } = buildPlaygroundSession("process3d", projection);
  const bridgeKey = (url: URL) => url.origin + url.pathname;
  const bridges = new Map(plugins.map(({ moduleUrl, pluginId }) => [bridgeKey(new URL(moduleUrl, target)), pluginId]));
  const delivered = new Set<string>();
  context.on("response", response => {
    const pluginId = bridges.get(bridgeKey(new URL(response.url())));
    if (pluginId && response.ok()) delivered.add(pluginId);
  });
  return () => [...delivered].sort();
}
