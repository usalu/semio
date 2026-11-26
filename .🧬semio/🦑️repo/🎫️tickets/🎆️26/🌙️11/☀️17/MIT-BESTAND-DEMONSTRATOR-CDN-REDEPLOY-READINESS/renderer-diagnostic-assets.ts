import { createHash } from "node:crypto";
import { readFile, realpath } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { demonstratorRuntimeAssetSources } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import { copyBrowserArtifacts } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";

const workspace = resolve(import.meta.dirname, "../../../../../../..");
const support = () => demonstratorRuntimeAssetSources(workspace, "release").filter(source => source.destination.startsWith("renderer-modules/"));

const digest = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");

async function rendererSupportInventory(canonical?: string) {
  const sources = support();
  if (!sources.length) throw new Error("The authored configuration supplies no Renderer support");
  const files = [];
  for (const source of sources) {
    const marker = JSON.parse(await readFile(join(source.root, ".semio-artifact.json"), "utf8"));
    if (marker.version !== 1 || marker.owner !== source.owner || !Array.isArray(marker.files) || !marker.files.length) {
      throw new Error("Renderer support needs the exact authored artifact marker");
    }
    for (const name of marker.files) {
      if (typeof name !== "string" || !name || name.includes(String.fromCharCode(92)) || name.split("/").some(part => !part || part === "." || part === "..")) {
        throw new Error("Renderer support requires a safe authored relative asset path");
      }
      const path = source.destination + "/" + name;
      if (canonical && join(canonical, path).length > 256) throw new Error("Renderer support path exceeds 256 characters");
      const bytes = await readFile(join(source.root, name));
      files.push({ path, bytes: bytes.length, sha256: digest(bytes) });
    }
  }
  return { sources, files };
}

export async function stageRendererSupport(output: string) {
  const generated = await realpath(join(import.meta.dirname, "🗑️generated"));
  const canonical = await realpath(output), inside = relative(generated, canonical);
  if (!inside || isAbsolute(inside) || inside === ".." || inside.startsWith(".." + sep) || dirname(canonical) !== generated) {
    throw new Error("Renderer support must be copied into a private top-level diagnostic site");
  }
  const { sources, files } = await rendererSupportInventory(canonical);
  const count = await copyBrowserArtifacts(canonical, sources);
  if (count !== files.length) throw new Error("Renderer support inventory changed");
  for (const file of files) {
    const bytes = await readFile(join(canonical, file.path));
    if (bytes.length !== file.bytes || digest(bytes) !== file.sha256) throw new Error("Renderer support bytes changed");
  }
  return files;
}

if (import.meta.main) {
  const [output] = process.argv.slice(2);
  if (output === "--inventory") {
    console.log(JSON.stringify((await rendererSupportInventory()).files));
    process.exit(0);
  }
  if (!output) throw new Error("Require the private site output");
  const marker = JSON.parse(await readFile(join(output, ".generator-frontend-diagnostic.json"), "utf8"));
  if (marker.diagnosticOnly !== true) throw new Error("Require a private diagnostic build marker");
  console.log(JSON.stringify(await stageRendererSupport(output)));
}
