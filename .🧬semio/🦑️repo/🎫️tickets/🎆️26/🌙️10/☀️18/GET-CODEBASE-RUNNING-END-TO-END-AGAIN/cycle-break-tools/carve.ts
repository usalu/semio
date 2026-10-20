/** ✂️ Module-root package carving: a source module becomes its own workspace package (manifest, exports, importer re-pointing, declarations). */
import { existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { listAll } from "../exports-stage-tools/lib.ts";
import { extensionLines, reexportsOf } from "../exports-stage-tools/glue-ops.ts";
import { originsOf } from "../exports-stage-tools/model.ts";
import { ROOT, bindingOf, cliClosure, declares, model, posix, workspaceOps } from "./lib.ts";
import { Ownership, type CarveRef } from "./own.ts";
import { isTest } from "./sim.ts";
import type { Plan } from "./plan.ts";

export type CarveSpec = {
  wave: string; name: string; dir: string; description: string;
  semio?: Record<string, unknown>;
  anchorDir?: string;
  extraThirdParty?: string[];
  dissolvePassThrough?: boolean;
  ignorePrefixes?: string[];
  moveFrom?: { pkg: string; keys?: string[]; skip?: string[] }[];
  extraExports?: Record<string, string>;
  deleteMovedGlue?: boolean;
  skipImporters?: (file: string) => boolean;
};
const lock = (() => { try { return Bun.JSONC.parse(readFileSync(resolve(ROOT, "bun.lock"), "utf8")) as { packages: Record<string, string[]> }; } catch { return { packages: {} }; } })();
const ascii = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "");
const stripExt = (s: string): string => s.replace(/\.(ts|tsx|mts|mjs|cjs|js)$/u, "");
const INDEX = /^🟦️\.(ts|tsx|mts)$/u;

export function keyFor(dir: string, file: string): string {
  const r = file.slice(dir.length + 1).replace(/(^|\/)🧪️tests(?=\/|$)/gu, "$1🧪️testing");
  const parts = r.split("/"), last = parts.pop()!;
  const json = last.endsWith(".json");
  if (json) {
    const stem = ascii(last.replace(/\.json$/u, ""));
    const dirs = parts.map(ascii).filter(Boolean);
    if (stem) return "./" + [...dirs, stem + ".json"].join("/");
    const lastDir = dirs.pop() ?? ascii(dir.split("/").pop()!);
    return "./" + [...dirs, lastDir + ".json"].join("/");
  }
  const tooling = /\.(mjs|cjs)$/u.exec(last)?.[1];
  const base = ascii(stripExt(last));
  const dirs = parts.map(ascii).filter(Boolean);
  const segs = INDEX.test(last) || (!base && !!tooling) ? [...dirs, ...(tooling && !base ? [tooling === "mjs" ? "tooling" : "commonjs"] : [])] : [...dirs, base];
  const key = segs.filter(Boolean).join("/");
  return key ? "./" + key : ".";
}

const coverIndex = (() => {
  let idx: Map<string, { pkg: string; key: string }[]> | null = null;
  return (): Map<string, { pkg: string; key: string }[]> => {
    if (idx) return idx;
    idx = new Map();
    for (const p of model.all) {
      if (!p.name) continue;
      for (const [key, v] of Object.entries(p.exports)) {
        if (typeof v !== "string" || !v.startsWith("./")) continue;
        const file = posix(resolve("/", p.dir, v)).slice(1);
        const add = (t: string): void => { (idx!.get(t) ?? idx!.set(t, []).get(t)!).push({ pkg: p.name, key }); };
        add(file);
        if (/\.(ts|tsx|mts)$/u.test(file) && existsSync(resolve(ROOT, file))) {
          let rx: ReturnType<typeof reexportsOf> = [];
          try { rx = reexportsOf(file); } catch { rx = []; }
          const ts = [...new Set(rx.map(r => r.target).filter((x): x is string => !!x))];
          if (ts.length === 1) add(ts[0]!);
        }
      }
    }
    return idx;
  };
})();

