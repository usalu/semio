import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
export default defineConfig({ root: fileURLToPath(new URL("../../../../../../../", import.meta.url)), test: { include: [process.env.VS_TEST ?? "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️volumes-rules/🟦️.ts"], testTimeout: 900000, hookTimeout: 900000 } });
