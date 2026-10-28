import { defineConfig } from "vitest/config";
import { fileURLToPath } from "node:url";
export default defineConfig({cacheDir:fileURLToPath(new URL("./🗑️generated/vitest",import.meta.url)),test:{root:fileURLToPath(new URL("../../../../../../../",import.meta.url)),include:["🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🟦️.ts"],environment:"node"}});
