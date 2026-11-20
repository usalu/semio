import { repoPlaywrightBrowsersPath } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoPlaywrightBrowsersPath(process.cwd());

export const { chromium, expect } = await import("@playwright/test");
