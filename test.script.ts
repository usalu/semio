#!/usr/bin/env bun
/** 🧪 Typecheck-focused builds (@semio/js, @semio/react, GraphQL codegen) then every Nx `test` except `workspace`. */
import { execFileSync, spawn } from "node:child_process";
import { createServer } from "node:net";

const root = import.meta.dir;
const slice = process.argv[2];

async function waitForUrl(url: string, timeoutMs: number): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Timed out waiting for ${url}`);
}

/** True if nothing is listening on `host:port` (same bind family as `dev.script.ts` storybook-static). */
function isTcpPortFree(port: number, host: string): Promise<boolean> {
  return new Promise((resolve) => {
    const server = createServer();
    server.unref();
    server.once("error", () => resolve(false));
    server.listen(port, host, () => {
      server.close(() => resolve(true));
    });
  });
}

/** First free port in `[preferred, preferred + span)` so Playwright can run when default 6010 is taken. */
async function pickStorybookStaticPort(preferred: number, span: number): Promise<number> {
  for (let port = preferred; port < preferred + span; port += 1) {
    if (await isTcpPortFree(port, "0.0.0.0")) {
      return port;
    }
  }
  throw new Error(`No free TCP port in ${preferred}..${preferred + span - 1} for storybook-static (set STORYBOOK_PORT).`);
}

async function runStorybookPlaywrightTests(): Promise<void> {
  const preferred = Number(process.env.STORYBOOK_PORT ?? 6010);
  const storybookPort = String(await pickStorybookStaticPort(preferred, 50));
  const baseUrl = `http://127.0.0.1:${storybookPort}/`;
  if (storybookPort !== String(preferred)) {
    console.log(`[DEBUG] STORYBOOK_PORT ${preferred} busy; using ${storybookPort} for storybook-static + Playwright`);
  }

  execFileSync("bun", ["./build.script.ts", "storybook"], {
    cwd: root,
    stdio: "inherit",
  });

  const server = spawn("bun", ["./dev.script.ts", "storybook-static"], {
    cwd: root,
    stdio: "inherit",
    env: {
      ...process.env,
      STORYBOOK_PORT: storybookPort,
    },
  });

  try {
    await waitForUrl(new URL("index.html", baseUrl).href, 120000);
    execFileSync("bunx", ["playwright", "test", "--config", ".storybook/playwright.config.ts"], {
      cwd: root,
      stdio: "inherit",
      env: {
        ...process.env,
        PLAYWRIGHT_BASE_URL: baseUrl,
        PLAYWRIGHT_BROWSERS_PATH: process.env.PLAYWRIGHT_BROWSERS_PATH ?? `${root}/node_modules/.cache/ms-playwright`,
        STORYBOOK_PORT: storybookPort,
      },
    });
  } finally {
    server.kill();
  }
}

if (slice === "storybook") {
  await runStorybookPlaywrightTests();
  process.exit(0);
}

execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/js", "@semio/react"], {
  cwd: root,
  stdio: "inherit",
});
execFileSync("bun", ["nx", "run", "semio/graphql:build"], { cwd: root, stdio: "inherit" });
execFileSync("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], {
  cwd: root,
  stdio: "inherit",
});
execFileSync("bun", ["nx", "run", "workspace:test-storybook"], {
  cwd: root,
  stdio: "inherit",
});
