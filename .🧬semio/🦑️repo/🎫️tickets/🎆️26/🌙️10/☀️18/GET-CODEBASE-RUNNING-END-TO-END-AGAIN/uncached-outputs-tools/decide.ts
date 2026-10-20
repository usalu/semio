import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { loadTargets, ROOT, TICKET } from "./targets.ts";
import { classifyAll, type Rec } from "./classify.ts";
import { walkFile } from "./files.ts";
import { OVERRIDES } from "./overrides.ts";

export type Resolution = "verdict" | "uncache" | "contract" | "stale";
export type Decision = {
  id: string; project: string; target: string; manifest: string; command: string; resolution: Resolution; basis: string; reason: string;
  evidence: { handler: string; std: string; ownedFunctions: number; files: number; spawns: Record<string, number>; strongWrites: number; otherWrites: number; suspectInfra: string[]; snapshotFiles: number; missingSnapshotFiles: string[] };
};

const spawnClass = (args: string): string => /["']test["']/.test(args) ? "bun-test" : /tsc|--noEmit/.test(args) ? "tsc" : /cargo|rustfmt/.test(args) ? "cargo-rustfmt" : /["']repo["']\s*,\s*["']run/.test(args) ? "build-tool" : "other";

function summarize(rec: Rec): Decision["evidence"] {
  const ev = rec.evidence!;
  const spawns: Record<string, number> = {};
  for (const s of ev.spawns) { const k = spawnClass(s.args); spawns[k] = (spawns[k] ?? 0) + 1; }
  let strong = ev.writes.filter((w) => w.strong).length, other = ev.writes.length - strong, owned = ev.owned.length, files = ev.files.length;
  const missing: string[] = [];
  for (const f of [...rec.handler.snapshotFiles, ...rec.handler.suites]) {
    if (!existsSync(join(ROOT, f))) { missing.push(f); continue; }
    const fe = walkFile(f, rec.root);
    strong += fe.writes.filter((w) => w.strong).length; other += fe.writes.filter((w) => !w.strong).length; owned += fe.owned.length; files += 1;
    for (const s of fe.spawns) { const k = spawnClass(s.args); spawns[k] = (spawns[k] ?? 0) + 1; }
  }
  return { handler: rec.handler.label, std: rec.handler.std, ownedFunctions: owned, files, spawns, strongWrites: strong, otherWrites: other, suspectInfra: rec.flags.filter((f) => f.startsWith("suspect-infra:")).map((f) => f.slice(14)), snapshotFiles: rec.handler.snapshotFiles.length, missingSnapshotFiles: missing };
}

const BASIS: Record<string, string> = {
  "rust-test": "artifact-rust standard runner: test runs runRepositoryCargoTests (cargo test in the leased compiler store); twins only print",
  "rust-check": "artifact-rust standard runner: check runs cargo check in the leased compiler store",
  "rust-snapshot": "artifact-rust standard runner: native lane = cargo test --lib sqlite_snapshot_, source lane = bun test of the listed snapshot suites",
  "ts-test": "TypeScript artifact standard runner: test builds into scratch/products store, type-checks and bun-tests the registered suites; project-assembly sets outputs = [] for it",
  "ts-check": "TypeScript artifact standard runner: bundle plus tsc --noEmit",
};

export function decideAll(): Decision[] {
  const recs = new Map(classifyAll().map((r) => [r.project + ":" + r.target, r]));
  const out: Decision[] = [];
  for (const t of loadTargets()) {
    const id = t.project + ":" + t.target;
    if (!t.def) {
      out.push({ id, project: t.project, target: t.target, manifest: t.manifest, command: "", resolution: "stale", basis: "absent", reason: "the target no longer exists in the manifest or its scripts (removed after the report)", evidence: { handler: "-", std: "-", ownedFunctions: 0, files: 0, spawns: {}, strongWrites: 0, otherWrites: 0, suspectInfra: [], snapshotFiles: 0, missingSnapshotFiles: [] } });
      continue;
    }
    const rec = recs.get(id)!;
    const evidence = summarize(rec);
    const manual = OVERRIDES[id];
    if (manual) { out.push({ id, project: t.project, target: t.target, manifest: t.manifest, command: t.command, resolution: manual.resolution, basis: "hand-read", reason: manual.reason, evidence }); continue; }
    const std = rec.handler.std;
    const basis = BASIS[std] ?? "handler analysis";
    const reason = std !== "none" ? basis : `handler ${rec.handler.label}: ${evidence.ownedFunctions} reachable functions, ${evidence.strongWrites} workspace-anchored write sites, spawns ${JSON.stringify(evidence.spawns)}`;
    if (evidence.strongWrites > 0 || rec.flags.some((f) => f.startsWith("suspect-infra:") || f === "unclassified-program")) {
      out.push({ id, project: t.project, target: t.target, manifest: t.manifest, command: t.command, resolution: "verdict", basis: "UNREVIEWED", reason: "flagged by the analysis but not hand-read: " + rec.flags.join(" "), evidence });
      continue;
    }
    out.push({ id, project: t.project, target: t.target, manifest: t.manifest, command: t.command, resolution: "verdict", basis: std !== "none" ? "std:" + std : "handler", reason, evidence });
  }
  return out.sort((a, b) => (a.id < b.id ? -1 : a.id > b.id ? 1 : 0));
}

if (import.meta.main) {
  const decisions = decideAll();
  const dir = join(TICKET, "🗑️generated", "uncached-outputs"); mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "decisions.json"), JSON.stringify(decisions, null, 1));
  const c: Record<string, number> = {}; for (const d of decisions) c[d.resolution + "/" + d.basis.split(":")[0]] = (c[d.resolution + "/" + d.basis.split(":")[0]] ?? 0) + 1;
  console.log(decisions.length, c);
  for (const d of decisions.filter((x) => x.basis === "UNREVIEWED")) console.log("UNREVIEWED", d.id, d.reason.slice(0, 160));
}
