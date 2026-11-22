import assert from "node:assert/strict";
import { join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { parseCatalogPublicationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🟦️.ts";
import { publishCatalogFilesV1, renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";

const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const ticket = fileURLToPath(new URL(".", import.meta.url));
const ownerRoot = relative(root, ticket).split(sep).join("/");
const packages = [
  ["space", "🌎️hub/🧩️compositions/🪐️space"],
  ["gis", "🌎️hub/🧩️compositions/🌍️gis"],
  ["stdio", "🌎️hub/🧩️compositions/🗄️stdio/🧩️component"],
];
const publication = parseCatalogPublicationV1({
  version: 1, ownerRoot, outputRoot: ownerRoot + "/🗑️generated/scatalog",
  contributions: [{ ownerRoot: "🌎️hub", packageRoots: packages.map(([, path]) => path) }],
});
const rendered = renderCatalogFiles(root, undefined, "refuse", publication);
assert.deepEqual(rendered.entries.map((plugin) => plugin.pluginId).sort(), packages.map(([id]) => id).sort());
const shell = rendered.playgrounds.find((row) => row.variant === "s");
assert.equal(shell?.pluginId, "space");
assert.equal(shell?.app, undefined);
assert.deepEqual(rendered.diagnostics, []);
for (const filename of Object.keys(rendered.files)) {
  assert(join(root, publication.outputRoot, filename).length <= 256, filename);
}
const directory = join(root, publication.outputRoot);
publishCatalogFilesV1(directory, rendered.files);
console.log(JSON.stringify({ directory, playground: shell.variant, plugins: rendered.entries.map((plugin) => plugin.pluginId) }));
