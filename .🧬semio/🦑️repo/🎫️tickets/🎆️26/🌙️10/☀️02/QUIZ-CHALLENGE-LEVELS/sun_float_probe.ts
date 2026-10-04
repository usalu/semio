/** 🌞️ Probes whether serde_json's default (not `float_roundtrip`) reading of `3.828e26` — the significand times a power of ten from its table — differs from the correctly rounded double JavaScript reads. */
console.log("[DEBUG] JS", 3.828e26, "3828*1e23", 3828 * 1e23, "equal", 3828 * 1e23 === 3.828e26, "3828e23/…", 3828 / 1e-23);
console.log("[DEBUG] 1e23", 1e23, "174e15", 174 * 1e15 === 1.74e17);
