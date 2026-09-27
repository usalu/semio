/** ⚖️ WG9 (ticket-local, from WG7's s12-codec-oracle) — asks the SERVED catalog-exact component (jco on V8, the browser shell's
 * runtime) for `codec.pack-schema-hash(<kind>)` and compares it with the trusted catalog's pin (wasmtime at publish).
 * Usage: node --experimental-wasm-jspi wg9-codec-probe.mjs <componentJs> <kind> <pinHex> */
import { pathToFileURL } from "node:url";
const [componentJs, kind, pinHex] = process.argv.slice(2);
const component = await import(pathToFileURL(componentJs).href);
console.log("exports", Object.keys(component), "codec", Object.keys(component.codec ?? {}));
try {
  const hash = Buffer.from(await component.codec.packSchemaHash(kind)).toString("hex");
  console.log(`${hash === pinHex ? "PASS" : "FAIL"} pack-schema-hash ${kind} jco=${hash} catalog=${pinHex}`);
} catch (error) {
  console.log(`FAIL pack-schema-hash ${kind} threw ${JSON.stringify(error?.payload ?? String(error)).slice(0, 400)}`);
}
