/** 🔌️ W14 preload: rewrites `node:fs` imports of the build library modules to the counting shim, so calls through `import * as fs` are seen (a patched `fs` object is not). */
const shim = (await import("node:url")).fileURLToPath(new URL("./fs-shim.mjs", import.meta.url));
Bun.plugin({ name: "w14-fs", setup(build) { build.onLoad({ filter: /📚️library\/🏗️build\/.*\.(mjs|ts)$/ }, async ({ path }) => { const text = await Bun.file(path).text(); return { contents: text.replace(/from\s*["']node:fs["']/g, `from ${JSON.stringify(shim)}`), loader: path.endsWith(".ts") ? "ts" : "js" }; }); } });
