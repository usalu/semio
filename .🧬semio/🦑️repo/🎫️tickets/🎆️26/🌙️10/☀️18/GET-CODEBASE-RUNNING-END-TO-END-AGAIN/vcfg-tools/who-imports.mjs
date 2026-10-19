import { registerHooks } from "node:module";
const needle = process.env.NEEDLE;
registerHooks({ resolve(specifier, context, next) { const r = next(specifier, context); if (r.url.includes(needle)) console.log("[DEBUG] attrs", JSON.stringify(context.importAttributes), "of", decodeURIComponent(r.url).split("/").slice(-4).join("/"), "from", decodeURIComponent(context.parentURL ?? "").split("/").slice(-5).join("/")); return r; } });
