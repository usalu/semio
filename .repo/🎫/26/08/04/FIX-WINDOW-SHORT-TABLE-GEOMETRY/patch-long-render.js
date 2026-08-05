const fs = require("fs");
const p = "print/tex/semio-table.sty";
let s = fs.readFileSync(p, "utf8");
const i = s.indexOf("  % Empty \\endfirsthead starts the head chunk");
const i2 = s.indexOf("  % Page-1 head must live in \\endfirsthead.");
const start = i >= 0 ? i : i2;
if (start < 0) {
  // maybe already partially patched — find by endfirsthead + chrome
  const alt = s.indexOf("  \\endfirsthead\n  \\noalign{\\global\\setbox\\LT@firsthead");
  console.log("alt", alt);
}
if (start < 0 && s.indexOf("  \\endfirsthead\n  \\noalign{\\global\\setbox\\LT@firsthead") < 0) {
  throw new Error("start not found: " + start);
}
const from = start >= 0 ? start : s.indexOf("  \\endfirsthead\n  \\noalign{\\global\\setbox\\LT@firsthead");
// walk back to comment start
let a = from;
while (a > 0 && s[a - 1] !== "\n") a--;
if (s.slice(a, a + 4) === "  % ") {
  // ok at comment
} else {
  // find preceding comment block start
  const c = s.lastIndexOf("\n  % ", from);
  a = c + 1;
}
const j = s.indexOf("  \\end{longtable}%", a);
const k = j + "  \\end{longtable}%".length;
const rep = `  % One head definition only. Filling BOTH firsthead and head made this
  % longtable stack emit chrome+column-labels TWICE on page 1 (measured: two
  % Marktplaetze chip rows ~48pt apart). With firsthead empty, \\endhead is
  % used for the first page as well as continuations — single chip, single
  % header, no floating gap.
  \\endfirsthead
  \\semio@table@long@continuation@chrome{#5}%
  #3%
  \\endhead
  \\noalign{\\vskip\\z@}% head->body transition: no negative pull (a -\\semio@table@body@height skip here pulled the first body row up over the header row, hiding it)
  #4%
  \\end{longtable}%`;
fs.writeFileSync(p, s.slice(0, a) + rep + s.slice(k));
console.log("[DEBUG] restored", a, k);