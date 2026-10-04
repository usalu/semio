#!/usr/bin/env bun
/** 🎪️ Playground public persisted owner and strict Source consumer router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { readdirSync } from "node:fs";
import { join, resolve } from "node:path";
if (process.argv[2] === "verify" && process.argv[3] === "snapshot-sqlite-source") {
  const repo = resolve(import.meta.dir, "../../../../../../.."), files: string[] = [], pending = [resolve(import.meta.dir, "../../🏅️standards")];
  while (pending.length) {
    const directory = pending.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory() && entry.name !== "🧪️tests") pending.push(path);
      else if (entry.isFile() && entry.name.endsWith(".ts")) files.push(path);
    }
  }
  await runRepositoryCommand(process.execPath, ["x", "tsc", ...files, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck"], repo, "playground:published-source-consumers", 120000);
  console.log(`[playground] checked published consumers=${files.length}`);
} else await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/demonstrator-playground", { suites: ["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"] });
