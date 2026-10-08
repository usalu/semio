// 📊 Read-only statistics of the launch files still present in the repository (r2 audit helper).
// Run from the repository root: bun <ticket>/r2-launch-stats.ts
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();

function stripJsonc(text: string): string {
  let out = "";
  let inString = false;
  for (let i = 0; i < text.length; i++) {
    const ch = text[i];
    const next = text[i + 1];
    if (inString) {
      out += ch;
      if (ch === "\\") { out += text[i + 1] ?? ""; i++; }
      else if (ch === '"') inString = false;
      continue;
    }
    if (ch === '"') { inString = true; out += ch; continue; }
    if (ch === "/" && next === "/") { while (i < text.length && text[i] !== "\n") i++; out += "\n"; continue; }
    if (ch === "/" && next === "*") { i += 2; while (i < text.length && !(text[i] === "*" && text[i + 1] === "/")) i++; i++; continue; }
    out += ch;
  }
  return out.replace(/,(\s*[}\]])/g, "$1");
}

function readDoc(relative: string): any {
  return JSON.parse(stripJsonc(readFileSync(join(root, relative), "utf8")));
}

const vscode = readDoc(".vscode/launch.json");
const configurations: any[] = vscode.configurations ?? [];
const compounds: any[] = vscode.compounds ?? [];
const inputs: any[] = vscode.inputs ?? [];

const countBy = (items: any[], key: (item: any) => string) => {
  const counts: Record<string, number> = {};
  for (const item of items) { const k = key(item); counts[k] = (counts[k] ?? 0) + 1; }
  return Object.fromEntries(Object.entries(counts).sort((a, b) => b[1] - a[1]));
};

const names = configurations.map((c) => c.name);
const duplicateNames = Object.entries(countBy(names.map((name) => ({ name })), (x) => x.name)).filter(([, n]) => n > 1).length;

const vscodeStats = {
  file: ".vscode/launch.json",
  version: vscode.version,
  configurations: configurations.length,
  compounds: compounds.length,
  inputs: inputs.length,
  inputTypes: countBy(inputs, (i) => String(i.type)),
  configurationTypes: countBy(configurations, (c) => String(c.type)),
  configurationRequests: countBy(configurations, (c) => String(c.request)),
  groups: countBy(configurations, (c) => String(c.presentation?.group ?? "(none)")),
  withServerReadyAction: configurations.filter((c) => c.serverReadyAction !== undefined).length,
  withEnv: configurations.filter((c) => c.env !== undefined).length,
  withCwdOtherThanWorkspace: configurations.filter((c) => c.cwd !== undefined && c.cwd !== "${workspaceFolder}").length,
  withInputVariable: configurations.filter((c) => JSON.stringify(c).includes("${input:")).length,
  referencingTicketFolder: configurations.filter((c) => JSON.stringify(c).includes(".🧬semio/🦑️repo/🎫️tickets")).length,
  duplicateNames,
  compoundNames: compounds.map((c) => c.name),
};

const seed = readDoc(".vscode/🧩️launch.seed.jsonc");
const seedStats = {
  file: ".vscode/🧩️launch.seed.jsonc",
  topLevelKeys: Object.keys(seed),
  configurations: Array.isArray(seed.configurations) ? seed.configurations.length : 0,
  generatedPlaceholders: Array.isArray(seed.configurations)
    ? seed.configurations.filter((c: any) => typeof c === "string" || String(c.name ?? "").startsWith("@generated:")).length
    : 0,
  inputs: Array.isArray(seed.inputs) ? seed.inputs.length : 0,
  devLaunchers: seed.devLaunchers ? Object.keys(seed.devLaunchers).length : 0,
  projectLaunchers: seed.projectLaunchers ? Object.keys(seed.projectLaunchers).length : 0,
};

const claude = readDoc(".claude/launch.json");
const claudeEntries: any[] = claude.configurations ?? [];
const claudeStats = {
  file: ".claude/launch.json",
  version: claude.version,
  entries: claudeEntries.length,
  runnable: claudeEntries.filter((e) => e.runtimeExecutable !== undefined).length,
  attachOnly: claudeEntries.filter((e) => e.runtimeExecutable === undefined && e.url !== undefined).length,
  entriesWithPort: claudeEntries.filter((e) => e.port !== undefined).length,
  rows: claudeEntries.map((e) => ({
    name: e.name,
    runtimeExecutable: e.runtimeExecutable ?? null,
    runtimeArgs: (e.runtimeArgs ?? []).join(" "),
    port: e.port ?? null,
    url: e.url ?? null,
  })),
};

console.log(JSON.stringify({ vscodeStats, seedStats, claudeStats }, null, 2));
