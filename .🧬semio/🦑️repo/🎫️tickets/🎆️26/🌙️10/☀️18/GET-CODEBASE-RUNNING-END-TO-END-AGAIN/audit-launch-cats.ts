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
const L = JSON.parse(strip(readFileSync(".vscode/launch.json", "utf8")));
const S = JSON.parse(strip(readFileSync(".vscode/🧩️launch.seed.jsonc", "utf8")));
const sObj = S.configurations.filter((c: any) => c && typeof c === "object");
const sPlace = S.configurations.filter((c: any) => typeof c === "string");
const lByName = new Map<string, any[]>();
for (const c of L.configurations) { const a = lByName.get(c.name) ?? []; a.push(c); lByName.set(c.name, a); }
const lCanon = new Set(L.configurations.map(canon));
const sCanon = new Set(sObj.map(canon));
const seedOnly = sObj.filter((c: any) => !lCanon.has(canon(c)));
const cats: Record<string, number> = {};
const bump = (k: string) => { cats[k] = (cats[k] ?? 0) + 1; };
for (const s of seedOnly) {
  const cands = lByName.get(s.name) ?? [];
  if (!cands.length) { bump("seed-only: name absent in launch"); continue; }
  const ks = new Set([...Object.keys(s), ...cands.flatMap((c) => Object.keys(c))]);
  const dk = [...ks].filter((k) => cands.every((c) => canon(c[k]) !== canon(s[k])));
  for (const k of dk) {
    if (k === "cwd" || k === "env" || k === "command") {
      const sv = canon(s[k]), lv = canon(cands[0][k]);
      if (k === "command" && sv.replace(/\\"/g, "").includes(lv.replace(/\\"/g, "")) ) bump("seed-only: command differs");
      else bump(`seed-only: ${k} differs`);
    } else bump(`seed-only: ${k} differs`);
  }
}
const lOnlyNameInSeed = L.configurations.filter((c: any) => !sCanon.has(canon(c)) && sObj.some((s: any) => s.name === c.name)).length;
const lOnlyNew = L.configurations.filter((c: any) => !sCanon.has(canon(c)) && !sObj.some((s: any) => s.name === c.name)).length;
const abs = (v: any) => (JSON.stringify(v).match(/\/Users\/ueli/g) ?? []).length;
const wf = (v: any) => (JSON.stringify(v).match(/\$\{workspaceFolder\}/g) ?? []).length;
const sortOrd = (arr: any[]) => arr.filter((c) => c.presentation && typeof c.presentation.order === "number" && !Number.isInteger(c.presentation.order * 1000)).length;
const prefixes = Object.values(S.devLaunchers ?? {}) as any[];
const devNamed = L.configurations.filter((c: any) => prefixes.some((p) => p.namePrefix && String(c.name).startsWith(p.namePrefix))).length;
const out = {
  launchConfigs: L.configurations.length,
  seedObjects: sObj.length,
  seedPlaceholders: sPlace.length,
  seedPlaceholderSamples: sPlace.slice(0, 3),
  devLaunchersKeys: Object.keys(S.devLaunchers ?? {}).length,
  projectLaunchersKeys: Object.keys(S.projectLaunchers ?? {}).length,
  seedObjectsNotCanonicallyInLaunch: seedOnly.length,
  seedOnlyBreakdown: cats,
  launchEntriesNotCanonicallyInSeed: L.configurations.filter((c: any) => !sCanon.has(canon(c))).length,
  launchOnlyNameExistsInSeed: lOnlyNameInSeed,
  launchOnlyNewName: lOnlyNew,
  launchEntriesMatchingDevLauncherPrefix: devNamed,
  absPathsInSeed: abs(sObj),
  absPathsInLaunch: abs(L.configurations),
  workspaceFolderInSeed: wf(sObj),
  workspaceFolderInLaunch: wf(L.configurations),
  shipInLaunch: (readFileSync(".vscode/launch.json", "utf8").match(/SEMIO_BUILD_MODE": "ship"/g) ?? []).length,
  shipInSeed: (readFileSync(".vscode/🧩️launch.seed.jsonc", "utf8").match(/SEMIO_BUILD_MODE": "ship"/g) ?? []).length,
  workspaceDevInLaunch: (readFileSync(".vscode/launch.json", "utf8").match(/workspace:dev -- /g) ?? []).length,
  nonIntegerOrderInLaunch: sortOrd(L.configurations),
  nonIntegerOrderInSeed: sortOrd(sObj),
  compoundsLaunch: L.compounds.length,
  compoundsSeed: S.compounds.length,
  compoundNamesSeedNotInLaunch: S.compounds.map((c: any) => c.name).filter((n: string) => !L.compounds.some((x: any) => x.name === n)).length,
  inputsLaunch: L.inputs.length,
  inputsSeed: S.inputs.length,
  inputsSeedNotInLaunch: S.inputs.map((i: any) => i.id).filter((id: string) => !L.inputs.some((x: any) => x.id === id)),
  inputsLaunchNotInSeed: L.inputs.length - L.inputs.filter((x: any) => S.inputs.some((i: any) => i.id === x.id)).length,
};
console.log(JSON.stringify(out, null, 1));
