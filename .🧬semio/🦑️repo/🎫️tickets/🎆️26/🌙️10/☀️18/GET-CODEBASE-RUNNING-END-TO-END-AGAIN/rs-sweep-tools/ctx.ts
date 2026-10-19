const [f, kind, re, n = "3"] = process.argv.slice(2);
const fs = require("fs");
const l = fs.readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x)).filter((x: any) => x.kind === kind && new RegExp(re).test(x.crate + " " + x.msg));
console.log(l.length, "matches");
for (const x of l.slice(0, +n)) {
  console.log("##", x.crate, x.file + ":" + x.line, "\n  ", x.msg.slice(0, 330));
  try { const lines = fs.readFileSync("/Users/ueli/Documents/semio/" + x.file, "utf8").split("\n"); console.log("   >", lines[x.line - 1]?.trim().slice(0, 260)); } catch {}
}
