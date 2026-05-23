#!/usr/bin/env bun
/**
 * 🧭 Grasshopper project router: `bun ./script.ts <build|test|generate> [segments…]`.
 */
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const verb = segs[0];

function runGenerateValueList(): void {
  const buildDir = join(cwd, "build");
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
  convertCsvToValueList(join(cwd, "..", "..", "..", "..", "..", "elements", "assets", "lists", "mimes.csv"), join(buildDir, "mimes.txt"), "Extension", "MIME");
  convertCsvToValueList(join(cwd, "..", "..", "..", "..", "..", "elements", "assets", "lists", "licenses.csv"), join(buildDir, "licenses.txt"), "Name", "SPDX");
  console.log("✅ Value lists generated");
}

if (verb === "generate" && segs[1] === "value-list") {
  runGenerateValueList();
} else if (verb === "test") {
  const fx = process.platform === "win32" ? "net48" : "net8.0";
  execFileSync("dotnet", ["test", join(cwd, "Semio.Grasshopper.Tests", "Semio.Grasshopper.Tests.csproj"), "-c", "UnitTest", "-f", fx], {
    cwd,
    stdio: "inherit",
  });
} else if (verb === "build") {
  runGenerateValueList();
  execFileSync("dotnet", ["clean", "Semio.Grasshopper.csproj", "-c", "Debug"], { cwd, stdio: "inherit" });
  execFileSync("dotnet", ["build", "Semio.Grasshopper.csproj", "-c", "Debug"], { cwd, stdio: "inherit" });
  const yakDistFolder = join(cwd, "yak", "dist");
  if (existsSync(yakDistFolder)) rmSync(yakDistFolder, { recursive: true });
  mkdirSync(yakDistFolder, { recursive: true });
  const binFolder = join(cwd, "bin", "Debug", "net48");
  for (const file of readdirSync(binFolder)) {
    cpSync(join(binFolder, file), join(yakDistFolder, file), { force: true, recursive: true });
  }
  console.log("✅ Grasshopper build complete");
} else {
  console.error("usage: bun ./script.ts <build|test|generate value-list>");
  process.exit(1);
}
