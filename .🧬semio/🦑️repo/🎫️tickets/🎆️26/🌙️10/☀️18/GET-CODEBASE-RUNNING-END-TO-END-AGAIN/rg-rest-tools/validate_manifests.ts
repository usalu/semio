import { TOML } from "bun";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
const R = "/Users/ueli/Documents/semio";
const files: string[] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const manifests = files.filter((f) => f.endsWith("Cargo.toml") && f.includes("📦️packages") || (f.endsWith("Cargo.toml") && !f.includes("/Cargo.toml") ) );
const mans = files.filter((f) => f.endsWith("/Cargo.toml") || f === "Cargo.toml");
const wsCache = new Map<string, any>();
const loadWs = (p: string) => { if (!wsCache.has(p)) wsCache.set(p, TOML.parse(readFileSync(p, "utf8")) as any); return wsCache.get(p); };
let bad = 0;
for (const m of mans) {
  const abs = join(R, m);
  if (!existsSync(abs)) continue;
  const t = TOML.parse(readFileSync(abs, "utf8")) as any;
  const dir = dirname(abs);
  if (t.package) {
    const wsRel = t.package.workspace;
    let wsAbs: string | null = null;
    if (wsRel) wsAbs = join(dir, wsRel, "Cargo.toml");
    if (wsAbs && !existsSync(wsAbs)) { console.log("NO WS", m); bad++; continue; }
    if (wsAbs) {
      const ws = loadWs(wsAbs);
      const members: string[] = ws.workspace?.members ?? [];
      const rel = relative(dirname(wsAbs), dir);
      if (!members.includes(rel) && !members.some((x) => x.endsWith("*"))) { console.log("NOT MEMBER", m.slice(-90), rel.slice(-60)); bad++; }
      const lib = t.lib?.path;
      if (lib && !existsSync(join(dir, lib))) { console.log("NO LIB", m.slice(-90), lib); bad++; }
      for (const sec of ["dependencies", "dev-dependencies", "build-dependencies"]) {
        for (const [k, v] of Object.entries(t[sec] ?? {}) as [string, any][]) {
          if (v && typeof v === "object") {
            if (v.path && !existsSync(join(dir, v.path, "Cargo.toml"))) { console.log("DEP PATH", m.slice(-90), k, v.path.slice(-60)); bad++; }
            if (v.workspace === true && !(ws.workspace?.dependencies && k in ws.workspace.dependencies)) { console.log("WS DEP MISSING", m.slice(-90), k); bad++; }
          }
        }
      }
      for (const tg of t.test ?? []) if (tg.path && !existsSync(join(dir, tg.path))) { console.log("TEST PATH", m.slice(-80), tg.path); bad++; }
    }
  }
}
console.log("checked", mans.length, "bad", bad);