type Alloc = { name: string; dir: string; keys: Map<string, string>; byFile: Map<string, string> };

export class Carver {
  specs: CarveSpec[] = [];
  own!: Ownership;
  private allocs = new Map<string, Alloc>();
  private files = listAll();
  private edgesAll = scan().edges;
  private bareAll = scan().bare;
  private thirdAll = scan().third;
  private fw = model;
  private closure = cliClosure(scan().edges);
  private indexWants = new Map<string, { exists: boolean; wave: string; why: string; wanted: Map<string, { target: string; type: boolean }> }>();
  private ignored = (f: string): boolean => this.specs.some(s => s.ignorePrefixes?.some(p => f === p || f.startsWith(p + "/")));
  constructor(private plan: Plan, specs: CarveSpec[], extraOwners: CarveRef[] = []) {
    this.specs = specs;
    this.own = new Ownership([...specs.map(s => ({ name: s.name, dir: s.dir })), ...extraOwners]);
    for (const s of specs) this.allocs.set(s.name, { name: s.name, dir: s.dir, keys: new Map(), byFile: new Map() });
  }
  private alloc(name: string): Alloc { return this.allocs.get(name)!; }
  private take(a: Alloc, file: string, preferred?: string): string {
    const have = a.byFile.get(file);
    if (have) return have;
    let key = preferred ?? keyFor(a.dir, file);
    let n = 2;
    while (a.keys.has(key) && a.keys.get(key) !== file) key = key.replace(/(\.json)?$/u, m => `-${n++}${m}`);
    a.keys.set(key, file); a.byFile.set(file, key);
    return key;
  }
  private specOf(name: string, key: string): string { return name + (key === "." ? "" : key.slice(1)); }
  ownerName(file: string): string { return this.own.pk(file); }
  private dirUnder(file: string, dir: string): boolean { return file.startsWith(dir + "/"); }

