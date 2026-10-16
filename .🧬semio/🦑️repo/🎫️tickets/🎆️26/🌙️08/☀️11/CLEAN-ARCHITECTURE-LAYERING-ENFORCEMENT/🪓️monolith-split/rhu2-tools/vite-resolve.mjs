import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import fs from "node:fs";
const root = "/Users/ueli/Documents/semio";
const barrel = root + "/🧰️framework/📦️packages/🟦️typescript/🟦️.ts";
const importer = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx";
const subs = Object.keys(JSON.parse(fs.readFileSync(root + "/🧰️framework/📦️packages/🟦️typescript/package.json", "utf8")).exports).filter(k => k !== ".");
const mine = ["./assets", "./canvas/react-renderer", "./replication", "./trace", "./tool-machine", "./value/resident", "./async/continuation", "./interaction", "./2d", "./tool-run", "./io/base64", "./assets/mesh", "./pixels/editing", "./pixels/selection", "./raster/video", "./io/stream-mux", "./time-travel", "./wire-contracts", "./app-channel", "./packs", "./document/check-in", "./canvas/runtime-assets", "./process/routing"].filter(s => subs.includes(s));
for (const [label, alias] of [["exact regex alias (the repo builders)", [{ find: /^@semio-tech\/framework$/u, replacement: barrel }]]]) {
  const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias }, optimizeDeps: { noDiscovery: true, include: [] } });
  console.log("==", label);
  let bad = 0;
  for (const s of ["@semio-tech/framework", ...mine.map(m => "@semio-tech/framework" + m.slice(1))]) {
    const r = await server.pluginContainer.resolveId(s, importer, { ssr: false });
    if (!r?.id) bad++;
    console.log(String(r?.id ?? "UNRESOLVED").replace(root + "/", "").slice(0, 110).padEnd(110), "<-", s);
  }
  console.log("unresolved:", bad);
  await server.close();
}
