/** 🔍️ Resolves SDK specifiers through a real Vite plugin container, with the old prefix alias and the new exact alias. */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
const root = "/Users/ueli/Documents/semio";
const barrel = root + "/🧰️framework/📦️packages/🟦️typescript/🟦️.ts";
const importer = root + "/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
const specs = ["@semio-tech/framework", "@semio-tech/framework/io/sqlite-snapshot", "@semio-tech/framework/io/sqlite-snapshot/artifact", "@semio-tech/framework/process/routing", "@semio-tech/framework/schema/artifact-reference", "@semio-tech/framework/pixels/compositing"];
for (const [label, alias] of [["old string alias", [{ find: "@semio-tech/framework", replacement: barrel }]], ["new exact alias", [{ find: /^@semio-tech\/framework$/u, replacement: barrel }]], ["no alias", []]]) {
  const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias }, optimizeDeps: { noDiscovery: true, include: [] } });
  console.log("==", label);
  for (const s of specs) {
    const r = await server.pluginContainer.resolveId(s, importer, { ssr: false });
    console.log(String(r?.id ?? "UNRESOLVED").replace(root + "/", "").slice(0, 120).padEnd(120), "<-", s);
  }
  await server.close();
}
