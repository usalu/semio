/** 🧮️ W1-C: independent (TS first-party BLAKE3 + hand-written LEB128) transaction id minting for the transaction-law fixture. */
import { blake3Hex } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";

function varint(out: number[], value: bigint): void {
  let rest = value;
  for (;;) {
    const byte = Number(rest & 0x7fn);
    rest >>= 7n;
    if (rest === 0n) return void out.push(byte);
    out.push(byte | 0x80);
  }
}
function str(out: number[], text: string): void {
  const bytes = new TextEncoder().encode(text);
  varint(out, BigInt(bytes.length));
  out.push(...bytes);
}
function mint(actor: string, clock: { actor: bigint; physical_ms: bigint; logical: bigint }, tool: string): string {
  const out: number[] = [];
  str(out, actor);
  varint(out, clock.actor);
  varint(out, clock.physical_ms);
  varint(out, clock.logical);
  str(out, tool);
  return `tx-${blake3Hex(Uint8Array.from(out)).slice(0, 16)}`;
}
const vectors: Array<[string, [bigint, bigint, bigint], string]> = [
  ["actor-1", [7n, 1002n, 0n], "demo#drag"],
  ["actor-1", [7n, 1002n, 1n], "demo#drag"],
  ["actor-2", [7n, 1002n, 0n], "demo#drag"],
  ["actor-1", [7n, 1002n, 0n], "demo#rotate"],
  ["Zoë 🧑‍🎨", [9007199254740991n, 1727654400000n, 300n], "puzzle#select"],
  ["", [0n, 0n, 0n], "t"],
  ["actor-1", [7n, 1005n, 0n], "demo#drag"],
  ["actor-1", [7n, 1007n, 0n], "demo#drag"],
];
for (const [actor, [a, p, l], tool] of vectors) console.log(JSON.stringify({ actor, clock: { actor: Number(a), physical_ms: Number(p), logical: Number(l) }, tool, id: mint(actor, { actor: a, physical_ms: p, logical: l }, tool) }));
