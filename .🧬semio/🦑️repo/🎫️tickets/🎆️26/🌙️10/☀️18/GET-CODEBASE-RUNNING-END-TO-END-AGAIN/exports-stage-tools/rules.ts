/**
 * ⚖️ The live dependency-cruiser policy (dumped by policy-dump.cjs) as an evaluator: exact `no-cross-package-relative-*` selectors and the remaining path-only rules.
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED } from "./lib.ts";

type Rule = { name: string; from?: { path?: string | string[]; pathNot?: string | string[] }; to?: { path?: string | string[]; pathNot?: string | string[]; dependencyTypes?: string[]; dependencyTypesNot?: string[] } };
const list = (v: string | string[] | undefined): string[] => (v === undefined ? [] : Array.isArray(v) ? v : [v]);
const compile = (patterns: string[]): RegExp | null => {
  if (!patterns.length) return null;
  try { return new RegExp(patterns.map(p => `(?:${p})`).join("|"), "u"); } catch { return new RegExp(patterns.map(p => `(?:${p})`).join("|")); }
};

export type Selector = { owner: string; fromNot: RegExp | null; toPath: RegExp | null; toNot: RegExp | null; names: string[] };
export type Policy = { selectors: Selector[]; byOwner: Map<string, Selector>; others: { name: string; fromPath: RegExp | null; fromNot: RegExp | null; toPath: RegExp | null; toNot: RegExp | null; types: string[]; typesNot: string[] }[] };

export function loadPolicy(): Policy {
  const p = JSON.parse(readFileSync(resolve(GENERATED, "policy.json"), "utf8")) as { forbidden: Rule[] };
  const byOwner = new Map<string, Selector>();
  const others: Policy["others"] = [];
  for (const r of p.forbidden) {
    if (r.name.startsWith("no-cross-package-relative-")) {
      const toPath = list(r.to?.path);
      const m = /^\^(.*?)\/\(\?!\(\?:node_modules/u.exec(toPath[0] ?? "");
      if (!m) throw Error("unreadable owner selector in " + r.name);
      const owner = m[1]!.replace(/\\(.)/gu, "$1");
      const have = byOwner.get(owner);
      const name = r.name.slice("no-cross-package-relative-".length);
      if (have) { have.names.push(name); continue; }
      byOwner.set(owner, { owner, fromNot: compile(list(r.from?.pathNot)), toPath: compile(toPath), toNot: compile(list(r.to?.pathNot)), names: [name] });
    } else {
      others.push({ name: r.name, fromPath: compile(list(r.from?.path)), fromNot: compile(list(r.from?.pathNot)), toPath: compile(list(r.to?.path)), toNot: compile(list(r.to?.pathNot)), types: r.to?.dependencyTypes ?? [], typesNot: r.to?.dependencyTypesNot ?? [] });
    }
  }
  return { selectors: [...byOwner.values()], byOwner, others };
}

export function ownerOfFile(policy: Policy, file: string): Selector | undefined {
  let best: Selector | undefined;
  for (const s of policy.selectors) if (file.startsWith(s.owner + "/") && (!best || s.owner.length > best.owner.length) && s.toPath!.test(file)) best = s;
  return best;
}

export function ncprViolation(policy: Policy, from: string, to: string): Selector | undefined {
  const s = ownerOfFile(policy, to);
  if (!s) return undefined;
  if (s.fromNot?.test(from)) return undefined;
  if (s.toNot?.test(to)) return undefined;
  return s;
}

export function otherViolations(policy: Policy, from: string, to: string, type: string, skip: (name: string) => boolean = () => false): string[] {
  const out: string[] = [];
  for (const r of policy.others) {
    if (skip(r.name)) continue;
    if (r.types.length && !r.types.includes(type)) continue;
    if (r.typesNot.includes(type)) continue;
    if (r.fromPath && !r.fromPath.test(from)) continue;
    if (r.fromNot?.test(from)) continue;
    if (r.toPath && !r.toPath.test(to)) continue;
    if (r.toNot?.test(to)) continue;
    if (!r.fromPath && !r.toPath && !r.types.length) continue;
    out.push(r.name);
  }
  return out;
}
