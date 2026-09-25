#!/usr/bin/env bun
/** 🧭 Engine package router: `bun ./script.ts <build|test|dev mcp> [segments…]`. */
import { execSync, spawn } from "node:child_process";
import { existsSync, readFileSync, rmSync, copyFileSync, cpSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/src/index.ts";

/** 🐍 Engine virtual environment (Python 3.14 via uv, zero-touch). */
const engineEnv = (root: string): NodeJS.ProcessEnv => ({ ...process.env, UV_PROJECT_ENVIRONMENT: join(root, ".venv") });

/** 🔄 Syncs the engine workspace member into its own virtual environment. */
const syncEngine = (root: string): void => {
  execSync("uv sync --python 3.14", { cwd: root, env: engineEnv(root), stdio: "inherit" });
};

/** 🖼️ Builds `dist/mcp-app.html` (the MCP App viewers served as `ui://semio/*` resources). */
const buildMcpApp = (repoRoot: string): void => {
  execSync("bunx vite build --config semio/client/bin/engine/vite.mcp-app.config.ts", { cwd: repoRoot, stdio: "inherit", shell: true });
};

class DevMcpScript extends BundleScript {
  run(): void {
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    syncEngine(this.root);
    try {
      buildMcpApp(this.repoRoot);
    } catch {
      console.warn("⚠️ MCP App viewers could not be built; the engine MCP runs without ui://semio viewers.");
    }
    const child = spawn("npx", ["--yes", "@mcpjam/inspector@latest", "uv", "--directory", this.root, "run", "main.py", "--mcp-stdio"], { stdio: "inherit", shell: true, env: { ...engineEnv(this.root), HOST: host }, cwd: this.repoRoot });
    child.on("exit", (c) => process.exit(c ?? 0));
  }
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "mcp") {
      console.error("usage: bun ./script.ts dev mcp");
      process.exit(1);
    }
    new DevMcpScript(this.root, this.repoRoot).run();
  }
}

class BuildPostScript extends BundleScript {
  run(): void {
    const exeExt = process.platform === "win32" ? ".exe" : "";
    const exePath = join(this.root, "dist", "semio-engine", `semio-engine${exeExt}`);
    const internalPath = join(this.root, "dist", "semio-engine", "_internal");
    const grasshopperBinPath = join(this.root, "..", "..", "ui", "gh", "Semio.Grasshopper", "bin", "Debug", "net48");
    const grasshopperExePath = join(grasshopperBinPath, `semio-engine${exeExt}`);
    const grasshopperInternalPath = join(grasshopperBinPath, "_internal");
    if (existsSync(grasshopperExePath)) rmSync(grasshopperExePath);
    if (existsSync(grasshopperInternalPath)) rmSync(grasshopperInternalPath, { recursive: true });
    copyFileSync(exePath, grasshopperExePath);
    cpSync(internalPath, grasshopperInternalPath, { force: true, recursive: true });
    console.log("✅ Post-build complete");
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] === "post") {
      new BuildPostScript(this.root, this.repoRoot).run();
      return;
    }
    const env = engineEnv(this.root);
    syncEngine(this.root);
    for (const d of ["build", "dist"]) {
      const p = join(this.root, d);
      if (existsSync(p)) rmSync(p, { recursive: true });
    }
    const addDataSep = process.platform === "win32" ? ";" : ":";
    const args = [
      "--name",
      "semio-engine",
      "--windowed",
      "--clean",
      "--noconfirm",
      "--copy-metadata",
      "ariadne",
      "--copy-metadata",
      "graphql",
      "--copy-metadata",
      "sqlalchemy",
      "--copy-metadata",
      "loguru",
      "--hidden-import=loguru",
      "--add-data",
      `schema.graphql${addDataSep}.`,
      "--add-data",
      `../../schema/openapi/schema.json${addDataSep}openapi/`,
      "--add-data",
      `../../../assets/icons/semio_512x512.png${addDataSep}icons/`,
      "--icon",
      "../../../assets/icons/semio.ico",
      "--paths",
      "../../../..",
      "main.py",
    ];
    execSync(`uv run pyinstaller ${args.join(" ")}`, { cwd: this.root, env, stdio: "inherit" });
    if (!process.argv.includes("--skip-post-build")) {
      execSync("bun ./script.ts build post", { cwd: this.root, stdio: "inherit" });
    }
    console.log("✅ Build complete");
  }
}

class TestScript extends BundleScript {
  run(): void {
    syncEngine(this.root);
    buildMcpApp(this.repoRoot);
    const openapiSchema = JSON.parse(readFileSync(join(this.repoRoot, "semio", "client", "schema", "openapi", "schema.json"), "utf8"));
    for (const [valid, message] of [
      [Boolean(openapiSchema.paths?.["/api/graphql"]?.post), "semio OpenAPI schema MUST expose the GraphQL store endpoint"],
      [Boolean(openapiSchema.components?.schemas?.GraphqlStoreRequest), "semio OpenAPI schema MUST define GraphqlStoreRequest"],
      [Boolean(openapiSchema.components?.schemas?.GraphqlStoreResponse), "semio OpenAPI schema MUST define GraphqlStoreResponse"],
    ] as const) {
      if (!valid) throw new Error(message);
    }
    execSync("uv run --python 3.14 python -m pytest --cov --cov-config=pyproject.toml --cov-report html", { cwd: this.root, env: engineEnv(this.root), stdio: "inherit" });
    console.log("✅ Tests complete");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
