const S = process.argv[2];
const mode = process.argv[3];
const list = (await Bun.file(`${S}/${mode === "all" ? "allsrc.txt" : "touched-src-nt.txt"}`).text()).split("\n").filter(Boolean);
const EMOJI = /^\p{Extended_Pictographic}/u;
const REEXPORT = /^(export\s+\*\s+from|export\s+(type\s+)?\{[^}]*\}\s+from|pub\s+use\s|pub\s+mod\s+\w+;|export\s+\*\s+as\s)/;
const COMMENT_BODY = /^(\t+|  +)\/\/(?![\/!])\s?\S/;
const TRAILING = /[;{})\]]\s+\/\/\s\S/;
const reexportOnly: string[] = [];
const commentHits: Record<string, number> = {};
const commentSamples: string[] = [];
const docNoEmoji: string[] = [];
let docTotal = 0, docOk = 0, filesRead = 0;
const cat = Bun.spawn(["git", "cat-file", "--batch"], { cwd: "/Users/ueli/Documents/semio", stdin: "pipe", stdout: "pipe" });
const reader = cat.stdout.getReader();
let buf = new Uint8Array(0);
async function pull(n: number) { while (buf.length < n) { const { value, done } = await reader.read(); if (done) break; const m = new Uint8Array(buf.length + value.length); m.set(buf); m.set(value, buf.length); buf = m; } }
const dec = new TextDecoder();
const CH = 200;
for (let i = 0; i < list.length; i += CH) {
  const chunk = list.slice(i, i + CH);
  cat.stdin.write(chunk.map((p) => `HEAD:${p}`).join("\n") + "\n");
  for (const p of chunk) {
    await pull(1);
    let nl = buf.indexOf(10); while (nl < 0) { await pull(buf.length + 1); nl = buf.indexOf(10); }
    const header = dec.decode(buf.slice(0, nl)); buf = buf.slice(nl + 1);
    const parts = header.split(" ");
    if (parts[1] === "missing") continue;
    const size = parseInt(parts[2]);
    await pull(size + 1);
    const text = dec.decode(buf.slice(0, size)); buf = buf.slice(size + 1);
    filesRead++;
    const lines = text.split("\n");
    const subst = lines.map((l) => l.trim()).filter((l) => l && !l.startsWith("//") && !l.startsWith("/*") && !l.startsWith("*") && !l.startsWith("#"));
    if (subst.length && subst.length <= 12 && subst.every((l) => REEXPORT.test(l))) reexportOnly.push(`${p} (${subst.length} re-export lines)`);
    if (mode !== "all") {
      let hits = 0;
      for (let k = 0; k < lines.length; k++) {
        const l = lines[k];
        if (COMMENT_BODY.test(l) || (TRAILING.test(l) && !/^\s*\/\//.test(l))) { hits++; if (commentSamples.length < 400 && Math.random() < 0.02) commentSamples.push(`${p}:${k + 1}: ${l.trim().slice(0, 140)}`); }
      }
      if (hits) commentHits[p] = hits;
      // docstrings: /** block and /// block
      for (let k = 0; k < lines.length; k++) {
        const t = lines[k].trim();
        if (t.startsWith("/**") && !t.startsWith("/***")) {
          docTotal++;
          const after = t.slice(3).trim().replace(/^\*+/, "").trim();
          const first = after || (lines[k + 1] ?? "").trim().replace(/^\*+/, "").trim();
          if (EMOJI.test(first)) docOk++; else docNoEmoji.push(`${p}:${k + 1}: ${(first || "(empty)").slice(0, 100)}`);
        } else if (t.startsWith("///") && !(k > 0 && lines[k - 1].trim().startsWith("///"))) {
          docTotal++;
          const first = t.replace(/^\/\/\/\s*/, "");
          if (EMOJI.test(first)) docOk++; else docNoEmoji.push(`${p}:${k + 1}: ${first.slice(0, 100)}`);
        } else if (t.startsWith("//!") && !(k > 0 && lines[k - 1].trim().startsWith("//!"))) {
          docTotal++;
          const first = t.replace(/^\/\/!\s*/, "");
          if (EMOJI.test(first)) docOk++; else docNoEmoji.push(`${p}:${k + 1}: ${first.slice(0, 100)}`);
        }
      }
    }
  }
}
cat.stdin.end();
await Bun.write(`${S}/reexport-${mode}.txt`, reexportOnly.join("\n") + "\n");
if (mode !== "all") {
  const total = Object.values(commentHits).reduce((a, b) => a + b, 0);
  const byArea: Record<string, number> = {};
  for (const [p, n] of Object.entries(commentHits)) { const a = p.split("/").slice(0, 2).join("/"); byArea[a] = (byArea[a] ?? 0) + n; }
  const top = Object.entries(commentHits).sort((a, b) => b[1] - a[1]).slice(0, 25);
  await Bun.write(`${S}/comments-touched.txt`, `files_read=${filesRead} files_with_body_comments=${Object.keys(commentHits).length} total_lines=${total}\nTOP FILES:\n${top.map(([p, n]) => `${n}\t${p}`).join("\n")}\nBY AREA:\n${Object.entries(byArea).sort((a, b) => b[1] - a[1]).slice(0, 30).map(([a, n]) => `${n}\t${a}`).join("\n")}\nSAMPLES:\n${commentSamples.join("\n")}\n`);
  await Bun.write(`${S}/docs-touched.txt`, `files_read=${filesRead} doc_blocks=${docTotal} emoji_start=${docOk} no_emoji=${docNoEmoji.length}\n${docNoEmoji.join("\n")}\n`);
}
console.log("mode", mode, "files_read", filesRead, "reexportOnly", reexportOnly.length);
