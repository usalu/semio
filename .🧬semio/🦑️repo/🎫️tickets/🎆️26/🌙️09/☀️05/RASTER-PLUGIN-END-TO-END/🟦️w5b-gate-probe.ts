/**
 * 🔬️ W5b probe — evaluates the live-tree clauses of the root `📜️script.ts` gate
 * `toolJobRasterEnvelopeCallerRetainedExact` one at a time, so a single `false` names the exact
 * clause instead of collapsing the whole law into one boolean.
 *
 * Successor of `🟦️w5-gate-probe.ts`: the gate lost its `wasm` parameter (the wasm-bindgen bridge was
 * deleted repo-wide in `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), the
 * retained paged-ingress law now lives in the editor cohort, and the schema source that used to be
 * read as `🦀️component.rs` is `🦀️.rs`.
 *
 * Read-only. Extracts the function text out of `📜️script.ts` by brace matching, splits its `return (…)`
 * expression on top-level `&&`, and evaluates each clause with the same four sources the real
 * `verify interactivity tool-jobs` run feeds it.
 *
 * @see 📜️script.ts `toolJobRasterEnvelopeCallerRetainedExact`
 * @see 📓️w5b-raster-gate-residuals.md
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

const CODEC = [
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs",
];

const EDITOR_FILE = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs";

for (const file of [...CODEC, ...MOUNTED, EDITOR_FILE]) {
  const source = readSafe(file);
  console.log(`[source] ${source.trim() === "" ? "MISSING" : `${source.length} bytes`}  ${file}`);
}

const store = readSafe("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs");
const raster = [...CODEC, ...MOUNTED].map((file) => readSafe(file)).join("\n");
const editor = readSafe(EDITOR_FILE);
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
function __probe(store, raster, editor, plugin, clauses) {
${preamble}
return clauses.map((clause) => { try { return Boolean(eval(clause)); } catch (error) { return String(error); } });
}
`;
const transpiled = new Bun.Transpiler({ loader: "ts" }).transformSync(wrapper);
const evaluate = new Function(`${transpiled}\nreturn __probe;`)() as (...args: unknown[]) => (boolean | string)[];

const results = evaluate(store, raster, editor, plugin, clauses);
const failed = results.map((result, index) => ({ result, clause: clauses[index]!.trim() })).filter((row) => row.result !== true);
console.log(`\n[probe] clauses=${clauses.length} passing=${results.filter((row) => row === true).length} failing=${failed.length}`);
for (const row of failed) console.log(`  FAIL  ${row.clause.replace(/\s+/g, " ").slice(0, 180)}`);

/**
 * 🧭️ Oracle pass — the same ingress clauses evaluated against the GIS Map editor, the live plugin
 * that already carries the post-bridge retained paged-ingress cohort. Every structural clause has to
 * pass there; only the four Raster-named identifiers may differ. This proves the rewritten clauses
 * describe code that exists in this repo today rather than a shape invented for the gate.
 */
const oracleEditor = readSafe("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
const ingressClauses = clauses.filter((clause) => clause.startsWith("editor.") || /^(credits|page|construct|admit)\b/.test(clause));
const oracle = evaluate(store, raster, oracleEditor, plugin, ingressClauses);
const oracleFailed = oracle.map((result, index) => ({ result, clause: ingressClauses[index]! })).filter((row) => row.result !== true);
console.log(`\n[oracle gis-map editor] ingress clauses=${ingressClauses.length} passing=${oracle.filter((row) => row === true).length} failing=${oracleFailed.length}`);
for (const row of oracleFailed) console.log(`  ONLY-RASTER-NAMED  ${row.clause.replace(/\s+/g, " ").slice(0, 180)}`);
