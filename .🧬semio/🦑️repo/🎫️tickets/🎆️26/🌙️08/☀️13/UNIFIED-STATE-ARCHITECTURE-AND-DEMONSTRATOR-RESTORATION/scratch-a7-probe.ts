import { policyStateLaneExhaustivenessBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";
const b = policyStateLaneExhaustivenessBreaches("/Users/ueli/Documents/semio");
const byKind = new Map<string, number>();
for (const x of b) byKind.set(x.kind, (byKind.get(x.kind) ?? 0) + 1);
console.log("total:", b.length);
for (const [k, n] of [...byKind].sort((a, c) => c[1] - a[1])) console.log(`  ${n}  ${k}`);
console.log("--- storage sites ---");
for (const x of b.filter((y) => y.kind.endsWith("storage-outside-config-lane"))) console.log(`${x.scope}:${x.line}`);
