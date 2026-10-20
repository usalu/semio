import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { relative } from "node:path";
import { Package } from "../../../rs-sweep-tools/cargo.ts";
import type { Owners } from "./resolve-owner.ts";
import { ROOT } from "./resolve-owner.ts";

export type ManifestPlan = { edited: number; wsDecls: number; issues: { manifest: string; issue: string }[] };

const FACADE_KEY = "semio-framework-value";
type Occ = { line: number; indent: string; form: "ws-inline" | "ws-dotted" | "path"; section: string; group: string };

const depSection = (h: string): { group: string; kind: "dependencies" | "dev-dependencies" | "build-dependencies" } | null => {
  const m = /^(target\..+\.)?(dev-|build-)?dependencies$/.exec(h);
  if (!m) return null;
  return { group: m[1] ?? "", kind: m[2] === "dev-" ? "dev-dependencies" : m[2] === "build-" ? "build-dependencies" : "dependencies" };
};

const rel = (from: string, to: string) => relative(from, to).split("\\").join("/");

/** 📦 Adds the owning-crate dependencies next to every facade dependency line, declaring them in the workspace when it inherits. */
export function planManifests(libsByPkg: Map<string, { pkg: Package; libs: Set<string> }>, owners: Owners, apply: boolean, outDir: string, sink?: Map<string, { before: string; after: string }>): ManifestPlan {
  const plan: ManifestPlan = { edited: 0, wsDecls: 0, issues: [] };
  const wsNeeds = new Map<string, { dir: string; names: Map<string, string> }>();
  const issue = (m: string, i: string) => plan.issues.push({ manifest: relative(ROOT, m), issue: i });
  for (const { pkg, libs } of libsByPkg.values()) {
    const text = readFileSync(pkg.manifest, "utf8");
    const lines = text.split("\n");
    const occ: Occ[] = [];
    const present = new Map<string, Set<string>>();
    let section = "";
    for (let i = 0; i < lines.length; i++) {
      const h = /^\s*\[([^\[\]]+)\]\s*$/.exec(lines[i]);
      if (h) {
        section = h[1].trim().replace(/"/g, "'");
        const tbl = /^((?:target\..+\.)?(?:dev-|build-)?dependencies)\.([A-Za-z0-9_-]+)$/.exec(section);
        if (tbl && tbl[2] === FACADE_KEY) issue(pkg.manifest, `table-form dependency [${section}] unsupported`);
        continue;
      }
      const ds = depSection(section);
      if (!ds) continue;
      const key = /^\s*([A-Za-z0-9_-]+)(?:\s*=|\.)/.exec(lines[i]);
      if (key) (present.get(section) ?? present.set(section, new Set()).get(section)!).add(key[1]);
      const indent = /^(\s*)/.exec(lines[i])![1];
      if (new RegExp(`^\\s*${FACADE_KEY}\\s*=\\s*\\{\\s*workspace\\s*=\\s*true\\b`).test(lines[i])) occ.push({ line: i, indent, form: "ws-inline", section, group: ds.group + ":" + ds.kind });
      else if (new RegExp(`^\\s*${FACADE_KEY}\\.workspace\\s*=\\s*true`).test(lines[i])) occ.push({ line: i, indent, form: "ws-dotted", section, group: ds.group + ":" + ds.kind });
      else if (new RegExp(`^\\s*${FACADE_KEY}\\s*=\\s*\\{[^}]*\\bpath\\s*=\\s*"[^"]+"`).test(lines[i])) occ.push({ line: i, indent, form: "path", section, group: ds.group + ":" + ds.kind });
      else if (new RegExp(`^\\s*${FACADE_KEY}\\s*(=|\\.)`).test(lines[i])) issue(pkg.manifest, `unsupported facade dependency form: ${lines[i].trim().slice(0, 100)}`);
    }
    if (!occ.length) {
      issue(pkg.manifest, "package rewrote files but declares no direct semio-framework-value dependency (foreign-mounted source?)");
      continue;
    }
    const byTarget = new Map<string, Occ[]>();
    for (const o of occ) {
      const tgt = o.group.split(":")[0];
      (byTarget.get(tgt) ?? byTarget.set(tgt, []).get(tgt)!).push(o);
    }
    const chosen: Occ[] = [];
    for (const list of byTarget.values()) {
      const rank = (o: Occ) => (o.group.endsWith("dependencies") && !o.group.includes("dev-") && !o.group.includes("build-") ? 0 : o.group.includes("dev-") ? 1 : 2);
      const best = Math.min(...list.map(rank));
      chosen.push(...list.filter((o) => rank(o) === best));
    }
    const inserts: { line: number; text: string }[] = [];
    for (const o of chosen) {
      for (const lib of [...libs].sort()) {
        const name = lib.replace(/_/g, "-");
        if (present.get(o.section)?.has(name)) continue;
        const owner = owners.byLib.get(lib)!;
        if (o.form === "path") inserts.push({ line: o.line, text: `${o.indent}${name} = { path = "${rel(pkg.dir, owner.dir)}" }` });
        else {
          inserts.push({ line: o.line, text: o.form === "ws-inline" ? `${o.indent}${name} = { workspace = true }` : `${o.indent}${name}.workspace = true` });
          if (!pkg.ws) issue(pkg.manifest, "workspace-inherited dependency but no workspace found");
          else if (!(pkg.ws.deps && name in pkg.ws.deps)) {
            const wm = `${pkg.ws.dir}/Cargo.toml`;
            const e = wsNeeds.get(wm) ?? { dir: pkg.ws.dir, names: new Map() };
            e.names.set(name, lib);
            wsNeeds.set(wm, e);
          }
        }
      }
    }
    if (!inserts.length) continue;
    inserts.sort((a, b) => b.line - a.line);
    const seen = new Set<string>();
    for (const ins of inserts) {
      const k = `${ins.line}|${ins.text}`;
      if (seen.has(k)) continue;
      seen.add(k);
      lines.splice(ins.line + 1, 0, ins.text);
    }
    plan.edited++;
    sink?.set(pkg.manifest, { before: text, after: lines.join("\n") });
    if (apply) {
      const fresh = readFileSync(pkg.manifest, "utf8");
      if (fresh !== text) {
        issue(pkg.manifest, "manifest changed during run, skipped");
        continue;
      }
      writeFileSync(pkg.manifest, lines.join("\n"));
    }
  }
  for (const [wm, need] of wsNeeds) {
    const text = readFileSync(wm, "utf8");
    const lines = text.split("\n");
    let tbl = -1;
    let inline = -1;
    let inWsDeps = false;
    for (let i = 0; i < lines.length; i++) {
      if (new RegExp(`^\\s*\\[workspace\\.dependencies\\.${FACADE_KEY}\\]\\s*$`).test(lines[i])) tbl = i;
      const h = /^\s*\[([^\[\]]+)\]\s*$/.exec(lines[i]);
      if (h) inWsDeps = h[1].trim() === "workspace.dependencies";
      else if (inWsDeps && new RegExp(`^\\s*${FACADE_KEY}\\s*=`).test(lines[i])) inline = i;
    }
    const names = [...need.names].sort();
    const fresh = names.filter(([n]) => !new RegExp(`^\\s*(\\[workspace\\.dependencies\\.${n}\\]|${n}\\s*=)`, "m").test(text));
    if (!fresh.length) continue;
    const add: string[] = [];
    let at: number;
    if (tbl >= 0) {
      const indent = /^(\s*)/.exec(lines[tbl])![1];
      let end = tbl;
      for (let i = tbl + 1; i < lines.length && !/^\s*\[/.test(lines[i]) && lines[i].trim() !== ""; i++) end = i;
      at = end + 1;
      for (const [n, lib] of fresh) add.push("", `${indent}[workspace.dependencies.${n}]`, `${indent}path = "${rel(need.dir, owners.byLib.get(lib)!.dir)}"`);
    } else if (inline >= 0) {
      const indent = /^(\s*)/.exec(lines[inline])![1];
      at = inline + 1;
      for (const [n, lib] of fresh) add.push(`${indent}${n} = { path = "${rel(need.dir, owners.byLib.get(lib)!.dir)}" }`);
    } else {
      issue(wm, "workspace declares no semio-framework-value dependency to anchor new declarations");
      continue;
    }
    lines.splice(at, 0, ...add);
    plan.wsDecls += fresh.length;
    sink?.set(wm, { before: text, after: lines.join("\n") });
    if (apply) {
      if (readFileSync(wm, "utf8") !== text) {
        issue(wm, "workspace manifest changed during run, skipped");
        continue;
      }
      writeFileSync(wm, lines.join("\n"));
    }
  }
  return plan;
}
export { createHash, existsSync };
