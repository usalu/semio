// 🔬️ Plain full-resolution oracle on a sandbox copy of one plugin at the working tree: usage `bun orig-oracle.ts <pluginDir> <rootFile> <crateName> <sandbox> [--test]`; writes short diagnostics next to the sandbox.
import {mkdirSync, rmSync, cpSync, readdirSync, readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname, resolve} from "node:path";

const [pluginDir, rootFile, crateName, sandbox, ...flags] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio";
rmSync(sandbox!, {recursive: true, force: true});
const dest = resolve(sandbox!, pluginDir!);
mkdirSync(dirname(dest), {recursive: true});
cpSync(process.env.SRC_DIR ? resolve(process.env.SRC_DIR) : resolve(ROOT, pluginDir!), dest, {recursive: true, filter: p => !/\/(node_modules|target|dist)(\/|$)/u.test(p)});
const prefixes = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"];
const walk = (d: string): void => {
  for (const e of readdirSync(d, {withFileTypes: true})) {
    const p = resolve(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name.endsWith(".rs")) {
      const t = readFileSync(p, "utf8");
      const q = t.replace(/^[ \t]*(?:#\[[^\]]*\]\s*)*extern\s+crate\s+[^;]+;[ \t]*$/gmu, "")
        .replace(new RegExp(`\\buse\\s+(?:${prefixes.map(x => x.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|")})[A-Za-z0-9_:]*::\\*;`, "gu"), "const _: () = ();");
      if (q !== t) writeFileSync(p, q);
    }
  }
};
walk(dest);
const r = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crateName!.replaceAll("-", "_"), "--emit=metadata", "--error-format=short", "-o", "/dev/null", ...flags, resolve(sandbox!, rootFile!)], {encoding: "utf8", maxBuffer: 1 << 30, cwd: sandbox});
writeFileSync(`${sandbox}.short`, r.stderr ?? "");
console.log("exit", r.status, "lines", (r.stderr ?? "").split("\n").length);
