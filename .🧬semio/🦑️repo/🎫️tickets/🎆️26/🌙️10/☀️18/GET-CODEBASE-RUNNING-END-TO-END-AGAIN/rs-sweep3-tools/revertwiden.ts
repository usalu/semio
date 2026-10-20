import { readFileSync, writeFileSync } from "node:fs";
const C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const ledger: Record<string, string> = JSON.parse(readFileSync(`${C}/🗑️generated/rs-sweep3-x/backup/ledger.json`, "utf8"));
const [listFile, keep = "^$", apply] = process.argv.slice(2);
const keepRe = new RegExp(keep);
const VIS = /\bpub(?:\s*\((?:crate|super|self|in [^)]+)\))?(?=\s)/;
const norm = (l: string) => l.replace(/^(\s*(?:#\[[^\]]*\]\s*)*)pub(?:\s*\((?:crate|super|self|in [^)]+)\))?\s+/, "$1");
const rank = (l: string) => (/^\s*pub\s*\(/.test(l) ? 1 : /^\s*pub\s/.test(l) ? 2 : 0);
let total = 0;
for (const rel of readFileSync(listFile, "utf8").split("\n").filter(Boolean)) {
  const file = rel.startsWith("/") ? rel : `/Users/ueli/Documents/semio/${rel}`;
  const key = Object.keys(ledger).find((k) => k.normalize() === file.normalize());
  if (!key) { console.log("no backup", rel); continue; }
  const b = readFileSync(`${C}/🗑️generated/rs-sweep3-x/backup/${ledger[key]}`, "utf8").split("\n");
  const c = readFileSync(file, "utf8").split("\n");
  const nb = b.map(norm), nc = c.map(norm);
  const n = b.length, m = c.length;
  const dp: Uint16Array[] = Array.from({ length: n + 1 }, () => new Uint16Array(m + 1));
  for (let i = n - 1; i >= 0; i--) for (let j = m - 1; j >= 0; j--) dp[i][j] = nb[i] === nc[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1]);
  let i = 0, j = 0, changed = 0;
  while (i < n && j < m) {
    if (nb[i] === nc[j]) {
      if (b[i] !== c[j] && rank(b[i]) < rank(c[j]) && !keepRe.test(c[j])) { if (process.env.VERBOSE) console.log("   ", c[j].trim().slice(0, 140), "<=", b[i].trim().slice(0, 60)); c[j] = b[i]; changed++; }
      i++; j++;
    } else if (dp[i + 1][j] >= dp[i][j + 1]) i++;
    else j++;
  }
  total += changed;
  console.log(String(changed).padStart(4), rel.slice(-70));
  if (apply === "apply" && changed) writeFileSync(file, c.join("\n"));
}
console.log("total", total);
