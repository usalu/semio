import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const reactDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/vcs-core": path.resolve(reactDir, "../core/index.ts"),
			"@semio-tech/ui-react": path.resolve(reactDir, "../../ui/react/index.tsx"),
		},
	},
	test: {
		include: ["index.tsx"],
		environment: "jsdom",
	},
});
