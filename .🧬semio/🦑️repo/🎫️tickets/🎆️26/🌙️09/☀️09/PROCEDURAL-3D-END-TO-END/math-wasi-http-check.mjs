import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { readdir } from "node:fs/promises";
import { join, resolve } from "node:path";

const local = resolve(process.argv[2]);
const ports = process.argv.slice(3);
if (!ports.length || ports.some(port => !/^[1-9][0-9]{3,4}$/.test(port))) {
  throw new Error("Supply each development host's loopback port");
}
const stem = "semio_s_plugin_flow_math_component";
const names = (await readdir(local)).filter(name => new RegExp(`^${stem}\\.(?:js|core[0-9]*\\.wasm)$`).test(name)).sort();
if (!names.includes(stem + ".js") || !names.includes(stem + ".core.wasm")) {
  throw new Error("Incomplete verified Math module");
}
const directory = ["🧩️extension-modules", "🧮️flow-extension-math"].map(encodeURIComponent).join("/");
async function fingerprint(stream) {
  const hash = createHash("sha256");
  let bytes = 0;
  for await (const chunk of stream) {
    hash.update(chunk);
    bytes += chunk.length;
  }
  return { bytes, hash: hash.digest("hex") };
}
for (const name of names) {
  const expected = await fingerprint(createReadStream(join(local, name)));
  for (const port of ports) {
    const response = await fetch(`http://127.0.0.1:${port}/${directory}/${name}`, { signal: AbortSignal.timeout(30000) });
    if (response.status !== 200 || !response.body) throw new Error(`HTTP ${response.status} on ${port} for ${name}`);
    const actual = await fingerprint(response.body);
    if (actual.bytes !== expected.bytes || actual.hash !== expected.hash) throw new Error(`Math digest mismatch on ${port} for ${name}`);
    console.log("[DEBUG] served Math", port, name, actual.bytes, actual.hash);
  }
}
