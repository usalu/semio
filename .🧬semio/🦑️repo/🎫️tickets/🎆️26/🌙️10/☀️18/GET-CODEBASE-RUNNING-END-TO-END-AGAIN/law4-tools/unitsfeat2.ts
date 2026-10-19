import { readFileSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const plugin = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const U = plugin + "/🧪️tests/📦️units";
await Parser.init(); const p = new Parser();
p.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", "/Users/ueli/Documents/semio/✏️s")), "out/tree-sitter-rust.wasm")));
const d = JSON.parse(readFileSync(U + "/📋️feature-owner.json", "utf8"));
const read = (x: string) => readFileSync(x, "utf8");
const seen: string[] = [];
const visit = (path: string) => { if (seen.includes(path)) return; seen.push(path); for (const m of read(path).matchAll(/#\[path\s*=\s*"([^"\n]+)"\]|include!\(\s*"([^"\n]+)"\s*\)/g)) visit(resolve(dirname(path), (m[1] ?? m[2])!)); };
const lawFiles = [plugin + "/🧪️testing/🦀️.rs"]; visit(plugin + "/🧪️testing/🧪️artifact-app-laws/🦀️.rs"); lawFiles.push(...seen);
console.log(seen.length, "files");
const act: any[] = [];
for (const f of lawFiles) { const t = p.parse(read(f)); if (t.rootNode.hasError()) console.log("ERR", f.slice(-60)); for (const n of t.rootNode.descendantsOfType("function_item")) act.push({ name: n.childForFieldName("name")?.text, sha256: createHash("sha256").update(n.text).digest("hex"), file: f.replace(plugin, "") }); }
const exp = d.remainingFunctions as any[];
const key = (x: any) => x.name + ":" + x.sha256;
const A = new Set(act.map(key)), E = new Set(exp.map(key));
console.log("act", act.length, "exp", exp.length, "missingFromAct", exp.filter(x => !A.has(key(x))).length, "extraInAct", act.filter(x => !E.has(key(x))).length);
console.log("missing", exp.filter(x => !A.has(key(x))).map(x => x.name).slice(0, 40).join(","));
console.log("extra", act.filter(x => !E.has(key(x))).map(x => x.name + "@" + x.file.slice(-50)).slice(0, 40).join(" | "));
const an = act.map(x => x.name), en = exp.map(x => x.name);
console.log("names missing", en.filter(n => !an.includes(n)), "names extra", an.filter(n => !en.includes(n)));

if (process.argv[2] === "write") { const fs = await import("node:fs"); d.remainingFunctions = act.map(x => ({ name: x.name, sha256: x.sha256 })); fs.writeFileSync(U + "/📋️feature-owner.json", JSON.stringify(d, null, 2) + "\n"); console.log("written", act.length); }
