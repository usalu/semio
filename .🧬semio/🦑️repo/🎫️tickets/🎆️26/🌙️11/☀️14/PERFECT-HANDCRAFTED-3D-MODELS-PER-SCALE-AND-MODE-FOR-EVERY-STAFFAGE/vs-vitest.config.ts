import { defineConfig } from "vitest/config";
export default defineConfig({ root: "/Users/ueli/Documents/semio", test: { include: [process.env.VS_TEST ?? "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️volumes-rules-b/🟦️.ts"], testTimeout: 900000, hookTimeout: 900000 } });
