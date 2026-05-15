#!/usr/bin/env bun
/**
 * 🧭 Engine package router: `bun ./script.ts <build|test|…> [segments…]`.
 */
import { execFileSync, execSync, spawn } from "node:child_process";
import { existsSync, readFileSync, rmSync, copyFileSync, cpSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const root = join(cwd, "..", "..", "..", "..");
const segs = process.argv.slice(2);
const verb = segs[0];

if (verb === "dev" && segs[1] === "mcp") {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  execSync("bunx vite build --config semio/client/bin/engine/vite.mcp-app.config.ts", { cwd: root, stdio: "inherit", shell: true });
  const child = spawn(
    "npx",
    ["--yes", "@mcpjam/inspector@latest", "uv", "--directory", cwd, "run", "main.py", "--mcp-stdio"],
    { stdio: "inherit", shell: true, env: { ...process.env, HOST: host }, cwd: root },
  );
  child.on("exit", (c) => process.exit(c ?? 0));
} else if (verb === "build" && segs[1] === "post") {
  const exeExt = process.platform === "win32" ? ".exe" : "";
  const exePath = join(cwd, "dist", "semio-engine", `semio-engine${exeExt}`);
  const internalPath = join(cwd, "dist", "semio-engine", "_internal");
  const grasshopperBinPath = join(cwd, "..", "gh", "Semio.Grasshopper", "bin", "Debug", "net48");
  const grasshopperExePath = join(grasshopperBinPath, `semio-engine${exeExt}`);
  const grasshopperInternalPath = join(grasshopperBinPath, "_internal");
  if (existsSync(grasshopperExePath)) rmSync(grasshopperExePath);
  if (existsSync(grasshopperInternalPath)) rmSync(grasshopperInternalPath, { recursive: true });
  copyFileSync(exePath, grasshopperExePath);
  cpSync(internalPath, grasshopperInternalPath, { force: true, recursive: true });
  console.log("✅ Post-build complete");
} else if (verb === "build") {
  const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(cwd, ".venv") };
  execSync("uv sync --python 3.14", { cwd: join(cwd, "../.."), env, stdio: "inherit" });
  for (const d of ["build", "dist"]) {
    const p = join(cwd, d);
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
    `../openapi/schema.json${addDataSep}openapi/`,
    "--add-data",
    `../assets/icons/semio_512x512.png${addDataSep}icons/`,
    "--icon",
    "../assets/icons/semio.ico",
    "main.py",
  ];
  execSync(`uv run pyinstaller ${args.join(" ")}`, { cwd, env, stdio: "inherit" });
  if (!process.argv.includes("--skip-post-build")) {
    execSync("bun ./script.ts build post", { cwd, stdio: "inherit" });
  }
  console.log("✅ Build complete");
} else if (verb === "test") {
  const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(cwd, ".venv") };
  const python = process.platform === "win32" ? join(cwd, ".venv", "Scripts", "python.exe") : join(cwd, ".venv", "bin", "python");
  const pythonCommand = existsSync(python) ? python : "python";
  const assertSchema = (condition: boolean, message: string): void => {
    if (!condition) throw new Error(message);
  };
  const graphqlSchema = readFileSync(join(cwd, "..", "graphql", "schema.graphql"), "utf8");
  const definitionBody = (definition: string): string => {
    const escapedDefinition = definition.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const match = graphqlSchema.match(new RegExp(`${escapedDefinition}\\s*\\{([\\s\\S]*?)\\n\\}`));
    assertSchema(Boolean(match), `${definition} MUST exist`);
    return match![1];
  };
  const disallowed = "leg" + "acy";
  assertSchema(!graphqlSchema.includes(disallowed), "semio GraphQL schema MUST NOT add compatibility-only field names in identifiers or generated descriptions");
  for (const definition of [
    "type Graph",
    "type Kit",
    "type Query",
    "type Mutation",
    "type Subscription",
    "type Type",
    "type Design",
    "type Piece",
    "type Connection",
    "input KitReadPointInput",
  ]) {
    definitionBody(definition);
  }
  const graphBody = definitionBody("type Graph");
  assertSchema(
    graphBody.includes("theKit(at: KitReadPointInput): Kit") && !graphBody.includes("kitReadScope"),
    "Graph MUST expose theKit(at:) materialized reads gated by KitReadPointInput",
  );
  const kitReadPointBody = definitionBody("input KitReadPointInput");
  for (const field of ["theKit:", "checkpointId:", "checkpointChangeId:", "checkpointOperationId:", "alternativeId:", "draftAlternativeId:", "draftId:", "draftChangeId:", "draftTransactionId:", "draftOperationId:"]) {
    assertSchema(kitReadPointBody.includes(field), `KitReadPointInput MUST expose ${field}`);
  }
  const mutationBody = definitionBody("type Mutation");
  assertSchema(mutationBody.includes("renameKit("), "Mutation MUST expose renameKit");
  assertSchema(mutationBody.includes("addFixedPieceToDesign(") && mutationBody.includes("dragPieceInDesign("), "Mutation MUST expose flat draft/transaction-scoped kit mutators");
  const subscriptionBody = definitionBody("type Subscription");
  assertSchema(
    subscriptionBody.includes("commandSucceeded: Command!") && subscriptionBody.includes("operationSucceeded: OperationKind!"),
    "Subscription MUST expose Rust command/operation streams",
  );
  execFileSync(
    pythonCommand,
    ["-c", "from pathlib import Path; from ariadne import gql, make_executable_schema; s=Path('../graphql/schema.graphql').read_text(encoding='utf-8'); make_executable_schema(gql(s))"],
    { cwd, env, stdio: "inherit" },
  );
  const openapiSchema = JSON.parse(readFileSync(join(cwd, "..", "openapi", "schema.json"), "utf8"));
  assertSchema(Boolean(openapiSchema.paths?.["/api/graphql"]?.post), "semio OpenAPI schema MUST expose the GraphQL store endpoint");
  assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreRequest), "semio OpenAPI schema MUST define GraphqlStoreRequest");
  assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreResponse), "semio OpenAPI schema MUST define GraphqlStoreResponse");
  execFileSync(pythonCommand, ["-m", "pytest", "--cov", "--cov-config=pyproject.toml", "--cov-report", "html"], { cwd, env, stdio: "inherit" });
  console.log("✅ Tests complete");
} else {
  console.error("usage: bun ./script.ts <build|build post|test|dev mcp>");
  process.exit(1);
}
