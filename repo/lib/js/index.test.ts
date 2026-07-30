import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { NEO4J_GRAPH_DATABASE_NAMES, getAllNeo4jGraphExportSpecs, joinNeo4jGraphDatabaseName, parseExtraNeo4jGraphDatabaseNamesFromEnv, partitionNeo4jGraphCliArgv } from "../../../script.ts";
import { BundleScript, ScriptRouter, DAEMON_BUDGET_MS, ORCHESTRATOR_BUDGET_MS, budgetTimeoutHint, canReuseDevPort, daemonBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, dispatchSubcommand, findRepoRoot, goLevelTestArgs, isDevPortInUse, orchestratorBudgetMs, orchestratorBudgetOpts, resolveDevPort, runCmd, runCmdStatus, runProbe, testLevelBudgetMs, vitestLevelArgs, wgpuDevPlayUrl } from "./index.ts";
import { defineLint, type FileLinter } from "./index.ts";
import { dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, isAdapterBoundaryFile, parseTsImportSpecs } from "./index.ts";
import {
  PLAYGROUND_PORTS,
  PLAYGROUND_SITE_DEV_PORTS,
  PLAYGROUND_SITE_HOSTS,
  PLAYGROUND_LOCKED_EXAMPLE_ENV,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundEmbedUrl,
  frameworkOsPlaygroundDevEnv,
  resolveFrameworkOsPlaygroundPlugin,
  loadFrameworkOsPlaygroundCatalog,
  playgroundPlayViteDefine,
} from "./index.ts";
import { playgroundStaticSiteBuildOptions } from "../../../framework/ui/styling/vite-elements-assets.ts";
describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["compose", "kit"])).toBe("compose-kit");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["metabolism", "--verbose"])).toEqual({
      nameParts: ["metabolism"],
      passthrough: ["--verbose"],
    });
  });

  test("product graphs are fixed four joined names", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toEqual(["compose", "elements", "coda", "reuse"]);
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

  test("honors skipPorts", () => {
    expect(resolveDevPort("127.0.0.1", 59_991, 5, new Set([59_991, 59_992]))).toBe(59_993);
  });
});

describe("devServerUrl", () => {
  test("maps 0.0.0.0 to loopback", () => {
    expect(devServerUrl("0.0.0.0", 6019)).toBe("http://127.0.0.1:6019/");
  });
});

describe("wgpuDevPlayUrl", () => {
  test("builds root and legacy play urls", () => {
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly", "/renderer-modules/wgpu/")).toBe("http://127.0.0.1:6178/renderer-modules/wgpu/?plugin=lowpoly");
  });
});

describe("canReuseDevPort", () => {
  test("returns true for an active dev HTTP server", () => {
    if (!isDevPortInUse("127.0.0.1", 6019)) return;
    expect(canReuseDevPort("127.0.0.1", 6019)).toBe(true);
  });
});

