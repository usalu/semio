// #region Header
/** @emoji 🔒 Injectable content hasher for `@semio-tech/vcs-core` — mirrors `semio-framework-hash::hash_bytes`
 * (blake3 hex digest). The default blake3 implementation is the ONLY module allowed to import a hashing
 * library directly; `index.ts` only ever depends on the {@link Hasher} interface. */
// #endregion Header

// #region 🔌Adapters
import { blake3 } from "@noble/hashes/blake3.js";
import { bytesToHex } from "@noble/hashes/utils.js";
// #endregion 🔌Adapters

//#region 🔖Hasher
/** @emoji 🔒 Content hasher contract — swap implementations without touching store logic. */
export interface Hasher {
  hash(bytes: Uint8Array): string;
}

/** @emoji 🌊 Default {@link Hasher}: blake3 hex digest, byte-identical to Rust's `hash_bytes`. */
export class Blake3Hasher implements Hasher {
  hash(bytes: Uint8Array): string {
    return bytesToHex(blake3(bytes));
  }
}

/** @emoji 🏭 Builds the default blake3 {@link Hasher}. */
export function createDefaultHasher(): Hasher {
  return new Blake3Hasher();
}
//#endregion 🔖Hasher

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/vcs-core hasher", () => {
    it("hashes deterministically to a 64-char hex digest", () => {
      const hasher = createDefaultHasher();
      const bytes = new TextEncoder().encode('{"op":"setN","n":1}');
      const digest = hasher.hash(bytes);
      expect(digest).toMatch(/^[0-9a-f]{64}$/);
      expect(hasher.hash(bytes)).toBe(digest);
    });

    it("hashes distinct inputs to distinct digests", () => {
      const hasher = createDefaultHasher();
      const a = hasher.hash(new TextEncoder().encode("a"));
      const b = hasher.hash(new TextEncoder().encode("b"));
      expect(a).not.toBe(b);
    });
  });
}
//#endregion 🧪Tests
