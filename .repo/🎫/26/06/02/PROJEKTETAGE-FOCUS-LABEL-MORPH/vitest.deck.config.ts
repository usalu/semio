import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const deckRoot = resolve(
	dirname(fileURLToPath(import.meta.url)),
	"../../../../../../mit-bestand/präsentation/33.projektetage",
);

export default defineConfig({
	root: deckRoot,
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
