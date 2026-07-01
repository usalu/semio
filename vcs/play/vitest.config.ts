import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const playDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/vcs-react": path.resolve(playDir, "../react/index.tsx"),
			"@semio-tech/vcs-core": path.resolve(playDir, "../core/index.ts"),
		},
	},
	test: {
		include: ["index.ts", "index.ts"],
		environment: "jsdom",
	},
});
