import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const L = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const files = readFileSync(`${L}/🗑️generated/lw2b/iw-files.txt`, "utf8").split("\n").filter(Boolean);
const manifests = files.filter((f) => f.endsWith("package.json"));
const sources = files.filter((f) => /\.(tsx?|mts|js|mjs)$/u.test(f) && !f.startsWith("."));
const nearestManifest = (file: string): string | null => { let dir = dirname(file); while (dir !== "." && dir !== "/") { if (existsSync(join(dir, "package.json"))) return join(dir, "package.json"); dir = dirname(dir); } return null; };
const importersByManifest = new Map<string, string[]>();
const NEEDLE = /["'`]@semio-tech\/infinite-world-r3f["'`]|["'`](?:\.\.\/)+🧰️framework\/📦️packages\/🟦️typescript\/♾️infinite\/🌍️world\/🎨️r3f\/🟦️\.ts["'`]/u;
for (const file of sources) {
  if (!existsSync(file)) continue;
  if (!NEEDLE.test(readFileSync(file, "utf8"))) continue;
  const manifest = nearestManifest(file) ?? "(none)";
  importersByManifest.set(manifest, [...(importersByManifest.get(manifest) ?? []), file]);
}
const declaring = manifests.filter((m) => existsSync(m) && m !== "🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/package.json" && (() => { const p = JSON.parse(readFileSync(m, "utf8")); return Boolean(p.dependencies?.["@semio-tech/infinite-world-r3f"] || p.devDependencies?.["@semio-tech/infinite-world-r3f"] || p.peerDependencies?.["@semio-tech/infinite-world-r3f"]); })());
const OLD = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript";
const NEW = "🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript";
const workspaceFiles = ["package.json", "♻️mit-bestand/package.json", "✏️s/package.json", "🌎️hub/package.json", "🎓️teaching/package.json", "🏢️semio-tech/package.json"].map((file) => {
  const prefix = file === "package.json" ? "" : "../";
  const entries: string[] = JSON.parse(readFileSync(file, "utf8")).workspaces;
  return { file, oldEntryPresent: entries.includes(`${prefix}${OLD}`), newEntryPresent: entries.includes(`${prefix}${NEW}`), neededEntry: `${prefix}${NEW}` };
});
const spec = {
  generatedBy: "lw2b",
  sdkKeys: [{
    package: "@semio-tech/framework",
    packageJson: "🧰️framework/📦️packages/🟦️typescript/package.json",
    add: { "./infinite/world/r3f": "./♾️infinite/🌍️world/🎨️r3f/🟦️.ts" },
    glue: "🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts",
    module: "🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx",
    afterAdding: "rewrite the cad importers from the relative glue specifier to @semio-tech/framework/infinite/world/r3f",
    importersToRewrite: [...importersByManifest.entries()].filter(([m]) => m.includes("📐️cad")).flatMap(([, f]) => f),
  }],
  workspaceEntriesToRewrite: workspaceFiles,
  bunLockFiles: ["bun.lock", "♻️mit-bestand/bun.lock", "✏️s/bun.lock", "🌎️hub/bun.lock", "🎓️teaching/bun.lock", "🏢️semio-tech/bun.lock"],
  bunLockNote: "regenerate with bun install; the workspace key moves from the os product path to the framework module path, and node_modules/@semio-tech/infinite-world-r3f symlinks in each workspace root were relinked by hand to the new package directory.",
  derivedFilesToRegenerate: ["🔒️dependencies.json (users paths of @react-three/drei, @react-three/fiber and the two Rust crates)"],
  packagesDeclaringInfiniteWorldR3f: declaring.map((m) => ({ manifest: m, importsIt: importersByManifest.has(m), importers: importersByManifest.get(m) ?? [] })),
  packagesDeclaringInfiniteWorldR3fCount: declaring.length,
  packagesDeclaringWithoutImport: declaring.filter((m) => !importersByManifest.has(m)),
  orphanOsExportKeys: {
    package: "@semio-tech/framework-os",
    packageJson: "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json",
    remove: {
      "./modules/store/blob/schema": "./🔨️modules/🏪️store/📦️blob/🧬️schema/🟦️.ts",
      "./modules/store/child/schema": "./🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts",
      "./modules/store/link/schema": "./🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts",
    },
    deleteGlueFiles: [
      "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/📦️blob/🧬️schema/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts",
    ],
    importersLeft: 0,
    replacement: "@semio-tech/framework/schema/artifact-reference",
  },
};
writeFileSync(`${L}/lw2b-tools/exports-needed.json`, JSON.stringify(spec, null, 2) + "\n");
console.log("declaring", declaring.length, "withoutImport", spec.packagesDeclaringWithoutImport.length, "importerManifests", importersByManifest.size);
for (const w of workspaceFiles) console.log(w.file, "old", w.oldEntryPresent, "new", w.newEntryPresent);
