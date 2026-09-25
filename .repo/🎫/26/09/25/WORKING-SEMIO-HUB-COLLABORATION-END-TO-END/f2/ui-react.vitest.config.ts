import { resolve } from "node:path";
import { defineConfig } from "vitest/config";
const root = resolve(process.cwd(), "ui/react");
export default defineConfig({
	root,
	resolve: { alias: [{ find: "@ui/react", replacement: resolve(root, "index.tsx") }] },
	test: { environment: "jsdom", includeSource: ["index.tsx"], include: [], passWithNoTests: true },
});
