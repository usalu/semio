import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import Parser from "web-tree-sitter";
const root = process.argv[2]!, path = process.argv[3]!, kind = process.argv[4]!, name = process.argv[5]!, want = process.argv[6]!;
const repo = "/Users/ueli/Documents/semio";
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser(); p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const sha = (s: string) => createHash("sha256").update(s).digest("hex");
const norm = (s: string) => s.split("\n").map(l => l.replace(/[ \t]+$/u, "")).join("\n");
const pick = (src: string) => { const t = p.parse(src); const n = t.rootNode.descendantsOfType(kind).find((x: any) => x.childForFieldName("name")?.text === name); const out = n?.text ?? null; t.delete(); return out; };
const cur = pick(readFileSync(resolve(root, path), "utf8"));
const commits = execFileSync("git", ["log", "--format=%h", "-n", "60", "--", path], { cwd: repo }).toString().trim().split("\n");
console.log("current sha", cur ? sha(cur) : null, "commits", commits.length);
for (const c of ["WORKTREE", ...commits]) {
  let src: string | null = null;
  try { src = c === "WORKTREE" ? readFileSync(resolve(repo, path), "utf8") : execFileSync("git", ["show", `${c}:${path}`], { cwd: repo, maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] }).toString("utf8"); } catch { continue; }
  const t = pick(src); if (!t) continue;
  const s = sha(t);
  console.log(c, s === want ? "MATCH-WANT" : "", "norm-equal-current:", cur !== null && norm(t) === norm(cur), "len", t.length);
  if (s === want) break;
}
