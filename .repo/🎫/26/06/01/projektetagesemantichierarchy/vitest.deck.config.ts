import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const deckRoot = resolve(
	dirname(fileURLToPath(import.meta.url)),
	"../../../../../../mit-bestand/präsentation/33.projektetage",
);
const repoRoot = resolve(deckRoot, "../../../");

export default defineConfig({
	root: deckRoot,
	resolve: {
		alias: {
			"@framework/presentation/core": resolve(repoRoot, "framework/product/presentation/core/index.ts"),
			"@framework/presentation/renderer/react": resolve(
				repoRoot,
				"framework/product/presentation/renderer/react/index.tsx",
			),
			"@framework/core": resolve(repoRoot, "framework/core/index.ts"),
			"@ui/react": resolve(repoRoot, "ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
