import { readFileSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createRequire } from "node:module";
import { dirname, resolve as resolvePath } from "node:path";

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(resolvePath(__dirname, "package.json"));
const esbuild = require("esbuild");

const runEmbeddedTests = process.env.SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS === "1";
const repoRoot = resolvePath(__dirname, "../../../../../");

/** @emoji 🔀 Playwright Node loader aliases (subset of sketchpad `vite.config.ts`). */
const RESOLVE_ALIASES = [
	["@semio/js", resolvePath(__dirname, "../../js/index.ts")],
	["@semio/rs-wasm", resolvePath(__dirname, "../../rs/pkg/semio.js")],
	["@semio/sketchpad", resolvePath(__dirname, "index.ts")],
	["@framework/core", resolvePath(repoRoot, "framework/core/index.ts")],
	["@framework/platform/core", resolvePath(repoRoot, "framework/product/platform/core/index.ts")],
	["@framework/platform/renderer/react", resolvePath(repoRoot, "framework/product/platform/renderer/react/index.tsx")],
	["@framework/playground/core", resolvePath(repoRoot, "framework/product/playground/core/index.ts")],
	["@framework/playground/renderer/react/puzzle/2d", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@framework/playground/renderer/react/puzzle/3d", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@framework/playground/renderer/react/puzzle/5d", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@framework/playground/renderer/react/shell", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@framework/playground/renderer/react/boot", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@framework/playground/renderer/react", resolvePath(repoRoot, "framework/product/playground/renderer/react/index.tsx")],
	["@ui/react", resolvePath(repoRoot, "ui/react/index.tsx")],
	["@infinite/cavas/react-renderer", resolvePath(repoRoot, "infinite/cavas/react-renderer/index.tsx")],
	["@infinite/world/r3f", resolvePath(repoRoot, "infinite/world/r3f/index.tsx")],
	["@gis/map/play", resolvePath(repoRoot, "gis/map/play/index.ts")],
	["@gis/map/react", resolvePath(repoRoot, "gis/map/react/index.tsx")],
	["@puzzle/2d/react", resolvePath(repoRoot, "puzzle/2d/react/index.tsx")],
	["@puzzle/3d/react", resolvePath(repoRoot, "puzzle/3d/react/index.tsx")],
	["@puzzle/5d/react", resolvePath(repoRoot, "puzzle/5d/react/index.tsx")],
];

function resolveAlias(specifier) {
	for (const [find, replacement] of RESOLVE_ALIASES) {
		if (specifier === find) return replacement;
	}
	return null;
}

function fileUrlPath(url) {
	return fileURLToPath(url.split("?")[0]);
}

function shouldEsbuildTransform(url) {
	const filePath = fileUrlPath(url).replace(/\\/g, "/");
	if (filePath.includes("/node_modules/")) return false;
	if (!/\.(tsx?|mts|cts)$/.test(filePath)) return false;
	return (
		filePath.includes("/semio/") ||
		filePath.includes("/framework/") ||
		filePath.includes("/puzzle/") ||
		filePath.includes("/ui/") ||
		filePath.includes("/cad/") ||
		filePath.includes("/elements/")
	);
}

export async function resolve(specifier, context, nextResolve) {
	const aliased = resolveAlias(specifier);
	if (aliased) {
		return { url: pathToFileURL(aliased).href, shortCircuit: true };
	}
	let result;
	try {
		result = await nextResolve(specifier, context);
	} catch (e) {
		if (specifier.endsWith(".css")) {
			return { url: "data:text/javascript,export default {}", format: "module", shortCircuit: true };
		}
		throw e;
	}
	if (result.url.endsWith(".css")) {
		return { ...result, format: "css-noop", shortCircuit: true };
	}
	return result;
}

function stubViteGlob(source) {
	let result = "";
	let i = 0;
	const needle = "import.meta.glob";
	while (i < source.length) {
		const idx = source.indexOf(needle, i);
		if (idx === -1) {
			result += source.slice(i);
			break;
		}
		result += source.slice(i, idx) + "((() => ({}))";
		i = idx + needle.length;
		if (source[i] === "<") {
			let depth = 1;
			i++;
			while (i < source.length && depth > 0) {
				if (source[i] === "<") depth++;
				else if (source[i] === ">") depth--;
				i++;
			}
		}
		if (source[i] === "(") {
			let depth = 1;
			i++;
			while (i < source.length && depth > 0) {
				if (source[i] === "(") depth++;
				else if (source[i] === ")") depth--;
				i++;
			}
		}
		result += ")";
	}
	return result;
}

function transformTypeScript(url, source) {
	source = stubViteGlob(source);
	const loader = url.endsWith(".tsx") ? "tsx" : "ts";
	return esbuild.transformSync(source, {
		loader,
		format: "esm",
		target: "esnext",
		sourcemap: false,
		jsx: "automatic",
		define: {
			__SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__: runEmbeddedTests ? "true" : "false",
			__SEMIO_JS_RUN_BENCHMARKS__: "false",
			__SEMIO_JS_RUN_EMBEDDED_TESTS__: "false",
		},
	}).code;
}

export async function load(url, context, nextLoad) {
	if (url.endsWith(".css") || context.format === "css-noop") {
		return { format: "module", shortCircuit: true, source: "export default {}" };
	}
	if (url.endsWith(".json") && !context.importAttributes?.type) {
		const json = readFileSync(fileUrlPath(url), "utf8");
		return { format: "module", shortCircuit: true, source: `export default ${json}` };
	}
	if (shouldEsbuildTransform(url)) {
		const source = readFileSync(fileUrlPath(url), "utf8");
		return { format: "module", shortCircuit: true, source: transformTypeScript(url, source) };
	}
	return nextLoad(url, context);
}
