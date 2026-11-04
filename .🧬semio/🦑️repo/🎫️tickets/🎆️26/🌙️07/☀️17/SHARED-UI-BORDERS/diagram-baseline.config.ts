import { defineConfig } from "vitest/config";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const base = (await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts")).href)).default;
export default defineConfig({ ...base, test: { ...base.test, setupFiles: [...base.test.setupFiles, resolve(import.meta.dirname, "diagram-baseline-setup.ts")] } });
