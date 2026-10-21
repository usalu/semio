import { createRequire } from "node:module";
import { join } from "node:path";
import { RuntimeCapabilityMissingV1 } from "../../🚫️missing/🟦️.ts";
import type { RuntimeBundleOptionsV1, RuntimeBundleResultV1 } from "../../📋️contract/🟦️.ts";

type EsbuildV1 = { build(options: Record<string, unknown>): Promise<{ errors: { text: string }[]; warnings: { text: string }[]; outputFiles?: { path: string; text: string; contents: Uint8Array }[]; metafile?: { inputs: Record<string, unknown> } }> };

function locate(root: string | undefined): EsbuildV1 {
  const explicit = process.env.SEMIO_RUNTIME_ESBUILD;
  const bases = [...(explicit === undefined ? [] : [explicit]), join(root ?? process.cwd(), "noop.js"), join(process.cwd(), "noop.js")];
  for (const base of bases) { try { return createRequire(base)(explicit === base ? base : "esbuild") as EsbuildV1; } catch { /* try the next base */ } }
  throw new RuntimeCapabilityMissingV1("bundle", "node", "the esbuild provider is not installed; set SEMIO_RUNTIME_ESBUILD to its module or install it beside the workspace");
}

function adaptPlugin(plugin: NonNullable<RuntimeBundleOptionsV1["plugins"]>[number]): Record<string, unknown> {
  return {
    name: plugin.name,
    setup(build: { onResolve: (options: object, callback: (arguments_: any) => any) => void; onLoad: (options: object, callback: (arguments_: any) => any) => void }) {
      return plugin.setup({
        onResolve: (constraints, callback) => build.onResolve({ filter: new RegExp(constraints.filter.source), namespace: constraints.namespace }, arguments_ => callback({ path: arguments_.path, importer: arguments_.importer, namespace: arguments_.namespace })),
        onLoad: (constraints, callback) => build.onLoad({ filter: new RegExp(constraints.filter.source), namespace: constraints.namespace }, async arguments_ => {
          const outcome = await callback({ path: arguments_.path, namespace: arguments_.namespace, loader: arguments_.pluginData?.loader ?? "js" });
          if (outcome === undefined || outcome === null) return undefined;
          if (outcome.exports !== undefined) return { contents: `export default ${JSON.stringify(outcome.exports)};${Object.keys(outcome.exports).filter(name => /^[A-Za-z_$][\w$]*$/u.test(name) && name !== "default").map(name => `export const ${name}=${JSON.stringify((outcome.exports as Record<string, unknown>)[name])};`).join("")}`, loader: "js" };
          return { contents: outcome.contents, loader: outcome.loader === "json" ? "json" : outcome.loader === "ts" ? "ts" : "js", resolveDir: undefined };
        }),
      });
    },
  };
}

/** 📦️ Bundles through the esbuild provider, resolved lazily from the workspace so no runtime depends on it unless a bundle is asked for. */
export async function bundleWithEsbuildV1(options: RuntimeBundleOptionsV1): Promise<RuntimeBundleResultV1> {
  const esbuild = locate(options.root);
  const minify = options.minify;
  const result = await esbuild.build({
    entryPoints: [...options.entrypoints], bundle: true, absWorkingDir: options.root, write: false, outdir: options.outdir ?? options.root ?? process.cwd(),
    platform: options.target === "browser" ? "browser" : "node", format: options.format ?? "esm", define: options.define, external: options.external === undefined ? undefined : [...options.external],
    minify: minify === true || (typeof minify === "object" && minify.whitespace === true), splitting: options.splitting, treeShaking: options.treeShaking,
    entryNames: options.naming === undefined ? undefined : options.naming.replace(/\.[^.]+$/u, ""), sourcemap: options.sourcemap === undefined || options.sourcemap === "none" ? false : options.sourcemap, metafile: options.metafile,
    packages: options.packages === "external" ? "external" : undefined, plugins: options.plugins?.map(adaptPlugin), logLevel: "silent",
  }).catch((error: { errors?: { text: string }[]; message?: string }) => ({ errors: error.errors ?? [{ text: error.message ?? String(error) }], warnings: [], outputFiles: [] as { path: string; text: string; contents: Uint8Array }[], metafile: undefined }));
  const outputs = (result.outputFiles ?? []).map(file => ({ path: file.path, kind: "entry-point", text: async () => file.text, bytes: async () => file.contents }));
  if (options.outdir !== undefined && result.errors.length === 0) {
    const { mkdirSync, writeFileSync } = process.getBuiltinModule("node:fs");
    for (const file of result.outputFiles ?? []) { mkdirSync(join(file.path, ".."), { recursive: true }); writeFileSync(file.path, file.contents); }
  }
  return { success: result.errors.length === 0, metafile: result.metafile, logs: [...result.errors.map(row => ({ message: row.text, level: "error" })), ...result.warnings.map(row => ({ message: row.text, level: "warning" }))], outputs };
}
