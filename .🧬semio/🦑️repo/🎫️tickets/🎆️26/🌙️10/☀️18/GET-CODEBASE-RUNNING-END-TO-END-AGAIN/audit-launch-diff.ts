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
const load = (f: string) => JSON.parse(strip(readFileSync(f, "utf8")));
const L = load(".vscode/launch.json"), S = load(".vscode/🧩️launch.seed.jsonc");
const sObj = S.configurations.filter((c: any) => c && typeof c === "object");
const lByName = new Map<string, any[]>(); for (const c of L.configurations) { const a = lByName.get(c.name) ?? []; a.push(c); lByName.set(c.name, a); }
const cats: Record<string, number> = {}; const ex: Record<string, string> = {};
const diffKeys = (a: any, b: any) => { const ks = new Set([...Object.keys(a), ...Object.keys(b)]); return [...ks].filter((k) => JSON.stringify(a[k]) !== JSON.stringify(b[k])); };
let noLaunch = 0;
for (const s of sObj) {
  const cands = lByName.get(s.name);
  if (!cands) { noLaunch++; cats["seedNameAbsentInLaunch"] = (cats["seedNameAbsentInLaunch"] ?? 0) + 1; ex["seedNameAbsentInLaunch"] ??= s.name; continue; }
  if (cands.some((c) => JSON.stringify(c) === JSON.stringify(s))) continue;
  // closest candidate
  const best = cands.map((c) => ({ c, d: diffKeys(s, c) })).sort((x, y) => x.d.length - y.d.length)[0];
  const k = "diffKeys:" + best.d.join(",");
  cats[k] = (cats[k] ?? 0) + 1; ex[k] ??= s.name;
}
console.log("seedObjNameAbsentInLaunch", noLaunch);
for (const [k, v] of Object.entries(cats).sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log(v, k, "| e.g.", ex[k]);
// compounds and inputs subset checks
const sc = new Set(S.compounds.map((c: any) => c.name)); const lc = L.compounds.map((c: any) => c.name);
console.log("compoundNamesLaunch", JSON.stringify(lc), "inSeed", lc.map((n: string) => sc.has(n)));
console.log("seedCompoundNames", JSON.stringify([...sc]).slice(0, 1200));
const si = new Set(S.inputs.map((i: any) => i.id)); const li = new Set(L.inputs.map((i: any) => i.id));
console.log("seedInputsNotInLaunch", [...si].filter((x) => !li.has(x)).length, "launchInputsNotInSeed", [...li].filter((x) => !si.has(x)).length, JSON.stringify([...li].filter((x) => !si.has(x)).slice(0, 10)));
// projectTarget input sample
const pt = L.inputs.find((i: any) => i.id === "projectTarget.generator"); console.log("ptSample", JSON.stringify(pt).slice(0, 600));
const ptSeed = S.inputs.find((i: any) => i.id === "projectTarget.generator"); console.log("ptSeedSample", JSON.stringify(ptSeed ?? null).slice(0, 300));
