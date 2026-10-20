import { realpathSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";

function canonical(path: string): string {
  const real = pathToFileURL(realpathSync(path)).href;
  return process.platform === "win32" ? real.toLowerCase() : real;
}

/** 🏁️ Tells whether the module at `metaUrl` is the process entry point, comparing real paths so symlinked launches agree. */
export function isMainModuleV1(metaUrl: string): boolean {
  const entry = process.argv[1];
  if (entry === undefined || entry === "") return false;
  try { return canonical(entry) === canonical(fileURLToPath(metaUrl)); } catch { return false; }
}
