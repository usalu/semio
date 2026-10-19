import { pathToFileURL } from "node:url";
import { resolve } from "node:path";
const [file, command = "serve", mode = "development"] = process.argv.slice(2);
const t0 = Date.now();
const m = await import(pathToFileURL(resolve(file)).href);
const c = typeof m.default === "function" ? await m.default({ command, mode, isSsrBuild: false, isPreview: false }) : await m.default;
const names = (c.plugins ?? []).flat(Infinity).map((p) => p?.name);
console.log("[DEBUG] loaded", file.split("/").slice(-4).join("/"), command, "keys", Object.keys(c).length, "plugins", names.length, "ms", Date.now() - t0);
console.log("[DEBUG] plugins", names.join(","));
