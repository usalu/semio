// usage: bun run blake3_vectors.ts '[sizes]' — prints [{size, hash}] from @noble/hashes over the fixture's byte pattern (byte i = (i * 31 + 7) mod 256).
import { blake3 } from "@noble/hashes/blake3.js";

const sizes = JSON.parse(process.argv[2]!) as number[];
const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
console.log(JSON.stringify(sizes.map((size) => ({ size, hash: hex(blake3(Uint8Array.from({ length: size }, (_, index) => (index * 31 + 7) & 0xff))) }))));
