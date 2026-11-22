import { readFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";

const stage = process.argv[2] ?? ".cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c";
const wasm = await import(pathToFileURL(resolve(stage, "semio_puzzle.js")));
wasm.initSync({ module: await readFile(resolve(stage, "semio_puzzle_bg.wasm")) });
const base = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples";
for (const [file, parts, fasteners] of [
  ["🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio", 180, 179],
  ["🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio", 1, 0],
  ["🌙️capsule-dream/🖼️assets/🌙️dream/🗣️.dsl.semio", 2880, 2864],
]) {
  const value = JSON.parse(wasm.puzzle5dParseDslJson(await readFile(resolve(base, file), "utf8")));
  const actual = [value.parts.length, (value.fasteners ?? []).length];
  if (actual[0] !== parts || actual[1] !== fasteners) throw new Error(`${file}: ${actual} expected ${parts},${fasteners}`);
  process.stdout.write(`${file}: ${actual.join(", ")}\n`);
}
