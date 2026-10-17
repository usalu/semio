/** 🔍️ Resolves the quiz stylesheet specifier under the quiz-react vitest alias list, with the old string alias and with the new exact-match alias. */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
const root = "/Users/ueli/Documents/semio";
const quiz = root + "/🧰️framework/🛍️products/❓️quiz";
const importer = quiz + "/🧪️tests/🌗️contrast-states/🟦️.tsx";
for (const [label, find] of [["old string alias", "@semio-tech/quiz-react"], ["new exact alias", /^@semio-tech\/quiz-react$/u]]) {
  const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [{ find, replacement: quiz + "/🎯️targets/⚛️react/🟦️.tsx" }] }, optimizeDeps: { noDiscovery: true, include: [] } });
  for (const spec of ["@semio-tech/quiz-react", "@semio-tech/quiz-react/stylesheet"]) {
    const r = await server.pluginContainer.resolveId(spec, importer, { ssr: false });
    console.log(label.padEnd(18), spec.padEnd(36), String(r?.id ?? "UNRESOLVED").replace(root + "/", "").slice(-70));
  }
  await server.close();
}
