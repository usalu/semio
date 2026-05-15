#!/usr/bin/env bun
/** 🧪 Typecheck-focused builds (@semio/js, @semio/react, GraphQL codegen) then every Nx `test` except `workspace`. */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
const slice = process.argv[2];

if (slice === "storybook") {
  execFileSync("bunx", ["playwright", "test", "--config", ".storybook/playwright.config.ts"], {
    cwd: root,
    stdio: "inherit",
    env: {
      ...process.env,
      PLAYWRIGHT_BROWSERS_PATH: process.env.PLAYWRIGHT_BROWSERS_PATH ?? `${root}/node_modules/.cache/ms-playwright`,
      STORYBOOK_PORT: process.env.STORYBOOK_PORT ?? "6010",
    },
  });
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
