#!/usr/bin/env bun
/** 📊 W9 periphery inventory — run from ticket folder: `bun ./📜️script.ts inventory` */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

const REPO_ROOT = join(import.meta.dir, "../../../../../..");
const TICKET_DIR = import.meta.dir;
const IMPL_SEGMENTS = ["⚡️implementations", "⚡️implementation"] as const;
const PRUNE = new Set(["node_modules", "target", ".git"]);

type Counts = Record<string, number>;

function walkImplDirs(root: string): string[] {
  const out: string[] = [];
  const stack = [root];
  while (stack.length) {
    const dir = stack.pop()!;
    let entries: { name: string; isDirectory: () => boolean }[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const e of entries) {
      if (PRUNE.has(e.name)) continue;
      const p = join(dir, e.name);
      if (!e.isDirectory()) continue;
      if (IMPL_SEGMENTS.includes(e.name as (typeof IMPL_SEGMENTS)[number])) {
        out.push(p);
        continue;
      }
      stack.push(p);
    }
  }
  return out;
}

function topArea(rel: string): string {
  return rel.split("/")[0] ?? rel;
}

function bucketFramework(relFromRepo: string): string {
  const parts = relFromRepo.replace(/^🧰️framework\//, "").split("/");
  if (parts[0] === "🛍️products" && parts[1]) {
    if (parts[1] === "💻️os" && parts[2] === "🔨️modules" && parts[3]) return `os/${parts[3]}`;
    return `products/${parts[1]}`;
  }
  if (parts[0] === "🔨️modules") return `modules/${parts[1] ?? "?"}`;
  return parts.slice(0, 2).join("/");
}

function findProjectJsonWithImpl(): { path: string; kind: "under-impl" | "stale-cwd" | "stale-input" }[] {
  const hits: { path: string; kind: "under-impl" | "stale-cwd" | "stale-input" }[] = [];
  const stack = [REPO_ROOT];
  while (stack.length) {
    const dir = stack.pop()!;
    let entries: { name: string; isDirectory: () => boolean }[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const e of entries) {
      if (PRUNE.has(e.name)) continue;
      const p = join(dir, e.name);
      if (e.isDirectory()) {
        if (e.name === ".🦑️repo" || e.name === "node_modules") continue;
        stack.push(p);
        continue;
      }
      if (e.name !== "📋️project.json") continue;
      const rel = relative(REPO_ROOT, p).replaceAll("\\", "/");
      let raw = "";
      try {
        raw = readFileSync(p, "utf8");
      } catch {
        continue;
      }
      if (!raw.includes("⚡️implementations") && !raw.includes("⚡️implementation")) continue;
      const underImpl = IMPL_SEGMENTS.some((s) => rel.includes(`/${s}/`));
      const staleCwd = /"cwd"\s*:\s*"[^"]*⚡️implementations/.test(raw) && rel.includes("📦️packages/");
      const staleInput = /⚡️implementations/.test(raw) && rel.includes("📦️packages/") && !underImpl;
      hits.push({
        path: rel,
        kind: staleCwd ? "stale-cwd" : staleInput ? "stale-input" : underImpl ? "under-impl" : "under-impl",
      });
    }
  }
  return hits;
}

function mdImplLinkCounts(): Counts {
  const counts: Counts = { ".cursor": 0, other: 0 };
  const stack = [REPO_ROOT];
  while (stack.length) {
    const dir = stack.pop()!;
    let entries: { name: string; isDirectory: () => boolean }[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const e of entries) {
      if (PRUNE.has(e.name)) continue;
      const p = join(dir, e.name);
      if (e.isDirectory()) {
        if (e.name === "node_modules" || e.name === ".git") continue;
        stack.push(p);
        continue;
      }
      if (!/\.mdx?$/u.test(e.name)) continue;
      const rel = relative(REPO_ROOT, p).replaceAll("\\", "/");
      if (rel.startsWith(".🦑️repo/")) continue;
      let text = "";
      try {
        text = readFileSync(p, "utf8");
      } catch {
        continue;
      }
      if (!text.includes("⚡️implementations") && !text.includes("⚡️implementation")) continue;
      if (rel.startsWith(".cursor/")) counts[".cursor"] = (counts[".cursor"] ?? 0) + 1;
      else counts.other = (counts.other ?? 0) + 1;
    }
  }
  return counts;
}

