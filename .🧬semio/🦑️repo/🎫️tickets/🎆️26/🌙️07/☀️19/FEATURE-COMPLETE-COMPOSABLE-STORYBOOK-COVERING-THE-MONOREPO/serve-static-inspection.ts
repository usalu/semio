import { mkdirSync, rmSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const output = resolve(ticket, "🗑️generated/s");
const load = (path: string) => import(pathToFileURL(resolve(path)).href);
const { prepareStorybookAssets } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🧰️preparation/🟦️.ts");
const { repositoryCachePartitionRootDataV1 } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts");
const cancellation = new AbortController();
const stop = () => cancellation.abort();
process.once("SIGTERM", stop);
process.once("SIGINT", stop);
try {
  prepareStorybookAssets(() => cancellation.signal.throwIfAborted());
  mkdirSync(output, { recursive: true });
  const build = Bun.spawn([process.execPath, "./node_modules/storybook/dist/bin/dispatcher.js", "build", "--output-dir", output, "--disable-telemetry"], {
    env: { ...process.env, SEMIO_BUILD_MODE: "prototype", STORYBOOK_SCOPE: "ui,styling", SEMIO_VITE_CACHE_DIRECTORY: repositoryCachePartitionRootDataV1(process.cwd(), "products") + "/storybook-build-inspect" },
    stdout: "inherit", stderr: "inherit",
  });
  const cancelBuild = () => build.kill("SIGTERM");
  cancellation.signal.addEventListener("abort", cancelBuild, { once: true });
  let code: number;
  try { code = await build.exited; } finally { cancellation.signal.removeEventListener("abort", cancelBuild); }
  if (code) throw new Error(`Storybook build exited ${code}`);
  cancellation.signal.throwIfAborted();
  const server = Bun.serve({
    hostname: "0.0.0.0", port: 6010,
    async fetch(request) {
      if (request.method !== "GET" && request.method !== "HEAD") return new Response("Method Not Allowed", { status: 405, headers: { Allow: "GET, HEAD" } });
      let name: string;
      try { name = decodeURIComponent(new URL(request.url).pathname); } catch { return new Response("Bad Request", { status: 400 }); }
      const path = resolve(output, "." + (name === "/" ? "/index.html" : name));
      const inside = relative(output, path);
      if (!inside || inside === ".." || inside.startsWith(".." + sep) || isAbsolute(inside)) return new Response("Forbidden", { status: 403 });
      const file = Bun.file(path);
      if (!await file.exists()) return new Response("Not Found", { status: 404 });
      return new Response(request.method === "HEAD" ? null : file, { headers: { "Content-Type": file.type || "application/octet-stream", "Cache-Control": "no-store" } });
    },
  });
  console.log("Static Storybook ready on 0.0.0.0:6010; no development WebSockets or file-writing endpoints.");
  try {
    await new Promise<void>(done => {
      if (cancellation.signal.aborted) done();
      else cancellation.signal.addEventListener("abort", () => done(), { once: true });
    });
  } finally { await server.stop(true); }
} finally {
  process.removeListener("SIGTERM", stop);
  process.removeListener("SIGINT", stop);
  rmSync(output, { recursive: true, force: true });
}
