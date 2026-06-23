import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const dir = resolve(ticketDir, "../../../../../../mit-bestand/präsentation/33.projektetage");
const repoRoot = resolve(dir, "../../../");

/** @emoji 🧪 Temporary vitest config for projektetage deck smoke. */
export default defineConfig({
	root: dir,
	resolve: {
		alias: [
			{ find: "@semio-tech/framework-presentation-core", replacement: resolve(repoRoot, "framework/product/presentation/core/index.ts") },
			{ find: "@semio-tech/framework-presentation-renderer-react", replacement: resolve(repoRoot, "framework/product/presentation/renderer/react/index.tsx") },
			{ find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "framework/core/index.ts") },
			{ find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "ui/react/index.tsx") },
			{ find: "@semio-tech/mit-bestand-praesentation-projektetage-spec", replacement: resolve(dir, "spec.ts") },
		],
	},
	test: {
		environment: "node",
		include: ["index.ts"],
	},
});
