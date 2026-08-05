const fs = require("fs");
const p = "print/tex/semio-table.sty";
let s = fs.readFileSync(p, "utf8");
const marker = "  \\setlength{\\LTcapwidth}{\\linewidth}%";
const a = s.indexOf(marker);
if (a < 0) throw new Error("capwidth not found");
const b = s.indexOf("  \\end{longtable}%", a);
if (b < 0) throw new Error("end longtable not found");
const end = b + "  \\end{longtable}%".length;
const rep = `  \\setlength{\\LTcapwidth}{\\linewidth}%
  % Do not call \\endfirsthead. An empty \\endfirsthead stores a non-void
  % empty \\LT@firsthead, so page 1 ships that blank box and skips \\LT@head
  % (measured: Abkuerzungen after Glossar lost chrome/header; cream gap +
  % pillar stubs between Test-Case and AP). With firsthead left void,
  % longtable uses \\LT@head on page 1 and every continuation — one chrome,
  % one column-header (filling BOTH firsthead and head doubled Marktplaetze
  % chips ~48pt apart).
  \\global\\setbox\\LT@firsthead=\\box\\voidb@x
  \\begin{longtable}{#1}%
  \\semio@table@long@continuation@chrome{#5}%
  #3%
  \\endhead
  \\noalign{\\vskip\\z@}% head->body transition: no negative pull (a -\\semio@table@body@height skip here pulled the first body row up over the header row, hiding it)
  #4%
  \\end{longtable}%`;
fs.writeFileSync(p, s.slice(0, a) + rep + s.slice(end));
console.log("[DEBUG] patched", a, end);
console.log(fs.readFileSync(p, "utf8").slice(a, a + 700));
