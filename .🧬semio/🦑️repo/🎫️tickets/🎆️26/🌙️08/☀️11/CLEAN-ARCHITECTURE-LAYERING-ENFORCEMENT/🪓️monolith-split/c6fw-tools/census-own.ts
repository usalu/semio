// 🧮️ c6fw own-closure census: canonical closure counter over the lib root of the c6fw crates (lib and cfg(test) closures); writes nothing.
// usage: bun census-own.ts [only,tags]
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const only = process.argv[2] ? new Set(process.argv[2].split(",")) : undefined;
const M = "🧰️framework/🔨️modules";
const PK = "/📦️packages/🦀️rust";
const B3 = `${M}/🧊️3d/📐️brep`;
const crates: [string, string, string][] = [
  ["ge", "semio-framework-3d-brep-engine", `${B3}/⚙️engine`],
  ["mp", "semio-framework-3d-brep-queries", `${B3}/💡️queries`],
  ["bl", "semio-framework-3d-brep-construction", `${B3}/🛠️operations`],
  ["bo", "semio-framework-3d-brep-boolean", `${B3}/🛠️operations/🔀️boolean`],
  ["mm", "semio-framework-3d", `${M}/🧊️3d`],
  ["ss", "semio-framework-3d-brep-intersect", `${B3}/💡️queries/✂️intersect`],
  ["te", "semio-framework-3d-brep-tessellation", `${B3}/💡️queries/🧩tessellation`],
  ["to", "semio-framework-3d-brep-representation", `${B3}/📸️representation`],
  ["md", "semio-framework-3d-mesh-document", `${M}/🧊️3d/🥽️mesh/📄️document`],
  ["ap", "semio-framework-actor-job-publication", `${M}/🎭️actor/💼️job/📨️publication`],
  ["av", "semio-framework-actor-vocabulary", `${M}/🎭️actor/📖️vocabulary`],
  ["ar", "semio-framework-actor", `${M}/🎭️actor`],
  ["jw", "semio-framework-job-worker", `${M}/🧵️job/👷️worker`],
  ["jr", "semio-framework-job", `${M}/🧵️job`],
];
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};
const libPath = (dir: string): string => {
  const t = Bun.TOML.parse(readFileSync(resolve(root, dir + PK + "/Cargo.toml"), "utf8")) as any;
  return resolve(root, dir + PK, t.lib?.path ?? "src/lib.rs");
};
for (const [tag, name, dir] of crates) {
  if (only && !only.has(tag)) continue;
  for (const test of [false, true]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : [])];
    try {
      const entry = libPath(dir);
      const inputs = await readRustSourceClosureV1([entry], root, cfg, readFile, {manifestDirectory: resolve(root, dir + PK), generated: () => undefined});
      const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      console.log(JSON.stringify({tag, name, test, files, bytes, activeLines: lines, ratio: +Math.max(files / 32, bytes / 262144, lines / 4000).toFixed(3)}));
    } catch (error) {
      console.log(JSON.stringify({tag, name, test, error: String((error as Error).message ?? error).slice(0, 200)}));
    }
  }
}
