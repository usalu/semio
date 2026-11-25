import { defineConfig } from "vitest/config";
export default defineConfig({ test: { root: "/Users/ueli/Documents/semio", include: (process.env.VITEST_FILES ?? "").split("|").filter(Boolean), environment: (process.env.VITEST_ENV ?? "node") as "node" | "jsdom" } });
