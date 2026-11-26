import { createHash } from "node:crypto";
import { constants } from "node:fs";
import { copyFile, link, lstat, mkdir, readFile, readdir, realpath, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { build, type InlineConfig } from "vite";
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";
import { stageRendererSupport } from "./renderer-diagnostic-assets.ts";
import createConfig from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts";

const [baselineInput, outputInput, inspect] = process.argv.slice(2);
if (!baselineInput || !outputInput || (inspect !== undefined && inspect !== "--inspect")) {
  throw new Error("Usage: build-generator-diagnostic.ts captured-site new-ticket-output [--inspect]");
}
const baseline = await realpath(baselineInput);
const generated = await realpath(join(import.meta.dirname, "🗑️generated"));
const output = resolve(outputInput);
const privatePath = relative(generated, output);
if (!privatePath || isAbsolute(privatePath) || privatePath === ".." || privatePath.startsWith(".." + sep) || await realpath(dirname(output)) !== dirname(output)) {
  throw new Error("Diagnostic output must be a new generated ticket directory");
}
const moduleDirectories = ["🔌️plugin-modules", "🧩️extension-modules"];
for (const directory of moduleDirectories) {
  if (!(await lstat(join(baseline, directory))).isDirectory()) throw new Error("Require prior captured browser actors");
}
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const shellHost = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost";
const rendererFiles = ["⏳️document-load-gate/🟦️.ts", "⚡️host-effects/🟦️.ts", "⚡️host-effects/🔁️dispatch-media-and-extension-effects/🟦️.ts"].map(file => join(shellHost, file));
async function rendererSource() {
  return Object.fromEntries(await Promise.all(rendererFiles.map(async file => [file, createHash("sha256").update(await readFile(join(workspace, file))).digest("hex")])));
}
const renderer = await rendererSource();
const products = repositoryCachePartitionRootDataV1(workspace, "products");
process.env.SEMIO_VITE_CACHE_DIRECTORY = join(products, "vite", "generator-private");
const config = await createConfig({ command: "build", mode: "release" });
const plugins = config.plugins?.flat() ?? [];
const pluginName = "semio-browser-artifacts";
const retained = plugins.filter(plugin => plugin?.name !== pluginName);
if (retained.length + 1 !== plugins.length) throw new Error("Require one exact production artifact copy stage");
console.log(JSON.stringify({ diagnosticOnly: true, baseline, output, omittedStage: pluginName, retainedPlugins: retained.length, renderer }));
if (inspect) process.exit(0);

await mkdir(output);
await build({
  ...config,
  configFile: false,
  mode: "release",
  cacheDir: join(generated, "fc"),
  plugins: retained,
  build: { ...config.build, emptyOutDir: false, minify: "esbuild", outDir: output },
} as InlineConfig);
if (JSON.stringify(await rendererSource()) !== JSON.stringify(renderer)) {
  throw new Error("Renderer source changed during private compilation");
}

async function mountTree(source: string, destination: string): Promise<void> {
  if (destination.length > 256) throw new Error("Browser diagnostic path exceeds 256 characters");
  await mkdir(destination);
  for (const entry of await readdir(source, { withFileTypes: true })) {
    const from = join(source, entry.name);
    const to = join(destination, entry.name);
    if (entry.isDirectory()) {
      await mountTree(from, to);
      continue;
    }
    if (!entry.isFile() || to.length > 256) throw new Error("Unsafe captured actor entry");
    try {
      await link(from, to);
    } catch (error) {
      if (!["EXDEV", "ENOSYS", "ENOTSUP", "EOPNOTSUPP"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error;
      await copyFile(from, to, constants.COPYFILE_EXCL);
    }
  }
}

const rendererAssets = await stageRendererSupport(output);
for (const directory of moduleDirectories) {
  await mountTree(join(baseline, directory), join(output, directory));
}
await writeFile(join(output, ".generator-frontend-diagnostic.json"), JSON.stringify({ version: 1, diagnosticOnly: true, baseline, oldActors: moduleDirectories, renderer, rendererAssets, freshGeneration: "mounted by focused verifier" }) + "\n", { flag: "wx" });
console.log("PRIVATE_DIAGNOSTIC_SITE " + output);
