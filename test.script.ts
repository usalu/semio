#!/usr/bin/env bun
/** 🧪 Typecheck-focused builds (@semio/js, @semio/react, GraphQL codegen) then every Nx `test` except `workspace`. */
import { execFileSync, spawn } from "node:child_process";

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

if (slice === "storybook") {
  const storybookPort = process.env.STORYBOOK_PORT ?? "65010";
  const baseUrl = process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/storybook-static/`;

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
