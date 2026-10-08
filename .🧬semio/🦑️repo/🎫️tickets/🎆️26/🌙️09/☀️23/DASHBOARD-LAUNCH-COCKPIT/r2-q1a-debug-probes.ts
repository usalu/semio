import { readFileSync, writeFileSync } from "node:fs";

const ui = "C:/git/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui";
const edits: Array<[string, RegExp, string]> = [
  ["📟️vt/🧪️tests/🔬️unit/🦀️.rs", /println!\("\[DEBUG\] terminal streams: \{\} shared cases", cases\.len\(\)\);/, 'assert!(!cases.is_empty(), "the shared terminal stream fixture lists cases");'],
  ["📟️vt/🧪️tests/🔬️unit/🦀️.rs", /println!\("\[DEBUG\] terminal keys: \{\} terminfo capabilities", cases\.len\(\)\);/, 'assert!(!cases.is_empty(), "the shared terminfo fixture lists capabilities");'],
  ["📟️vt/🧪️tests/🔬️unit/🦀️.rs", /println!\("\[DEBUG\] terminal pointer: \{\} shared reports", cases\.len\(\)\);/, 'assert!(!cases.is_empty(), "the shared pointer fixture lists reports");'],
  ["🧪️tests/⌨️input-decoding/🦀️.rs", /println!\("\[DEBUG\] input decoding: \{\} shared vectors", rows\.len\(\)\);/, 'assert!(!rows.is_empty(), "the shared decoding fixture lists vectors");'],
];
for (const [file, pattern, replacement] of edits) {
  const path = `${ui}/${file}`;
  const text = readFileSync(path, "utf8");
  if (!pattern.test(text)) throw new Error(`${file}: ${pattern} not found`);
  writeFileSync(path, text.replace(pattern, replacement), "utf8");
}
console.log("probes replaced by assertions");
