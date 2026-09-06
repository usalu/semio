/**
 * 🔬️ W5 probe — evaluates the live-tree clauses of the root `📜️script.ts` gate
 * `toolJobRasterEnvelopeCallerRetainedExact` one at a time, so a single `false` names the exact
 * clause instead of collapsing the whole law into one boolean.
 *
 * Read-only. Extracts the function text out of `📜️script.ts` by brace matching, splits its `return (…)`
 * expression on top-level `&&`, and evaluates each clause with the same five sources the real
 * `verify interactivity tool-jobs` run feeds it.
 *
 * @see 📜️script.ts `toolJobRasterEnvelopeCallerRetainedExact`
 * @see 📓️w5-raster-export-gate.md
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const SCRIPT = readFileSync(join(ROOT, "📜️script.ts"), "utf8");

function extract(name: string): string {
  const start = SCRIPT.indexOf(`function ${name}(`);
  if (start < 0) throw new Error(`missing ${name}`);
  const end = SCRIPT.indexOf("\n}\n", start);
  if (end < 0) throw new Error(`unterminated ${name}`);
  return SCRIPT.slice(start, end + 2);
}

function readSafe(...parts: string[]): string {
  try {
    return readFileSync(join(ROOT, ...parts), "utf8");
  } catch {
    return "";
  }
}

const MOUNTED = [
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️87a/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs",
];

for (const file of MOUNTED) {
  const source = readSafe(file);
  console.log(`[mounted] ${source.trim() === "" ? "MISSING" : `${source.length} bytes`}  ${file}`);
}

const store = readSafe("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs");
const raster = [
  readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"),
  readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs"),
  readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"),
  readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"),
  ...MOUNTED.map((file) => readSafe(file)),
].join("\n");
const editor = readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
const wasm = readSafe("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs");
const plugin = readSafe("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");

const body = extract("toolJobRasterEnvelopeCallerRetainedExact");
const preamble = body.slice(body.indexOf("{") + 1, body.lastIndexOf("return ("));
const expression = body.slice(body.lastIndexOf("return (") + "return (".length, body.lastIndexOf(");"));

const clauses = expression
  .split("\n")
  .map((line) => line.trim().replace(/\s*&&$/, ""))
  .filter((line) => line !== "");

const helper = extract("toolJobStoreInitializerRetainedExact");
const wrapper = `${helper}
function __probe(store, raster, editor, wasm, plugin, clauses) {
${preamble}
return clauses.map((clause) => { try { return Boolean(eval(clause)); } catch (error) { return String(error); } });
}
`;
const transpiled = new Bun.Transpiler({ loader: "ts" }).transformSync(wrapper);
const evaluate = new Function(`${transpiled}\nreturn __probe;`)() as (...args: unknown[]) => (boolean | string)[];

const results = evaluate(store, raster, editor, wasm, plugin, clauses);
const failed = results.map((result, index) => ({ result, clause: clauses[index]!.trim() })).filter((row) => row.result !== true);
console.log(`\n[probe] clauses=${clauses.length} passing=${results.filter((row) => row === true).length} failing=${failed.length}`);
for (const row of failed) console.log(`  FAIL  ${row.clause.replace(/\s+/g, " ").slice(0, 180)}`);
