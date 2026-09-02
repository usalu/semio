#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

type CrateAudit = {
  manifest: string;
  packageName: string;
  invocationFiles: string[];
  serde: boolean;
  serdeJson: boolean;
};

function findWorkspace(start: string): string {
  let directory = start;
  while (dirname(directory) !== directory) {
    if (existsSync(join(directory, "Cargo.toml")) && existsSync(join(directory, "✏️s"))) return directory;
    directory = dirname(directory);
  }
  throw new Error(`workspace root not found above ${start}`);
}

const workspace = findWorkspace(import.meta.dir);
const sourceRoot = join(workspace, "✏️s");

function nearestManifest(file: string): string {
  let directory = dirname(file);
  while (directory.startsWith(sourceRoot)) {
    for (const manifest of [join(directory, "Cargo.toml"), join(directory, "📦️packages", "🦀️rust", "Cargo.toml")]) {
      if (existsSync(manifest)) return manifest;
    }
    directory = dirname(directory);
  }
  throw new Error(`no Cargo.toml owns ${relative(workspace, file)}`);
}

function dependencyFlags(manifestText: string): { serde: boolean; serdeJson: boolean } {
  let section = "";
  let serde = false;
  let serdeJson = false;
  for (const rawLine of manifestText.split("\n")) {
    const line = rawLine.replace(/#.*/, "").trim();
    const sectionMatch = line.match(/^\[([^\]]+)]$/);
    if (sectionMatch) {
      section = sectionMatch[1];
      continue;
    }
    if (!/(^|\.)dependencies$/.test(section)) continue;
    if (/^serde\s*=/.test(line)) serde = true;
    if (/^serde_json\s*=/.test(line)) serdeJson = true;
  }
  return { serde, serdeJson };
}

async function audit(): Promise<void> {
  const mentionFiles: string[] = [];
  const invocationFiles: string[] = [];
  const glob = new Bun.Glob("**/*.rs");
  for await (const path of glob.scan({ cwd: sourceRoot, absolute: true })) {
    const source = readFileSync(path, "utf8");
    if (!source.includes("app_commands!")) continue;
    mentionFiles.push(path);
    if (/app_commands!\s*\{/.test(source)) invocationFiles.push(path);
  }

  const crates = new Map<string, CrateAudit>();
  for (const file of invocationFiles) {
    const manifest = nearestManifest(file);
    let crate = crates.get(manifest);
    if (!crate) {
      const manifestText = readFileSync(manifest, "utf8");
      const packageName = manifestText.match(/\[package\][\s\S]*?^name\s*=\s*"([^"]+)"/m)?.[1];
      if (!packageName) throw new Error(`missing package.name in ${relative(workspace, manifest)}`);
      crate = { manifest, packageName, invocationFiles: [], ...dependencyFlags(manifestText) };
      crates.set(manifest, crate);
    }
    crate.invocationFiles.push(file);
  }

  const rows = [...crates.values()].sort((left, right) => left.packageName.localeCompare(right.packageName));
  const serdeFree = rows.filter((row) => !row.serde);
  console.log("# `app_commands!` Fleet Manifest Audit\n");
  console.log(`- Files mentioning \`app_commands!\`: ${mentionFiles.length}`);
  console.log(`- Files containing an invocation: ${invocationFiles.length}`);
  console.log(`- Owning crates: ${rows.length}`);
  console.log(`- Invoking crates without a direct serde dependency: ${serdeFree.length}`);
  console.log(`- Invoking crates with a direct serde dependency: ${rows.length - serdeFree.length}\n`);
  console.log("| Package | Invocations | serde | serde_json | Manifest |");
  console.log("| --- | ---: | :---: | :---: | --- |");
  for (const row of rows) {
    console.log(`| \`${row.packageName}\` | ${row.invocationFiles.length} | ${row.serde ? "yes" : "no"} | ${row.serdeJson ? "yes" : "no"} | \`${relative(workspace, row.manifest)}\` |`);
  }
  console.log("\n## Serde-free invoking packages\n");
  for (const row of serdeFree) console.log(`- \`${row.packageName}\``);
  console.log("\n## Cargo package arguments\n");
  console.log(rows.map((row) => `-p ${row.packageName}`).join(" "));
}

if (Bun.argv[2] !== "audit" || Bun.argv.length !== 3) {
  console.error("usage: bun 📜️script.ts audit");
  process.exit(2);
}

await audit();
