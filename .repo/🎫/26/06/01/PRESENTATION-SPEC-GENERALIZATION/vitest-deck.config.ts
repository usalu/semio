import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const ticketRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticketRoot, "../../../../../../");
const deckRoot = resolve(repoRoot, "mit-bestand/präsentation/33.projektetage");

export default defineConfig({
	root: deckRoot,
	resolve: {
		alias: {
			"@framework/presentation/core": resolve(repoRoot, "framework/product/presentation/core/index.ts"),
			"@framework/presentation/renderer/react": resolve(repoRoot, "framework/product/presentation/renderer/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
