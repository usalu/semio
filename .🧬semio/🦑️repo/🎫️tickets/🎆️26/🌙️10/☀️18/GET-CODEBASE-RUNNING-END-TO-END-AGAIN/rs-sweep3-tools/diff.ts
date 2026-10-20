const [a, b, kind, n = "15"] = process.argv.slice(2);
const load = (f: string) => require("fs").readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x));
const key = (x: any) => `${x.kind}|${x.file}|${x.msg.replace(/@\d+/g, "@N")}`;
const A = new Set(load(a).map(key));
const nw = load(b).filter((x: any) => !A.has(key(x)) && (!kind || x.kind === kind));
console.log("new in b:", nw.length);
for (const x of nw.slice(0, +n)) console.log(x.kind, x.crate, x.file.slice(-60) + ":" + x.line, "|", x.msg.slice(0, 220));
