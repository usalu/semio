#!/usr/bin/env bun
/** 📦 Nx `publish` for a named app (`desktop`, `play`, `sketchpad`, `docs`, `coda-desktop`). */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const slice = process.argv[2];

const map: Record<string, string> = {
  desktop: "@semio/desktop:publish",
  play: "@semio/play:publish",
  sketchpad: "@semio/sketchpad:publish",
  docs: "@semio/docs:publish",
  "coda-desktop": "@coda/desktop:publish",
};

if (!slice) {
  console.error(`[publish] usage: bun ./publish.script.ts <${Object.keys(map).join(" | ")}>`);
  process.exit(1);
}

const target = map[slice];
if (!target) {
  console.error(`[publish] unknown slice ${JSON.stringify(slice)}`);
  process.exit(1);
}

execFileSync("bun", ["nx", "run", target], { cwd: root, stdio: "inherit" });