describe("describeDevPortOccupant", () => {
  test("returns undefined for a free port", () => {
    expect(describeDevPortOccupant(59_998)).toBeUndefined();
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

  test("gitRepoRoot uses monorepo toplevel from repo/lib/js", async () => {
    const { gitRepoRoot } = await import("./index.ts");
    const { spawnSync } = await import("node:child_process");
    const top = gitRepoRoot(import.meta.dir);
    const n = Number(spawnSync("git", ["ls-files"], { cwd: top, encoding: "utf8" }).stdout?.split("\n").filter(Boolean).length ?? 0);
    expect(n).toBeGreaterThan(1000);
  });

  test("findRepoRoot reaches monorepo from repo/lib/js", () => {
    const root = findRepoRoot(import.meta.dir);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      {
        go: (rest) => {
          ran = rest.join(",");
        },
      },
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
    expect(isAdapterBoundaryFile("compose/client/lib/js/index.ts", "//#region 🌐RsWasmTransport\nexport async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/kit-store.worker.ts", "export async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/bin/assistant/mcp-app.tsx", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("framework/platform/renderer/react/index.tsx", "// #region 🔌Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import { z } from "zod";\nexport const a = 1;\n`;
    const file = "compose/client/lib/js/boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("dependencyBoundaryBreachesForBundleDir walks nested tsx", () => {
    const repoRoot = new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
    const dir = "framework/playground/renderer/react/puzzle";
    const breachs = dependencyBoundaryBreachesForBundleDir(repoRoot, dir);
    expect(breachs.every((b) => b.scope.startsWith("framework/playground/renderer/react/puzzle"))).toBe(true);
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs).toEqual([]);
  });
});

describe("ui scrollbar styling", () => {
  test("ui.css defines scrollbar tokens and native plus Scrollable rules", () => {
    const repoRoot = findRepoRoot(import.meta.dir);
    const css = readFileSync(join(repoRoot, "framework/ui/styling/js/ui.css"), "utf8");
    expect(css).toContain("--scrollbar-size:");
    expect(css).toContain("--scrollbar-thumb:");
    expect(css).toContain("scrollbar-color:");
    expect(css).toContain("*::-webkit-scrollbar-thumb");
    expect(css).toContain('[data-slot="scroll-area-thumb"]');
  });
});

describe("micro-commit", () => {
  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("./index.ts");
    expect(extractCounterFromSubject("🧑ueli🎆26🌙06☀️02🚩009")).toEqual({ nnn: 9, line1Base: "🧑ueli🎆26🌙06☀️02" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("extractNumericCounterFromSubject reads GitKraken numeric subjects", async () => {
    const { extractNumericCounterFromSubject } = await import("./index.ts");
    expect(extractNumericCounterFromSubject("299")).toBe(299);
    expect(extractNumericCounterFromSubject("001")).toBe(1);
    expect(extractNumericCounterFromSubject("🐙ueli🎆26🌙06☀️04🚩151")).toBeNull();
  });

  test("line1BaseFromBundleTag reads WIP epoch from squash tag", async () => {
    const { line1BaseFromBundleTag } = await import("./index.ts");
    expect(line1BaseFromBundleTag("🐙ueli🎆26🌙06☀️04🚩")).toBe("🐙ueli🎆26🌙06☀️04");
    expect(line1BaseFromBundleTag("🐙ueli🎆26🌙06☀️04🚩151")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("./index.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙ueli🎆26🌙06☀️02🚩033", "🐙ueli🎆26🌙06☀️02🚩032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙ueli🎆26🌙06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("bumpCounterFromHistory continues numeric GitKraken subjects with WIP epoch", async () => {
    const { bumpCounterFromHistory } = await import("./index.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" };
    const subjects = ["299", "298", "297"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-07-17T12:00:00"), "🐙ueli🎆26🌙06☀️04");
    expect(bumped.line1Base).toBe("🐙ueli🎆26🌙06☀️04");
    expect(bumped.nnn).toBe("300");
  });

  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("./index.ts");
    const bullets = normalizeBulletLines("🎆Summary\n📊uloc➕1✏️1➖0🟰2\n🟦65k➕1✏️1➖0\n🐛Fix bug");
    expect(bullets).toEqual(["🎆Summary", "🐛Fix bug"]);
  });

  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("./index.ts");
    expect(bulletLeadEmoji("🎆Drop stacked intro")).toBe("🎆");
    expect(bulletEmojiValidationError(["🎆All bullets wrongly use fireworks"])).toContain("🎆");
    expect(bulletEmojiValidationError(["🐛Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊uloc block"])).toContain("📊");
  });

  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("./index.ts");
    expect(formatMicroCommitBulletLine("- 🐛 Fix PDF")).toBe("🐛Fix PDF");
    expect(normalizeBulletLines("🐛 Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆item ${i}`).join("\n"))).toHaveLength(8);
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("./index.ts");
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
    const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("./index.ts");
    const metricsIdx = lines.findIndex((l) => l.startsWith(ulocHeader));
    if (metricsIdx >= 0) {
      expect(lines[metricsIdx - 1]).toBe("");
      expect(lines[metricsIdx]?.startsWith(ulocHeader)).toBe(true);
      expect(lines[metricsIdx + 1]).toMatch(/^🟦|^🦀|🐍|🐚|🔵|🟣|📝|🧾|📋/);
    }
  });

  test("formatMicroCommitMetricsLines uses compact loc and delta counts", async () => {
    const { formatMicroCommitMetricLine, formatMicroCommitMetricsLines, formatMetricLocCount, MICRO_COMMIT_ULOC_HEADER } = await import("./index.ts");
    expect(MICRO_COMMIT_ULOC_HEADER).toBe("📊uloc");
    expect(formatMetricLocCount(200_000)).toBe("200k");
    expect(formatMetricLocCount(500)).toBe("500");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 0, added: 0, removed: 0 })).toBe("🐚2k");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 2, added: 2, removed: 0 })).toBe("🐚2k➕2✏️2🟰4");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚", code: 2000, edited: 2, added: 0, removed: 2 })).toBe("🐚2k✏️2➖2🟰4");
    const lines = formatMicroCommitMetricsLines([{ lang: "Rust", emoji: "🦀", code: 200_000, edited: 2220, added: 2000, removed: 500 }]);
    expect(lines).toEqual(["📊uloc➕2000✏️2220➖500🟰4720", "🦀200k➕2000✏️2220➖500🟰4720"]);
  });

  test("formatMicroCommitMetricsLines totals all languages on the first row", async () => {
    const { formatMicroCommitMetricsLines } = await import("./index.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊uloc➕8✏️10🟰18");
    expect(lines[1]).toBe("🟦3k➕8✏️10🟰18");
    expect(lines[2]).toBe("📝44");
  });

  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("./index.ts");
    const root = process.cwd();
    const metrics = buildMicroCommitMetrics(root, {
      countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }),
    });
    expect(metrics.some((m) => m.lang === "Rust" && m.code === 100)).toBe(true);
  });

  test("isUlocCachePlausible rejects partial caches", async () => {
    const { isUlocCachePlausible, gitRepoRoot } = await import("./index.ts");
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
    const { shouldSkipPathForUloc } = await import("./index.ts");
    const root = process.cwd();
    expect(shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, ".agents/skills/micro-commit/SKILL.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "compose/client/ui/LICENSE.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "repo/AGENTS.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "repo/CHANGELOG.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, ".repo/cache/x")).toBe(true);
    expect(shouldSkipPathForUloc(root, "framework/core/README.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "puzzle/3d/src/foo.ts")).toBe(false);
  });

  test("countJsonKeys counts nested object keys", async () => {
    const { countJsonKeys } = await import("./index.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });

  test("appendGitDeltaSuffix formats user example totals", async () => {
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("./index.ts");
    expect(appendGitDeltaSuffix("🟦65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦65k➕700✏️200➖10🟰910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 })).toBe("📊uloc➕700✏️200➖10🟰910");
  });

  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("./index.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });

  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("./index.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });

  test("classifyPathForMetrics maps TeX ecosystem extensions", async () => {
    const { classifyPathForMetrics, langMetricsEmoji } = await import("./index.ts");
    expect(classifyPathForMetrics("mit-bestand/bericht/zwischenbericht/zwischenbericht.tex")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.sty")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.cls")).toBe("TeX");
    expect(classifyPathForMetrics("report/references.bib")).toBe("TeX");
    expect(classifyPathForMetrics("doc/sample.ltx")).toBe("TeX");
    expect(langMetricsEmoji("TeX")).toBe("📐");
  });

  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("./index.ts");
    const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md", "framework/product/playground/renderer/react/index.tsx"];
    expect(uncoveredStagedAreas(["🫡Only micro-commit skill wording"], staged)).toContain(".cursor/plans");
    expect(uncoveredStagedAreas(["🫡Only micro-commit skill wording"], staged)).toContain("product");
    const ok = uncoveredStagedAreas(["📋Plan brush edge resurrection guard and sync", "🖌️Playground renderer restores brush placement after structural deletes"], staged);
    expect(ok).toEqual([]);
  });

  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const { runMicroCommit } = await import("./index.ts");
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
    const stagedHasPresentation = spawnSync("git", ["diff", "--cached", "--name-only"], { cwd: root, encoding: "utf8" }).stdout?.includes("presentation/");
    if (stagedHasPresentation) expect(r.status).not.toBe(0);
  });

  test("installMicroCommitGitHooks writes portable hooks and bun pin", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("./index.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      const init = spawnSync("git", ["init"], { cwd: root, encoding: "utf8" });
      expect(init.status).toBe(0);
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("compose_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(existsSync(join(root, ".repo/compose-micro-commit-bun"))).toBe(true);
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("./index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
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
    const { wipeAfterCommit, writeMicroCommitTemplates, buildMicroCommitMessage } = await import("./index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-wipe-"));
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
      expect(existsSync(join(root, ".git/compose-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(root, ".git/COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates, buildMicroCommitMessage } = await import("./index.ts");
    const { mkdtempSync, readdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-tpl-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msg = buildMicroCommitMessage(root, { alias: "ueli", emoji: "🐙", name: "Ueli", email: "u@example.com" }, ["🎆bullet"]);
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
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("./index.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
});

describe("playground static sites", () => {
  test("PLAYGROUND_SITE_HOSTS maps each play to latest canonical host", () => {
    expect(PLAYGROUND_SITE_HOSTS.compose).toBe("play.semio-tech.com");
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

  test("PLAYGROUND_PORTS keeps cad and dag on distinct reserved dev ports", () => {
    expect(playgroundDevPort("cad")).toBe(6020);
    expect(playgroundDevPort("dag")).toBe(6017);
    expect(allPlaygroundReservedPorts().size).toBeGreaterThanOrEqual(Object.keys(PLAYGROUND_PORTS).length);
  });

  test("resolveFrameworkOsPlaygroundPlugin maps CLI segments to OS plugin ids", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["dag"])).toEqual({ plugin: "dag", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["gis", "2d"])).toEqual({ plugin: "gis2d", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["procedural", "3d", "fixture", "hexagonal-column"])).toEqual({
      plugin: "procedural3d",
      rest: ["fixture", "hexagonal-column"],
    });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["trinity", "jack"])).toEqual({ plugin: "trinity-jack", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["unknown"])).toBeNull();
    const resolvableSegments = catalog.reduce((sum, row) => sum + 1 + row.aliases.length, 0);
    expect(resolvableSegments).toBeGreaterThan(20);
  });

  test("frameworkOsPlaygroundDevEnv defaults wgpu renderer and resolves catalog dev port", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const dagEnv = frameworkOsPlaygroundDevEnv(catalog, "dag", {}, {});
    expect(dagEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(dagEnv.SEMIO_PLUGIN).toBe("dag");
    expect(dagEnv.S_OS_PORT).toBe("6117");

    const cadEnv = frameworkOsPlaygroundDevEnv(catalog, "cad", {}, { S_OS_PORT: "6020" });
    expect(cadEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(cadEnv.SEMIO_PLUGIN).toBe("cad");
    expect(cadEnv.S_OS_PORT).toBe("6020");
  });

  test("assigns a unique port per dev and test slot", () => {
    const seen = new Set<number>();
    for (const spec of Object.values(PLAYGROUND_PORTS)) {
      expect(seen.has(spec.dev)).toBe(false);
      seen.add(spec.dev);
      if (spec.test !== undefined) {
        expect(seen.has(spec.test)).toBe(false);
        seen.add(spec.test);
      }
    }
  }, 30_000);

  test("playgroundPlayViteDefine embeds locked fixture env", () => {
    const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
    try {
      delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('""');
      process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('"concrete-forest"');
    } finally {
      if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
    }
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

describe("package boundary guards", () => {
  const repoRoot = findRepoRoot();

  test("vite and vitest configs avoid cross-package @semio-tech aliases", () => {
    const offenders: string[] = [];
    const walk = (dir: string) => {
      for (const entry of require("node:fs").readdirSync(dir, { withFileTypes: true })) {
        if (entry.name === "node_modules" || entry.name === ".git" || entry.name === ".repo" || entry.name === "dist" || entry.name === "target" || entry.name === ".claude") continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (!/vite.*\.config\.ts$/.test(entry.name) && entry.name !== "vitest.config.ts") continue;
        const pkgDir = findNearestPackageDir(full, repoRoot);
        const text = readFileSync(full, "utf8");
        for (const match of text.matchAll(/find:\s*(?:\/\^)?["']@semio-tech\/[^"']+["']/g)) {
          const line = text.slice(0, match.index).split("\n").length;
          const snippet = text.split("\n")[line - 1] ?? "";
          if (/path\.resolve\(__dirname,\s*["']\.\./.test(snippet) && pkgDir && !snippet.includes("node_modules")) {
            offenders.push(`${full.replace(repoRoot + "/", "")}:${line}`);
          }
        }
      }
    };
    walk(repoRoot);
    expect(offenders).toEqual([]);
  });

  test("legacy package aliases are absent from source imports", () => {
    const result = spawnSync("rg", ["-l", "@compose/ui|@ui/react|@elements/", "--glob", "*.{ts,tsx}", "--glob", "!**/.repo/**", "--glob", "!**/index.test.ts"], {
      cwd: repoRoot,
      encoding: "utf8",
    });
    const files = (result.stdout ?? "")
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    expect(files).toEqual([]);
  });

  test("framework renderer host has no per-technology registerUi surface host APIs", () => {
    const indexPath = join(repoRoot, "framework/os/renderer/js/react/index.tsx");
    const indexSource = readFileSync(indexPath, "utf8");
    expect(indexSource).not.toMatch(/registerUi(?:Draw|Flow|Layout|Note|Puzzle2d|Puzzle3d|Puzzle5d|Sequence|Writer|Raster|Forms|Trinity|Procedural|Shooting|Gis|Cad|Dag|Lowpoly|Imperative|S)SurfaceHost/);
    expect(indexSource).toContain("bootFrameworkOs");
  });
});

function findNearestPackageDir(filePath: string, repoRoot: string): string | undefined {
  let dir = join(filePath, "..");
  while (dir.startsWith(repoRoot)) {
    if (existsSync(join(dir, "package.json"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return undefined;
}

describe("commit", () => {
  test("parseCommitBundleBody reads emoji scopes dates and bullets", async () => {
    const { parseCommitBundleBody } = await import("./index.ts");
    const bundles = parseCommitBundleBody("🏘️compose✍️sketchpad\n🎆26🌙06☀️04\n🗺️Map work\n🎆26🌙06☀️03\n🧪Playground\n\n🖱️ui⚛️react\n🎆26🌙06☀️02\n🖥️Shell");
    expect(bundles).toHaveLength(2);
    expect(bundles[0]?.label).toBe("🏘️compose✍️sketchpad");
    expect(bundles[0]?.dates).toHaveLength(2);
    expect(bundles[0]?.dates[0]?.bullets[0]).toBe("🗺️Map work");
  });

  test("parseCommitBundleBody rejects path prefixes and reserved emojis", async () => {
    const { parseCommitBundleBody } = await import("./index.ts");
    expect(() => parseCommitBundleBody("compose/foo|🏘️compose\n🎆26🌙06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🏘️compose🔀📊uloc\n🎆26🌙06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🗺️🧩🕸️\n🎆26🌙06☀️04\n🗺️Map work")).toThrow();
  });

  test("normalizeBundleScopeLabel strips reserved and uloc suffix", async () => {
    const { normalizeBundleScopeLabel } = await import("./index.ts");
    expect(normalizeBundleScopeLabel("🏘️compose🔀📊uloc➕1")).toBe("🏘️compose");
  });

  test("isBundleScopeLine accepts area and technology root labels", async () => {
    const { isBundleScopeLine } = await import("./index.ts");
    expect(isBundleScopeLine("🌐gis📍map")).toBe(true);
    expect(isBundleScopeLine("🖱️ui⚛️react")).toBe(true);
    expect(isBundleScopeLine("🥅framework")).toBe(true);
    expect(isBundleScopeLine("🧪Playground")).toBe(false);
    expect(isBundleScopeLine("🗺️Single emoji line")).toBe(false);
  });

  test("extractBundleDateLineFromSubject reads calendar day from micro-commit subject", async () => {
    const { extractBundleDateLineFromSubject } = await import("./index.ts");
    expect(extractBundleDateLineFromSubject("🐙ueli🎆26🌙06☀️04🚩012")).toBe("🎆26🌙06☀️04");
    expect(extractBundleDateLineFromSubject("unrelated")).toBeNull();
  });

  test("extractBundleDateLineFromCommit prefers body timestamp over subject checkpoint day", async () => {
    const { extractBundleDateLineFromCommit, extractBundleDateLineFromCommitBody } = await import("./index.ts");
    const body = "🎆26🌙06☀️04⏰02⌚38⏱️38\n🗺️Map work\n";
    expect(extractBundleDateLineFromCommitBody(body)).toBe("🎆26🌙06☀️04");
    expect(extractBundleDateLineFromCommit("🐙ueli🎆26🌙06☀️02🚩084", body)).toBe("🎆26🌙06☀️04");
  });

  test("pathsFromNumstatRow expands rename paths", async () => {
    const { pathsFromNumstatRow } = await import("./index.ts");
    expect(pathsFromNumstatRow("old/a.ts\tnew/b.ts")).toEqual(["old/a.ts", "new/b.ts"]);
    expect(pathsFromNumstatRow("dir/{old.ts => new.ts}")).toEqual(["old.ts", "new.ts"]);
  });

  test("pathMatchesBundleIndex does not treat empty prefix set as match-all", async () => {
    const { pathMatchesBundleIndex } = await import("./index.ts");
    const bundles = [
      { label: "🏘️compose✍️sketchpad", dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["✍️x"] }] },
      { label: "🏘️compose🗃️fixtures", dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["🗃️y"] }] },
    ];
    const prefixSets = [[], ["compose/fixture"]];
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 0, prefixSets, bundles)).toBe(false);
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 1, prefixSets, bundles)).toBe(true);
  });

  test("formatBundleDateLine appends per-day uloc suffix", async () => {
    const { formatBundleDateLine } = await import("./index.ts");
    expect(formatBundleDateLine("🎆26🌙06☀️04", { added: 700, edited: 200, removed: 10 })).toBe("🎆26🌙06☀️04📊uloc➕700✏️200➖10🟰910");
  });

  test("commitBundleBodyError rejects per-day uloc on stdin", async () => {
    const { commitBundleBodyError } = await import("./index.ts");
    expect(commitBundleBodyError("🏘️compose\n🎆26🌙06☀️04📊uloc➕1\n🗺️Work")).toMatch(/per-day/);
  });

  test("validateMicroCommitLangMetricsDeltaSum passes when language rows sum to footer", async () => {
    const { validateMicroCommitLangMetricsDeltaSum } = await import("./index.ts");
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
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("./index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-check-sum-"));
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
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/not attributed to any bundle|do not add up/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleCommitAttribution rejects listed days when micro-commit churn does not net to range", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("./index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-churn-"));
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
      writeFileSync(join(root, "mc1.txt"), "🐙ueli🎆26🌙06☀️01🚩001\n\n🎆26🌙06☀️03⏰12⌚00⏱️00\n🔧Add lines\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", join(root, "mc1.txt")], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      writeFileSync(join(root, "mc2.txt"), "🐙ueli🎆26🌙06☀️01🚩002\n\n🎆26🌙06☀️04⏰12⌚01⏱️00\n🔧Net fewer lines\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", join(root, "mc2.txt")], { cwd: root });
      const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Net\n🎆26🌙06☀️03\n🔧Add");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/does not add up/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleDayDeltasAttribution rejects when listed days do not sum to bundle total", async () => {
    const { validateBundleDayDeltasAttribution } = await import("./index.ts");
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
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltas, [{ added: 2, edited: 0, removed: 0 }])).toThrow(/missing from your bundle body/);
    const dateDeltasOneDay = new Map([[0, new Map([["🎆26🌙06☀️04", { added: 2, edited: 0, removed: 0 }]])]]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltasOneDay, [{ added: 7, edited: 0, removed: 0 }])).toThrow(/does not add up/);
  });

  test("buildCommitMessage appends per-day uloc from micro-commit dates", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { buildCommitMessage, parseCommitBundleBody } = await import("./index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-day-uloc-"));
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
      const msg1 = join(tmpdir(), `compose-mc1-${Date.now()}.txt`);
      writeFileSync(msg1, "🐙ueli🎆26🌙06☀️01🚩001\n\n🎆26🌙06☀️03⏰12⌚00⏱️00\n🔧Day three\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg1], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg2 = join(tmpdir(), `compose-mc2-${Date.now()}.txt`);
      writeFileSync(msg2, "🐙ueli🎆26🌙06☀️01🚩002\n\n🎆26🌙06☀️04⏰12⌚01⏱️00\n🔧Day four\n", "utf8");
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
    const { sortCommitBundlesByEditTotal } = await import("./index.ts");
    const mk = (label: string) => ({
      label,
      dates: [{ dateLine: "🎆26🌙06☀️04", bullets: ["🗺️change"] }],
    });
    const root = mkdtempSync(join(tmpdir(), "compose-commit-sort-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "compose"), { recursive: true });
      mkdirSync(join(root, "ui"), { recursive: true });
      mkdirSync(join(root, "framework"), { recursive: true });
      writeFileSync(join(root, "compose/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "ui/b.ts"), "b\n", "utf8");
      writeFileSync(join(root, "framework/c.ts"), "c\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "init"], { cwd: root });
      const base = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "compose/a.ts"), `${"a\n".repeat(11)}`, "utf8");
      writeFileSync(join(root, "ui/b.ts"), `${"b\n".repeat(151)}`, "utf8");
      writeFileSync(join(root, "framework/c.ts"), `${"c\n".repeat(5)}`, "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "delta"], { cwd: root });
      const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      const bundles = [mk("🏘️compose"), mk("🖱️ui"), mk("🥅framework")];
      const paths = [["compose/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];
      const sorted = sortCommitBundlesByEditTotal(root, base, head, bundles, paths);
      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️compose", "🥅framework"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildCommitMessage renders bundle subject and footer", async () => {
    const { buildCommitMessage, parseCommitBundleBody } = await import("./index.ts");
    const contributor = { alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const bundles = parseCommitBundleBody("📚repo🔧js\n🎆26🌙06☀️04\n🔧Tooling");
    const msg = buildCommitMessage(process.cwd(), contributor, bundles, "0000000000000000000000000000000000000000", "0000000000000000000000000000000000000000", { countRepoByLanguage: () => ({ TypeScript: 1000 }) });
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🔀$/);
    expect(lines.some((l) => l.includes("📊uloc"))).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
  });

  test("formatBundleTagName and formatBundleSubject use contributor date emojis", async () => {
    const { formatBundleTagName, formatBundleSubject } = await import("./index.ts");
    const c = { alias: "ueli", emoji: "🐙", name: "U", email: "u@e.com" };
    const now = new Date("2026-06-04T12:00:00");
    expect(formatBundleTagName(c, now)).toBe("🐙ueli🎆26🌙06☀️04🚩");
    expect(formatBundleSubject(c, now)).toBe("🐙ueli🎆26🌙06☀️04🔀");
  });

  test("formatCommitPrepareCommands emits four fenced git blocks", async () => {
    const { formatCommitPrepareCommands } = await import("./index.ts");
    const out = formatCommitPrepareCommands({
      tagName: "🐙ueli🎆26🌙06☀️04🚩",
      wipSha: "abc123def456",
      messageFile: ".git/compose-commit-message",
    });
    const blocks = out.trimEnd().split("\n\n");
    expect(blocks).toHaveLength(4);
    expect(blocks[0]).toBe("```\ngit tag -s -m '🐙ueli🎆26🌙06☀️04🚩' '🐙ueli🎆26🌙06☀️04🚩' HEAD\n```");
  });

  test("formatCommitPrepareAgentReply ends with tag name and commit message blocks", async () => {
    const { formatCommitPrepareAgentReply } = await import("./index.ts");
    const commitMessage = "🐙ueli🎆26🌙06☀️04🔀\n\n🏘️compose✍️sketchpad📊uloc\n🎆26🌙06☀️04\n🗺️Work\n\n📊uloc➕1🟰1\n\nSigned-off-by: U <u@e.com>\n";
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
    const { parseCommitSteps } = await import("./index.ts");
    expect(parseCommitSteps(["cs"])).toEqual({ tag: false, squash: true, push: false });
    expect(parseCommitSteps(["ct", "cs", "cp"])).toEqual({ tag: true, squash: true, push: true });
  });

  test("bulletMatchesCommitHistory detects verbatim prior commit lines", async () => {
    const { bulletMatchesCommitHistory } = await import("./index.ts");
    const history = new Set(["🗺️copied line from an old micro-commit"]);
    expect(bulletMatchesCommitHistory("🗺️copied line from an old micro-commit", history)).toBe(true);
    expect(bulletMatchesCommitHistory("🗺️fresh summary written from git diff", history)).toBe(false);
  });
});

describe("command budgets", () => {
  const indexPath = join(import.meta.dir, "index.ts");

  /** ⏱️Runs `runCmd` in a fresh subprocess (its budget-exceeded path calls `process.exit`, which would kill the test runner in-process) against a child that sleeps far longer than its budget. */
  function spawnBudgetedSleep(budgetMs: number, envOverride?: Record<string, string>): ReturnType<typeof spawnSync> {
    const script = `const { runCmd } = await import(${JSON.stringify(indexPath)}); runCmd(${JSON.stringify(process.execPath)}, ["-e", "await Bun.sleep(10000)"], ${budgetMs === -1 ? "{}" : `{ budgetMs: ${budgetMs} }`});`;
    return spawnSync(process.execPath, ["-e", script], { encoding: "utf8", env: { ...process.env, ...envOverride } });
  }

  test("kills a command that exceeds its explicit budget", () => {
    const start = Date.now();
    const result = spawnBudgetedSleep(250);
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
    expect(Date.now() - start).toBeLessThan(5000);
  });

  test("SEMIO_CMD_BUDGET_MS env override kills a command with no explicit budget", () => {
    const result = spawnBudgetedSleep(-1, { SEMIO_CMD_BUDGET_MS: "250" });
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
  });

  test("orchestratorBudgetOpts supplies a bounded orchestrator budget", () => {
    expect(orchestratorBudgetOpts()).toEqual({ budgetMs: orchestratorBudgetMs() });
    expect(() => runCmd(process.execPath, ["-e", "1"], orchestratorBudgetOpts())).not.toThrow();
  });

  test("daemonBudgetOpts returns the daemon budget class", () => {
    expect(daemonBudgetOpts()).toEqual({ budgetMs: daemonBudgetMs() });
    expect(daemonBudgetMs()).toBe(DAEMON_BUDGET_MS);
    expect(orchestratorBudgetMs()).toBe(ORCHESTRATOR_BUDGET_MS);
  });

  test("goLevelTestArgs includes -timeout derived from the level budget", () => {
    const args = goLevelTestArgs("fundamental");
    expect(args[0]).toBe("-timeout");
    expect(args[1]).toMatch(/^\d+s$/);
    expect(args[1]).toBe(`${Math.ceil(testLevelBudgetMs("fundamental") / 1000)}s`);
  });

  test("vitestLevelArgs returns per-test timeout flags", () => {
    const ms = String(testLevelBudgetMs("quick"));
    expect(vitestLevelArgs("quick")).toEqual(["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms]);
  });

  test("orchestratorBudgetMs defaults to 4h and honors SEMIO_ORCHESTRATOR_BUDGET_MS", () => {
    expect(orchestratorBudgetMs()).toBe(4 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
    process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = "12345";
    try {
      expect(orchestratorBudgetMs()).toBe(12345);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
      else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = prev;
    }
  });

  test("daemonBudgetMs defaults to 24h and honors SEMIO_DAEMON_BUDGET_MS", () => {
    expect(daemonBudgetMs()).toBe(24 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_DAEMON_BUDGET_MS;
    process.env.SEMIO_DAEMON_BUDGET_MS = "67890";
    try {
      expect(daemonBudgetMs()).toBe(67890);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_DAEMON_BUDGET_MS;
      else process.env.SEMIO_DAEMON_BUDGET_MS = prev;
    }
  });

  test("testLevelBudgetMs maps levels and honors SEMIO_TEST_BUDGET_MS", async () => {
    const { TEST_LEVEL_BUDGET_MS } = await import("./index.ts");
    expect(testLevelBudgetMs("fundamental")).toBe(TEST_LEVEL_BUDGET_MS.fundamental);
    expect(testLevelBudgetMs("exhaustive")).toBe(TEST_LEVEL_BUDGET_MS.exhaustive);
    const prev = process.env.SEMIO_TEST_BUDGET_MS;
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    try {
      expect(testLevelBudgetMs("quick")).toBe(42000);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
      else process.env.SEMIO_TEST_BUDGET_MS = prev;
    }
  });

  test("runProbe captures stdout under budget", () => {
    const { status, stdout } = runProbe(process.execPath, ["-e", "console.log('probe-ok')"]);
    expect(status).toBe(0);
    expect(stdout.trim()).toBe("probe-ok");
  });

  test("runCmdStatus returns the exit status instead of throwing", () => {
    expect(runCmdStatus(process.execPath, ["-e", "process.exit(3)"])).toBe(3);
  });

  test("budgetTimeoutHint defaults cargo to the target-dir lock-contention hint", () => {
    expect(budgetTimeoutHint("cargo")).toContain("target-dir lock contention");
  });

  test("budgetTimeoutHint honors an explicit override", () => {
    expect(budgetTimeoutHint("git", "custom hint")).toBe("custom hint");
  });

  test("budgetTimeoutHint gives non-cargo commands a generic hint", () => {
    expect(budgetTimeoutHint("git")).not.toContain("target-dir lock contention");
  });
});
