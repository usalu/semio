// 🔒️ SipHash-1-3 with zero keys — mirrors Rust std `DefaultHasher::new()`.
const M = (1n << 64n) - 1n;
const rotl = (x: bigint, b: bigint) => ((x << b) | (x >> (64n - b))) & M;

function sipRound(v: bigint[]): void {
  v[0] = (v[0] + v[1]) & M; v[1] = rotl(v[1], 13n); v[1] ^= v[0]; v[0] = rotl(v[0], 32n);
  v[2] = (v[2] + v[3]) & M; v[3] = rotl(v[3], 16n); v[3] ^= v[2];
  v[0] = (v[0] + v[3]) & M; v[3] = rotl(v[3], 21n); v[3] ^= v[0];
  v[2] = (v[2] + v[1]) & M; v[1] = rotl(v[1], 17n); v[1] ^= v[2]; v[2] = rotl(v[2], 32n);
}

export function siphash13(bytes: Uint8Array): bigint {
  const v = [
    0x736f6d6570736575n, 0x646f72616e646f6dn, 0x6c7967656e657261n, 0x7465646279746573n,
  ];
  const len = bytes.length;
  const blocks = Math.floor(len / 8);
  const read = (off: number): bigint => {
    let m = 0n;
    for (let i = 7; i >= 0; i -= 1) m = (m << 8n) | BigInt(bytes[off + i]);
    return m;
  };
  for (let i = 0; i < blocks; i += 1) {
    const m = read(i * 8);
    v[3] ^= m;
    sipRound(v);
    v[0] ^= m;
  }
  let b = BigInt(len & 0xff) << 56n;
  for (let i = blocks * 8, shift = 0n; i < len; i += 1, shift += 8n) b |= BigInt(bytes[i]) << shift;
  v[3] ^= b;
  sipRound(v);
  v[0] ^= b;
  v[2] ^= 0xffn;
  sipRound(v); sipRound(v); sipRound(v);
  return (v[0] ^ v[1] ^ v[2] ^ v[3]) & M;
}

/** 🪪️ Rust `catalog_child_handle`: hash of `serde_json::to_string(&types)` as a `String`. */
export function catalogChildId(types: { id: string; name: string; category: string }[]): string {
  const canonical = JSON.stringify(types);
  const utf8 = new TextEncoder().encode(canonical);
  const bytes = new Uint8Array(utf8.length + 1);
  bytes.set(utf8, 0);
  bytes[utf8.length] = 0xff;
  return `catalog-${siphash13(bytes).toString(16).padStart(16, "0")}`;
}
