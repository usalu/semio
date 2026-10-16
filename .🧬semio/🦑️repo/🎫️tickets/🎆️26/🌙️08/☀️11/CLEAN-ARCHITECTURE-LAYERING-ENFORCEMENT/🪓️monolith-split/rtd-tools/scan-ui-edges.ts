// 📋️ Lists every renderer manifest edge on the semio-framework-ui aggregate (kind, alias, features) as TSV.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {readFileSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const list = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
for (const m of list) {
  const t = Bun.TOML.parse(readFileSync(`${ROOT}/${m}`, "utf8")) as any;
  const name = t.package?.name ?? "?";
  const scan = (tbl: any, kind: string) => {
    for (const [k, v] of Object.entries(tbl ?? {}) as [string, any][]) {
      const pkg = (typeof v === "object" ? v.package : undefined) ?? k;
      if (pkg === "semio-framework-ui") console.log([name, kind, k, JSON.stringify(typeof v === "object" ? {features: v.features, def: v["default-features"], ws: v.workspace} : v), m.replace(/^.*🎯️targets\//u, "").replace("/📦️packages/🦀️rust/Cargo.toml", "")].join("\t"));
    }
  };
  scan(t.dependencies, "dep"); scan(t["dev-dependencies"], "dev"); scan(t["build-dependencies"], "build");
  for (const [tg, b] of Object.entries(t.target ?? {}) as [string, any][]) { scan(b.dependencies, "dep@" + tg); scan(b["dev-dependencies"], "dev@" + tg); }
}
