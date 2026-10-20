import { writeFileSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { loadTargets, ROOT, TICKET, type T } from "./targets.ts";
import { entryFor, walk, narrow, domainOf, rel, type Evidence } from "./walk.ts";

export type Std = "rust-test" | "rust-check" | "rust-build" | "rust-snapshot" | "ts-test" | "ts-check" | "ts-build" | "none";
export type Rec = {
  project: string; target: string; manifest: string; root: string; command: string; script: string;
  handler: { kind: string; label: string; std: Std; snapshotFiles: string[]; suites: string[] };
  evidence: Evidence | null; flags: string[];
};

export const SUSPECT = /^(buildCargoArtifacts|buildRepositoryCargoArtifacts|pinExecutableArtifact|publishedCargoArtifactDirectoryV1|generateCollectionCatalogV1|withArtifactDestinationTaskV1|withRepositoryRustToolchainV1|withRepositoryToolingStoreV1|repoTestArtifactEnvironment|runMcpClientEndToEnd|runRepositoryMutationBridgeScriptV1|retireResourceLeasesV1|retireResourceLeaseV1|retireResourceQueueClaimsV1|buildSchedulerV1|checkArtifactTypeScriptSourceV1|prepareCargoWorkspaceInvocation)@/;
export const GENERATORISH = /\b(generate\w*|write\w*|apply\w*|update\w*|regenerate\w*|publish\w*|install\w*|restate|prepare|provision\w*|preview\w*|sync|fetch|download|bootstrap|migrate|report|discover|distribution|scale-fixture|tiles\w*)\b/;

function literalList(src: string, key: string): string[] {
  const i = src.indexOf(key); if (i < 0) return [];
  const open = src.indexOf("[", i); if (open < 0 || open - i > 40) return [];
  let d = 0, end = open; for (; end < src.length; end++) { if (src[end] === "[") d++; else if (src[end] === "]") { d--; if (d === 0) break; } }
  return [...src.slice(open, end).matchAll(/["'`]([^"'`]+)["'`]/g)].map((m) => m[1]!);
}

function stdOf(t: T, ev: Evidence): { std: Std; snapshotFiles: string[]; suites: string[] } {
  const w = t.words[0] ?? "";
  const rust = ev.texts.find((x) => x.text.includes("runArtifactRustPackageMain"));
  const ts = ev.texts.find((x) => x.text.includes("runArtifactTypeScriptPackageMain"));
  if (rust) {
    const std: Std = w === "test" ? "rust-test" : w === "check" ? "rust-check" : w === "build" ? "rust-build" : w === "test-snapshot-sqlite" ? "rust-snapshot" : "none";
    const files = std === "rust-snapshot" ? rust.text.length ? literalList(rust.text, "snapshotSqliteTests").map((p) => resolve(ROOT, t.cwd, p)).map(rel) : [] : [];
    return { std, snapshotFiles: files, suites: [] };
  }
  if (ts) {
    const std: Std = w === "test" ? "ts-test" : w === "check" ? "ts-check" : w === "build" ? "ts-build" : "none";
    return { std, snapshotFiles: [], suites: literalList(ts.text, "suites").map((p) => rel(resolve(ROOT, t.cwd, "../..", p))) };
  }
  return { std: "none", snapshotFiles: [], suites: [] };
}

export function classifyAll(): Rec[] {
  const out: Rec[] = [];
  const cache = new Map<string, Evidence>();
  for (const t of loadTargets()) {
    if (!t.def) continue;
    const e = entryFor(t);
    const domain = domainOf(t.root);
    const k = e.label + "|" + domain + "|" + t.words.join(" ");
    if (!cache.has(k)) cache.set(k, walk(e.entries.map((x) => ({ file: x.file, fn: narrow(x.fn, t.words) })), domain, t.words));
    const ev = cache.get(k)!;
    const s = stdOf(t, ev);
    const rec: Rec = { project: t.project, target: t.target, manifest: t.manifest, root: t.root, command: t.command, script: t.script!, handler: { kind: e.kind, label: e.label, ...s }, evidence: ev, flags: [] };
    for (const i of ev.infra) if (SUSPECT.test(i)) rec.flags.push("suspect-infra:" + i.split("@")[0]);
    for (const w of ev.writes) if (w.strong) rec.flags.push(`strong-write:${w.fn.split("#")[1]}:${w.line}`);
    if (GENERATORISH.test(t.words.join(" ") + " " + t.target)) rec.flags.push("generatorish-name");
    if (e.kind === "program" && s.std === "none") rec.flags.push("unclassified-program");
    out.push(rec);
  }
  return out;
}
if (import.meta.main) {
  const recs = classifyAll();
  const dir = join(TICKET, "🗑️generated", "uncached-outputs"); mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "records.json"), JSON.stringify(recs));
  const stds: Record<string, number> = {}; for (const r of recs) stds[r.handler.std + "/" + r.handler.kind] = (stds[r.handler.std + "/" + r.handler.kind] ?? 0) + 1;
  console.log(recs.length, "records;", recs.filter((r) => r.flags.length).length, "flagged"); console.log(stds);
}
