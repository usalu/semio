/** 🏗️ Static vite build of the architecture course site into this ticket's generated folder, without a dev server: the assignment PDFs the build plugin asks for come from a local stub that serves a minimal PDF. Usage (repo root): node web-build.mjs */
import { spawn } from "node:child_process";
import { createServer } from "node:http";
import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
const root = resolve(import.meta.dirname, "../../../../../../..");
const site = join(root, "🎓️teaching/🏛️architecture/📚️course");
const out = join(import.meta.dirname, "🗑️generated/web-dist");
const cache = join(root, ".cache", "b", "p", "m", "vite", "prototype", "architecture-course-direct");
mkdirSync(cache, { recursive: true });
const secret = "stub";
const server = createServer((request, response) => {
  if (request.headers.authorization !== "Bearer " + secret) { response.statusCode = 404; return void response.end(); }
  const body = Buffer.from("%PDF-1.4\n1 0 obj<<>>endobj\ntrailer<<>>\n%%EOF\n");
  response.setHeader("content-type", "application/pdf");
  response.setHeader("content-length", body.length);
  response.end(body);
});
await new Promise((accept) => server.listen(0, "127.0.0.1", accept));
const channels = JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${server.address().port}/pdfs`, secret }]);
const args = ["tools", "exec", "vite", "--", "build", "--mode", "prototype", "--configLoader", "native", "--config", "../../🏗️builder/🌐️vite/🟦️.ts", "--outDir", out, "--emptyOutDir"];
const child = spawn("repo", args, { cwd: join(site, "📦️packages/🟦️typescript"), stdio: "inherit", env: { ...process.env, SEMIO_BUILD_MODE: "prototype", SEMIO_VITE_CACHE_DIRECTORY: cache, SEMIO_VITE_RUNTIME_CHANNELS_V1: channels, PROCTOR_URL: "http://127.0.0.1:59999" } });
child.on("exit", (status) => { server.close(); server.closeAllConnections(); process.exit(status ?? 1); });
