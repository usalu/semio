import { resolve } from "node:path";
import { createServer, loadConfigFromFile } from "vite";

const products = {
  play: ["🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts", 6192],
  demonstrator: ["♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts", 6193],
  course: ["🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts", 6172],
  staffage: ["🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts", 6173],
};
const product = process.argv[2] ?? "course";
const [file, port] = products[product];
if (["play", "demonstrator"].includes(product)) {
  process.env.SEMIO_BUILD_MODE = "prototype";
  process.env.SEMIO_VITE_CACHE_DIRECTORY = resolve(".cache/b/p/m", "vite", product);
}
const loaded = await loadConfigFromFile({ command: ["play", "demonstrator"].includes(product) ? "build" : "serve", mode: "prototype" }, resolve(file));
const server = await createServer({
  ...loaded.config,
  configFile: false,
  mode: "prototype",
  plugins: loaded.config.plugins.flat().filter(({ name }) => name !== "architecture-course-assignment-pdfs"),
  server: { ...loaded.config.server, host: "127.0.0.1", port, strictPort: true, watch: null },
});
await server.listen();
server.printUrls();
