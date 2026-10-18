import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const STORE = "/Users/ueli/Documents/semio/node_modules/.bun";

/** 🧪 Emulates the effect of declaring packages at the repository root: maps each bare name to its dependency-store directory (diagnostic only, never shipped). */
export function storeAliases(names: Record<string, string>) {
  const entries = readdirSync(STORE);
  return Object.entries(names).map(([name, version]) => {
    const prefix = name.replace("/", "+") + "@" + version;
    const hit = entries.filter(entry => entry === prefix || entry.startsWith(prefix + "+")).sort()[0];
    const directory = hit ? join(STORE, hit, "node_modules", name) : "";
    if (!existsSync(directory)) throw Error("Store entry missing for " + name + "@" + version);
    return { find: new RegExp("^" + name.replace(/[/@.]/g, "\\$&") + "(?=/|$)"), replacement: directory };
  });
}
