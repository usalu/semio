#!/usr/bin/env bun
/** 🧭 Grasshopper project router: `bun ./script.ts <build|test|publish|setup|generate value-list>`. */
import { copyFileSync, cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, dotnetCoverageArgs, runCmd, runTestBudgeted } from "../../../../../repo/lib/js/index.ts";

const yakWin8 = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
const yakWin7 = "C:\\Program Files\\Rhino 7\\System\\Yak.exe";

function yakExecutable(): string {
  return process.platform === "win32" ? yakWin8 : process.platform === "darwin" ? "/Applications/Rhino 8.app/Contents/Resources/bin/yak" : "yak";
}

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
  convertCsvToValueList(join(root, "..", "..", "..", "..", "..", "elements", "assets", "lists", "📋mimes.csv"), join(buildDir, "mimes.txt"), "Extension", "MIME");
  convertCsvToValueList(join(root, "..", "..", "..", "..", "..", "elements", "assets", "lists", "📋licenses.csv"), join(buildDir, "licenses.txt"), "Name", "SPDX");
  console.log("✅ Value lists generated");
}

function runYakBuild(root: string): void {
  const yakRoot = join(root, "yak");
  const distDir = join(yakRoot, "dist");
  for (const f of [join(distDir, "🖼️compose_512x512.png"), join(distDir, "manifest.yml")]) {
    if (existsSync(f)) rmSync(f);
  }
  if (!existsSync(distDir)) mkdirSync(distDir);
  copyFileSync(join(root, "..", "..", "..", "..", "..", "assets", "icons", "🖼️compose_512x512.png"), join(distDir, "🖼️compose_512x512.png"));
  copyFileSync(join(yakRoot, "manifest.yml"), join(distDir, "manifest.yml"));
  runCmd(yakExecutable(), ["build", "--platform", "win"], { cwd: distDir });
  console.log("✅ Yak package built");
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
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "push") {
      const packageFile = segments[1] || "compose-2.1.0-any-win.yak";
      runCmd(yakWin8, ["push", "--source", "https://test.yak.rhino3d.com", packageFile]);
      console.log(`✅ Pushed ${packageFile} to test server`);
      return;
    }
    if (segments[0] === "search") {
      runCmd(yakWin8, ["search", "--source", "https://test.yak.rhino3d.com", "--all", "--prerelease", "compose"]);
      return;
    }
    const { level, rest } = resolveTestLevel(segments);
    const fx = process.platform === "win32" ? "net48" : "net8.0";
    await runTestBudgeted("dotnet", ["test", join(this.root, "Compose.Grasshopper.Tests", "Compose.Grasshopper.Tests.csproj"), "-c", "UnitTest", "-f", fx, ...dotnetLevelArgs(level), ...dotnetCoverageArgs(this.repoRoot, this.root), ...rest], {
      cwd: this.root,
    });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runGenerateValueList(this.root);
    runCmd("dotnet", ["clean", "Compose.Grasshopper.csproj", "-c", "Debug"], { cwd: this.root });
    runCmd("dotnet", ["build", "Compose.Grasshopper.csproj", "-c", "Debug"], { cwd: this.root });
    const yakDistFolder = join(this.root, "yak", "dist");
    if (existsSync(yakDistFolder)) rmSync(yakDistFolder, { recursive: true });
    mkdirSync(yakDistFolder, { recursive: true });
    const binFolder = join(this.root, "bin", "Debug", "net48");
    for (const file of readdirSync(binFolder)) {
      cpSync(join(binFolder, file), join(yakDistFolder, file), { force: true, recursive: true });
    }
    runYakBuild(this.root);
    console.log("✅ Grasshopper build complete");
  }
}

class PublishScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] === "yank") {
      const version = segments[1] || "5.1.0-beta";
      runCmd(yakWin7, ["yank", "compose", version]);
      console.log(`✅ Yanked compose ${version}`);
      return;
    }
    if (segments[0] === "unyank") {
      const version = segments[1] || "5.1.0-beta";
      runCmd(yakWin7, ["unyank", "compose", version]);
      console.log(`✅ Unyanked compose ${version}`);
      return;
    }
    const dist = join(this.root, "yak", "dist");
    const manifestContent = readFileSync(join(dist, "manifest.yml"), "utf-8");
    const versionMatch = manifestContent.match(/version:\s*(.+)/);
    if (!versionMatch) throw new Error("Could not find version in manifest.yml");
    const version = versionMatch[1]!.trim();
    const buildName = `compose-${version}-rh8_10-win.yak`;
    runCmd(yakWin8, ["push", buildName], { cwd: dist });
    console.log("✅ Yak package published");
  }
}

class SetupScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "login") {
      console.error("usage: bun ./script.ts setup login");
      process.exit(1);
    }
    runCmd(yakWin8, ["login"]);
    console.log("✅ Logged in to Yak");
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("test", TestScript).register("build", BuildScript).register("publish", PublishScript).register("setup", SetupScript);

await runBundleScriptMain(router, import.meta.url);
