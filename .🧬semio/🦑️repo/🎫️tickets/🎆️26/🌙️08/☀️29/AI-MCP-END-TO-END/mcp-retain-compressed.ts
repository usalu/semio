import { createHash } from "node:crypto";
import { createReadStream, createWriteStream } from "node:fs";
import { rename, stat } from "node:fs/promises";
import { resolve, sep } from "node:path";
import { pipeline } from "node:stream/promises";
import { createGunzip, createGzip } from "node:zlib";

const generated = resolve(import.meta.dir, "🗑️generated");
const [source, destination, expected] = process.argv.slice(2);
if (!source || !destination || !/^[a-f0-9]{64}$/u.test(expected ?? "")) throw Error("Source, archive, and known SHA-256 required");
const input = resolve(source);
const output = resolve(destination);
if (input === output || ![input, output].every(path => path.startsWith(generated + sep))) throw Error("Distinct ticket scratch paths required");
const partial = `${output}.partial`;
await pipeline(createReadStream(input), createGzip({ level: 1 }), createWriteStream(partial, { flags: "wx" }));
const hash = createHash("sha256");
let restored = 0;
for await (const chunk of createReadStream(partial).pipe(createGunzip())) {
  hash.update(chunk);
  restored += chunk.byteLength;
}
if (restored !== (await stat(input)).size || hash.digest("hex") !== expected) throw Error("Archive does not reproduce the expected input");
await rename(partial, output);
console.log(`[DEBUG] Verified ${restored} restored bytes at SHA-256 ${expected}; archive has ${(await stat(output)).size} bytes`);
