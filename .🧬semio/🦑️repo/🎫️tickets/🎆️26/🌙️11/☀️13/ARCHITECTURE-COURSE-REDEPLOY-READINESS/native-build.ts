/** 🚑️ Graph-free release build of the course site: serves the assignment PDFs of `PDF_DIRECTORY` over the runtime channel the Vite plugin reads, then runs `vite build --mode release` with the course config. */
import { randomBytes } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { createServer } from "node:http";
import { join, resolve } from "node:path";

const repo = resolve(import.meta.dir, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const pdfs = process.env.PDF_DIRECTORY ?? "";
const placeholder = new TextEncoder().encode("%PDF-1.4\n1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj 2 0 obj<</Type/Pages/Kids[]/Count 0>>endobj\ntrailer<</Root 1 0 R>>\n%%EOF\n");
const secret = randomBytes(32).toString("hex");
const server = createServer((request, response) => {
  const name = new URL(request.url ?? "/", "http://localhost").pathname.replace(/^\/pdfs\//, "");
  if (request.headers.authorization !== "Bearer " + secret || name.includes("/")) { response.statusCode = 403; return void response.end(); }
  const path = join(pdfs, name);
  const bytes = pdfs && existsSync(path) ? readFileSync(path) : placeholder;
  if (bytes === placeholder) console.log("[DEBUG] placeholder PDF for " + name);
  response.setHeader("content-type", "application/pdf");
  response.setHeader("content-length", bytes.length);
  response.end(bytes);
});
await new Promise<void>(accept => server.listen(0, "127.0.0.1", accept));
const address = server.address();
if (!address || typeof address === "string") throw Error("listener required");
const child = Bun.spawn([process.execPath, join(repo, "node_modules/vite/bin/vite.js"), "build", "--config", join(site, "🏗️builder/🌐️vite/🟦️.ts"), "--mode", process.env.SEMIO_BUILD_MODE ?? "release"], {
  cwd: site,
  env: { ...process.env, SEMIO_BUILD_MODE: process.env.SEMIO_BUILD_MODE ?? "release", SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-course-native"), SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${address.port}/pdfs`, secret }]) },
  stdout: "inherit", stderr: "inherit",
});
const code = await child.exited;
server.close();
process.exit(code);
