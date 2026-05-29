#!/usr/bin/env bun
/** 🧩 Neo4j ticket router: `bun ./script.ts <migrate|kit-field|rename-ops|gen-domain|stitch>`. */
const cmd = process.argv[2] ?? "migrate";

switch (cmd) {
  case "migrate":
    await import("./migrate.neo4j.ts");
    break;
  case "kit-field": {
    const { existsSync } = await import("node:fs");
    const { dirname, join } = await import("node:path");
    const { runKitFieldDeclaredIsRepair } = await import("./kit-field-declared-is.ts");
    let dir = import.meta.dir;
    for (let i = 0; i < 24; i++) {
      if (existsSync(join(dir, "script.ts"))) break;
      const parent = dirname(dir);
      if (parent === dir) throw new Error("[kit-field] repo root not found");
      dir = parent;
    }
    const repoRoot = dir;
    const cacheDir = join(repoRoot, ".repo", "cache", "neo4j-migrate");
    const { mkdirSync } = await import("node:fs");
    mkdirSync(cacheDir, { recursive: true });
    runKitFieldDeclaredIsRepair({
      repoRoot,
      database: process.env.NEO4J_DATABASE || "semio",
      cacheDir,
    });
    console.log("[kit-field] repair ok.");
    break;
  }
  case "rename-ops":
    await import("./rename-operations-imperative.ts");
    break;
  case "gen-domain":
    await import("./gen-domain-operation-classes.ts");
    break;
  case "stitch":
    await import("./stitch-operation-command-migrations.ts");
    break;
  default:
    console.error("usage: bun ./script.ts <migrate|kit-field|rename-ops|gen-domain|stitch>");
    process.exit(1);
}
