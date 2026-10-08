import { readFileSync, renameSync, statSync, writeFileSync } from "node:fs";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui";

/** 🧩 Applies exact multi-line replacements to one file, keeping its line endings; the swap goes through a temporary file so stale tail bytes cannot survive. */
export function patch(file: string, edits: Array<[string, string]>) {
  const path = `${ui}/${file}`;
  const original = readFileSync(path, "utf8");
  const crlf = original.includes("\r\n");
  let text = original.replaceAll("\r\n", "\n");
  for (const [from, to] of edits) {
    const first = text.indexOf(from);
    if (first < 0) throw new Error(`${file}: missing ${JSON.stringify(from.slice(0, 80))}`);
    if (text.indexOf(from, first + 1) >= 0) throw new Error(`${file}: ambiguous ${JSON.stringify(from.slice(0, 80))}`);
    text = text.slice(0, first) + to + text.slice(first + from.length);
  }
  const output = crlf ? text.replaceAll("\n", "\r\n") : text;
  const staging = `${path}.q1a-tmp`;
  writeFileSync(staging, output, "utf8");
  renameSync(staging, path);
  if (statSync(path).size !== Buffer.byteLength(output, "utf8")) throw new Error(`${file}: size mismatch after write`);
}
