import fixture from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧪️tests/🔣️transport/🧫️fixtures/🔣️.json";
import { pdfCosFromNativeJson, pdfFunctionFromNativeJson } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import schemaJson from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json" with { type: "json" };
import * as leaf from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { schema as nativeSchema } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts";
const cos = pdfCosFromNativeJson((fixture as any).pdf.cos), fn = pdfFunctionFromNativeJson((fixture as any).pdf.function);
const same = (a: unknown, b: unknown) => JSON.stringify(a, (_k, v) => typeof v === "bigint" ? v.toString() + "n" : v) === JSON.stringify(b, (_k, v) => typeof v === "bigint" ? v.toString() + "n" : v);
if (!same(leaf.parsePdfObject(cos), cos)) throw new Error("parsePdfObject round trip");
if (!same(leaf.parsePdfFunction(fn), fn)) throw new Error("parsePdfFunction round trip");
let refused = 0;
try { leaf.parsePdfObject((fixture as any).pdf.cos); } catch { refused++; }
try { leaf.parsePdfFunction((fixture as any).pdf.function); } catch { refused++; }
if (refused !== 2) throw new Error("native numeric transport must be refused");
const names = Object.keys((schemaJson as any).$defs);
let unresolved = 0;
for (const name of names) {
  try { leaf.validateAgainst(schemaJson as any, "/$defs/" + name, {}); } catch (e: any) { if (String(e?.message).includes("unresolved pointer") || String(e?.message).includes("unknown schema document")) unresolved++; }
}
console.log(JSON.stringify({ defs: names.length, nativeDefs: Object.keys((nativeSchema as any).$defs).length, unresolved, parsers: Object.keys(leaf).filter((k) => k.startsWith("parse")).length }));
