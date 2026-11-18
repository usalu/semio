/** 🏗️ Release build of the course site for the reconnect storm into the ticket's own folder, baking the proctor origin 8797 in. */
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, readFileSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const out = join(here, "🗑️generated/reconnect", "site");
const pdfs = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9d0eceba-b1b8-4814-b9cd-caae5bbd19aa/scratchpad/pdfs/out";
const secret = randomBytes(32).toString("hex");
const channel = createServer((request, response) => {
  const name = new URL(request.url ?? "/", "http://localhost").pathname.replace(/^\/pdfs\//, "");
  if (request.headers.authorization !== "Bearer " + secret || name.includes("/") || !existsSync(join(pdfs, name))) { response.statusCode = 403; return void response.end(); }
  const bytes = readFileSync(join(pdfs, name));
  response.setHeader("content-type", "application/pdf");
  response.setHeader("content-length", bytes.length);
  response.end(bytes);
});
await new Promise((accept) => channel.listen(0, "127.0.0.1", accept));
rmSync(out, { recursive: true, force: true });
const child = spawn(process.execPath, [join(repo, "node_modules/vite/bin/vite.js"), "build", "--config", join(site, "🏗️builder/🌐️vite/🟦️.ts"), "--mode", "release", "--outDir", out, "--emptyOutDir"], {
  cwd: site, stdio: "inherit",
  env: { ...process.env, SEMIO_BUILD_MODE: "release", PROCTOR_URL: process.env.PROCTOR_URL ?? "http://127.0.0.1:8797", SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-course-native-reconnect-release"), SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${channel.address().port}/pdfs`, secret }]) },
});
const code = await new Promise((done) => child.once("exit", done));
channel.close();
process.exit(code ?? 1);
