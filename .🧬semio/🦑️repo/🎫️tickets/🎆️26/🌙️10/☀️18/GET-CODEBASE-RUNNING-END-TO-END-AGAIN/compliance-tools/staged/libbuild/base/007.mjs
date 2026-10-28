/** 🧂️ SHA-256 digests, ordering and decoding of texts by their UTF-8 bytes on the graph's hot paths without a hash object: a digest is one call, so inference holds no `Hash` and no digest buffer per fact (a graph build asked for ~800 thousand of them through hash objects). */
import { hash } from "node:crypto";

const ASCII_DELIMITER_V1 = /^[\x00-\x7f]+$/u;

/** 🧂️ The lowercase hex SHA-256 of a text (its UTF-8 bytes; a lone surrogate is encoded as U+FFFD, as `update(text)` does) or of bytes. */
export function sha256HexV1(data) { return hash("sha256", data, "hex"); }

/** 🔪️ The first byte of the SHA-256 of a text, from the hex digest (no digest buffer). */
export function sha256FirstByteV1(text) { return Number.parseInt(hash("sha256", text, "hex").slice(0, 2), 16); }

/** 🧵️ The hex SHA-256 of `first` (when given) and then every text, each followed by `delimiter`: exactly what `update(text).update(delimiter)` for each of them digests. The delimiter must be ASCII so no surrogate pair can form across a text boundary; the whole input is one string instead of two native calls per text, and `first` saves copying a roster behind a prefix. */
export function sha256DelimitedHexV1(texts, delimiter, first) {
  if (!ASCII_DELIMITER_V1.test(delimiter)) throw new TypeError("sha256DelimitedHexV1 requires a non-empty ASCII delimiter");
  const head = first === undefined ? "" : first + delimiter;
  return hash("sha256", texts.length === 0 ? head : head + texts.join(delimiter) + delimiter, "hex");
}

/** ⚖️ Orders two texts by their UTF-8 bytes (what `Buffer.compare(Buffer.from(left), Buffer.from(right))` answers) without encoding them: two texts whose first differing UTF-16 units are both below the surrogate range are ordered by those units, and only a pair that involves a surrogate or a unit above it falls back to the encoded comparison. */
export function compareUtf8V1(left, right) {
  const limit = Math.min(left.length, right.length);
  for (let index = 0; index < limit; index++) {
    const a = left.charCodeAt(index), b = right.charCodeAt(index);
    if (a === b) continue;
    if (a < 0xd800 && b < 0xd800) return a < b ? -1 : 1;
    return Buffer.compare(Buffer.from(left), Buffer.from(right));
  }
  return left.length < right.length ? -1 : left.length > right.length ? 1 : 0;
}

/** 📜️ The UTF-8 text of bytes, read in place: a Buffer decodes itself and any other view is decoded without the copy `Buffer.from(bytes)` makes. */
export function utf8TextV1(bytes) {
  return Buffer.isBuffer(bytes) ? bytes.toString("utf8") : Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength).toString("utf8");
}
