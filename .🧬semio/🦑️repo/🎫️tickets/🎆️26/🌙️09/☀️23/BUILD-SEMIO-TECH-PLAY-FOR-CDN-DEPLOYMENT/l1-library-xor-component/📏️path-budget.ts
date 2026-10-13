/** 📏️ L4 budget for every component crate of wave L1: package name length, longest absolute source path and the worst-case uplifted
 * output path of a fresh release generation (`⚡️cache/fXXXXXX/{t,b}`), all in UTF-16 units. */
import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const manifests = execSync("git ls-files --cached --others --exclude-standard -- '*/🧩️component/📦️packages/🦀️rust/Cargo.toml'", { cwd: repo, encoding: "utf8" }).trim().split("\n").filter(Boolean);
const generation = join(repo, ".🧬semio/🦑️repo/⚡️cache/fXXXXXX");
const rows = manifests.map((manifest) => {
  const name = readFileSync(join(repo, manifest), "utf8").match(/^name = "([^"]+)"/m)![1]!, lib = name.replaceAll("-", "_");
  const owner = manifest.replace("/📦️packages/🦀️rust/Cargo.toml", "");
  const sources = execSync(`git ls-files --cached --others --exclude-standard -- ${JSON.stringify(owner)}`, { cwd: repo, encoding: "utf8" }).trim().split("\n").filter(Boolean).map((path) => join(repo, path));
  const uplifted = [join(generation, "b/wasm32-wasip2/wasm-release/build", name, "0123456789abcdef/deps", `${lib}-0123456789abcdef.wasm`), join(generation, "t/wasm32-wasip2/wasm-release", `${lib}.wasm`), join(repo, owner, "📦️packages/🦀️rust/dist/components", `${lib}.wasm`)];
  return { name, nameLength: name.length, source: Math.max(...sources.map((path) => path.length)), uplifted: Math.max(...uplifted.map((path) => path.length)) };
});
const worst = (key: "nameLength" | "source" | "uplifted") => rows.reduce((left, right) => (right[key] > left[key] ? right : left));
console.log(JSON.stringify({ components: rows.length, maxName: [worst("nameLength").name, worst("nameLength").nameLength], maxSource: [worst("source").name, worst("source").source], maxUplifted: [worst("uplifted").name, worst("uplifted").uplifted], overName: rows.filter((row) => row.nameLength > 48).length, overSource: rows.filter((row) => row.source > 256).length, overUplifted: rows.filter((row) => row.uplifted > 240).length }));
