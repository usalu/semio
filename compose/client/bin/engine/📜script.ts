#!/usr/bin/env bun
/** 🧭 Engine package router: `bun ./📜script.ts <build|test|dev mcp> [segments…]`. */
import { existsSync, readFileSync, rmSync, copyFileSync, cpSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, runBunx, runBundleScriptMain, runCmd, runTestBudgeted, resolveTestLevel, pytestLevelArgs, pytestCoverageArgs, spawnDaemon } from "../../../../repo/lib/js/index.ts";

class DevMcpScript extends BundleScript {
  run(): void {
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    runBunx(["vite", "build", "--config", "compose/client/bin/engine/vite.mcp-app.config.ts"], this.repoRoot);
    const daemon = spawnDaemon("npx", ["--yes", "@mcpjam/inspector@latest", "uv", "--directory", this.root, "run", "main.py", "--mcp-stdio"], {
      cwd: this.repoRoot,
      env: { ...process.env, HOST: host },
    });
    daemon.child.on("exit", (c) => process.exit(c ?? 0));
  }
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "mcp") {
      console.error("usage: bun ./📜script.ts dev mcp");
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
    const grasshopperBinPath = join(this.root, "..", "gh", "Semio.Grasshopper", "bin", "Debug", "net48");
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
    const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(this.root, ".venv") };
    runCmd("uv", ["sync", "--python", "3.14"], { cwd: join(this.root, "../.."), env });
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
      `🔗schema.graphql${addDataSep}.`,
      "--add-data",
      `../openapi/schema.json${addDataSep}openapi/`,
      "--add-data",
      `../asset/icon/semio_512x512.png${addDataSep}icon/`,
      "--icon",
      "../asset/icon/semio.ico",
      "main.py",
    ];
    runCmd("uv", ["run", "pyinstaller", ...args], { cwd: this.root, env, budgetMs: buildBudgetMs() });
    if (!process.argv.includes("--skip-post-build")) {
      runCmd("bun", ["./📜script.ts", "build", "post"], { cwd: this.root });
    }
    console.log("✅ Build complete");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(this.root, ".venv") };
    const python = process.platform === "win32" ? join(this.root, ".venv", "Scripts", "python.exe") : join(this.root, ".venv", "bin", "python");
    const pythonCommand = existsSync(python) ? python : "python";
    const assertSchema = (condition: boolean, message: string): void => {
      if (!condition) throw new Error(message);
    };
    const graphqlSchema = readFileSync(join(this.root, "..", "graphql", "🔗schema.graphql"), "utf8");
    const definitionBody = (definition: string): string => {
      const escapedDefinition = definition.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const match = graphqlSchema.match(new RegExp(`${escapedDefinition}\\s*\\{([\\s\\S]*?)\\n\\}`));
      assertSchema(Boolean(match), `${definition} MUST exist`);
      return match![1];
    };
    const disallowed = "leg" + "acy";
    assertSchema(!graphqlSchema.includes(disallowed), "semio GraphQL schema MUST NOT add compatibility-only field names in identifiers or generated descriptions");
    for (const definition of ["type Graph", "type Kit", "type Query", "type Mutation", "type Subscription", "type Type", "type Design", "type Piece", "type Connection", "input KitReadPointInput"]) {
      definitionBody(definition);
    }
    const graphBody = definitionBody("type Graph");
    assertSchema(graphBody.includes("theKit(at: KitReadPointInput): Kit") && !graphBody.includes("kitReadScope"), "Graph MUST expose theKit(at:) materialized reads gated by KitReadPointInput");
    const kitReadPointBody = definitionBody("input KitReadPointInput");
    for (const field of ["theKit:", "checkpointId:", "checkpointChangeId:", "checkpointOperationId:", "alternativeId:", "draftAlternativeId:", "draftId:", "draftChangeId:", "draftTransactionId:", "draftOperationId:"]) {
      assertSchema(kitReadPointBody.includes(field), `KitReadPointInput MUST expose ${field}`);
    }
    const mutationBody = definitionBody("type Mutation");
    assertSchema(mutationBody.includes("renameKit("), "Mutation MUST expose renameKit");
    assertSchema(mutationBody.includes("addFixedPieceToDesign(") && mutationBody.includes("dragPieceInDesign("), "Mutation MUST expose flat draft/transaction-scoped kit mutators");
    const subscriptionBody = definitionBody("type Subscription");
    assertSchema(subscriptionBody.includes("commandSucceeded: Command!") && subscriptionBody.includes("operationSucceeded: OperationKind!"), "Subscription MUST expose Rust command/operation streams");
    await runTestBudgeted(pythonCommand, ["-c", "from pathlib import Path; from ariadne import gql, make_executable_schema; s=Path('../graphql/🔗schema.graphql').read_text(encoding='utf-8'); make_executable_schema(gql(s))"], {
      cwd: this.root,
      env,
    });
    const openapiSchema = JSON.parse(readFileSync(join(this.root, "..", "openapi", "schema.json"), "utf8"));
    assertSchema(Boolean(openapiSchema.paths?.["/api/graphql"]?.post), "semio OpenAPI schema MUST expose the GraphQL store endpoint");
    assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreRequest), "semio OpenAPI schema MUST define GraphqlStoreRequest");
    assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreResponse), "semio OpenAPI schema MUST define GraphqlStoreResponse");
    await runTestBudgeted(pythonCommand, ["-m", "pytest", "--cov", "--cov-config=pyproject.toml", "--cov-report", "html", ...pytestLevelArgs(level), ...pytestCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root, env });
    console.log("✅ Tests complete");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
