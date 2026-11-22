/** 🎟️ Stages the same-origin capture client on the dev Shell host port. */
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { join } from "node:path";

const generated = join(import.meta.dir,"🗑️generated","browser-first-archive-client");
await mkdir(generated,{recursive:true});
const host = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev" + "/🪝️host-ports";
const owner = host + "/🟦️.ts";
const client = host + "/💾️first-archive/🟦️.ts";
const [prior, source] = await Promise.all([Bun.file(owner).text(),Bun.file(join(import.meta.dir,"browser-first-archive-client.ts")).text()]);
assert(!await Bun.file(client).exists());
const replace = (body:string, old:string, next:string) => {
  assert.equal(body.split(old).length,2,old);
  return body.replace(old,next);
};
let updated=replace(prior,'import { WASM_BINDINGS_V1 } from "./🕸️wasm-bindings/🟦️.ts";',[
  'import { WASM_BINDINGS_V1 } from "./🕸️wasm-bindings/🟦️.ts";',
  'import { createDevFirstArchiveCaptureV1 } from "./💾️first-archive/🟦️.ts";',
].join("\n"));
updated=replace(updated,"    wasmBindings: WASM_BINDINGS_V1,","    wasmBindings: WASM_BINDINGS_V1,\n    firstArchiveCapture: createDevFirstArchiveCaptureV1(),");
for(const s of [source,updated]) new Bun.Transpiler({loader:"ts"}).transformSync(s);
const sha = new Bun.CryptoHasher("sha256").update(prior).digest("hex");
await Bun.write(join(generated,"overlay.json"),JSON.stringify([[owner,sha,updated],[client,null,source]]));
console.log("[DEBUG] Staged the Browser first-archive capture host port");
