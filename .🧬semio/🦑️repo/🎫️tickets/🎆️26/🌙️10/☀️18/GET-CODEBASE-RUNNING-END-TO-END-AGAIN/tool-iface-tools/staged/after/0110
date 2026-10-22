import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { findToolV1, gitSyncV1, readToolSourceV1, readToolDigestsV1, scanToolCallsV1, scanToolSpawnsV1, toolCallerAllowedV1, toolGlobMatchesV1, toolScanLanguageV1, toolRawSpawnAllowedV1, TOOL_REGISTRY_V1, type ToolRegistryV1, type ToolSpawnSiteV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";
import ledgerDocument from "./🧾️ledger/🔣️.json" with { type: "json" };
import ledgerSchema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🧾️ One ledgered offender: a test file that still starts a registered tool itself, counted per tool and form. */
export type ToolSpawnLedgerEntryV1 = Readonly<{ path: string; tool: string; form: "spawn" | "shell"; count: number }>;
/** 🗂️ The test-code ledger: the direct spawns test files still make, with the reason they stay. Production code has no ledger; every production spawn outside an adapter is a finding. */
export type ToolSpawnLedgerV1 = Readonly<{ version: 1; reason: Readonly<{ en: string; de: string }>; entries: readonly ToolSpawnLedgerEntryV1[] }>;
/** 📋️ Every offending spawn site counted per file, tool, form and scope, and the commands manifests declare per tool and scope (reported, not enforced: the manifest wave rewrites them). */
export type ToolSpawnSummaryV1 = Readonly<{ entries: readonly (ToolSpawnLedgerEntryV1 & Readonly<{ scope: "prod" | "test" }>)[]; declared: readonly Readonly<{ tool: string; scope: "prod" | "test"; count: number }>[] }>;
/** 🚩️ One violated rule of the law. */
export type ToolSpawnFindingV1 = Readonly<{ rule: "raw-spawn" | "caller-not-allowed" | "unknown-tool" | "registry"; path: string; tool: string; detail: string }>;

/** 🏠️ The directory of this law, relative to the repository root. */
export const TOOL_SPAWN_LAW_DIRECTORY_V1 = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧰️direct-tool-spawn";
/** 📍️ The ledger file, relative to the repository root. */
export const TOOL_SPAWN_LEDGER_FILE_V1 = `${TOOL_SPAWN_LAW_DIRECTORY_V1}/🧾️ledger/🔣️.json`;
const SELF = [TOOL_SPAWN_LAW_DIRECTORY_V1 + "/**", "🧰️framework/🔨️modules/🏃️process/🧰️tools/**", "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/**"];

/** 📖️ Validates a ledger document against its schema. */
export function readToolSpawnLedgerV1(document: unknown): ToolSpawnLedgerV1 {
  const errors = validateJsonSchemaSubset(ledgerSchema, document);
  if (errors.length) throw new Error(`Tool spawn ledger invalid: ${errors.slice(0, 5).join("; ")}`);
  return document as ToolSpawnLedgerV1;
}

/** 📚️ The committed ledger. */
export const TOOL_SPAWN_LEDGER_V1: ToolSpawnLedgerV1 = readToolSpawnLedgerV1(ledgerDocument);

/** 🗒️ Lists the repository files git knows (tracked and not ignored), the set the law reads. */
export function listToolSpawnFilesV1(root: string): readonly string[] {
  const result = gitSyncV1(["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 512 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git ls-files failed: ${result.stderr.trim()}`);
  return result.stdout.split("\0").filter(Boolean);
}

const skipped = (path: string, registry: ToolRegistryV1): boolean => {
  const segments = path.split("/");
  return segments.some(segment => registry.scan.ignoredSegments.includes(segment)) || segments[segments.length - 1]!.startsWith(".") || SELF.some(pattern => toolGlobMatchesV1(pattern, path));
};

/** 🔍️ Scans the listed files and answers every spawn site of a registered tool plus every call of the tool interface. */
export function scanRepositoryToolSpawnsV1(root: string, files: readonly string[], registry: ToolRegistryV1 = TOOL_REGISTRY_V1, read: (path: string) => string | undefined = path => { try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; } }): Readonly<{ sites: readonly ToolSpawnSiteV1[]; calls: ReturnType<typeof scanToolCallsV1>; scanned: number }> {
  const sites: ToolSpawnSiteV1[] = [], calls: ReturnType<typeof scanToolCallsV1>[number][] = [];
  let scanned = 0;
  for (const path of files) {
    if (skipped(path, registry) || toolScanLanguageV1(path, registry) === undefined) continue;
    const text = read(path);
    if (text === undefined || text.length > 4_000_000) continue;
    scanned++;
    sites.push(...scanToolSpawnsV1(path, text, registry));
    calls.push(...scanToolCallsV1(path, text, registry));
  }
  return { sites, calls, scanned };
}

/** 🧮️ Counts the offending sites (those outside the adapter scope of their tool) per file, tool, form and scope; declared manifest commands are counted per tool and scope. */
export function summarizeToolSpawnsV1(sites: readonly ToolSpawnSiteV1[], registry: ToolRegistryV1 = TOOL_REGISTRY_V1): ToolSpawnSummaryV1 {
  const entries = new Map<string, ToolSpawnSummaryV1["entries"][number]>(), declared = new Map<string, ToolSpawnSummaryV1["declared"][number]>();
  for (const site of sites) {
    const entry = site.tool === null ? undefined : findToolV1(site.tool, registry);
    if (entry === undefined) continue;
    if (site.form === "declared") { const key = `${site.tool}\0${site.scope}`, row = declared.get(key); declared.set(key, { tool: site.tool!, scope: site.scope, count: (row?.count ?? 0) + 1 }); continue; }
    if (toolRawSpawnAllowedV1(entry, site.path)) continue;
    const key = `${site.path}\0${site.tool}\0${site.form}\0${site.scope}`, row = entries.get(key);
    entries.set(key, { path: site.path, tool: site.tool!, form: site.form, scope: site.scope, count: (row?.count ?? 0) + 1 });
  }
  const order = (a: string, b: string): number => a < b ? -1 : a > b ? 1 : 0;
  return { entries: [...entries.values()].sort((a, b) => order(a.path, b.path) || order(a.tool, b.tool) || order(a.form, b.form)), declared: [...declared.values()].sort((a, b) => order(a.tool, b.tool) || order(a.scope, b.scope)) };
}

/** 🩺️ Checks the registry against the repository: adapters and installers exist, pinned tools have readable pins and digests, adapter scopes match files, calls name registered tools from allowed callers. */
export function toolRegistryProblemsV1(root: string, files: ReadonlySet<string>, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): readonly ToolSpawnFindingV1[] {
  const problems: ToolSpawnFindingV1[] = [], found = (tool: string, path: string, detail: string): void => { problems.push({ rule: "registry", path, tool, detail }); };
  const exists = (path: string): boolean => files.has(path) || existsSync(join(root, path));
  for (const tool of registry.tools) {
    if (!exists(tool.adapter)) found(tool.id, tool.adapter, "the adapter module does not exist");
    if (tool.provision.installer !== undefined && !exists(tool.provision.installer)) found(tool.id, tool.provision.installer, "the installer module does not exist");
    if ((tool.pin.state === "pinned" || tool.pin.state === "constrained") && readToolSourceV1(tool.pin.version, root) === null) found(tool.id, tool.pin.version?.file ?? "", "the pin file gives no version");
    if (tool.pin.digests !== undefined && readToolDigestsV1(tool.pin.digests, root).length === 0) found(tool.id, tool.pin.digests.file, "the pin file gives no digests");
    if (tool.pin.state === "floating" && tool.pin.gap === undefined) found(tool.id, "", "a floating pin must state its gap");
    for (const pattern of [...tool.adapterScope, ...tool.allowedCallers]) if (!pattern.includes("**") && ![...files].some(path => toolGlobMatchesV1(pattern, path))) found(tool.id, pattern, "the glob matches no file");
  }
  for (const { path, marker } of registry.funnels ?? []) {
    let text = ""; try { text = readFileSync(join(root, path), "utf8"); } catch { found("funnels", path, "the process funnel does not exist"); continue; }
    if (!text.includes(marker)) found("funnels", path, `the process funnel no longer routes registered tools through the registry (${marker} is missing)`);
  }
  return problems;
}

/** ⚖️ Compares the scan with the ledger: every production spawn outside an adapter is a finding; a test spawn beyond its ledger row, an uncalled-for caller or an unknown tool id are findings too; a ledger larger than the truth is only stale. */
export function toolSpawnFindingsV1(scan: Pick<ReturnType<typeof scanRepositoryToolSpawnsV1>, "sites" | "calls">, ledger: ToolSpawnLedgerV1, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): Readonly<{ findings: readonly ToolSpawnFindingV1[]; stale: readonly string[] }> {
  const findings: ToolSpawnFindingV1[] = [], stale: string[] = [], actual = summarizeToolSpawnsV1(scan.sites, registry);
  const key = (row: ToolSpawnLedgerEntryV1): string => `${row.path}\0${row.tool}\0${row.form}`, tolerated = new Map(ledger.entries.map(row => [key(row), row.count]));
  for (const row of actual.entries) {
    const allowed = row.scope === "prod" ? 0 : tolerated.get(key(row)) ?? 0;
    if (row.count > allowed) findings.push({ rule: "raw-spawn", path: row.path, tool: row.tool, detail: `${row.count - allowed} ${row.form} of ${row.tool} outside its adapter${row.scope === "test" ? ` (${allowed} ledgered)` : ""}` });
  }
  const seen = new Map(actual.entries.filter(row => row.scope === "test").map(row => [key(row), row.count]));
  for (const row of ledger.entries) if ((seen.get(key(row)) ?? 0) < row.count) stale.push(`${row.path} ${row.tool} ${row.form} ${seen.get(key(row)) ?? 0}/${row.count}`);
  for (const call of scan.calls) {
    const entry = findToolV1(call.tool, registry);
    if (entry === undefined) findings.push({ rule: "unknown-tool", path: call.path, tool: call.tool, detail: `line ${call.line}: ${call.callee}("${call.tool}") names no registered tool` });
    else if (!toolCallerAllowedV1(entry, call.path, registry)) findings.push({ rule: "caller-not-allowed", path: call.path, tool: call.tool, detail: `line ${call.line}: ${call.path} is not an allowed caller of ${call.tool}` });
  }
  return { findings, stale };
}

/** 🪜️ Shrinks the ledger to the truth: counts only fall, rows vanish at zero, and new rows appear only when `acceptNew` says so. */
export function pruneToolSpawnLedgerV1(ledger: ToolSpawnLedgerV1, actual: ToolSpawnSummaryV1, acceptNew = false): ToolSpawnLedgerV1 {
  const key = (row: ToolSpawnLedgerEntryV1): string => `${row.path}\0${row.tool}\0${row.form}`, now = new Map(actual.entries.filter(row => row.scope === "test").map(row => [key(row), row])), kept = new Map(ledger.entries.map(row => [key(row), row]));
  const entries = [...(acceptNew ? now.keys() : kept.keys())].flatMap(name => { const truth = now.get(name); if (truth === undefined) return []; const old = kept.get(name); return [{ path: truth.path, tool: truth.tool, form: truth.form, count: acceptNew || old === undefined ? truth.count : Math.min(old.count, truth.count) }]; });
  return { version: 1, reason: ledger.reason, entries };
}

/** 🖨️ Renders the ledger deterministically, one row per line, so a ledger diff shows exactly the rows that moved. */
export function renderToolSpawnLedgerV1(ledger: ToolSpawnLedgerV1): string {
  return `{"$schema": "../🧬️schema/🔣️.json", "version": 1,\n "reason": ${JSON.stringify(ledger.reason)},\n "entries": [\n${ledger.entries.map(row => "  " + JSON.stringify(row)).join(",\n")}\n ]}\n`;
}

/** 💾️ Writes the ledger through a temporary file and an atomic rename. */
export function writeToolSpawnLedgerV1(root: string, ledger: ToolSpawnLedgerV1): void {
  const target = join(root, TOOL_SPAWN_LEDGER_FILE_V1), temporary = `${target}.${process.pid}.tmp`;
  writeFileSync(temporary, renderToolSpawnLedgerV1(ledger));
  renameSync(temporary, target);
}

/** 🪫️ Fails when a repository file starts a registered external tool itself instead of going through `runToolV1`, when the interface is called by a file the registry does not allow, or when the registry disagrees with the repository; each finding names the file, the tool and the repair. */
export function policyDirectToolSpawnBreaches(repoRoot: string): BreachRecord[] {
  const files = listToolSpawnFilesV1(repoRoot), scan = scanRepositoryToolSpawnsV1(repoRoot, files);
  return [...toolSpawnFindingsV1(scan, TOOL_SPAWN_LEDGER_V1).findings, ...toolRegistryProblemsV1(repoRoot, new Set(files))].map(finding => ({
    id: `direct-tool-spawn-${finding.rule}-${finding.tool}-${finding.path}`,
    summary: `${finding.rule} ${finding.path} ${finding.tool}: ${finding.detail}`,
    kind: "repo/direct-tool-spawn",
    scope: finding.path,
    priority: "high" as const,
    autofixable: false,
    reason: "External programs are swappable only while every start goes through the tool registry; a direct spawn pins the repository to one implementation.",
    solution: `Call runToolV1("${finding.tool}", args, options) from the tools module (repo tools list shows the adapter and the allowed callers), or fix the registry row.`,
  }));
}
