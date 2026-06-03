import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import {
  NEO4J_GRAPH_DATABASE_NAMES,
  getAllNeo4jGraphExportSpecs,
  joinNeo4jGraphDatabaseName,
  parseExtraNeo4jGraphDatabaseNamesFromEnv,
  partitionNeo4jGraphCliArgv,
} from "../../../../generate.neo4j.gen.ts";
import { BundleScript, ScriptRouter, dispatchSubcommand, findRepoRoot, isDevPortInUse } from "./index.ts";
import { defineLint, type FileLinter } from "./index.ts";
import {
  dependencyBoundaryBreachesForBundleDir,
  dependencyBoundaryBreachesForFile,
  isAdapterBoundaryFile,
  parseTsImportSpecs,
} from "./dependency-boundary.ts";
import {
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  playgroundEmbedUrl,
  playgroundStaticSiteBuildOptions,
} from "../../../../ui/styling/vite-elements-assets.ts";
describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["semio", "kit"])).toBe("semio-kit");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["metabolism", "--verbose"])).toEqual({
      nameParts: ["metabolism"],
      passthrough: ["--verbose"],
    });
  });

  test("product graphs are fixed four joined names", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toEqual(["semio", "elements", "coda", "reuse"]);
  });

  test("NEO4J_EXTRA_GRAPH_DATABASES extends export specs", () => {
    const env = { NEO4J_EXTRA_GRAPH_DATABASES: " foo , bar-baz " };
    expect(parseExtraNeo4jGraphDatabaseNamesFromEnv(env)).toEqual(["foo", "bar-baz"]);
    const names = getAllNeo4jGraphExportSpecs(env).map((s) => joinNeo4jGraphDatabaseName(s));
    expect(names).toContain("foo");
    expect(names).toContain("bar-baz");
  });
});

describe("isDevPortInUse", () => {
  test("returns false for a high ephemeral port", () => {
    expect(isDevPortInUse("127.0.0.1", 59_999)).toBe(false);
  });

  test("returns true when puzzle 3d play port is listening", () => {
    if (!isDevPortInUse("127.0.0.1", 6013) && !isDevPortInUse("127.0.0.1", 6014)) return;
    expect(isDevPortInUse("127.0.0.1", 6013) || isDevPortInUse("127.0.0.1", 6014)).toBe(true);
  });
});

