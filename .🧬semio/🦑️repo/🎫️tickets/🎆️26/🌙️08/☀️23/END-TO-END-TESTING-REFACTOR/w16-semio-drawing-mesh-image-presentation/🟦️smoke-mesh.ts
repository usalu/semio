// 🧪️ Scratch smoke test: the mesh independent implementation against the committed cube artifact and
// every committed per-kind specification vector. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, w16.
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import { parseDsl, printDsl, parsePack, packBytes, applyMutation, inverseMutation } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-mesh/🟦️component.ts";

const REPO = "/Users/ueli/Documents/semio";
const base = `${REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh`;
const dsl = readFileSync(`${base}/📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio`);
const pack = readFileSync(`${base}/📚️examples/🧊️cube/🖼️assets/🎒️example.pack.semio`);
const doc = parseDsl(dsl.toString("utf8"));
const printed = Buffer.from(printDsl(doc), "utf8");
console.log("dsl byte-exact:", printed.equals(dsl), printed.length, dsl.length);
if (!printed.equals(dsl)) { console.log("got:", printed.toString()); console.log("exp:", dsl.toString()); }
const unpacked = parsePack(new Uint8Array(pack));
console.log("pack==dsl:", JSON.stringify(unpacked) === JSON.stringify(doc));
const repacked = Buffer.from(packBytes(doc));
console.log("pack byte-exact:", repacked.equals(pack), repacked.length, pack.length);

const root = `${base}/🧬️schema/🧬️mutations`;
let ok = true;
for (const d of readdirSync(root).sort()) {
  const tests = join(root, d, "🧪️tests");
  if (!existsSync(tests)) continue;
  for (const f of readdirSync(tests).sort()) {
    const p = join(tests, f);
    const rd = (rel: string) => JSON.parse(readFileSync(join(p, rel), "utf8"));
    const b = rd("📸️snapshot/⬅️before/🔣️component.json");
    const a = rd("📸️snapshot/➡️after/🔣️component.json");
    const m = rd("🦠️mutation/🔣️component.json");
    let got, back;
    try {
      got = applyMutation(b, m);
      back = inverseMutation(b, m).reduce((acc, step) => applyMutation(acc, step), got);
    } catch (error) { console.log("FAIL", d, (error as Error).message); ok = false; continue; }
    const fw = JSON.stringify(got) === JSON.stringify(a);
    const iv = JSON.stringify(back) === JSON.stringify(b);
    if (!(fw && iv)) { ok = false; console.log("FAIL", d, fw ? "" : "FWD", iv ? "" : "INV"); if (!fw) { console.log("  got:", JSON.stringify(got).slice(0, 600)); console.log("  exp:", JSON.stringify(a).slice(0, 600)); } }
    else console.log("OK  ", d);
  }
}
console.log("ALL", ok);
