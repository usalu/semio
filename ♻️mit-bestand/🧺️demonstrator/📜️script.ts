#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <dev|build> [args…]`. */
import { cpSync, existsSync, mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBundleScriptMain,
  runCmdStatus,
  runViteBunxDev,
  spawnDaemon,
} from "../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { DEMONSTRATOR_HOST } from "./🟦️brand.ts";

const repoRoot = resolve(import.meta.dir, "../..");
const demonstratorRoot = import.meta.dir;
const distDir = join(demonstratorRoot, "dist");
const stagingRoot = join(demonstratorRoot, "dist-staging");

const APP_SLUGS = ["generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"] as const;
const PLAYGROUND_VARIANTS = ["generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"] as const;

//#region 📦️AssembleDist
function copyTreeMerge(src: string, dest: string): void {
  if (!existsSync(src)) return;
  mkdirSync(dest, { recursive: true });
  for (const name of readdirSync(src, { withFileTypes: true })) {
    const from = join(src, name.name);
    const to = join(dest, name.name);
    if (name.isDirectory()) copyTreeMerge(from, to);
    else cpSync(from, to);
  }
}

function resolveAppIndexHtml(stagingDir: string): string | undefined {
  const emoji = join(stagingDir, "🌐️index.html");
  if (existsSync(emoji)) return emoji;
  const plain = join(stagingDir, "index.html");
  if (existsSync(plain)) return plain;
  return undefined;
}

/** @emoji 📦️ Merges per-app playground builds and the landing build into one static deploy tree. */
function assembleDemonstratorDist(): void {
  mkdirSync(distDir, { recursive: true });
  for (const slug of APP_SLUGS) {
    const stagingDir = join(stagingRoot, slug);
    const slugDir = join(distDir, slug);
    mkdirSync(slugDir, { recursive: true });
    const indexSource = resolveAppIndexHtml(stagingDir);
    if (indexSource) cpSync(indexSource, join(slugDir, "index.html"));
    for (const name of readdirSync(stagingDir, { withFileTypes: true })) {
      if (name.name === "🌐️index.html" || name.name === "index.html") continue;
      const from = join(stagingDir, name.name);
      const to = join(distDir, name.name);
      if (name.isDirectory()) copyTreeMerge(from, to);
      else {
        mkdirSync(dirname(to), { recursive: true });
        cpSync(from, to);
      }
    }
  }
  writeFileSync(join(distDir, ".nojekyll"), "");
  writeFileSync(join(distDir, "🌐️CNAME"), `${DEMONSTRATOR_HOST}\n`);
}
//#endregion 📦️AssembleDist

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const osDevScript = join(repoRoot, "./🧰️framework/🛍️product/💻️os/🔨️module/🧑️‍💻️dev/⚡️implementation/🟦️typescript/📜️script.ts");
    for (const variant of PLAYGROUND_VARIANTS) {
      spawnDaemon("bun", [osDevScript, "dev", variant], { cwd: dirname(osDevScript), env: process.env });
    }
    runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "MIT_BESTAND_DEMONSTRATOR_PORT",
      defaultPort: "6029",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    rmSync(stagingRoot, { recursive: true, force: true });
    mkdirSync(stagingRoot, { recursive: true });
    const osDevScript = join(repoRoot, "./🧰️framework/🛍️product/💻️os/🔨️module/🧑️‍💻️dev/⚡️implementation/🟦️typescript/📜️script.ts");
    for (const variant of PLAYGROUND_VARIANTS) {
      const status = runCmdStatus("bun", [osDevScript, "build", variant], { cwd: dirname(osDevScript), env: process.env });
      if (status !== 0) throw new Error(`demonstrator app build failed: ${variant}`);
    }
    if (runCmdStatus("bun", ["run", "vite", "build", "--config", "⚙️vite.config.ts", ...segments], { cwd: this.root, env: process.env, ...playPollingEnv() }) !== 0) {
      throw new Error("demonstrator landing build failed");
    }
    assembleDemonstratorDist();
    console.log(`[build] demonstrator dist assembled at ${distDir}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