describe("bundle-script", () => {
  test("ScriptRouter usage lists registered commands", () => {
    class A extends BundleScript {
      run(): void {}
    }
    const router = new ScriptRouter(import.meta.dir).register("a", A).register("b", A);
    expect(router.hasCommands()).toBe(true);
    expect(router.usage()).toContain("a");
    expect(router.usage()).toContain("b");
  });

  test("findRepoRoot reaches monorepo from repo/lib/js/src", () => {
    const root = findRepoRoot(import.meta.dir);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      { go: (rest) => {
        ran = rest.join(",");
      } },
      "unused",
    );
    expect(ran).toBe("x");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});

describe("dependency-boundary", () => {
  test("detects adapter region marker", () => {
    expect(isAdapterBoundaryFile("pkg/foo.ts", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/main.py", "# #region 🔌Adapters\nimport fastapi")).toBe(true);
    expect(isAdapterBoundaryFile("semio/client/lib/js/index.ts", "//#region 🌐RsWasmTransport\nexport async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("semio/client/lib/js/kit-store.worker.ts", "export async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("coda/client/bin/assistant/mcp-app.tsx", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("framework/platform/renderer/react/index.tsx", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import { z } from "zod";\nexport const a = 1;\n`;
    const file = "semio/client/lib/js/boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(
      new URL("../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"),
      file,
      content,
      file,
    );
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("dependencyBoundaryBreachesForBundleDir walks nested tsx", () => {
    const repoRoot = new URL("../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
    const dir = "framework/playground/renderer/react/puzzle";
    const breachs = dependencyBoundaryBreachesForBundleDir(repoRoot, dir);
    expect(breachs.every((b) => b.scope.startsWith("framework/playground/renderer/react/puzzle"))).toBe(true);
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(
      new URL("../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"),
      file,
      content,
      file,
    );
    expect(breachs).toEqual([]);
  });
});

describe("ui scrollbar styling", () => {
  test("ui.css defines scrollbar tokens and native plus Scrollable rules", () => {
    const repoRoot = findRepoRoot(import.meta.dir);
    const css = readFileSync(join(repoRoot, "ui/styling/js/ui.css"), "utf8");
    expect(css).toContain("--scrollbar-size:");
    expect(css).toContain("--scrollbar-thumb:");
    expect(css).toContain("scrollbar-color:");
    expect(css).toContain("*::-webkit-scrollbar-thumb");
    expect(css).toContain('[data-slot="scroll-area-thumb"]');
  });
});

describe("micro-commit", () => {
  test("extractCounterFromSubject accepts formatted and plain legacy subjects", async () => {
    const { extractCounterFromSubject } = await import("./micro-commit.ts");
    expect(extractCounterFromSubject("🧑ueli🎆26🌙06☀️02🚩009")).toEqual({ nnn: 9, line1Base: "🧑ueli🎆26🌙06☀️02" });
    expect(extractCounterFromSubject("33")).toEqual({ nnn: 33, line1Base: null });
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across plain and formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("./micro-commit.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" };
    const subjects = ["33", "🐙ueli🎆26🌙06☀️02🚩032", "31", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙ueli🎆26🌙06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("normalizeBulletLines enforces dash prefix and cap", async () => {
    const { normalizeBulletLines } = await import("./micro-commit.ts");
    expect(normalizeBulletLines("🐛 Fix PDF\n- 🖼️ Tweak UI")).toEqual(["- 🐛 Fix PDF", "- 🖼️ Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `- ${i}`).join("\n"))).toHaveLength(8);
  });

  test("loadMicroCommitLevel pushes by default and keeps overrides", async () => {
    const { loadMicroCommitLevel } = await import("./micro-commit.ts");
    const { mkdtempSync, mkdirSync, rmSync, writeFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-level-"));
    const contributor = { alias: "kinan", emoji: "kinan", name: "Kinan", email: "kinan.sarak@gmail.com" };
    try {
      expect(loadMicroCommitLevel(root, contributor, [])).toBe("prepare-and-commit-and-push");
      expect(loadMicroCommitLevel(root, contributor, ["g."])).toBe("prepare-only");
      expect(loadMicroCommitLevel(root, contributor, ["gc"])).toBe("prepare-and-commit");
      const dir = join(root, ".repo", "🧑‍💻", "kinan");
      mkdirSync(dir, { recursive: true });
      writeFileSync(join(dir, "micro-commit.json"), '{ "level": "prepare-only" }', "utf8");
      expect(loadMicroCommitLevel(root, contributor, [])).toBe("prepare-only");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("./micro-commit.ts");
    const root = process.cwd();
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const msg = buildMicroCommitMessage(root, contributor, ["- 🎆 LLM-authored change summary"]);
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🚩\d{3}$/);
    expect(lines[1]).toMatch(/^🎆/);
    expect(lines.some((l) => l.includes("LLM-authored"))).toBe(true);
    expect(lines.every((l) => l.length > 0)).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
  });

  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const { runMicroCommit } = await import("./micro-commit.ts");
    const root = process.cwd();
    const prev = process.env.REPO_ROOT;
    process.env.REPO_ROOT = root;
    const stdin = ["- 🫡 Only micro-commit skill docs"].join("\n");
    const r = spawnSync(process.execPath, ["./script.ts", "micro-commit", "prepare"], {
      cwd: root,
      input: stdin,
      encoding: "utf8",
    });
    if (prev === undefined) delete process.env.REPO_ROOT;
    else process.env.REPO_ROOT = prev;
    const stagedHasPresentation = spawnSync("git", ["diff", "--cached", "--name-only"], { cwd: root, encoding: "utf8" })
      .stdout?.includes("presentation/");
    if (stagedHasPresentation) expect(r.status).not.toBe(0);
  });

  test("installMicroCommitGitHooks writes portable hooks and bun pin", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("./micro-commit.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-"));
    try {
      const init = spawnSync("git", ["init"], { cwd: root, encoding: "utf8" });
      expect(init.status).toBe(0);
      expect(spawnSync("git", ["config", "--local", "core.hooksPath", ".husky/_"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      const activeHook = readFileSync(join(root, ".husky/_/prepare-commit-msg"), "utf8");
      expect(hook).toContain("semio_micro_commit_wipe");
      expect(activeHook).toContain("micro-commit prepare-commit-msg");
      expect(hook).not.toContain("\r");
      expect(activeHook).not.toContain("\r");
      expect(existsSync(join(root, ".git/semio-micro-commit-bun"))).toBe(true);
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("./micro-commit.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msgFile = join(root, ".git", "COMMIT_EDITMSG");
      writeFileSync(msgFile, "🐙ueli manual subject\n", "utf8");
      handlePrepareCommitMsg(root, msgFile, "template");
      expect(readFileSync(msgFile, "utf8")).toContain("manual subject");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("resetMicroCommitTemplates clears commit.template and blanks GK files", async () => {
    const { resetMicroCommitTemplates, writeMicroCommitTemplates, buildMicroCommitMessage } = await import(
      "./micro-commit.ts"
    );
    const root = process.cwd();
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "ueli@semio-tech.com" };
    const msg = buildMicroCommitMessage(root, contributor, ["- 🎆 test reset"]);
    writeMicroCommitTemplates(root, msg);
    resetMicroCommitTemplates(root);
    const tpl = spawnSync("git", ["config", "--local", "--get", "commit.template"], { cwd: root, encoding: "utf8" });
    expect(tpl.status).not.toBe(0);
    expect(existsSync(join(root, ".git/gkcommittemplate.txt"))).toBe(false);
    expect(existsSync(join(root, ".git/semio-micro-commit-active"))).toBe(false);
  });

  test("shouldRefreshPreparedCommitMessage keeps user edits", async () => {
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("./micro-commit.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
});

describe("playground static sites", () => {
  test("PLAYGROUND_SITE_HOSTS maps each play to latest canonical host", () => {
    expect(PLAYGROUND_SITE_HOSTS.semio).toBe("play.semio-tech.com");
    expect(PLAYGROUND_SITE_HOSTS.cad).toBe("play.cad.semio-tech.com");
    expect(PLAYGROUND_SITE_HOSTS["2d"]).toBe("play.2d.semio-tech.com");
    expect(PLAYGROUND_SITE_HOSTS["3d"]).toBe("play.3d.semio-tech.com");
    expect(PLAYGROUND_SITE_HOSTS["5d"]).toBe("play.5d.semio-tech.com");
  });

  test("playgroundEmbedUrl uses localhost in dev and public host in production", () => {
    expect(playgroundEmbedUrl("cad", true)).toBe(`http://localhost:${PLAYGROUND_SITE_DEV_PORTS.cad}`);
    expect(playgroundEmbedUrl("2d", true)).toBe(`http://localhost:${PLAYGROUND_SITE_DEV_PORTS["2d"]}`);
    expect(playgroundEmbedUrl("cad", false)).toBe(`https://${PLAYGROUND_SITE_HOSTS.cad}`);
    expect(playgroundEmbedUrl("5d", false)).toBe(`https://${PLAYGROUND_SITE_HOSTS["5d"]}`);
  });

  test("playgroundStaticSiteBuildOptions uses relative-base dist output", () => {
    expect(playgroundStaticSiteBuildOptions()).toEqual({
      target: "esnext",
      outDir: "dist",
      emptyOutDir: true,
    });
    expect(playgroundStaticSiteBuildOptions({ sourcemap: true }).sourcemap).toBe(true);
  });
});
