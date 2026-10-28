import { posix } from "node:path";
const root = process.cwd();
const m = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts`);
const disk = m.diskZeroTouchTreeV1(root);
const roster = m.readZeroTouchRosterV1(disk.read(m.ZERO_TOUCH_CLOSURES_FILE)!);
const [id, target] = process.argv.slice(2);
const closure = roster.closures.find((c: any) => c.id === id)!;
const EXT = ["", ".ts", ".tsx", ".mts", ".js", ".mjs"];
const resolveRel = (from: string, s: string) => { const b = posix.normalize(posix.join(posix.dirname(from), s.split("?")[0]!)); for (const e of EXT) if (/\.(?:[cm]?[jt]s|tsx)$/.test(b + e) && disk.read(b + e) !== undefined) return b + e; };
const parent = new Map<string, [string, string]>(), pending = [closure.entry], seen = new Set<string>();
while (pending.length) {
  const f = pending.pop()!; if (seen.has(f)) continue; seen.add(f);
  for (const { specifier, kind } of m.scanModuleSpecifiersV1(f, disk.read(f) ?? "")) {
    if (closure.mode === "static" && kind !== "static") continue;
    if (!specifier.startsWith(".")) continue;
    const t = resolveRel(f, specifier); if (!t || seen.has(t)) continue;
    if (!parent.has(t)) parent.set(t, [f, kind]);
    pending.push(t);
  }
}
let cur = target; const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
while (cur) { const p = parent.get(cur); console.log(cur.replace(L, "L/"), p ? `<- ${p[1]}` : ""); cur = p?.[0] as string; }
