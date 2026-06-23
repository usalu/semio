#!/usr/bin/env bun
/** 🧭 Grasshopper project router: `bun ./script.ts <build|test|generate value-list>`. */
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../repo/lib/js/src/index.ts";

function runGenerateValueList(root: string): void {
  const buildDir = join(root, "build");
  if (!existsSync(buildDir)) mkdirSync(buildDir);
  function convertCsvToValueList(csvPath: string, outputPath: string, keyColumn: string, valueColumn: string): void {
    const csvContent = readFileSync(csvPath, "utf-8");
    const [headerLine, ...dataLines] = csvContent.split(/\r?\n/).filter((line) => line.trim().length > 0);
    const headers = headerLine.split(",");
    const keyIndex = headers.indexOf(keyColumn);
    const valueIndex = headers.indexOf(valueColumn);
    if (keyIndex === -1 || valueIndex === -1) {
      throw new Error(`Missing CSV columns ${keyColumn} / ${valueColumn} in ${csvPath}`);
    }
    const records = dataLines.map((line) => {
      const values = line.split(",");
      return { [keyColumn]: values[keyIndex] ?? "", [valueColumn]: values[valueIndex] ?? "" };
    });
    const lines = records.map((record: Record<string, string>) => `${record[keyColumn]} = "${record[valueColumn]}"`);
    writeFileSync(outputPath, lines.join("\n"), "utf-8");
  }
  convertCsvToValueList(join(root, "..", "..", "..", "..", "..", "elements", "assets", "lists", "mimes.csv"), join(buildDir, "mimes.txt"), "Extension", "MIME");
  convertCsvToValueList(join(root, "..", "..", "..", "..", "..", "elements", "assets", "lists", "licenses.csv"), join(buildDir, "licenses.txt"), "Name", "SPDX");
  console.log("✅ Value lists generated");
}

class GenerateValueListScript extends BundleScript {
  run(): void {
    runGenerateValueList(this.root);
  }
}

class GenerateScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] === "value-list") {
      new GenerateValueListScript(this.root, this.repoRoot).run();
      return;
    }
    console.error("usage: bun ./script.ts generate value-list");
    process.exit(1);
  }
}

class TestScript extends BundleScript {
  run(): void {
    const fx = process.platform === "win32" ? "net48" : "net8.0";
    execFileSync("dotnet", ["test", join(this.root, "Compose.Grasshopper.Tests", "Compose.Grasshopper.Tests.csproj"), "-c", "UnitTest", "-f", fx], {
      cwd: this.root,
      stdio: "inherit",
    });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runGenerateValueList(this.root);
    execFileSync("dotnet", ["clean", "Compose.Grasshopper.csproj", "-c", "Debug"], { cwd: this.root, stdio: "inherit" });
    execFileSync("dotnet", ["build", "Compose.Grasshopper.csproj", "-c", "Debug"], { cwd: this.root, stdio: "inherit" });
    const yakDistFolder = join(this.root, "yak", "dist");
    if (existsSync(yakDistFolder)) rmSync(yakDistFolder, { recursive: true });
    mkdirSync(yakDistFolder, { recursive: true });
    const binFolder = join(this.root, "bin", "Debug", "net48");
    for (const file of readdirSync(binFolder)) {
      cpSync(join(binFolder, file), join(yakDistFolder, file), { force: true, recursive: true });
    }
    console.log("✅ Grasshopper build complete");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("test", TestScript)
  .register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url);