function inventory() {
  const implDirs = walkImplDirs(REPO_ROOT);
  const byTop: Counts = {};
  const byFramework: Counts = {};
  for (const abs of implDirs) {
    const rel = relative(REPO_ROOT, abs).replaceAll("\\", "/");
    const top = topArea(rel);
    byTop[top] = (byTop[top] ?? 0) + 1;
    if (top === "🧰️framework") byFramework[bucketFramework(rel)] = (byFramework[bucketFramework(rel)] ?? 0) + 1;
  }

  const singular = implDirs.length === 0 ? [] : walkImplDirs(REPO_ROOT).filter((d) => d.endsWith("⚡️implementation"));
  const projectJson = findProjectJsonWithImpl();

  const goWork = existsSync(join(REPO_ROOT, "go.work"))
    ? readFileSync(join(REPO_ROOT, "go.work"), "utf8")
        .split("\n")
        .filter((l) => l.includes("⚡️implementations"))
    : [];

  const vitestKnownBroken = existsSync(join(REPO_ROOT, "🧪️vitest.config.ts"))
    ? [...readFileSync(join(REPO_ROOT, "🧪️vitest.config.ts"), "utf8").matchAll(/"([^"]+⚡️implementations[^"]+)"/g)].map((m) => m[1])
    : [];

  const taxonomyOld = join(
    REPO_ROOT,
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/🔣️taxonomy.json",
  );
  const taxonomyNew = join(REPO_ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");

  const snapshot = {
    generatedAt: new Date().toISOString(),
    ticket: "26/08/06/PERIPHERY-PROJECT-JSON-AND-FINALIZATION",
    goal: "🎯aioptimizedrepo",
    implementations: {
      total: implDirs.length,
      singularImplementationDirs: singular.length,
      byTopArea: byTop,
      frameworkBuckets: byFramework,
    },
    projectJson: {
      totalReferencingImpl: projectJson.length,
      underImplPath: projectJson.filter((p) => p.kind === "under-impl").length,
      staleCwdOrInputUnderPackages: projectJson.filter((p) => p.kind !== "under-impl").length,
      stalePackages: projectJson.filter((p) => p.kind !== "under-impl"),
      sampleUnderImpl: projectJson.filter((p) => p.kind === "under-impl").slice(0, 15).map((p) => p.path),
    },
    staleConfig: {
      goWorkImplLines: goWork,
      devcontainerPostCreateImpl: existsSync(join(REPO_ROOT, ".devcontainer/post-create.sh"))
        ? readFileSync(join(REPO_ROOT, ".devcontainer/post-create.sh"), "utf8")
            .split("\n")
            .filter((l) => l.includes("⚡️implementations"))
        : [],
      vitestKnownBrokenPaths: vitestKnownBroken,
      taxonomyJsonOldPathExists: existsSync(taxonomyOld),
      taxonomyJsonNewPathExists: existsSync(taxonomyNew),
      rootScriptRepoLibImportExists: existsSync(
        join(REPO_ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️implementations/🟦️typescript/📦️index.ts"),
      ),
      dependencyCruiserTaxonomyLoadOk: (() => {
        try {
          require(join(REPO_ROOT, ".dependency-cruiser.cjs"));
          return true;
        } catch {
          return false;
        }
      })(),
    },
    entryFiles: (() => {
      const glueRs=[], libRs=[], glueTs=[], indexTs=[];
      const stack=[REPO_ROOT];
      while (stack.length) {
        const dir=stack.pop();
        let entries; try { entries=readdirSync(dir,{withFileTypes:true}); } catch { continue; }
        for (const e of entries) {
          if (PRUNE.has(e.name) || e.name===".🦑️repo" || e.name==="node_modules") continue;
          const p=join(dir,e.name);
          if (e.isDirectory()) { stack.push(p); continue; }
          const rel=relative(REPO_ROOT,p).replaceAll("\\","/");
          if (!rel.includes("📦️packages/")) continue;
          if (e.name==="📦️glue.rs") glueRs.push(rel);
          else if (e.name==="📦️lib.rs") libRs.push(rel);
          else if (e.name==="🟦️glue.ts") glueTs.push(rel);
          else if (e.name==="📦️index.ts" || e.name==="📦️index.tsx") indexTs.push(rel);
        }
      }
      return {
        glueRsUnderPackages: glueRs.length,
        libRsUnderPackages: libRs.length,
        glueTsUnderPackages: glueTs.length,
        indexTsUnderPackages: indexTs.length,
        sampleLibRs: libRs.slice(0,20),
        sampleGlueRs: glueRs.slice(0,10),
      };
    })(),
    markdownWithImplLinks: mdImplLinkCounts(),
    outsideFrameworkImplDirs: implDirs
      .map((d) => relative(REPO_ROOT, d).replaceAll("\\", "/"))
      .filter((r) => !r.startsWith("🧰️framework/")),
  };

  const outPath = join(TICKET_DIR, "🧪️w9-inventory-snapshot.json");
  writeFileSync(outPath, `${JSON.stringify(snapshot, null, 2)}\n`);
  console.log(outPath);
}

const router = {
  inventory,
};

const cmd = process.argv[2];
if (!cmd || !(cmd in router)) {
  console.log("usage: bun ./📜️script.ts inventory");
  process.exit(cmd ? 1 : 0);
}
(router as Record<string, () => void>)[cmd]();
