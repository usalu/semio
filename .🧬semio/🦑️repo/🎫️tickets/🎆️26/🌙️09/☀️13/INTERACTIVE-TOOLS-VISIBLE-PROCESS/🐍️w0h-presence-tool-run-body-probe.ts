/** ⏯️ W0-H probe: the TypeScript standalone tool run summary codec agrees with the shared presence peer corpus. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { decodePresenceToolRun, encodePresenceToolRun } from "../../../../../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";

const repo = join(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(join(repo, "🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json"), "utf8"));
let failures = 0;
let checked = 0;
for (const row of fixture.cases.filter((row: { id: string }) => row.id.startsWith("tool-run-"))) {
  const peer = Buffer.from(row.prefixHex + row.repeatHex.repeat(row.repeatCount) + row.suffixHex, "hex");
  const body = new Uint8Array(peer.subarray(5));
  checked += 1;
  try {
    const toolRun = decodePresenceToolRun(body);
    if (!row.accepted) throw new Error("accepted a refused body");
    const encoded = Buffer.from(encodePresenceToolRun(toolRun)).toString("hex");
    if (encoded !== Buffer.from(body).toString("hex")) throw new Error(`re-encoded ${encoded}`);
    if (JSON.stringify(toolRun) !== JSON.stringify(row.expected.toolRun)) throw new Error(`decoded ${JSON.stringify(toolRun)}`);
  } catch (error) {
    if (row.accepted) {
      failures += 1;
      console.log(`FAIL ${row.id}: ${(error as Error).message}`);
    }
  }
}
console.log(`presence tool run bodies checked=${checked} failures=${failures}`);
process.exit(failures === 0 && checked > 0 ? 0 : 1);
