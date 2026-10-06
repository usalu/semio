/** 🔤️ Physical bitmap base64 and JSON byte fields. */
const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const VALUES = new Map<string, number>([...ALPHABET].map((character, index) => [character, index]));

/** 🔤️ RFC 4648 §4 standard base64, padded. */
export function encodeBase64(bytes: Uint8Array): string {
  let out = "";
  for (let start = 0; start < bytes.length; start += 3) {
    const b0 = bytes[start]!;
    const b1 = bytes[start + 1] ?? 0;
    const b2 = bytes[start + 2] ?? 0;
    const triple = (b0 << 16) | (b1 << 8) | b2;
    out += ALPHABET[(triple >> 18) & 63]! + ALPHABET[(triple >> 12) & 63]!;
    out += start + 1 < bytes.length ? ALPHABET[(triple >> 6) & 63]! : "=";
    out += start + 2 < bytes.length ? ALPHABET[triple & 63]! : "=";
  }
  return out;
}

/** 🔤️ The exact inverse; a malformed buffer throws rather than truncating. */
export function decodeBase64(text: string): Uint8Array {
  if (text.length % 4 !== 0) throw new Error("base64 length is not a multiple of four");
  const out: number[] = [];
  for (let start = 0; start < text.length; start += 4) {
    const quad = text.slice(start, start + 4);
    const pad = [...quad].filter((character) => character === "=").length;
    if (pad > 2 || (pad > 0 && !quad.endsWith("=".repeat(pad)))) throw new Error("misplaced base64 padding");
    let triple = 0;
    for (let offset = 0; offset < 4; offset += 1) {
      const character = quad[offset]!;
      if (character === "=") continue;
      const value = VALUES.get(character);
      if (value === undefined) throw new Error(`byte ${character} is outside the base64 alphabet`);
      triple |= value << (18 - 6 * offset);
    }
    out.push((triple >> 16) & 0xff);
    if (pad < 2) out.push((triple >> 8) & 0xff);
    if (pad < 1) out.push(triple & 0xff);
  }
  const bytes = Uint8Array.from(out);
  if (encodeBase64(bytes) !== text) throw new Error("noncanonical base64");
  return bytes;
}


/** 📥️ Binds declared physical byte fields before semantic admission. */
export function bitmapJsonBind<T>(value: unknown): T {
  function bind(value: unknown, key = ""): unknown {
    if ((key === "pixels" || key === "inputPixels") && value != null) {
      if (typeof value !== "string") throw new Error("physical bitmap pixels require base64");
      return decodeBase64(value);
    }
    if (Array.isArray(value)) return value.map(value => bind(value));
    if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key,value]) => [key,bind(value,key)]));
    return key === "seed" && value != null ? BigInt(value as string | number | bigint) : value;
  }
  return bind(value) as T;
}
/** 📤️ Lowers intrinsic bitmap byte fields into their physical JSON spelling. */
export function bitmapJsonValue(value: unknown): unknown {
  if (value instanceof Uint8Array) return encodeBase64(value);
  if (typeof value === "bigint") return value.toString();
  if (Array.isArray(value)) return value.map(bitmapJsonValue);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key,value])=>[key,bitmapJsonValue(value)]));
  return value;
}
