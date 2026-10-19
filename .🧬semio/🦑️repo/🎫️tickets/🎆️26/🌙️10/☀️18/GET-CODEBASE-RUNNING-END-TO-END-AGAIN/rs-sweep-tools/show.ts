const [f, kind, re, n = "6"] = process.argv.slice(2);
const l = require("fs").readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x)).filter((x: any) => x.kind === kind && new RegExp(re).test(x.crate + " " + x.msg + " " + x.file));
console.log(l.length);
for (const x of l.slice(0, +n)) console.log(x.crate, "|", x.file + ":" + x.line, "|", x.msg.slice(0, 400), x.fix ?? "");
