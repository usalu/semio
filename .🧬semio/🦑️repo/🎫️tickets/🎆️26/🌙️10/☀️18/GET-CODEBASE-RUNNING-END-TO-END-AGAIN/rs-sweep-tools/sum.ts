const [f, crateRe, fileRe] = process.argv.slice(2);
const l = require("fs").readFileSync(f, "utf8").trim().split("\n").map((x: string) => JSON.parse(x)).filter((x: any) => new RegExp(crateRe ?? ".").test(x.crate) && new RegExp(fileRe ?? ".").test(x.file));
const m = new Map<string, any>();
for (const x of l) {
  const msg = x.msg.replace(/ \[(lib|test|lib\+wasm|lib\+wasm\+test)\]/, "");
  const k = `${x.crate}|${x.file}|${x.line}|${msg}`;
  if (!m.has(k)) m.set(k, { ...x, msg });
}
const byLine = new Map<string, string[]>();
for (const v of m.values()) { const k = `${v.crate}|${v.file}|${v.line}`; (byLine.get(k) ?? byLine.set(k, []).get(k)!).push(v.msg); }
for (const v of m.values()) {
  const sibs = byLine.get(`${v.crate}|${v.file}|${v.line}`)!;
  const p = /^path `([^`]+)`/.exec(v.msg);
  if (p && sibs.some((s) => s !== v.msg && s.startsWith("path `" + p[1] + "::"))) continue;
  console.log(`${v.crate.replace("semio-", "")} ${v.file.split("/").slice(-4).join("/")}:${v.line} ${v.kind} ${v.msg.slice(0, 260)}`);
}
