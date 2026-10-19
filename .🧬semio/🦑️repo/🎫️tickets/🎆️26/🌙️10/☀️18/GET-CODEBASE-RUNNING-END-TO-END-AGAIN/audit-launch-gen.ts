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
const sPlace = S.configurations.filter((c: any) => typeof c === "string");
console.log("seedPlaceholders", sPlace.length, JSON.stringify(sPlace.slice(0, 12)));
console.log("seedPlaceholderSample", sPlace.slice(-6));
const sObj = S.configurations.filter((c: any) => c && typeof c === "object");
const lNames = new Map<string, number>(); for (const c of L.configurations) lNames.set(c.name, (lNames.get(c.name) ?? 0) + 1);
const sNames = new Map<string, number>(); for (const c of sObj) sNames.set(c.name, (sNames.get(c.name) ?? 0) + 1);
let missingInLaunch = 0; const missSample: string[] = [];
for (const [n, k] of sNames) if ((lNames.get(n) ?? 0) < k) { missingInLaunch++; if (missSample.length < 8) missSample.push(n); }
console.log("seedObjects", sObj.length, "distinctSeedNames", sNames.size, "seedNamesMissingOrFewerInLaunch", missingInLaunch, JSON.stringify(missSample));
// deep-equal of seed objects vs launch entries with same name (multiset by JSON)
const key = (c: any) => JSON.stringify(c);
const lByName = new Map<string, string[]>(); for (const c of L.configurations) { const a = lByName.get(c.name) ?? []; a.push(key(c)); lByName.set(c.name, a); }
let differ = 0; const diffSample: string[] = [];
for (const c of sObj) { const a = lByName.get(c.name) ?? []; if (!a.includes(key(c))) { differ++; if (diffSample.length < 6) diffSample.push(c.name); } }
console.log("seedObjectsNotByteEqualInLaunch", differ, JSON.stringify(diffSample));
// launch entries not in seed (generated)
const sKeys = new Set(sObj.map(key));
const gen = L.configurations.filter((c: any) => !sKeys.has(key(c)));
console.log("launchEntriesNotIdenticalToSeedObject", gen.length);
const genNames = gen.map((c: any) => c.name);
console.log("genNameSample", JSON.stringify(genNames.slice(0, 10)));
const prefixes = Object.entries(S.devLaunchers ?? {}).map(([k, v]: any) => [k, v.namePrefix, v.order, v.wgpuOrder]);
console.log("devLaunchers", prefixes.length, JSON.stringify(prefixes.slice(0, 6)));
const byPrefix: Record<string, number> = {};
for (const n of genNames) { const hit = prefixes.find(([, p]) => p && n.startsWith(p)); const k = hit ? hit[0] : "NOPREFIX"; byPrefix[k] = (byPrefix[k] ?? 0) + 1; }
console.log("genByDevLauncher", JSON.stringify(byPrefix));
const noPrefix = genNames.filter((n: string) => !prefixes.some(([, p]) => p && n.startsWith(p)));
console.log("genNoPrefixCount", noPrefix.length, JSON.stringify([...new Set(noPrefix)].slice(0, 20)));
console.log("projectLaunchers", JSON.stringify(S.projectLaunchers));
console.log("compoundsLaunch", L.compounds.length, "seed", S.compounds.length, "equal", JSON.stringify(L.compounds) === JSON.stringify(S.compounds));
console.log("inputsLaunch", L.inputs.length, "seed", S.inputs.length, "equal", JSON.stringify(L.inputs) === JSON.stringify(S.inputs));
