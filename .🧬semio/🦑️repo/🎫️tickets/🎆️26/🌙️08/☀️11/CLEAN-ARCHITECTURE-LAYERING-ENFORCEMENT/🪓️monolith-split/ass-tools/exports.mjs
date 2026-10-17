/** 📦️ Lists the `exports` key count of workspace packages named on the command line (or all linked `@semio-tech/*`). */
import { readFileSync, readdirSync, existsSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const dir = root + "/node_modules/@semio-tech";
const names = process.argv.slice(2).length ? process.argv.slice(2) : readdirSync(dir);
for (const n of names) {
  const short = n.replace("@semio-tech/", "");
  const p = `${dir}/${short}/package.json`;
  if (!existsSync(p)) { console.log(short, "NO-LINK"); continue; }
  const pkg = JSON.parse(readFileSync(p, "utf8"));
  const ex = pkg.exports;
  const keys = ex && typeof ex === "object" ? Object.keys(ex) : [];
  console.log(short, "exports=" + (ex === undefined ? "none" : typeof ex === "string" ? "string" : keys.length), "sub=" + keys.filter((k) => k !== "." && k.startsWith("./")).length);
}
