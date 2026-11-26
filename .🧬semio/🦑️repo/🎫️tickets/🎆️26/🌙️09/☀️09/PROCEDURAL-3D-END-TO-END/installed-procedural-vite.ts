import { existsSync, readFileSync } from "node:fs";
import { open } from "node:fs/promises";
import { extname, join } from "node:path";
import type { Plugin } from "vite";
import { developmentRuntimeRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

/** 📦 Serves the installed procedural component at the native development host's ordinary module URL. */
export function installedProceduralVite(renderer: "react" | "wgpu", workspace = process.cwd()): Plugin {
  const root = join(developmentRuntimeRoot(workspace, "generation3d", "prototype", renderer), "components", "🌀️procedural");
  const prefix = "/🔌️plugin-modules/🌀️procedural/";
  const mime: Record<string, string> = { ".wasm": "application/wasm", ".js": "text/javascript", ".json": "application/json" };
  return {
    name: "generation3d-installed-procedural",
    enforce: "pre",
    configureServer(server) {
      const marker = join(root, ".semio-artifact.json");
      if (!existsSync(marker)) return;
      const manifest = JSON.parse(readFileSync(marker, "utf8"));
      if (manifest.owner !== "development-component:procedural") throw Error("Unexpected local procedural component owner");
      const files = new Set<string>(manifest.files);
      server.middlewares.use(async (request, response, next) => {
        let pathname: string;
        try { pathname = decodeURIComponent((request.url ?? "").split("?")[0]); }
        catch { response.writeHead(400).end(); return; }
        if (!pathname.startsWith(prefix)) return next();
        const name = pathname.slice(prefix.length);
        if (!files.has(name) || name.includes("/") || name.includes("\\") || name.startsWith(".")) {
          response.writeHead(404).end(); return;
        }
        if (request.method !== "GET" && request.method !== "HEAD") { response.writeHead(405).end(); return; }
        try {
          const file = await open(join(root, name), "r");
          try {
            response.writeHead(200, { "Content-Type": mime[extname(name)] ?? "application/octet-stream", "Content-Length": (await file.stat()).size, "Cache-Control": "no-store" });
            if (request.method === "HEAD") { await file.close(); response.end(); return; }
            const stream = file.createReadStream();
            response.on("close", () => stream.destroy());
            stream.on("error", () => response.destroy()).pipe(response);
          } catch (error) { await file.close(); throw error; }
        } catch (error) { next(error); }
      });
    },
  };
}
