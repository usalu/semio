/** 🧭️ Generalised model of the cruiser's `no-cross-package-relative-*` selector over every workspace package. Read-only helpers. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { ROOT, scanSource, type ImportRecord } from "../sdkb-tools/lib.ts";

export const nfc = (s: string) => s.normalize("NFC");
export interface PolicyPkg { owner: string; name: string; sourceOwner: string; exports: string[]; exportTargets: { subpath: string; target: string | null }[] }
export interface PolicyRule { name: string; from: { path: string; pathNot: string }; to: { path: string[]; pathNot: string[] } }
export interface Policy { packages: PolicyPkg[]; rules: PolicyRule[] }
export interface Group { sourceOwner: string; packages: PolicyPkg[]; tsPackage: PolicyPkg | null; rules: PolicyRule[]; toPath: RegExp[]; toPathNot: RegExp[]; fromPathNot: RegExp }

export function loadPolicy(path: string): Policy {
  const p: Policy = JSON.parse(readFileSync(path, "utf8"));
  for (const k of p.packages) { k.owner = nfc(k.owner); k.sourceOwner = nfc(k.sourceOwner); }
  return p;
}

const rx = (s: string) => { try { return new RegExp(s.normalize("NFC"), "u"); } catch { return new RegExp(s.normalize("NFC")); } };

export function groupsOf(policy: Policy): Group[] {
  const by = new Map<string, Group>();
  for (const pkg of policy.packages) {
    const g = by.get(pkg.sourceOwner) ?? { sourceOwner: pkg.sourceOwner, packages: [], tsPackage: null, rules: [], toPath: [], toPathNot: [], fromPathNot: /$^/ };
    g.packages.push(pkg);
    by.set(pkg.sourceOwner, g);
  }
  for (const r of policy.rules) {
    const pkg = policy.packages.find(p => "no-cross-package-relative-" + p.name === r.name);
    if (!pkg) continue;
    const g = by.get(pkg.sourceOwner)!;
    if (g.rules.length === 0) { g.toPath = r.to.path.map(rx); g.toPathNot = r.to.pathNot.map(rx); g.fromPathNot = rx(r.from.pathNot); }
    g.rules.push(r);
  }
  for (const g of by.values()) g.tsPackage = g.packages.find(p => p.owner.includes("/🟦️typescript")) ?? null;
  return [...by.values()];
}

export function listSourceFiles(): string[] {
  const run = (args: string[]) => execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: ROOT, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] }).toString().split("\n").filter(Boolean);
  const roots = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
  const re = /\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u;
  const all = new Set([...run(["ls-files", "--", ...roots]), ...run(["ls-files", "--others", "--exclude-standard", "--", ...roots])].map(nfc).filter(f => re.test(f) && !f.includes("/node_modules/")));
  return [...all].sort();
}

export const isTestPath = (p: string) => p.includes("/🧪️tests/") || p.includes("/🧪️test/") || p.includes("/🧪️testing/");

export interface Edge { sourceOwner: string; file: string; line: number; spec: string; kind: ImportRecord["kind"]; resolved: string; test: boolean; named: string[]; defaultName?: string; namespaceName?: string; exportStar: boolean; typeOnly: boolean; sideEffect: boolean; membersUnknown?: boolean; members?: string[] }

export function scanEdges(files: string[], groups: Group[]): Edge[] {
  const index = new Map(groups.map(g => [g.sourceOwner, g]));
  const edges: Edge[] = [];
  for (const f of files) {
    let text: string;
    try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
    if (!/(?:from|import|require)\s*\(?\s*["'`]/u.test(text)) continue;
    for (const r of scanSource(f, text)) {
      if (!r.resolved) continue;
      const res = nfc(r.resolved);
      for (let p = res.indexOf("/"); p >= 0; p = res.indexOf("/", p + 1)) {
        const g = index.get(res.slice(0, p));
        if (!g) continue;
        if (g.fromPathNot.test(f)) continue;
        if (!g.toPath.some(x => x.test(res)) || g.toPathNot.some(x => x.test(res))) continue;
        edges.push({ sourceOwner: g.sourceOwner, file: f, line: r.line, spec: r.spec, kind: r.kind, resolved: res, test: isTestPath(f), named: r.named.map(n => n.imported), defaultName: r.defaultName, namespaceName: r.namespaceName, exportStar: r.exportStar, typeOnly: r.typeOnly, sideEffect: r.sideEffect, membersUnknown: r.membersUnknown, members: r.members });
      }
    }
  }
  return edges;
}

export const segName = (seg: string) => seg.replace(/^(?:\p{Extended_Pictographic}(?:\u{FE0F}|\u{200D}\p{Extended_Pictographic}\u{FE0F}?)*)+/u, "").replace(/^\u{FE0F}/u, "");
export const isAsset = (p: string) => !/\.(ts|tsx)$/u.test(p);
