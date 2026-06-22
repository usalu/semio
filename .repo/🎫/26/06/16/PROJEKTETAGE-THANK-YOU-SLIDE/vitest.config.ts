import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const ticketRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticketRoot, "../../../../../..");
const projektetageRoot = resolve(repoRoot, "mit-bestand/präsentation/33.projektetage");

export default defineConfig({
	root: projektetageRoot,
	resolve: {
		alias: [
			{
				find: "@framework/presentation/core",
				replacement: resolve(repoRoot, "framework/product/presentation/core/index.ts"),
			},
		],
	},
	test: {
		environment: "node",
		include: ["index.ts"],
	},
});
