/** 🔢️ W3-GLTF: rewrites committed glTF fixture JSON whose numbers use a non-canonical spelling (`1.0`) into the corpus
 * convention — two-space indentation, wire member order, ECMAScript/RFC 8785 number spelling (`1`) — leaving every value
 * unchanged. JSON has one number type, so `1.0` and `1` are the same value; the Rust law compares numbers by value. */
import { Glob } from "bun";

const corpus = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧫️fixtures/🧬️mutations";
const check = process.argv.includes("--check");
let rewritten = 0;
for await (const relative of new Glob("**/🔣️.json").scan(corpus)) {
  const text = await Bun.file(`${corpus}/${relative}`).text();
  const canonical = `${JSON.stringify(JSON.parse(text), null, 2)}\n`;
  if (canonical === text) continue;
  rewritten += 1;
  console.log(`${check ? "non-canonical" : "rewrote"} ${relative}`);
  if (!check) await Bun.write(`${corpus}/${relative}`, canonical);
}
console.log(`[w3-gltf-canonical-numbers] ${check ? "non-canonical" : "rewrote"}=${rewritten}`);
if (check && rewritten > 0) process.exit(1);
