import { fileURLToPath } from "node:url";
import { registerIsolatedDependencies, run, serve } from "./isolated-browser-runner.ts";

const local = (path) => fileURLToPath(new URL(path, import.meta.url));
const scratch = local("./🗑️generated/w");
const [action, ...args] = process.argv.slice(2);
if (action === "serve") await serve(scratch, local("./staffage-isolated-vite.ts"), 6271);
else if (action === "test") {
  process.env.STAFFAGE_SCRATCH = scratch;
  await run(scratch, local("./staffage-isolated-vitest.ts"));
} else if (action === "browser" || action === "stage") {
  process.argv.splice(2, process.argv.length - 2, ...args);
  const deregister = registerIsolatedDependencies(scratch);
  try {
    const { chromium, webkit } = await import("playwright");
    const browser = action === "browser" && process.env.STAFFAGE_BROWSER_ENGINE === "webkit"
      ? await webkit.launch()
      : await chromium.launch(process.env.STAFFAGE_CHROME_HEADLESS_SHELL === "true" ? {} : { channel: "chromium" });
    try {
      globalThis.staffagePhoneBrowser = browser;
      const server = await serve(scratch, local("./staffage-isolated-vite.ts"), 6271, true);
      try {
        const { routeViteRequests } = await import("./isolated-vite-route.ts");
        globalThis.routeTicketBrowser = (target, origin) => routeViteRequests(server, target, origin);
        await run(scratch, local(`./staffage-phone-${action}.mjs`));
      } finally {
        await server.close();
      }
    } finally {
      await browser.close();
    }
  } finally {
    deregister();
  }
} else if (action === "catalog") {
  const { summarizeCatalog } = await run(scratch, local("./staffage-catalog-check.ts"));
  const catalog = local("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
  const result = summarizeCatalog(catalog);
  console.log("[DEBUG]", JSON.stringify(result));
  if (result.failures.length > 0) process.exitCode = 1;
} else throw new Error("Expected serve, test, browser, stage or catalog");
