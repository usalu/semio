import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest entry for `@elements/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	test: {
		environment: "jsdom",
		include: ["js/index.ts", "react/index.tsx"],
		includeSource: ["js/index.ts", "react/index.tsx", "react/reconciler-host.ts"],
		passWithNoTests: true,
	},
});
