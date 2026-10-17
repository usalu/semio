/** 🎨️ Loads `@semio-tech/quiz-react/stylesheet` and the package stub through a real Vite SSR load under the exact-match quiz-react alias: the facade must carry the real stylesheet text, the stub only an `@import`. */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const quiz = root + "/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react";
const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [{ find: /^@semio-tech\/quiz-react$/u, replacement: quiz + "/📦️packages/🟦️typescript/🟦️.tsx" }] }, optimizeDeps: { noDiscovery: true, include: [] } });
const mod = await server.ssrLoadModule(quiz + "/🧪️tests/x-probe.ts").catch(() => null);
const importer = root + "/🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx";
const facade = await server.pluginContainer.resolveId("@semio-tech/quiz-react/stylesheet", importer, { ssr: true });
const loaded = await server.ssrLoadModule(String(facade.id));
const text = loaded.quizStylesheet;
const real = readFileSync(quiz + "/🎨️.css", "utf8");
const stub = readFileSync(quiz + "/📦️packages/🟦️typescript/🎨️.css", "utf8");
console.log(JSON.stringify({ facade: String(facade.id).replace(root + "/", ""), length: text.length, equalsRealCss: text === real, startsWithImport: text.trimStart().startsWith("@import"), stubText: stub.trim(), stubLength: stub.length }));
await server.close();
