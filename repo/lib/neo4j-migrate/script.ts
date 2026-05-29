#!/usr/bin/env bun
/** 🧩 Neo4j graph migration router: `bun ./script.ts <migrate|kit-field|rename-ops|gen-domain|stitch>`. */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../js/src/bundle-script.ts";

class MigrateScript extends BundleScript {
  async run(): Promise<void> {
    await import("./migrate.neo4j.ts");
  }
}

class KitFieldScript extends BundleScript {
  async run(): Promise<void> {
    const { runKitFieldDeclaredIsRepair } = await import("./kit-field-declared-is.ts");
    const cacheDir = join(this.repoRoot, ".repo", "cache", "neo4j-migrate");
    mkdirSync(cacheDir, { recursive: true });
    runKitFieldDeclaredIsRepair({
      repoRoot: this.repoRoot,
      database: process.env.NEO4J_DATABASE || "semio",
      cacheDir,
    });
    console.log("[kit-field] repair ok.");
  }
}

class RenameOpsScript extends BundleScript {
  async run(): Promise<void> {
    await import("./rename-operations-imperative.ts");
  }
}

class GenDomainScript extends BundleScript {
  async run(): Promise<void> {
    await import("./gen-domain-operation-classes.ts");
  }
}

class StitchScript extends BundleScript {
  async run(): Promise<void> {
    await import("./stitch-operation-command-migrations.ts");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("migrate", MigrateScript)
  .register("kit-field", KitFieldScript)
  .register("rename-ops", RenameOpsScript)
  .register("gen-domain", GenDomainScript)
  .register("stitch", StitchScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "migrate" });
