import { registerHooks, stripTypeScriptTypes } from "node:module";
import { fileURLToPath } from "node:url";

registerHooks({
  load(url, context, nextLoad) {
    const loaded = nextLoad(url, context);
    if (!url.startsWith("file:") || !loaded.format?.includes("typescript") || !loaded.source) return loaded;
    try {
      stripTypeScriptTypes(Buffer.from(loaded.source).toString("utf8"), { mode: "strip", sourceUrl: url });
    } catch (error) {
      process.stderr.write(`[DEBUG] ${fileURLToPath(url)}: ${error.message}\n`);
    }
    return loaded;
  },
});
