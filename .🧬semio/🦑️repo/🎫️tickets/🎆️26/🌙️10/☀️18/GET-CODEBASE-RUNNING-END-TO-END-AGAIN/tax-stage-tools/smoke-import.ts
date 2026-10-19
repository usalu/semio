// 🔬 Imports every changed non-test script and module of the mirror in isolation (one process each) to catch cycles, missing exports at link time and import-time failures. usage: bun smoke-import.ts <mirror> [filter]
import {spawnSync} from "node:child_process";
import {readFileSync, writeFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {C, LIB} from "./vfs.ts";
const mirror = resolve(process.argv[2]!), filter = process.argv[3] ?? "";
const ops = JSON.parse(readFileSync(`${C}/tax-stage-tools/staged/ops.json`, "utf8")).ops as {path: string; kind: string}[];
const files = ops.filter((o) => o.kind === "write" && /\.(ts|mjs)$/u.test(o.path) && (o.path.endsWith("📜️script.ts") || o.path.includes("🔬️script-commands") || o.path.includes("package-commands") || /🏗️build\/🟦️\.ts$|🏗️bazel\/🟦️\.ts$|🛠️tools\/🟦️\.ts$/u.test(o.path) || o.path.endsWith("📦️child/🟦️.ts") === false && !o.path.includes("🧪️tests"))).map((o) => o.path).filter((p) => p.includes(filter));
let bad = 0;
for (const rel of files) {
  const probe = `try{await import(${JSON.stringify(join(mirror, rel))});console.log("OK")}catch(e){console.log("FAIL "+String(e?.message??e).slice(0,300))}`;
  const r = spawnSync("bun", ["-e", probe], {cwd: mirror, env: {...process.env, SEMIO_WORKSPACE_ROOT: mirror}, encoding: "utf8", timeout: 240000});
  const last = (r.stdout.trim().split("\n").at(-1) ?? "") || ("EXIT " + r.status + " " + r.stderr.trim().split("\n").at(-1));
  if (!last.startsWith("OK")) bad++;
  console.log(`${last.startsWith("OK") ? "ok  " : "BAD "} ${rel.replace(LIB, "L").slice(-110)} ${last.startsWith("OK") ? "" : last.slice(0, 260)}`);
}
console.log(JSON.stringify({imported: files.length, bad}));
