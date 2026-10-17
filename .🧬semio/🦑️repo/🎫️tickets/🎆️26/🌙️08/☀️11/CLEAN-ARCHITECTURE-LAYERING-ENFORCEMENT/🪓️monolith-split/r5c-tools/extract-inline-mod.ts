// ✂️ Moves the body of one top-level inline module into its own file next to the host (`#[path] mod name;` replaces it, attributes stay). Usage: <host.rs> <module> <folder> <emoji-docline>
import {readFileSync, writeFileSync, mkdirSync, existsSync, copyFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [host, name, folder, doc] = process.argv.slice(2);
const text = readFileSync(host!, "utf8");
const node = parseRustSourceTree(text).modules.find(m => m.name === name && m.inline);
if (!node) throw new Error("no inline module " + name);
const body = text.slice(node.bodyStart, node.bodyEnd).replace(/^\n/, "").replace(/\n$/, "");
const target = join(dirname(host!), folder!, "🦀️.rs");
if (existsSync(target)) throw new Error("exists " + target);
mkdirSync(dirname(target), {recursive: true});
writeFileSync(target, `${doc}\n\n${body}\n`);
const head = text.slice(node.start, node.declStart);
const decl = `${head}#[path = "${folder}/🦀️.rs"]\n${node.visibility === "private" ? "" : node.visibility + " "}mod ${name};`;
writeFileSync(host!, text.slice(0, node.start) + decl + text.slice(node.end));
console.log("moved", name, body.length, "bytes");
