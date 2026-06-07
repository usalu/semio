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
import { BundleScript, ScriptRouter, dispatchSubcommand, findRepoRoot, isDevPortInUse, resolveDevPort } from "./index.ts";
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

describe("resolveDevPort", () => {
  test("returns preferred port when free", () => {
    expect(resolveDevPort("127.0.0.1", 59_990)).toBe(59_990);
  });

  test("skips occupied ports", () => {
    if (!isDevPortInUse("127.0.0.1", 6013)) return;
    expect(resolveDevPort("127.0.0.1", 6013)).toBeGreaterThan(6013);
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

  test("gitRepoRoot uses monorepo toplevel from repo/lib/js/src", async () => {
    const { gitRepoRoot } = await import("./uloc-metrics.ts");
    const { spawnSync } = await import("node:child_process");
    const top = gitRepoRoot(import.meta.dir);
    const n = Number(spawnSync("git", ["ls-files"], { cwd: top, encoding: "utf8" }).stdout?.split("\n").filter(Boolean).length ?? 0);
    expect(n).toBeGreaterThan(1000);
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
  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("./micro-commit.ts");
    expect(extractCounterFromSubject("🧑ueli🎆26🌙06☀️02🚩009")).toEqual({ nnn: 9, line1Base: "🧑ueli🎆26🌙06☀️02" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("./micro-commit.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙ueli🎆26🌙06☀️02🚩033", "🐙ueli🎆26🌙06☀️02🚩032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙ueli🎆26🌙06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("./micro-commit.ts");
    const bullets = normalizeBulletLines("🎆Summary\n📊uloc➕1✏️1➖0🟰2\n🟦65k➕1✏️1➖0\n🐛Fix bug");
    expect(bullets).toEqual(["🎆Summary", "🐛Fix bug"]);
  });

  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("./micro-commit.ts");
    expect(bulletLeadEmoji("🎆Drop stacked intro")).toBe("🎆");
    expect(bulletEmojiValidationError(["🎆All bullets wrongly use fireworks"])).toContain("🎆");
    expect(bulletEmojiValidationError(["🐛Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊uloc block"])).toContain("📊");
  });

  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("./micro-commit.ts");
    expect(formatMicroCommitBulletLine("- 🐛 Fix PDF")).toBe("🐛Fix PDF");
    expect(normalizeBulletLines("🐛 Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆item ${i}`).join("\n"))).toHaveLength(8);
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("./micro-commit.ts");
    const root = process.cwd();
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const msg = buildMicroCommitMessage(root, contributor, ["🎆LLM-authored change summary"], {
      countRepoByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),
    });
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🚩\d{3}$/);
    expect(lines[1]).toMatch(/^🎆/);
    expect(lines.some((l) => l.includes("LLM-authored"))).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
    expect(lines.at(-2)).toBe("");
    const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("./uloc-metrics.ts");
    const metricsIdx = lines.findIndex((l) => l.startsWith(ulocHeader));
    if (metricsIdx >= 0) {
      expect(lines[metricsIdx - 1]).toBe("");
      expect(lines[metricsIdx]?.startsWith(ulocHeader)).toBe(true);
      expect(lines[metricsIdx + 1]).toMatch(/^🟦|^🦀|🐍|🐚|🔵|🟣|📝|🧾|📋/);
    }
  });

  test("formatMicroCommitMetricsLines uses compact loc and delta counts", async () => {
    const {
      formatMicroCommitMetricLine,
      formatMicroCommitMetricsLines,
      formatMetricLocCount,
      MICRO_COMMIT_ULOC_HEADER,
    } = await import("./uloc-metrics.ts");
    expect(MICRO_COMMIT_ULOC_HEADER).toBe("📊uloc");
    expect(formatMetricLocCount(200_000)).toBe("200k");
    expect(formatMetricLocCount(500)).toBe("500");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 0, added: 0, removed: 0 })).toBe(
      "🐚2k",
    );
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 2, added: 2, removed: 0 })).toBe(
      "🐚2k➕2✏️2🟰4",
    );
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 2, added: 0, removed: 2 })).toBe(
      "🐚2k✏️2➖2🟰4",
    );
    const lines = formatMicroCommitMetricsLines([
      { lang: "Rust", emoji: "🦀", code: 200_000, edited: 2220, added: 2000, removed: 500 },
    ]);
    expect(lines).toEqual([
      "📊uloc➕2000✏️2220➖500🟰4720",
      "🦀200k➕2000✏️2220➖500🟰4720",
    ]);
  });

  test("formatMicroCommitMetricsLines totals all languages on the first row", async () => {
    const { formatMicroCommitMetricsLines } = await import("./uloc-metrics.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊uloc➕8✏️10🟰18");
    expect(lines[1]).toBe("🟦3k➕8✏️10🟰18");
    expect(lines[2]).toBe("📝44");
  });

  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("./uloc-metrics.ts");
    const root = process.cwd();
    const metrics = buildMicroCommitMetrics(root, {
      countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }),
    });
    expect(metrics.some((m) => m.lang === "Rust" && m.code === 100)).toBe(true);
  });

  test("isUlocCachePlausible rejects partial caches", async () => {
    const { isUlocCachePlausible, gitRepoRoot } = await import("./uloc-metrics.ts");
    const root = gitRepoRoot(process.cwd());
    expect(isUlocCachePlausible(root, { TypeScript: 3000, JavaScript: 78, Markdown: 44, JSON: 43 })).toBe(false);
    expect(
      isUlocCachePlausible(root, {
        JSON: 400_000,
        TypeScript: 150_000,
        Go: 90_000,
        Rust: 44_000,
        Markdown: 10_000,
        Python: 30_000,
      }),
    ).toBe(true);
  });

  test("shouldSkipPathForUloc skips dot paths license templates and .repo", async () => {
    const { shouldSkipPathForUloc } = await import("./uloc-metrics.ts");
    const root = process.cwd();
    expect(shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, ".agents/skills/micro-commit/SKILL.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "semio/client/ui/LICENSE.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "repo/AGENTS.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "repo/CHANGELOG.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, ".repo/cache/x")).toBe(true);
    expect(shouldSkipPathForUloc(root, "framework/core/README.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "puzzle/3d/src/foo.ts")).toBe(false);
  });

  test("countJsonKeys counts nested object keys", async () => {
    const { countJsonKeys } = await import("./uloc-metrics.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });

  test("appendGitDeltaSuffix formats user example totals", async () => {
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("./uloc-metrics.ts");
    expect(appendGitDeltaSuffix("🟦65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦65k➕700✏️200➖10🟰910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 })).toBe("📊uloc➕700✏️200➖10🟰910");
  });

  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("./uloc-metrics.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });

  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("./uloc-metrics.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });

  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("./micro-commit.ts");
    const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md", "framework/product/playground/renderer/react/index.tsx"];
    expect(uncoveredStagedAreas(["🫡Only micro-commit skill wording"], staged)).toContain(".cursor/plans");
    expect(uncoveredStagedAreas(["🫡Only micro-commit skill wording"], staged)).toContain("product");
    const ok = uncoveredStagedAreas(
      [
        "📋Plan brush edge resurrection guard and sync",
        "🖌️Playground renderer restores brush placement after structural deletes",
      ],
      staged,
    );
    expect(ok).toEqual([]);
  });

  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const { runMicroCommit } = await import("./micro-commit.ts");
    const root = process.cwd();
    const prev = process.env.REPO_ROOT;
    process.env.REPO_ROOT = root;
    const stdin = ["🫡Only micro-commit skill docs"].join("\n");
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
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("semio_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(existsSync(join(root, ".repo/semio-micro-commit-bun"))).toBe(true);
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

  test("wipeAfterCommit clears all GK templates and prepare state", async () => {
    const { wipeAfterCommit, writeMicroCommitTemplates, buildMicroCommitMessage } = await import("./micro-commit.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-wipe-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "u@example.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "U"], { cwd: root });
      const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" };
      const msg = buildMicroCommitMessage(root, contributor, ["🎆test reset"]);
      writeMicroCommitTemplates(root, msg);
      writeFileSync(join(root, ".git/gkcommittemplate-099.txt"), "stale numbered", "utf8");
      wipeAfterCommit(root);
      const tpl = spawnSync("git", ["config", "--local", "--get", "commit.template"], { cwd: root, encoding: "utf8" });
      expect(tpl.status).toBe(0);
      expect(tpl.stdout?.trim()).toContain("gkcommittemplate.txt");
      expect(readFileSync(join(root, ".git/gkcommittemplate.txt"), "utf8")).toBe("");
      const gkLeft = readdirSync(join(root, ".git")).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gkLeft).toEqual(["gkcommittemplate.txt"]);
      expect(existsSync(join(root, ".git/semio-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(root, ".git/COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates, buildMicroCommitMessage } = await import("./micro-commit.ts");
    const { mkdtempSync, readdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-tpl-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msg = buildMicroCommitMessage(
        root,
        { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" },
        ["🎆bullet"],
      );
      writeMicroCommitTemplates(root, msg);
      const gk = readdirSync(join(root, ".git")).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gk).toEqual(["gkcommittemplate.txt"]);
      const tpl = spawnSync("git", ["config", "--local", "--get", "commit.template"], { cwd: root, encoding: "utf8" });
      expect(tpl.stdout?.trim()).toContain("gkcommittemplate.txt");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
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

describe("commit", () => {
  test("parseCommitBundleBody reads emoji scopes dates and bullets", async () => {
    const { parseCommitBundleBody } = await import("./commit.ts");
    const bundles = parseCommitBundleBody(
      "🏘️semio✍️sketchpad\n🎆26🌙06☀️04\n🗺️Map work\n🎆26🌙06☀️03\n🧪Playground\n\n🖱️ui⚛️react\n🎆26🌙06☀️02\n🖥️Shell",
    );
    expect(bundles).toHaveLength(2);
    expect(bundles[0]?.label).toBe("🏘️semio✍️sketchpad");
    expect(bundles[0]?.dates).toHaveLength(2);
    expect(bundles[0]?.dates[0]?.bullets[0]).toBe("🗺️Map work");
  });

  test("parseCommitBundleBody rejects path prefixes and reserved emojis", async () => {
    const { parseCommitBundleBody } = await import("./commit.ts");
    expect(() =>
      parseCommitBundleBody("semio/foo|🏘️semio\n🎆26🌙06☀️04\n🗺️Map work"),
    ).toThrow();
    expect(() =>
      parseCommitBundleBody("🏘️semio🔀📊uloc\n🎆26🌙06☀️04\n🗺️Map work"),
    ).toThrow();
    expect(() => parseCommitBundleBody("🗺️🧩🕸️\n🎆26🌙06☀️04\n🗺️Map work")).toThrow();
  });

  test("normalizeBundleScopeLabel strips reserved and uloc suffix", async () => {
    const { normalizeBundleScopeLabel } = await import("./commit.ts");
    expect(normalizeBundleScopeLabel("🏘️semio🔀📊uloc➕1")).toBe("🏘️semio");
  });

  test("isBundleScopeLine accepts area and technology root labels", async () => {
    const { isBundleScopeLine } = await import("./commit.ts");
    expect(isBundleScopeLine("🌐gis📍map")).toBe(true);
    expect(isBundleScopeLine("🖱️ui⚛️react")).toBe(true);
    expect(isBundleScopeLine("🥅framework")).toBe(true);
    expect(isBundleScopeLine("🧪Playground")).toBe(false);
    expect(isBundleScopeLine("🗺️Single emoji line")).toBe(false);
  });

  test("extractBundleDateLineFromSubject reads calendar day from micro-commit subject", async () => {
    const { extractBundleDateLineFromSubject } = await import("./commit.ts");
    expect(extractBundleDateLineFromSubject("🐙ueli🎆26🌙06☀️04🚩012")).toBe("🎆26🌙06☀️04");
    expect(extractBundleDateLineFromSubject("unrelated")).toBeNull();
  });

  test("extractBundleDateLineFromCommit prefers body timestamp over subject checkpoint day", async () => {
    const { extractBundleDateLineFromCommit, extractBundleDateLineFromCommitBody } = await import("./commit.ts");
    const body = "🎆26🌙06☀️04⏰02⌚38⏱️38\n🗺️Map work\n";
    expect(extractBundleDateLineFromCommitBody(body)).toBe("🎆26🌙06☀️04");
    expect(extractBundleDateLineFromCommit("🐙ueli🎆26🌙06☀️02🚩084", body)).toBe("🎆26🌙06☀️04");
  });

  test("pathsFromNumstatRow expands rename paths", async () => {
    const { pathsFromNumstatRow } = await import("./commit.ts");
    expect(pathsFromNumstatRow("old/a.ts\tnew/b.ts")).toEqual(["old/a.ts", "new/b.ts"]);
    expect(pathsFromNumstatRow("dir/{old.ts => new.ts}")).toEqual(["old.ts", "new.ts"]);
  });

  test("pathMatchesBundleIndex does not treat empty prefix set as match-all", async () => {
    const { pathMatchesBundleIndex } = await import("./commit.ts");
    const bundles = [
      { label: "🏘️semio✍️sketchpad", dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["✍️x"] }] },
      { label: "🏘️semio🗃️fixtures", dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["🗃️y"] }] },
    ];
    const prefixSets = [[], ["semio/fixtures"]];
    expect(pathMatchesBundleIndex("semio/fixtures/a.json", 0, prefixSets, bundles)).toBe(false);
    expect(pathMatchesBundleIndex("semio/fixtures/a.json", 1, prefixSets, bundles)).toBe(true);
  });

  test("formatBundleDateLine appends per-day uloc suffix", async () => {
    const { formatBundleDateLine } = await import("./commit.ts");
    expect(formatBundleDateLine("🎆26🌙06☀️04", { added: 700, edited: 200, removed: 10 })).toBe(
      "🎆26🌙06☀️04📊uloc➕700✏️200➖10🟰910",
    );
  });

  test("commitBundleBodyError rejects per-day uloc on stdin", async () => {
    const { commitBundleBodyError } = await import("./commit.ts");
    expect(commitBundleBodyError("🏘️semio\n🎆26🌙06☀️04📊uloc➕1\n🗺️Work")).toMatch(/per-day/);
  });

  test("validateMicroCommitLangMetricsDeltaSum passes when language rows sum to footer", async () => {
    const { validateMicroCommitLangMetricsDeltaSum } = await import("./uloc-metrics.ts");
    expect(() =>
      validateMicroCommitLangMetricsDeltaSum([
        { lang: "TypeScript", emoji: "🟦", code: 100, edited: 5, added: 3, removed: 0 },
        { lang: "Rust", emoji: "🦀", code: 50, edited: 2, added: 1, removed: 1 },
      ]),
    ).not.toThrow();
  });

  test("validateBundleCommitAttribution requires bundle headers to sum to range total", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("./commit.ts");
    const root = mkdtempSync(join(tmpdir(), "semio-commit-check-sum-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      mkdirSync(join(root, "other"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙ueli🎆26🌙06☀️01🔀"], { cwd: root });
      const wip = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\nc\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      const msg = join(root, "mc.txt");
      writeFileSync(msg, "🐙ueli🎆26🌙06☀️01🚩001\n\n🎆26🌙06☀️04⏰12⌚00⏱️00\n🔧Work\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg], { cwd: root });
      const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Only repo");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(
        /not attributed to any bundle|do not add up/,
      );
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleCommitAttribution rejects listed days when micro-commit churn does not net to range", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("./commit.ts");
    const root = mkdtempSync(join(tmpdir(), "semio-commit-churn-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙ueli🎆26🌙06☀️01🔀"], { cwd: root });
      const wip = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\ne\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      writeFileSync(
        join(root, "mc1.txt"),
        "🐙ueli🎆26🌙06☀️01🚩001\n\n🎆26🌙06☀️03⏰12⌚00⏱️00\n🔧Add lines\n",
        "utf8",
      );
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", join(root, "mc1.txt")], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      writeFileSync(
        join(root, "mc2.txt"),
        "🐙ueli🎆26🌙06☀️01🚩002\n\n🎆26🌙06☀️04⏰12⌚01⏱️00\n🔧Net fewer lines\n",
        "utf8",
      );
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", join(root, "mc2.txt")], { cwd: root });
      const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Net\n🎆26🌙06☀️03\n🔧Add");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/does not add up/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleDayDeltasAttribution rejects when listed days do not sum to bundle total", async () => {
    const { validateBundleDayDeltasAttribution } = await import("./commit.ts");
    const bundles = [
      {
        label: "📚repo🔧js",
        dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["🔧Only day four listed"] }],
      },
    ];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆26🌙06☀️04", { added: 2, edited: 0, removed: 0 }],
          ["🎆26🌙06☀️03", { added: 5, edited: 0, removed: 0 }],
        ]),
      ],
    ]);
    expect(() =>
      validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltas, [{ added: 2, edited: 0, removed: 0 }]),
    ).toThrow(/missing from your bundle body/);
    const dateDeltasOneDay = new Map([[0, new Map([["🎆26🌙06☀️04", { added: 2, edited: 0, removed: 0 }]])]]);
    expect(() =>
      validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltasOneDay, [{ added: 7, edited: 0, removed: 0 }]),
    ).toThrow(/does not add up/);
  });

  test("buildCommitMessage appends per-day uloc from micro-commit dates", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { buildCommitMessage, parseCommitBundleBody } = await import("./commit.ts");
    const root = mkdtempSync(join(tmpdir(), "semio-commit-day-uloc-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙ueli🎆26🌙06☀️01🔀"], { cwd: root });
      const wip = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg1 = join(tmpdir(), `semio-mc1-${Date.now()}.txt`);
      writeFileSync(
        msg1,
        "🐙ueli🎆26🌙06☀️01🚩001\n\n🎆26🌙06☀️03⏰12⌚00⏱️00\n🔧Day three\n",
        "utf8",
      );
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg1], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg2 = join(tmpdir(), `semio-mc2-${Date.now()}.txt`);
      writeFileSync(
        msg2,
        "🐙ueli🎆26🌙06☀️01🚩002\n\n🎆26🌙06☀️04⏰12⌚01⏱️00\n🔧Day four\n",
        "utf8",
      );
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg2], { cwd: root });
      const contributor = { alias: "ueli", emoji: "🐙", name: "U", email: "u@e.com" };
      const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Day four\n🎆26🌙06☀️03\n🔧Day three");
      const msg = buildCommitMessage(root, contributor, bundles, wip, "HEAD");
      expect(msg).toMatch(/🎆26🌙06☀️04📊uloc➕\d+🟰\d+/);
      expect(msg).toMatch(/🎆26🌙06☀️03📊uloc➕\d+🟰\d+/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("sortCommitBundlesByEditTotal orders bundles by descending gitDeltaLineTotal", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { sortCommitBundlesByEditTotal } = await import("./commit.ts");
    const mk = (label: string) => ({
      label,
      dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["🗺️change"] }],
    });
    const root = mkdtempSync(join(tmpdir(), "semio-commit-sort-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "semio"), { recursive: true });
      mkdirSync(join(root, "ui"), { recursive: true });
      mkdirSync(join(root, "framework"), { recursive: true });
      writeFileSync(join(root, "semio/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "ui/b.ts"), "b\n", "utf8");
      writeFileSync(join(root, "framework/c.ts"), "c\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "init"], { cwd: root });
      const base = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "semio/a.ts"), `${"a\n".repeat(11)}`, "utf8");
      writeFileSync(join(root, "ui/b.ts"), `${"b\n".repeat(151)}`, "utf8");
      writeFileSync(join(root, "framework/c.ts"), `${"c\n".repeat(5)}`, "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "delta"], { cwd: root });
      const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      const bundles = [mk("🏘️semio"), mk("🖱️ui"), mk("🥅framework")];
      const paths = [["semio/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];
      const sorted = sortCommitBundlesByEditTotal(root, base, head, bundles, paths);
      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️semio", "🥅framework"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildCommitMessage renders bundle subject and footer", async () => {
    const { buildCommitMessage, parseCommitBundleBody } = await import("./commit.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Tooling");
    const msg = buildCommitMessage(
      process.cwd(),
      contributor,
      bundles,
      "0000000000000000000000000000000000000000",
      "0000000000000000000000000000000000000000",
      { countRepoByLanguage: () => ({ TypeScript: 1000 }) },
    );
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🔀$/);
    expect(lines.some((l) => l.includes("📊uloc"))).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
  });

  test("formatBundleTagName and formatBundleSubject use contributor date emojis", async () => {
    const { formatBundleTagName, formatBundleSubject } = await import("./commit.ts");
    const c = { alias: "ueli", emoji: "🐙", name: "U", email: "u@e.com" };
    const now = new Date("2026-06-04T12:00:00");
    expect(formatBundleTagName(c, now)).toBe("🐙ueli🎆26🌙06☀️04🚩");
    expect(formatBundleSubject(c, now)).toBe("🐙ueli🎆26🌙06☀️04🔀");
  });

  test("formatCommitPrepareCommands emits four fenced git blocks", async () => {
    const { formatCommitPrepareCommands } = await import("./commit.ts");
    const out = formatCommitPrepareCommands({
      tagName: "🐙ueli🎆26🌙06☀️04🚩",
      wipSha: "abc123def456",
      messageFile: ".git/semio-commit-message",
    });
    const blocks = out.trimEnd().split("\n\n");
    expect(blocks).toHaveLength(4);
    expect(blocks[0]).toBe(
      "```\ngit tag -s -m '🐙ueli🎆26🌙06☀️04🚩' '🐙ueli🎆26🌙06☀️04🚩' HEAD\n```",
    );
  });

  test("formatCommitPrepareAgentReply ends with tag name and commit message blocks", async () => {
    const { formatCommitPrepareAgentReply } = await import("./commit.ts");
    const commitMessage = "🐙ueli🎆26🌙06☀️04🔀\n\n🏘️semio✍️sketchpad📊uloc\n🎆26🌙06☀️04\n🗺️Work\n\n📊uloc➕1🟰1\n\nSigned-off-by: U <u@e.com>\n";
    const out = formatCommitPrepareAgentReply({
      tagName: "🐙ueli🎆26🌙06☀️04🚩",
      wipSha: "abc",
      commitMessage,
    });
    const fenceBodies = [...out.matchAll(/```\n([\s\S]*?)\n```/g)].map((m) => m[1]!);
    expect(fenceBodies).toHaveLength(6);
    expect(fenceBodies[4]).toBe("🐙ueli🎆26🌙06☀️04🚩");
    expect(fenceBodies[5]).toBe(commitMessage.trimEnd());
  });

  test("parseCommitSteps treats cs as squash without tag", async () => {
    const { parseCommitSteps } = await import("./commit.ts");
    expect(parseCommitSteps(["cs"])).toEqual({ tag: false, squash: true, push: false });
    expect(parseCommitSteps(["ct", "cs", "cp"])).toEqual({ tag: true, squash: true, push: true });
  });

  test("bulletMatchesCommitHistory detects verbatim prior commit lines", async () => {
    const { bulletMatchesCommitHistory } = await import("./commit.ts");
    const history = new Set(["🗺️copied line from an old micro-commit"]);
    expect(bulletMatchesCommitHistory("🗺️copied line from an old micro-commit", history)).toBe(true);
    expect(bulletMatchesCommitHistory("🗺️fresh summary written from git diff", history)).toBe(false);
  });
});
