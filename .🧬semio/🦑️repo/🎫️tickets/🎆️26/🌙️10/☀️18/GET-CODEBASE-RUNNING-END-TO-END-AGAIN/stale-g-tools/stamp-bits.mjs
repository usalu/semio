// 🧪️ stale-g: decodes the verified stamp of the current generation pointer: time, covered owner count, and whether named owners are covered. usage: bun stamp-bits.mjs <owner>...
import {readFileSync} from "node:fs";
const dir = "/Users/ueli/Documents/semio/.cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const owners = Object.keys(m.owners).sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
const v = c.verified, bytes = Buffer.from(v.owners, "base64"), covered = new Set(); owners.forEach((n, i) => { if (bytes[i >> 3] >> (i & 7) & 1) covered.add(n); });
console.log(JSON.stringify({generation: c.generation, reason: c.reason, verifiedAt: new Date(v.atMs).toString(), count: v.count, owners: owners.length, covered: covered.size, builtKind: m.built.kind, builtBase: m.built.base, dirty: m.built.dirty?.length}));
for (const n of process.argv.slice(2)) console.log(n, covered.has(n));
