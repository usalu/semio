import assert from "node:assert/strict";
import { resolveConfig } from "vite";
import { resolve } from "node:path";
const config = await resolveConfig({ configFile: resolve("♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts"), configLoader: "bundle", root: resolve("♻️mit-bestand/🧺️demonstrator") }, "build");
const names = config.plugins.map(row => row.name);
assert.ok(names.includes("semio-browser-artifacts"));
assert.ok(!names.some(name => /static-dir-build.*(?:plugin-modules|extension-modules)/.test(name)));
assert.ok(!names.includes("semio-plugin-hot-swap"));
console.log("[DEBUG] Actual Demonstrator Vite production config loads with the immutable artifact copy plugin PASS");
