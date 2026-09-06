#!/usr/bin/env node
/** 🔢️ Independent oracle for `semio.pack.dynamic-integer/v1`.
 *
 * Regenerates (and by default only verifies) every `wireHex` in
 * `🧰️framework/🛍️products/💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json` from the
 * fixture's tagged decimal strings, using TWO independent LEB128 implementations:
 *
 *   1. a from-scratch `BigInt` unsigned/zig-zag LEB128 written here, and
 *   2. the third-party `@webassemblyjs/leb128` (Apache-2.0) `encodeUIntBuffer`, fed the
 *      little-endian 64-bit image of the same magnitude — the varint conversion itself is
 *      entirely the library's.
 *
 * Usage: `node fable-pack-lossless-integer-oracle.mjs [--write]`
 */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "..", "..", "..", "..", "..", "..", "..");
const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
const require = createRequire(join(repoRoot, "package.json"));
const leb = require("@webassemblyjs/leb128/lib/leb.js").default;

const U64_MAX = (1n << 64n) - 1n;
const I64_MIN = -(1n << 63n);
const I64_MAX = (1n << 63n) - 1n;

const hex = bytes => Array.from(bytes, byte => byte.toString(16).padStart(2, "0")).join("");

/** 🧮️ From-scratch canonical unsigned LEB128 over `BigInt`. */
function ownUleb(value) {
  if (value < 0n || value > U64_MAX) throw new Error(`u64 out of range: ${value}`);
  const out = [];
  let remaining = value;
  for (;;) {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    if (remaining === 0n) { out.push(byte); return out; }
    out.push(byte | 0x80);
  }
}

/** 🧮️ Third-party LEB128 over the same magnitude's little-endian 64-bit image. */
function foreignUleb(value) {
  if (value < 0n || value > U64_MAX) throw new Error(`u64 out of range: ${value}`);
  const image = new Uint8Array(8);
  let remaining = value;
  for (let index = 0; index < 8; index++) { image[index] = Number(remaining & 0xffn); remaining >>= 8n; }
  return Array.from(leb.encodeUIntBuffer(image));
}

function uleb(value) {
  const own = ownUleb(value);
  const foreign = foreignUleb(value);
  if (hex(own) !== hex(foreign)) throw new Error(`LEB128 oracle disagreement for ${value}: own=${hex(own)} foreign=${hex(foreign)}`);
  return own;
}

const zigzag = value => (value < 0n ? (-value * 2n) - 1n : value * 2n);

function encodeValue(node, out) {
  const keys = Object.keys(node);
  if (keys.length !== 1) throw new Error(`fixture value node needs exactly one tag: ${JSON.stringify(node)}`);
  const [tag] = keys;
  if (tag === "uint") {
    const value = BigInt(node.uint);
    if (value < 0n || value > U64_MAX) throw new Error(`uint out of range: ${node.uint}`);
    out.push(0x04, ...uleb(value));
    return;
  }
  if (tag === "int") {
    const value = BigInt(node.int);
    if (value < I64_MIN || value > I64_MAX) throw new Error(`int out of range: ${node.int}`);
    out.push(0x03, ...uleb(zigzag(value)));
    return;
  }
  if (tag === "f64LeHex") {
    if (!/^[0-9a-f]{16}$/u.test(node.f64LeHex)) throw new Error(`f64LeHex must be 16 lowercase hex digits: ${node.f64LeHex}`);
    out.push(0x05, ...node.f64LeHex.match(/../gu).map(pair => Number.parseInt(pair, 16)));
    return;
  }
  if (tag === "list") {
    out.push(0x0c, ...uleb(BigInt(node.list.length)));
    for (const item of node.list) encodeValue(item, out);
    return;
  }
  if (tag === "map") {
    const encoder = new TextEncoder();
    const entries = node.map.map(([key, value]) => [encoder.encode(key), value]);
    entries.sort(([a], [b]) => { const min = Math.min(a.length, b.length); for (let i = 0; i < min; i++) if (a[i] !== b[i]) return a[i] - b[i]; return a.length - b.length; });
    out.push(0x10, ...uleb(BigInt(entries.length)));
    for (const [key, value] of entries) { out.push(0x07, ...uleb(BigInt(key.length)), ...key); encodeValue(value, out); }
    return;
  }
  throw new Error(`unsupported fixture value tag ${tag}`);
}

/** 📦️ `store::pack_rt::encode_wire_value` framing: symbol table, one `Shape::Value` field. */
function encodeWire(node) {
  const out = [0x00, 0x01, 0x01, 0x11];
  encodeValue(node, out);
  return out;
}

function maxDepth(node) {
  if (node.list) return 1 + Math.max(0, ...node.list.map(maxDepth));
  if (node.map) return 1 + Math.max(0, ...node.map.map(([, value]) => maxDepth(value)));
  return 1;
}

const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const write = process.argv.includes("--write");
let mismatches = 0;
for (const row of fixture.accept) {
  const expected = hex(encodeWire(row.value));
  const depth = maxDepth(row.value);
  if (depth > 3) throw new Error(`${row.id}: depth ${depth} exceeds the corpus bound of 3`);
  if (expected.length / 2 > 256) throw new Error(`${row.id}: ${expected.length / 2} bytes exceeds the corpus bound of 256`);
  if (row.wireHex !== expected) {
    mismatches += 1;
    console.log(`${write ? "rewrote" : "MISMATCH"} ${row.id}: fixture=${row.wireHex} oracle=${expected}`);
    row.wireHex = expected;
  } else {
    console.log(`ok ${row.id}: ${expected} (${expected.length / 2} bytes, depth ${depth})`);
  }
}
if (write && mismatches) writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`pack-dynamic-integer oracle: accept=${fixture.accept.length} reject=${fixture.reject.length} mismatches=${mismatches}`);
if (mismatches && !write) process.exitCode = 1;
