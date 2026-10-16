/** 🧭️ Shared model of the three owning packages and the cruiser's `no-cross-package-relative-*` selector. Read-only helpers. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { ROOT, scanSource, type ImportRecord } from "../sdkb-tools/lib.ts";
import { execFileSync } from "node:child_process";

const require = createRequire(import.meta.url);
const { authoredPackageExportAuthority } = require(ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs");

export const nfc = (s: string) => s.normalize("NFC");
export const MODULES = nfc("🧰️framework/🔨️modules/");
export const OWNERS = {
  schema: { sourceOwner: nfc(MODULES + "🧬️schema"), packages: ["@semio-tech/framework-schema", "@semio-tech/schema-ts"] },
  validator: { sourceOwner: nfc(MODULES + "🧬️schema/✅️validator"), packages: ["@semio-tech/schema-validator-ts", "@semio-tech/schema-validator-rs"] },
  value: { sourceOwner: nfc(MODULES + "🌱️value"), packages: ["@semio-tech/value-ts", "@semio-tech/value-rs"] },
} as const;
export type OwnerKey = keyof typeof OWNERS;

export interface Pkg { name: string; owner: string; sourceOwner: string; exportTargets: { subpath: string; target: string | null }[] }

export function loadPackages(): Pkg[] {
  const root = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));
  const out: Pkg[] = [];
  for (const w of root.workspaces as string[]) {
    let manifest: any;
    try { manifest = JSON.parse(readFileSync(join(ROOT, w, "package.json"), "utf8")); } catch { continue; }
    const a = authoredPackageExportAuthority(nfc(w), manifest);
    out.push({ name: manifest.name, owner: nfc(w), sourceOwner: nfc(a.sourceOwner), exportTargets: a.exportTargets });
  }
  for (const dir of [nfc(MODULES + "🧬️schema/📦️packages/🟦️typescript")]) {
    if (out.some(p => p.owner === dir)) continue;
    try {
      const manifest = JSON.parse(readFileSync(join(ROOT, dir, "package.json"), "utf8"));
      const a = authoredPackageExportAuthority(dir, manifest);
      out.push({ name: manifest.name, owner: dir, sourceOwner: nfc(a.sourceOwner), exportTargets: a.exportTargets });
    } catch {}
  }
  return out;
}

export function ownedRegion(pkgs: Pkg[], sourceOwner: string) {
  const nested = [...new Set(pkgs.map(p => p.sourceOwner).filter(o => o.startsWith(sourceOwner + "/")))];
  return (path: string) => path.startsWith(sourceOwner + "/") && !nested.some(n => path.startsWith(n + "/"));
}

export function exportTargetSet(pkgs: Pkg[], sourceOwner: string): Set<string> {
  const out = new Set<string>();
  for (const p of pkgs) if (p.sourceOwner === sourceOwner) for (const r of p.exportTargets) if (r.target) out.add(nfc(p.owner + "/" + r.target.slice(2)));
  return out;
}

export function listSourceFiles(): string[] {
  const run = (args: string[]) => execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: ROOT, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] }).toString().split("\n").filter(Boolean);
  const roots = ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
  const re = /\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u;
  const all = new Set([...run(["ls-files", "--", ...roots]), ...run(["ls-files", "--others", "--exclude-standard", "--", ...roots])].map(nfc).filter(f => re.test(f) && !f.includes("/node_modules/")));
  return [...all].sort();
}

export interface Edge { owner: OwnerKey; file: string; line: number; spec: string; kind: ImportRecord["kind"]; resolved: string; test: boolean; named: string[]; defaultName?: string; namespaceName?: string; exportStar: boolean; typeOnly: boolean; sideEffect: boolean; membersUnknown?: boolean; members?: string[] }

export const isTestPath = (p: string) => p.includes("/🧪️tests/") || p.includes("/🧪️test/") || p.includes("/🧪️testing/");

export function scanEdges(files: string[], pkgs: Pkg[]): Edge[] {
  const regions = Object.fromEntries((Object.keys(OWNERS) as OwnerKey[]).map(k => [k, ownedRegion(pkgs, OWNERS[k].sourceOwner)])) as Record<OwnerKey, (p: string) => boolean>;
  const allowed = Object.fromEntries((Object.keys(OWNERS) as OwnerKey[]).map(k => [k, exportTargetSet(pkgs, OWNERS[k].sourceOwner)])) as Record<OwnerKey, Set<string>>;
  const edges: Edge[] = [];
  for (const f of files) {
    let text: string;
    try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
    if (!/(?:from|import|require)\s*\(?\s*["'`]/u.test(text)) continue;
    for (const r of scanSource(f, text)) {
      if (!r.resolved) continue;
      const res = nfc(r.resolved);
      for (const k of Object.keys(OWNERS) as OwnerKey[]) {
        if (!regions[k](res) || regions[k](f) || allowed[k].has(res)) continue;
        edges.push({ owner: k, file: f, line: r.line, spec: r.spec, kind: r.kind, resolved: res, test: isTestPath(f), named: r.named.map(n => n.imported), defaultName: r.defaultName, namespaceName: r.namespaceName, exportStar: r.exportStar, typeOnly: r.typeOnly, sideEffect: r.sideEffect, membersUnknown: r.membersUnknown, members: r.members });
      }
    }
  }
  return edges;
}

export const OWNER_DIR: Record<OwnerKey, string> = {
  value: nfc(MODULES + "🌱️value/📦️packages/🟦️typescript"),
  validator: nfc(MODULES + "🧬️schema/✅️validator/📦️packages/🟦️typescript"),
  schema: nfc(MODULES + "🧬️schema/📦️packages/🟦️typescript"),
};
export const PKG_NAME: Record<OwnerKey, string> = { value: "@semio-tech/value-ts", validator: "@semio-tech/schema-validator-ts", schema: "@semio-tech/schema-ts" };

export const segName = (seg: string) => seg.replace(/^(?:\p{Extended_Pictographic}(?:\u{FE0F}|\u{200D}\p{Extended_Pictographic}\u{FE0F}?)*)+/u, "").replace(/^\u{FE0F}/u, "");
export const isAsset = (p: string) => !/\.(ts|tsx)$/u.test(p);

export function loadAllPackages(): Pkg[] {
  const seen = new Map<string, Pkg>();
  for (const p of loadPackages()) seen.set(p.owner, p);
  for (const dir of ["✏️s", "🌎️hub", "♻️mit-bestand", "🏢️semio-tech", "🎓️teaching"]) {
    let ws: string[] = [];
    try { const m = JSON.parse(readFileSync(join(ROOT, dir, "package.json"), "utf8")); ws = Array.isArray(m.workspaces) ? m.workspaces : (m.workspaces?.packages ?? []); } catch { continue; }
    for (const w of ws) {
      if (w.includes("*") || w.startsWith("!")) continue;
      const owner = nfc(join(dir, w).replace(/\\/gu, "/"));
      const norm = nfc(require("node:path").normalize(join(dir, w)));
      if (seen.has(norm)) continue;
      let manifest: any;
      try { manifest = JSON.parse(readFileSync(join(ROOT, norm, "package.json"), "utf8")); } catch { continue; }
      const a = authoredPackageExportAuthority(norm, manifest);
      seen.set(norm, { name: manifest.name, owner: norm, sourceOwner: nfc(a.sourceOwner), exportTargets: a.exportTargets });
      void owner;
    }
  }
  return [...seen.values()];
}
