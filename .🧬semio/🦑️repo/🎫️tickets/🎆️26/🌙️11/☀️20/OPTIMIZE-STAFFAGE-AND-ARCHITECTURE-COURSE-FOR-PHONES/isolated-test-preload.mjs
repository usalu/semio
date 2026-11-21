import { fileURLToPath } from "node:url";
import { registerIsolatedDependencies } from "./isolated-browser-runner.ts";

registerIsolatedDependencies(fileURLToPath(new URL("./🗑️generated/w", import.meta.url)));
