import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { plugin } from "bun";

const workspace = process.cwd();
const ticket = import.meta.dir;
const outputRoot = relative(workspace, join(ticket, "🗑️generated", "note-catalog")).replaceAll("\\", "/");
const frameworkOS = "🧰️framework/🛍️products/💻️os/🔨️modules";
const specificCatalog = "✏️s/🧑‍💻dev/🧩️catalog";
const variant = "✏️s/🧑‍💻dev/🎭️variants/🗒️note";
const { renderCatalogFiles } = await import(join(workspace, frameworkOS, "🔌️plugin/📇️registry/📽️projection/🟦️.ts"));
const { specificCatalogPublicationV1 } = await import(join(workspace, specificCatalog, "📣️publication/🟦️.ts"));
const publication = specificCatalogPublicationV1((path: string) => readFileSync(join(workspace, path), "utf8"));
const catalog = renderCatalogFiles(workspace, undefined, "exclude", { ...publication, outputRoot });
const note = catalog.playgrounds.find((row: { variant: string }) => row.variant === "note");
assert.ok(note?.devContribution);
assert.ok(catalog.entries.some((row: { pluginId: string }) => row.pluginId === "note"));
for (const name of ["🧩️plugins/🟦️.ts", "🎮️playgrounds/🟦️.ts"]) {
  const path = join(workspace, outputRoot, name);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, catalog.files[name]);
}
plugin({
  name: "isolated-note-catalog",
  setup(build) {
    build.onResolve({ filter: /^\.\.?\/🤖️generated\/.*\.ts$/u }, ({ path, importer }) => {
      if (!importer?.startsWith(join(workspace, specificCatalog))) return;
      const name = path.split("/").slice(-2).join("/");
      if (!(name in catalog.files)) return;
      return { path: join(workspace, outputRoot, name) };
    });
  },
});
const { composeSpecificOsCatalogV1 } = await import(join(workspace, specificCatalog, "🟦️.ts"));
const admission = { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: Date.now() + 30000, now: Date.now, cancelled: () => false, progress: () => {} };
const { rows } = composeSpecificOsCatalogV1("http://127.0.0.1:6080/", admission);
assert.equal(rows.playgrounds.find((row: { variant: string }) => row.variant === "note")?.app, "s.note.note@1/*#editor");
console.log("[DEBUG] Specific Note catalog admits the editor variant");
process.env.SEMIO_PLUGIN = "note";
process.env.SEMIO_BUILD_MODE = "prototype";
const { default: configuration } = await import(join(workspace, variant, "🏗️builder/🌐️vite/🟦️.ts"));
assert.equal(typeof configuration, "function");
console.log("[DEBUG] Note Vite config module loaded");
try {
  const config = await configuration({ command: "serve", mode: "prototype" });
  assert.ok(config.plugins.length > 0);
  console.log("[DEBUG] Note Vite config resolved");
} catch (error) {
  const cause = error as NodeJS.ErrnoException;
  const receipt = join(workspace, ".cache/b/p/m/browser/prototype/runtime/react/note/activation/🔣️receipt.json");
  if (cause.code !== "ENOENT" || cause.path !== receipt) throw error;
  console.log("[DEBUG] Note Vite configuration reached its required activation receipt");
}
