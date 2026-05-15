#!/usr/bin/env bun
/**
 * 💻 Root `dev` entry: default `@semio/desktop` via Nx; `mcp`, `mcp repo`, `mcp engine` delegate to bundle-local MCP scripts.
 */
import { execFileSync } from "node:child_process";
import { stat } from "node:fs/promises";
import { extname, join, resolve } from "node:path";

const root = import.meta.dir;
const argv = process.argv.slice(2);

if (argv[0] === "storybook") {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env.STORYBOOK_PORT ?? "6010";
  const extra = argv.slice(1);
  const useExactPort =
    process.env.STORYBOOK_EXACT_PORT === "1" || process.env.STORYBOOK_EXACT_PORT === "true";
  /** Without `--exact-port`, Storybook can use the next free port when `STORYBOOK_PORT` is busy. Set `STORYBOOK_EXACT_PORT=1` for fail-fast (e.g. CI). */
  const storybookArgs = [
    "storybook",
    "dev",
    "-c",
    ".storybook",
    "-p",
    port,
    ...(useExactPort ? ["--exact-port"] : []),
    "--host",
    host,
    "--no-open",
    "--debug",
    ...extra,
  ];
  execFileSync("bunx", storybookArgs, {
    cwd: root,
    stdio: "inherit",
    env: {
      ...process.env,
      WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
      CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    },
  });
  process.exit(0);
}

if (argv[0] === "storybook-static") {
  /** `0.0.0.0` so `localhost` (IPv4/IPv6) and `127.0.0.1` both reach Playwright smoke tests on Windows. */
  const host = process.env.STORYBOOK_STATIC_HOST ?? "0.0.0.0";
  const port = Number(process.env.STORYBOOK_PORT ?? "6010");
  const repoRootPath = resolve(root);
  /** Storybook build output; serve this folder as site root so `/iframe.html` matches the build’s asset paths. */
  const documentRoot = resolve(repoRootPath, "storybook-static");

  const server = Bun.serve({
    hostname: host,
    port,
    async fetch(request) {
      const requestUrl = new URL(request.url);
      const requestPath = decodeURIComponent(requestUrl.pathname);
      const candidatePath = resolve(documentRoot, `.${requestPath}`);

      if (!candidatePath.startsWith(documentRoot)) {
        return new Response("Forbidden", { status: 403 });
      }

      const filePath = await (async () => {
        try {
          const fileInfo = await stat(candidatePath);
          if (fileInfo.isDirectory()) {
            return resolve(candidatePath, "index.html");
          }
          return candidatePath;
        } catch {
          if (extname(candidatePath) === "") {
            return resolve(candidatePath, "index.html");
          }
          return candidatePath;
        }
      })();

      try {
        const file = Bun.file(filePath);
        if (!(await file.exists())) {
          return new Response("Not Found", { status: 404 });
        }
        return new Response(file);
      } catch {
        return new Response("Not Found", { status: 404 });
      }
    },
  });

  console.log(`storybook-static listening on http://${host}:${port}`);
  await new Promise(() => {});
}

if (argv[0] === "mcp") {
  const mode = argv[1];
  if (mode === "engine") {
    execFileSync("bun", [join(root, "semio", "client", "bin", "engine", "dev.mcp.script.ts")], {
      cwd: root,
      stdio: "inherit",
    });
  } else if (mode === "repo") {
    execFileSync("bun", [join(root, "dev.mcp.inspector.script.ts"), "repo"], { cwd: root, stdio: "inherit" });
  } else {
    execFileSync("bun", [join(root, "dev.mcp.inspector.script.ts")], { cwd: root, stdio: "inherit" });
  }
  process.exit(0);
}

execFileSync("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: root, stdio: "inherit" });
