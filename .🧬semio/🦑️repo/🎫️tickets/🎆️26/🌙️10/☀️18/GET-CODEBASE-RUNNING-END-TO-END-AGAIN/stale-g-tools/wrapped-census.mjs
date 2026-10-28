// 🧪️ stale-g: census of process-owner wrapped targets (generic-target recipes whose command is `repo "<owner-command driver>" owner-command … -- <child>`), newest record per project root. Streams records; RSS guard 700 MB.
import {readFileSync, readdirSync, statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const D = "/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const newest = new Map();
for (const name of readdirSync(D)) { if (!name.endsWith(".json")) continue; const m = statSync(D + name).mtimeMs; let t; try { t = brotliDecompressSync(readFileSync(D + name)).toString("utf8"); } catch { continue; } const at = t.indexOf("\n"); let v; try { v = JSON.parse(t.slice(at + 1)); } catch { continue; } if (!v.root) continue; const prev = newest.get(v.root); if (prev && prev.m >= m) continue; const wrapped = [], all = Object.values(v.closureRecipes ?? {}); for (const r of all) { const c = r.target?.options?.command; if (r.fn === "generic-target" && typeof c === "string" && /owner-command --cwd/.test(c) && !/ native owner-command /.test(c)) wrapped.push(c); } newest.set(v.root, {m, wrapped: wrapped.length, recipes: all.length, name: v.name}); if (process.memoryUsage().rss > 700e6) { console.error("rss"); process.exit(3); } }
let projects = 0, wrappedProjects = 0, wrappedRecipes = 0; const sample = [];
for (const [root, r] of newest) { projects++; if (r.wrapped) { wrappedProjects++; wrappedRecipes += r.wrapped; sample.push(r.name + " " + r.wrapped); } }
console.log(JSON.stringify({projects, wrappedProjects, wrappedRecipes})); console.log(sample.join("\n"));
