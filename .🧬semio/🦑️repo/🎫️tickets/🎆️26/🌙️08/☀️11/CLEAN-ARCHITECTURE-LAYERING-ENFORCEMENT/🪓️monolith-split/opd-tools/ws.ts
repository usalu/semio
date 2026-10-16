// 🗺️ Maps each package that owns an edited file to its workspace root and package name (opd: real-check commands).
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const dirs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const parse = (d: string): any => { try { return Bun.TOML.parse(readFileSync(join(ROOT, d, "Cargo.toml"), "utf8")); } catch { return {}; } };
for (const d of dirs) {
  const m = parse(d);
  const pointer = m.package?.workspace as string | undefined;
  let ws: string;
  if (pointer) ws = relative(ROOT, resolve(join(ROOT, d), pointer)).replaceAll("\\", "/");
  else { ws = d; for (let cur = d; ; cur = dirname(cur) === "." ? "" : dirname(cur)) { if (existsSync(join(ROOT, cur, "Cargo.toml")) && parse(cur).workspace) { ws = cur; break; } if (cur === "") { ws = ""; break; } } }
  console.log(`${m.package?.name}\t${ws || "."}\t${d}`);
}
