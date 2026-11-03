import { mkdirSync, openSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const generated = resolve(ticket, "🗑️generated/b");
const output = resolve(generated, "s");
mkdirSync(generated, { recursive: true });
const load = (file: string) => import(pathToFileURL(resolve(file)).href);
const { repositoryCachePartitionRootDataV1 } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts");
const { prepareStorybookAssets } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🧰️preparation/🟦️.ts");
const stop = new AbortController();
process.once("SIGINT", () => stop.abort());
process.once("SIGTERM", () => stop.abort());
prepareStorybookAssets(() => stop.signal.throwIfAborted());
const cache = repositoryCachePartitionRootDataV1(process.cwd(), "products") + "/storybook-build-audit";
const child = Bun.spawn([process.execPath, "./node_modules/storybook/dist/bin/dispatcher.js", "build", "--output-dir", output, "--disable-telemetry"], {
  env: { ...process.env, SEMIO_BUILD_MODE: "prototype", STORYBOOK_SCOPE: "ui,styling", SEMIO_VITE_CACHE_DIRECTORY: cache },
  stdout: openSync(resolve(generated, "build.log"), "a"), stderr: openSync(resolve(generated, "build.log"), "a"),
});
const cancel = () => child.kill("SIGTERM");
stop.signal.addEventListener("abort", cancel, { once: true });
try {
  const code = await child.exited;
  if (code) {
    process.exitCode = code;
    console.log(readFileSync(resolve(generated, "build.log"), "utf8").slice(-10000));
  } else {
    readFileSync(resolve(output, "index.html"));
    const index = JSON.parse(readFileSync(resolve(output, "index.json"), "utf8"));
    const count = Object.values(index.entries).filter((entry: any) => entry.type === "story").length;
    console.log(`Built ${count} story variants.`);
    const server = Bun.serve({ hostname: "127.0.0.1", port: 16011, async fetch(request) {
      const name = decodeURIComponent(new URL(request.url).pathname);
      const path = resolve(output, "." + (name === "/" ? "/index.html" : name));
      const inside = relative(output, path);
      if (!inside || inside === ".." || inside.startsWith(".." + sep) || isAbsolute(inside)) return new Response("Forbidden", { status: 403 });
      const file = Bun.file(path);
      return await file.exists() ? new Response(file) : new Response("Not Found", { status: 404 });
    } });
    try {
      const tests = Bun.spawn([process.execPath, "./node_modules/@playwright/test/cli.js", "test", "--config", ".storybook/🧪️tests/🧪️browser-runner/🟦️.ts", ".storybook/🧪️tests/🧪️catalog-browser/🟦️.ts", ".storybook/🧪️tests/🧪️visual-layout/🟦️.ts"], {
        env: { ...process.env, PLAYWRIGHT_BASE_URL: "http://127.0.0.1:16011/", PLAYWRIGHT_OUTPUT_DIR: resolve(generated, "p"), CI: "" },
        stdout: "inherit", stderr: "inherit",
      });
      const cancelTests = () => tests.kill("SIGTERM");
      stop.signal.addEventListener("abort", cancelTests, { once: true });
      try { process.exitCode = await tests.exited; } finally { stop.signal.removeEventListener("abort", cancelTests); }
      writeFileSync(resolve(ticket, "static-validation.md"), `# Static Storybook Validation\n\nStorybook 10.6.5 built ${count} UI/styling variants. Browser catalog test exit code: ${process.exitCode}.\n\nBuilt artifacts and logs were removed after validation.\n`);
    } finally { await server.stop(true); }
  }
} finally {
  stop.signal.removeEventListener("abort", cancel);
  rmSync(generated, { recursive: true, force: true });
}
