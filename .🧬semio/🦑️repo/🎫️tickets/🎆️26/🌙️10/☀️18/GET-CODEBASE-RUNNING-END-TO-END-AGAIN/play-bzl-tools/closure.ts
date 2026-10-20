// 🧮 Computes the playground lane boot closures (runtimeComponentClosure grammar) from tracked Cargo manifests; read-only.
import { readFileSync, existsSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio", OUT = join(import.meta.dir, "..", "🗑️generated", "play-bzl-run");
mkdirSync(OUT, { recursive: true });
const files = new TextDecoder().decode(Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--", ":(glob)**/Cargo.toml"], { cwd: ROOT }).stdout).split("\0").filter(f => f && !f.startsWith(".🧬semio") && existsSync(join(ROOT, f)));
type C = { pluginId: string; crate: string; dependsOn: string[]; consumes: string[]; contributes: string[]; extends?: string; host?: unknown; kind: string; dir?: string; playground: any[]; pkg: string };
const comps = new Map<string, C>();
for (const f of files) {
  let m: any; try { m = Bun.TOML.parse(readFileSync(join(ROOT, f), "utf8")); } catch { continue; }
  const md = m.package?.metadata, s = md?.semio;
  if (!md?.component?.package || !["plugin", "extension"].includes(s?.["component-kind"])) continue;
  const id = md.component.package.slice(6);
  comps.set(id, { pluginId: id, crate: dirname(f), pkg: m.package.name, dependsOn: s["depends-on"] ?? [], consumes: s.consumes ?? [], contributes: s.contributes ?? [], extends: s.extends, host: s.host, kind: s["component-kind"], playground: s.playground ?? [] });
}
const contributors = new Map<string, string[]>();
for (const c of comps.values()) for (const t of c.contributes) contributors.set(t, [...(contributors.get(t) ?? []), c.pluginId]);
function closure(root: string, appScoped: boolean) {
  const sel = new Set<string>(), pending: any[] = [{ id: root, shallow: false, appScoped }];
  while (pending.length) {
    const { id, shallow, appScoped } = pending.pop(); if (sel.has(id)) continue;
    const c = comps.get(id); if (!c) { sel.add("?" + id); continue; }
    sel.add(id); if (shallow) continue;
    for (const d of c.dependsOn) pending.push({ id: d, shallow: c.extends === d, appScoped: false });
    for (const t of c.consumes) for (const k of contributors.get(t) ?? []) pending.push({ id: k, shallow: false, appScoped: false });
    if (c.host && !appScoped) for (const h of comps.keys()) pending.push({ id: h, shallow: false, appScoped: false });
  }
  return [...sel].sort();
}
const lanes: Record<string, any> = {}, union = new Set<string>(), fullUnion = new Set<string>();
for (const c of comps.values()) for (const p of c.playground) {
  const boot = closure(c.pluginId, true), full = closure(c.pluginId, p.app !== undefined);
  lanes[p.variant] = { plugin: c.pluginId, crate: c.crate, boot: boot.length, full: full.length, bootIds: boot, app: p.app ?? null, engines: p.engines ?? [], nativeHost: !!p.nativeHost };
  boot.forEach(x => union.add(x)); full.forEach(x => fullUnion.add(x));
}
const usage = new Map<string, number>(); for (const l of Object.values(lanes)) for (const id of l.bootIds) usage.set(id, (usage.get(id) ?? 0) + 1);
const summary = { components: comps.size, lanes: Object.keys(lanes).length, bootUnion: union.size, fullUnion: fullUnion.size, sumBoot: Object.values(lanes).reduce((a: number, l: any) => a + l.boot, 0), maxBoot: Math.max(...Object.values<any>(lanes).map(l => l.boot)), minBoot: Math.min(...Object.values<any>(lanes).map(l => l.boot)), usageTop: [...usage].sort((a, b) => b[1] - a[1]).slice(0, 8), crates: [...new Set([...union].map(i => comps.get(i.replace("?", ""))?.crate))].length };
writeFileSync(join(OUT, "closure.json"), JSON.stringify({ summary, lanes, comps: Object.fromEntries([...comps].map(([k, v]) => [k, { crate: v.crate, pkg: v.pkg, kind: v.kind }])) }, null, 1));
console.log(JSON.stringify(summary));
const play = ["demonstrator","imperative","stdio-png","lowpoly","remodel","draw","raster","layout","shooting","block2d","wfc2d","fem2d","energy","architect","din4108","note","writer","forms","mathematical","reasoning-wires","dag","trinity-jack","stdio-mp4","stdio-step","bim","stdio-ifc-2x3","stdio-gltf","stdio-pdf-1-4-a","stdio-docx","stdio-semio-brep","stdio-binary","animate","sequence","vcs","home"];
const pu = new Set<string>(); for (const v of play) { const l = lanes[v]; if (!l) { console.log("missing lane", v); continue; } l.bootIds.forEach((x: string) => pu.add(x)); }
console.log("play lanes", play.length, "play boot union", pu.size, "play sum", play.reduce((a, v) => a + (lanes[v]?.boot ?? 0), 0));
