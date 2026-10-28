// 🧪️ stale-g: lists the inference project records (np) whose project root matches a substring, newest first, with their mtime and closure recipe digests. usage: bun records.mjs <root-substring>
import {readFileSync, readdirSync, statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const D = "/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const needle = process.argv[2], hits = [];
for (const name of readdirSync(D)) { if (!name.endsWith(".json")) continue; const text = brotliDecompressSync(readFileSync(D + name)).toString("utf8"), at = text.indexOf("\n"); if (!text.slice(at).includes(needle)) continue; let value; try { value = JSON.parse(text.slice(at + 1)); } catch { continue; } if (typeof value.root === "string" && value.root.endsWith(needle)) hits.push({name, mtime: statSync(D + name).mtimeMs, header: text.slice(0, at).slice(0, 300), root: value.root, keys: Object.keys(value)}); }
hits.sort((a, b) => b.mtime - a.mtime);
for (const h of hits) console.log(new Date(h.mtime).toISOString(), h.name, h.root.slice(-50), h.header.slice(0, 200));
