import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
const [cfg, loader, root] = process.argv.slice(2);
const require = createRequire(root + "/package.json");
const vite = await import(pathToFileURL(require.resolve("vite")).href);
try {
  const r = await vite.loadConfigFromFile({ command: "build", mode: "prototype" }, cfg, root, "silent", undefined, loader);
  console.log("[DEBUG] loaded", loader, Object.keys(r.config).join(","));
} catch (e) { console.log("[DEBUG] FAILED", loader, String(e).split("\n")[0]); }
