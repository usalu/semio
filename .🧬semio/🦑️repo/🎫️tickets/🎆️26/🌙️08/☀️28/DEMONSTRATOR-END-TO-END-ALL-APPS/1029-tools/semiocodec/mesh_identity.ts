// usage: bun run mesh_identity.ts — the identity-round-trip law of the mesh adapter, outside the repo test host.
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { packBytes, parseDsl, parsePack, printDsl } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts";

const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/";
const read = (path: string): Uint8Array => new Uint8Array(readFileSync(root + path));
const sha = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
for (const [dsl, pack] of [
  ["🧪️tests/🧫️fixtures/🔺️mutate-semio-mesh/🗣️.dsl.semio", "🧪️tests/🧫️fixtures/🔺️mutate-semio-mesh/🎒️.pack.semio"],
  ["📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio", "📚️examples/🧊️cube/🖼️assets/🎒️.pack.semio"],
] as const) {
  const dslBytes = read(dsl);
  const parsed = parseDsl(new TextDecoder().decode(dslBytes));
  if (sha(new TextEncoder().encode(printDsl(parsed))) !== sha(dslBytes)) throw new Error("dsl reprint differs");
  const packBuffer = read(pack);
  const unpacked = parsePack(packBuffer);
  const a = JSON.stringify(unpacked);
  const b = JSON.stringify(parsed);
  if (a !== b) {
    let at = 0;
    while (a[at] === b[at]) at += 1;
    throw new Error(`pack differs from dsl at ${at}: ${a.slice(Math.max(0, at - 80), at + 80)} <> ${b.slice(Math.max(0, at - 80), at + 80)}`);
  }
  if (sha(packBytes(parsed)) !== sha(packBuffer)) throw new Error("repack differs");
  console.log("OK", pack.split("/")[2], packBuffer.length);
}
