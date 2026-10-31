import path from "node:path";
import schema from "./browser-output-paths.schema.json" with { type: "json" };
/** 📁️ Plans bounded ticket outputs before filesystem or browser effects. */
export function planBrowserOutputPaths(ticket, rawName, children, paths = path) {
  const runName = rawName.replace(/[^a-zA-Z0-9_.-]/g, "-");
  if (!runName || runName === "." || runName === "..") throw Error("Invalid browser output name");
  const outDir = paths.join(ticket, "🗑️generated", runName);
  const outputs = [outDir, ...children.map(child => paths.join(outDir, child))];
  if (outputs.some(output => output.length > schema.properties.units.items.maximum)) throw Error("Browser output path exceeds 256 UTF-16 units");
  return { runName, outDir };
}
