import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const plugin = resolve("✏️s/🔌️plugins/🏭️process");
const { checkRetainedRoutes } = await import(pathToFileURL(resolve(plugin, "🧪️tests/🔬️typescript-package-commands/🟦️.ts")).href);
await checkRetainedRoutes(plugin);
