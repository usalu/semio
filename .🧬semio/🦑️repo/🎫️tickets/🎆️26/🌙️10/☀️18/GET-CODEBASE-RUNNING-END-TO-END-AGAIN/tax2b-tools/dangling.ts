// 🔎 Lists quoted literals anywhere in the repo that are a path suffix of a path tax2 renamed. usage: bun dangling.ts <pairs.tsv> <out.json>
import {execFileSync} from "node:child_process";
import {readFileSync, writeFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio";
const pairs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t") as [string, string]);
const bases = [...new Set(pairs.map(([f]) => f.split("/").at(-1)!))];
const out = new Map<string, string>();
for (let i = 0; i < bases.length; i += 8) {
  const args = ["grep", "--untracked", "-I", "-F", "-n"];
  for (const b of bases.slice(i, i + 8)) args.push("-e", b);
  args.push("--", ".", ":!.🧬semio", ":!node_modules", ":!*/node_modules/*", ":!*/target/*", ":!*/📤️dist/*");
  let text = ""; try { text = execFileSync("git", args, {cwd: R, maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"]}).toString(); } catch (e: any) { text = e.stdout?.toString() ?? ""; }
  for (const line of text.split("\n")) { const m = /^([^:]+):(\d+):(.*)$/u.exec(line); if (m) out.set(`${m[1]}:${m[2]}`, m[3]!); }
}
const hits: {file: string; line: number; literal: string; removed: string[]; successor: string}[] = [];
for (const [key, content] of out) {
  const [file, line] = [key.slice(0, key.lastIndexOf(":")), Number(key.slice(key.lastIndexOf(":") + 1))];
  for (const m of content.matchAll(/(["'`])((?:(?!\1)[^\n\\])*)\1/gu)) {
    const v = m[2]!;
    if (!bases.some((b) => v.endsWith(b))) continue;
    const frag = v.replace(/^(\.\.?\/)+/u, "");
    const cands = pairs.filter(([f]) => f === frag || f.endsWith("/" + frag));
    if (cands.length) hits.push({file, line, literal: v, removed: cands.map(([f]) => f).slice(0, 3), successor: cands[0]![1]});
  }
}
writeFileSync(process.argv[3]!, JSON.stringify(hits, null, 1));
console.log(hits.length);
for (const h of hits) console.log(`${h.file.slice(-95)}:${h.line}  ${h.literal.slice(-60)}  [${h.removed.length}+ cands]`);
