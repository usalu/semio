import { readFileSync } from "node:fs";
const strip = (s: string) => {
  let out = "", i = 0, inStr = false;
  while (i < s.length) {
    const c = s[i];
    if (inStr) { out += c; if (c === "\\") { out += s[i + 1]; i += 2; continue; } if (c === '"') inStr = false; i++; continue; }
    if (c === '"') { inStr = true; out += c; i++; continue; }
    if (c === "/" && s[i + 1] === "/") { while (i < s.length && s[i] !== "\n") i++; continue; }
    if (c === "/" && s[i + 1] === "*") { i += 2; while (i < s.length && !(s[i] === "*" && s[i + 1] === "/")) i++; i += 2; continue; }
    out += c; i++;
  }
  return out.replace(/,(\s*[}\]])/g, "$1");
};
const canon = (v: any): string => v === undefined ? "UNDEF" : Array.isArray(v) ? "[" + v.map(canon).join(",") + "]" : v && typeof v === "object" ? "{" + Object.keys(v).sort().map((k) => JSON.stringify(k) + ":" + canon(v[k])).join(",") + "}" : JSON.stringify(v);
const load = (f: string) => JSON.parse(strip(readFileSync(f, "utf8")));
const L = load(".vscode/launch.json"), S = load(".vscode/🧩️launch.seed.jsonc");
const sObj = S.configurations.filter((c: any) => c && typeof c === "object");
const lSet = new Map<string, number>(); for (const c of L.configurations) lSet.set(canon(c), (lSet.get(canon(c)) ?? 0) + 1);
const sSet = new Map<string, number>(); for (const c of sObj) sSet.set(canon(c), (sSet.get(canon(c)) ?? 0) + 1);
const seedOnly = sObj.filter((c: any) => !lSet.has(canon(c)));
const launchOnlyCanon = L.configurations.filter((c: any) => !sSet.has(canon(c)));
console.log("seedObjectsNotCanonicallyInLaunch", seedOnly.length, "launchEntriesNotCanonicallyInSeed", launchOnlyCanon.length);
const byName = new Map<string, any[]>(); for (const c of L.configurations) { const a = byName.get(c.name) ?? []; a.push(c); byName.set(c.name, a); }
const shown: Record<string, number> = {};
for (const s of seedOnly.slice(0, 400)) {
  const cands = byName.get(s.name) ?? [];
  const ks = new Set([...Object.keys(s), ...cands.flatMap((c) => Object.keys(c))]);
  const dk = cands.length ? [...ks].filter((k) => cands.every((c) => canon(c[k]) !== canon(s[k]))) : ["ABSENT"];
  const sig = dk.join(",");
  if ((shown[sig] ?? 0) < 2) { shown[sig] = (shown[sig] ?? 0) + 1; console.log("SEED", JSON.stringify(s.name), "diff:", sig); for (const k of dk.slice(0, 2)) { console.log("  seed.", k, canon(s[k]).slice(0, 260)); if (cands[0]) console.log("  launch.", k, canon(cands[0][k]).slice(0, 260)); } }
}
const lo: Record<string, number> = {}; for (const c of launchOnlyCanon) lo[c.name?.slice(0, 40)] = (lo[c.name?.slice(0, 40)] ?? 0) + 1;
console.log("launchOnlyCanonicalNameSamples", JSON.stringify(Object.keys(lo).slice(0, 15)));
const uniqLaunchOnly = launchOnlyCanon.map((c: any) => c.name);
console.log("launchOnlyNamesPresentInSeed", uniqLaunchOnly.filter((n: string) => sObj.some((s: any) => s.name === n)).length);
console.log("launchOnlyDistinctNames", new Set(uniqLaunchOnly).size);
