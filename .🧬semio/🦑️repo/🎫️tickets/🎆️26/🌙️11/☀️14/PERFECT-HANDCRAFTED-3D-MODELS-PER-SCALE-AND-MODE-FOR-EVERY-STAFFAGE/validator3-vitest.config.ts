import { defineConfig } from "vitest/config";
export default defineConfig({ root: "/Users/ueli/Documents/semio", test: { include: [process.env.VALIDATOR_TEST ?? "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️volumes-rules/🟦️.ts"], testTimeout: 600000, hookTimeout: 600000 } });
