// 🧾️ Closure rows for each carved crate root (lib and test cfg) via the canonical counter: argv[2]=repo root, argv[3]=json file of [{id,root,dir,features}]
import {readFileSync,writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const [root, spec, out] = process.argv.slice(2);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const rows = JSON.parse(readFileSync(spec, "utf8")) as {id: string; root: string; dir: string; testkit?: boolean}[];
const result: any[] = [];
for (const row of rows) {
  for (const label of ["lib", "test"]) {
    const extra = label === "test" ? ["test", ...(row.testkit ? ["feature=testkit"] : [])] : [];
    try {
      const inputs = await readRustSourceClosureV1([root + "/" + row.root], root, [...host, "debug_assertions", ...extra], read, {manifestDirectory: root + "/" + row.dir, generated: () => undefined});
      result.push({id: row.id, cfg: label, files: inputs.length, bytes: inputs.reduce((s, i) => s + i.bytes, 0), lines: inputs.reduce((s, i) => s + i.activeLines, 0), list: inputs.map(i => [i.path.replace(root + "/", ""), i.kind, i.bytes, i.activeLines])});
    } catch (e) { result.push({id: row.id, cfg: label, error: String((e as Error).message).slice(0, 300)}); }
  }
}
writeFileSync(out, JSON.stringify(result));
