import { readFileSync, writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/", G = process.env.G!;
const paths = readFileSync(G + "/cpf-manifests.txt", "utf8").split("\n").filter(Boolean);
const out: any[] = []; let bad = 0;
for (const p of paths) {
  let t: any;
  try { t = (Bun as any).TOML.parse(readFileSync(R + p, "utf8")); } catch (e) { bad++; out.push({ path: p, error: String(e).slice(0, 80) }); continue; }
  const deps = (tbl: any, section: string, target?: string) => Object.entries(tbl ?? {}).map(([name, v]: any) => ({ name: (v && typeof v === "object" && v.package) || name, key: name, section, target, features: (v && typeof v === "object" && v.features) || [], defaultFeatures: !(v && typeof v === "object" && v["default-features"] === false), optional: !!(v && typeof v === "object" && v.optional), path: v && typeof v === "object" ? v.path : undefined, workspace: v && typeof v === "object" ? v.workspace : undefined }));
  const d: any[] = [...deps(t.dependencies, "dependencies"), ...deps(t["dev-dependencies"], "dev-dependencies"), ...deps(t["build-dependencies"], "build-dependencies")];
  for (const [tg, tv] of Object.entries(t.target ?? {}) as any) d.push(...deps(tv.dependencies, "dependencies", tg), ...deps(tv["dev-dependencies"], "dev-dependencies", tg), ...deps(tv["build-dependencies"], "build-dependencies", tg));
  const wsdeps = t.workspace?.dependencies ? deps(t.workspace.dependencies, "workspace.dependencies") : [];
  out.push({
    path: p, name: t.package?.name, role: t.package?.metadata?.semio?.role, features: t.features ?? {}, lib: t.lib?.path ?? (t.package ? "src/lib.rs" : null),
    tests: (t.test ?? []).map((x: any) => x.path).filter(Boolean), bins: (t.bin ?? []).map((x: any) => x.path).filter(Boolean), deps: d, wsdeps,
  });
}
writeFileSync(G + "/cpf-manifests.json", JSON.stringify(out));
console.log("manifests", out.length, "bad", bad);