  run(): void {
    const moved = new Map<string, { s: CarveSpec; pkg: string; key: string; value: string; target: string; glue?: string; passThrough?: { pkg: string; key: string } }>();
    const deletedGlue = new Set<string>();
    const commonDir = (ts: string[]): string => { const parts = ts.map(t => t.split("/").slice(0, -1)); let n = Math.min(...parts.map(p => p.length)); const first = parts[0]!; let i = 0; while (i < n && parts.every(p => p[i] === first[i])) i++; return first.slice(0, i).join("/"); };
    for (const s of this.specs) {
      const a = this.alloc(s.name);
      for (const mf of s.moveFrom ?? []) {
        const P0 = this.fw.byName.get(mf.pkg);
        if (!P0) { this.plan.flag(s.wave, "blocker", `unknown package ${mf.pkg}`); continue; }
        for (const [key, value] of Object.entries(P0.exports)) {
          if (mf.keys && !mf.keys.includes(key)) continue;
          if (mf.skip?.includes(key)) continue;
          if (typeof value !== "string" || !value.startsWith("./")) continue;
          if (moved.has(P0.name + "\0" + key)) continue;
          const v = posix(resolve("/", P0.dir, value)).slice(1);
          let target: string | null = null, glue: string | undefined;
          if (this.own.pk(v) === s.name) target = v;
          else if (existsSync(resolve(ROOT, v)) && /\.(ts|tsx|mts)$/u.test(v) && v.startsWith(P0.dir + "/")) {
            let rx: ReturnType<typeof reexportsOf> = [];
            try { rx = reexportsOf(v); } catch { rx = []; }
            const ts = [...new Set(rx.map(r => r.target).filter((x): x is string => !!x))];
            if (!ts.length) continue;
            const owners = new Set(ts.map(t => this.own.pk(t)));
            if (owners.size === 1 && owners.has(s.name)) {
              glue = v;
              if (ts.length === 1) target = ts[0]!;
              else {
                const dirs = [commonDir(ts), s.dir];
                const idx = dirs.flatMap(d => [`${d}/🟦️.ts`, `${d}/🟦️.tsx`]).find(p => existsSync(resolve(ROOT, p)) && this.own.pk(p) === s.name);
                const idxPath = idx ?? `${dirs[0]}/🟦️.ts`;
                const wanted = new Map<string, { target: string; type: boolean }>();
                for (const r of rx) {
                  if (!r.target) continue;
                  const o = originsOf(r.target);
                  const names = r.star ? [...(o?.keys() ?? [])].filter(k => k !== "default" && !k.startsWith("*bare:")) : r.names.map(n => n.split(" as ").pop()!);
                  for (const n of names) { const origin = o?.get(n); wanted.set(n, { target: r.target, type: r.typeOnly || !!origin?.typeKind || !!origin?.via }); }
                }
                const acc = this.indexWants.get(idxPath) ?? this.indexWants.set(idxPath, { exists: !!idx, wave: s.wave, why: `${s.name} public index (names of ${P0.name}${key})`, wanted: new Map() }).get(idxPath)!;
                for (const [n, w] of wanted) acc.wanted.set(n, w);
                target = idxPath;
              }
            } else if (owners.size === 1 && s.dissolvePassThrough !== false) {
              const Q = [...owners][0]!;
              if (Q === P0.name || Q === "-" || !ts.every(t => this.dirUnder(t, s.dir) )) continue;
              const cover = ts.length === 1 ? (coverIndex().get(ts[0]!) ?? []).filter(c => c.pkg === Q && !(c.pkg === P0.name)) : [];
              if (!cover.length) { this.plan.flag(s.wave, "blocker", `pass-through ${P0.name}${key} -> ${Q} has no covering key in ${Q}: ${ts[0]}`); continue; }
              moved.set(P0.name + "\0" + key, { s, pkg: P0.name, key, value, target: ts[0]!, glue: v, passThrough: { pkg: Q, key: cover[0]!.key } });
              this.plan.manifest({ op: "remove", packageJson: P0.packageJson, package: P0.name, field: "exports", key, value, wave: s.wave, why: `pass-through to ${Q}${cover[0]!.key === "." ? "" : cover[0]!.key.slice(1)} is dissolved (importers use the owner's key)` });
              deletedGlue.add(v);
              continue;
            } else continue;
          }
          if (!target) continue;
          const nk = this.take(a, target);
          moved.set(P0.name + "\0" + key, { s, pkg: P0.name, key, value, target, glue });
          this.plan.manifest({ op: "remove", packageJson: P0.packageJson, package: P0.name, field: "exports", key, value, wave: s.wave, why: `key moves to ${s.name}${nk === "." ? "" : nk.slice(1)}` });
          if (glue && s.deleteMovedGlue !== false) deletedGlue.add(glue);
        }
      }
      for (const [k, rel] of Object.entries(s.extraExports ?? {})) { a.keys.set(k, `${s.dir}/${rel.slice(2)}`); a.byFile.set(`${s.dir}/${rel.slice(2)}`, k); }
    }
    // glue that stays exported by another key must not be deleted
    for (const g of [...deletedGlue]) {
      const stillKeyed = [...this.fw.byName.values()].some(p => Object.entries(p.exports).some(([k, v]) => typeof v === "string" && posix(resolve("/", p.dir, v)).slice(1) === g && !moved.has(p.name + "\0" + k)));
      if (stillKeyed) { deletedGlue.delete(g); this.plan.flag("carve", "note", `glue ${g} stays: another key exports it`); }
    }
    for (const [idxPath, acc] of this.indexWants) {
      const ext = extensionLines(idxPath, acc.wanted);
      if (!ext.lines.length) continue;
      if (acc.exists) this.plan.edit({ file: idxPath, from: "", to: ext.lines.join("\n") + "\n", mode: "append", wave: acc.wave, why: acc.why });
      else this.plan.newFile({ path: idxPath, content: `/** 🧩️ Public index: ${acc.why}. */\n` + ext.lines.join("\n") + "\n", wave: acc.wave, why: acc.why });
    }
    const movedSpec = new Map<string, string>();
    for (const [k, m] of moved) { const [pkg, key] = k.split("\0") as [string, string]; movedSpec.set(pkg + (key === "." ? "" : key.slice(1)), m.passThrough ? this.specOf(m.passThrough.pkg, m.passThrough.key) : this.specOf(m.s.name, this.alloc(m.s.name).byFile.get(m.target)!)); }

    const usersOf = new Map<string, Set<string>>();
    const need = (carve: string, file: string, importer: string): void => { const k = carve; (usersOf.get(k) ?? usersOf.set(k, new Set()).get(k)!).add(importer); void file; };
    const rewriteBare = (importer: string, from: string, to: string, why: string, wave: string, owner: string): void => {
      this.plan.edit({ file: importer, from, to, mode: "specifier", wave, why });
      void owner;
    };
    const deps = new Map<string, Map<string, "dependencies" | "devDependencies">>();
    const declareDep = (importer: string, ownerPkg: string, toName: string, wave: string): void => {
      if (ownerPkg === toName) return;
      const field: "dependencies" | "devDependencies" = isTest(importer) ? "devDependencies" : "dependencies";
      const m = deps.get(ownerPkg) ?? deps.set(ownerPkg, new Map()).get(ownerPkg)!;
      if (m.get(toName) !== "dependencies") m.set(toName, field);
      const b = bindingOf(importer);
      if (b.packageJson !== "package.json" && b.name !== toName && !declares(b, toName)) this.plan.manifest({ op: "add", packageJson: b.packageJson, package: b.name, field, key: toName, value: "workspace:*", wave, why: `binding manifest of ${importer} imports ${toName}` });
    };

    // relative edges that target a glue file which is deleted: re-point to the owner's key (or, inside the zero-touch closure, to the real module file)
    const glueDest = new Map<string, { pkg: string; key: string; target: string; wave: string }>();
    for (const m of moved.values()) if (m.glue && deletedGlue.has(m.glue) && !glueDest.has(m.glue)) glueDest.set(m.glue, m.passThrough ? { pkg: m.passThrough.pkg, key: m.passThrough.key, target: m.target, wave: m.s.wave } : { pkg: m.s.name, key: this.alloc(m.s.name).byFile.get(m.target)!, target: m.target, wave: m.s.wave });
    for (const e of this.edgesAll) {
      if (deletedGlue.has(e.file) || e.spec.includes("?") || this.ignored(e.file) || this.ignored(e.target)) continue;
      const gd = glueDest.get(e.target);
      if (!gd) continue;
      const a = this.own.owner(e.file);
      if (!a) continue;
      if (a.name === gd.pkg || this.closure.has(e.file)) {
        let r = posix(relative(dirname(resolve(ROOT, e.file)), resolve(ROOT, gd.target)));
        if (!r.startsWith(".")) r = "./" + r;
        this.plan.edit({ file: e.file, from: e.spec, to: r, mode: "specifier", wave: gd.wave, why: `the glue ${e.target} is gone: the import points at the module file${a.name === gd.pkg ? "" : " (zero-touch closure stays relative)"}` });
      } else rewriteBare(e.file, e.spec, this.specOf(gd.pkg, gd.key), `the glue ${e.target} is gone: use ${gd.pkg}`, gd.wave, a.name);
      declareDep(e.file, a.name, gd.pkg, gd.wave);
    }
    // relative edges (cross-owner after the carve)
    for (const e of this.edgesAll) {
      if (deletedGlue.has(e.file) || e.spec.includes("?") || this.ignored(e.file) || this.ignored(e.target)) continue;
      const a = this.own.owner(e.file), b = this.own.owner(e.target);
      if (!a || !b || a.name === b.name) continue;
      const spec = this.specs.find(s => s.name === b.name);
      if (!spec) continue;
      if (spec.skipImporters?.(e.file)) continue;
      const al = this.alloc(spec.name);
      const key = this.take(al, e.target);
      if (this.closure.has(e.file)) this.plan.flag(spec.wave, "note", `zero-touch CLI closure file keeps its relative import: ${e.file} -> ${e.target}`);
      else rewriteBare(e.file, e.spec, this.specOf(spec.name, key), `relative import crosses into carved package ${spec.name}`, spec.wave, a.name);
      declareDep(e.file, a.name, spec.name, spec.wave);
      need(spec.name, e.target, e.file);
    }
    // bare imports of moved keys
    for (const b of this.bareAll) {
      if (deletedGlue.has(b.file) || this.ignored(b.file)) continue;
      const to = movedSpec.get(b.pkg + (b.sub === "." ? "" : b.sub.slice(1)));
      if (!to) continue;
      const m = moved.get(b.pkg + "\0" + b.sub);
      if (!m) continue;
      const a = this.own.owner(b.file);
      const destPkg = m.passThrough?.pkg ?? m.s.name;
      if (a?.name === destPkg) {
        let r = posix(relative(dirname(resolve(ROOT, b.file)), resolve(ROOT, m.target)));
        if (!r.startsWith(".")) r = "./" + r;
        this.plan.edit({ file: b.file, from: b.spec.split("?")[0]!, to: r, mode: "specifier", wave: m.s.wave, why: `import of ${destPkg} from inside itself becomes relative` });
        continue;
      }
      rewriteBare(b.file, b.spec.split("?")[0]!, to, `key ${b.pkg}${b.sub === "." ? "" : b.sub.slice(1)} moved to ${destPkg}`, m.s.wave, a?.name ?? "-");
      if (a) declareDep(b.file, a.name, destPkg, m.s.wave);
    }
    // blockers: carved files that still import the former owner
    for (const s of this.specs) {
      const inside = this.files.filter(f => f.startsWith(s.dir + "/") && this.own.pk(f) === s.name && /\.(ts|tsx|mts|js|mjs|cjs)$/u.test(f) && !isTest(f));
      const insideSet = new Set(inside);
      for (const e of this.edgesAll) {
        if (!insideSet.has(e.file) || e.spec.includes("?") || this.ignored(e.file)) continue;
        const b = this.own.owner(e.target);
        if (!b || b.name === s.name) continue;
        const other = this.specs.find(x => x.name === b.name);
        if (other) {
          const al = this.alloc(other.name); const key = this.take(al, e.target);
          rewriteBare(e.file, e.spec, this.specOf(other.name, key), `${s.name} imports ${other.name} (carved)`, s.wave, s.name);
          declareDep(e.file, s.name, other.name, s.wave);
        } else {
          const cover = (coverIndex().get(e.target) ?? []).find(c => c.pkg === b.name) ?? (coverIndex().get(e.target) ?? [])[0];
          if (cover && !this.closure.has(e.file)) {
            rewriteBare(e.file, e.spec, cover.pkg + (cover.key === "." ? "" : cover.key.slice(1)), `${s.name} imports ${cover.pkg} through its existing key`, s.wave, s.name);
            declareDep(e.file, s.name, cover.pkg, s.wave);
          } else if (this.closure.has(e.file)) { this.plan.flag(s.wave, "note", `closure file keeps relative import ${e.file} -> ${e.target}`); declareDep(e.file, s.name, b.name, s.wave); }
          else if (!cover) this.plan.flag(s.wave, "blocker", `${s.name} imports ${b.name} without an existing key: ${e.file} -> ${e.target}`);
        }
      }
    }
    // new manifests
    const thirdOf = (s: CarveSpec): Map<string, "dependencies" | "devDependencies"> => {
      const out = new Map<string, "dependencies" | "devDependencies">();
      for (const t of this.thirdAll) {
        if (!t.file.startsWith(s.dir + "/") || this.own.pk(t.file) !== s.name || this.ignored(t.file)) continue;
        const f: "dependencies" | "devDependencies" = isTest(t.file) ? "devDependencies" : "dependencies";
        if (out.get(t.pkg) !== "dependencies") out.set(t.pkg, f);
      }
      return out;
    };
    const versionOf = (name: string): string | null => {
      const counts = new Map<string, number>();
      for (const p of this.fw.all) for (const f of ["dependencies", "devDependencies"] as const) { const v = (p.json[f] ?? {})[name]; if (typeof v === "string" && !v.startsWith("workspace:")) counts.set(v, (counts.get(v) ?? 0) + 1); }
      const best = [...counts].sort((a, b) => b[1] - a[1])[0]?.[0];
      if (best) return best;
      const e = lock.packages[name]; return e?.[0] ? e[0].slice(name.length + 1) : null;
    };
    for (const s of this.specs) {
      const a = this.alloc(s.name);
      const exportsObj: Record<string, string> = {};
      for (const [k, f] of [...a.keys].sort((x, y) => (x[0] < y[0] ? -1 : 1))) exportsObj[k] = "./" + f.slice(s.dir.length + 1);
      const wsDeps = new Map<string, "dependencies" | "devDependencies">();
      for (const [pkg, m] of deps) if (pkg === s.name) for (const [n, f] of m) wsDeps.set(n, f);
      const third = thirdOf(s);
      for (const n of s.extraThirdParty ?? []) if (!third.has(n)) third.set(n, "dependencies");
      const dependencies: Record<string, string> = {}, devDependencies: Record<string, string> = {};
      for (const [n, f] of [...wsDeps].sort()) (f === "dependencies" ? dependencies : devDependencies)[n] = "workspace:*";
      for (const [n, f] of [...third].sort()) { if (this.fw.byName.has(n) || n === s.name) continue; const v = versionOf(n); if (!v) { this.plan.flag(s.wave, "note", `no declared version for third-party ${n} used by ${s.name}`); continue; } (f === "dependencies" ? dependencies : devDependencies)[n] = v; }
      const manifest: Record<string, unknown> = {
        $schema: "https://json.schemastore.org/package.json", name: s.name, version: "0.1.0", description: s.description, type: "module", private: true,
        ...(s.semio ? { semio: s.semio } : {}),
        exports: exportsObj,
        ...(Object.keys(dependencies).length ? { dependencies } : {}),
        license: "LGPL-3.0-or-later", bundleKind: "library",
        ...(Object.keys(devDependencies).length ? { devDependencies } : {}),
      };
      this.plan.newFile({ path: `${s.dir}/package.json`, content: JSON.stringify(manifest, null, 2) + "\n", wave: s.wave, why: `module-root package ${s.name}` });
    }
    for (const [pkg, m] of deps) {
      if (this.specs.some(x => x.name === pkg)) continue;
      const P = this.fw.byName.get(pkg);
      if (!P) continue;
      for (const [n, f] of m) if (!declares(P, n)) this.plan.manifest({ op: "add", packageJson: P.packageJson, package: P.name, field: f, key: n, value: "workspace:*", wave: "carve", why: `${P.name} imports ${n}` });
    }
    for (const s of this.specs) {
      const anchor = s.anchorDir ?? this.fw.byName.get(s.moveFrom?.[0]?.pkg ?? "")?.dir;
      if (!anchor) { this.plan.flag(s.wave, "note", `no workspace anchor for ${s.name}`); continue; }
      for (const o of workspaceOps(s.dir, anchor)) this.plan.manifest({ op: "add", packageJson: o.packageJson, package: o.package, field: "workspaces", key: o.key, value: "", wave: s.wave, why: `workspace member ${s.name}` });
    }
    for (const g of deletedGlue) this.plan.del(g, "carve", "glue of a moved key is dead after the carve");
    void relative; void dirname;
  }
  keysOf(name: string): Map<string, string> { return this.alloc(name).keys; }
}
