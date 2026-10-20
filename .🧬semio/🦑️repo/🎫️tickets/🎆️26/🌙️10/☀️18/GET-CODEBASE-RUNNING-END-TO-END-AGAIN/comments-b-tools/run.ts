import { readFileSync, statSync, writeFileSync, mkdirSync, existsSync, appendFileSync } from "node:fs";
import { applyRemovals } from "./keep.ts";
import { planTs, tsSignature, tsKind } from "./ts-engine.ts";
import { initRust, planRs, rsSignature } from "./rs-engine.ts";
import { planRsLex, rsLexSignature } from "./rs-lexer.ts";

const ROOT = "/Users/ueli/Documents/semio";
const OUT = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/comments-b`;
mkdirSync(OUT, { recursive: true });
const args = process.argv.slice(2);
const flag = (n: string, d?: string) => {
  const i = args.indexOf(n);
  return i >= 0 ? args[i + 1] : d;
};
const mode = flag("--mode", "dry")!;
const prefixes = (flag("--prefix") ?? "✏️s,🌎️hub,🎓️teaching,🏢️semio-tech,♻️mit-bestand").split(",");
const recentMin = Number(flag("--recent-min", "15"));
const sampleN = Number(flag("--sample", "50"));
const withScripts = args.includes("--scripts");
const genOk = args.includes("--gen-ok");
const tag = flag("--tag", mode)!;

const listFile = `${OUT}/files.nul`;
if (!existsSync(listFile) || args.includes("--refresh")) {
  const r = Bun.spawnSync(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech", "♻️mit-bestand"], { cwd: ROOT, maxBuffer: 1 << 30 });
  writeFileSync(listFile, r.stdout);
}
const listed = readFileSync(listFile, "utf8").split("\0").filter((f) => f && prefixes.some((p) => f.startsWith(p)));
const EXT = /\.(ts|tsx|mts|cts|js|mjs|cjs|jsx|rs)$/;
const SKIP_PATH = /(^|\/)(node_modules|dist|target|🗑️generated|\.next|\.storybook-static|coverage)\//;
const GEN_HEAD = /@generated|DO NOT EDIT|auto-?generated|Code generated|GENERATED FILE/i;

type Row = { path: string; removed: number; kept: number; error?: string };
const skipped: Record<string, string[]> = { generated: [], recent: [], scripts: [], noparse: [], gone: [], pattern: [] };
const rows: Row[] = [];
const changed: string[] = [];
const verifyFail: string[] = [];
const lexFallback: string[] = [];
const xcheck: string[] = [];
const samples: { path: string; lang: string; text: string }[] = [];

const rsWasm = `${ROOT}/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm`;
await initRust(rsWasm);

const now = Date.now();
let seen = 0;
for (const rel of listed.sort()) {
  if (!EXT.test(rel)) continue;
  if (SKIP_PATH.test(rel) || /\.d\.ts$/.test(rel) || /\.min\.(js|mjs)$/.test(rel) || /jcoprobe/.test(rel)) {
    skipped.pattern.push(rel);
    continue;
  }
  if (!withScripts && /(^|\/)📜️script\.ts$/.test(rel)) {
    skipped.scripts.push(rel);
    continue;
  }
  const abs = `${ROOT}/${rel}`;
  let st;
  let text: string;
  try {
    st = statSync(abs);
    if (!st.isFile()) continue;
    text = readFileSync(abs, "utf8");
  } catch {
    skipped.gone.push(rel);
    continue;
  }
  if (text.charCodeAt(0) === 0xfeff) text = text.slice(1);
  if (!genOk && GEN_HEAD.test(text.slice(0, 600))) {
    skipped.generated.push(rel);
    continue;
  }
  const isRs = rel.endsWith(".rs");
  let plan = isRs ? planRs(text) : planTs(rel, text);
  let lexOnly = false;
  if (isRs) {
    const lx = planRsLex(text);
    if (plan.error) {
      plan = lx;
      lexOnly = true;
      lexFallback.push(rel);
    } else if (!lx.error) {
      const a = plan.removals.map((x) => x.pos).join(",");
      const b = lx.removals.map((x) => x.pos).join(",");
      if (a !== b) xcheck.push(`${rel}\t${plan.removals.length}\t${lx.removals.length}`);
    }
  }
  seen++;
  if (plan.error) {
    skipped.noparse.push(rel);
    continue;
  }
  if (plan.removals.length === 0) {
    rows.push({ path: rel, removed: 0, kept: plan.kept });
    continue;
  }
  if (now - st.mtimeMs < recentMin * 60_000) {
    skipped.recent.push(rel);
    continue;
  }
  rows.push({ path: rel, removed: plan.removals.length, kept: plan.kept });
  if (samples.length < 4000 && Math.random() < 0.05) {
    const pick = plan.removals[Math.floor(Math.random() * plan.removals.length)];
    const a = text.lastIndexOf("\n", text.lastIndexOf("\n", pick.pos - 1) - 1) + 1;
    let b = text.indexOf("\n", pick.end);
    if (b >= 0) b = text.indexOf("\n", b + 1);
    if (b < 0) b = text.length;
    samples.push({ path: rel, lang: isRs ? "rs" : "ts", text: text.slice(a, b) });
  }
  const next = applyRemovals(text, plan.removals);
  const rsSig = lexOnly ? rsLexSignature : rsSignature;
  const sigA = isRs ? rsSig(text) : tsSignature(rel, text);
  const sigB = isRs ? rsSig(next) : tsSignature(rel, next);
  const same = sigA && sigB && sigA.length === sigB.length && sigA.every((v, i) => v === sigB[i]);
  const again = isRs ? (lexOnly ? planRsLex(next) : planRs(next)) : planTs(rel, next);
  if (!same || again.removals.length !== 0 || again.error) {
    verifyFail.push(`${rel}\t${same ? "" : "signature"}\t${again.removals.length}\t${again.error ?? ""}`);
    continue;
  }
  if (mode === "apply") {
    const st2 = statSync(abs);
    if (st2.mtimeMs !== st.mtimeMs) {
      skipped.recent.push(rel);
      continue;
    }
    const bom = readFileSync(abs, "utf8").charCodeAt(0) === 0xfeff ? "﻿" : "";
    writeFileSync(abs, bom + next);
    changed.push(rel);
    let line = 1;
    let at = 0;
    for (const x of [...plan.removals].sort((p, q) => p.pos - q.pos)) {
      for (; at < x.pos; at++) if (text[at] === "\n") line++;
      appendFileSync(`${OUT}/${tag}-removed.jsonl`, JSON.stringify({ path: rel, line, text: text.slice(x.pos, x.end) }) + "\n");
    }
  }
}

const sum = (k: keyof Row) => rows.reduce((a, x) => a + (x[k] as number), 0);
const byDir: Record<string, number> = {};
for (const x of rows) if (x.removed) byDir[x.path.split("/").slice(0, 2).join("/")] = (byDir[x.path.split("/").slice(0, 2).join("/")] ?? 0) + x.removed;
const summary = {
  mode,
  filesParsed: seen,
  filesWithRemovals: rows.filter((x) => x.removed).length,
  removed: sum("removed"),
  kept: sum("kept"),
  verifyFail: verifyFail.length,
  lexFallback: lexFallback.length,
  xcheckMismatch: xcheck.length,
  skipped: Object.fromEntries(Object.entries(skipped).map(([k, v]) => [k, v.length])),
  changed: changed.length,
  byDir,
};
writeFileSync(`${OUT}/${tag}-summary.json`, JSON.stringify(summary, null, 2));
writeFileSync(`${OUT}/${tag}-rows.tsv`, rows.filter((x) => x.removed).map((x) => `${x.removed}\t${x.kept}\t${x.path}`).join("\n") + "\n");
for (const [k, v] of Object.entries(skipped)) writeFileSync(`${OUT}/${tag}-skipped-${k}.txt`, v.join("\n") + "\n");
writeFileSync(`${OUT}/${tag}-lexfallback.txt`, lexFallback.join("\n") + "\n");
writeFileSync(`${OUT}/${tag}-xcheck.txt`, xcheck.join("\n") + "\n");
writeFileSync(`${OUT}/${tag}-verifyfail.txt`, verifyFail.join("\n") + "\n");
if (mode === "apply") writeFileSync(`${OUT}/${tag}-changed.txt`, changed.join("\n") + "\n");
const pickTs = samples.filter((s) => s.lang === "ts").sort(() => Math.random() - 0.5).slice(0, Math.ceil(sampleN / 2));
const pickRs = samples.filter((s) => s.lang === "rs").sort(() => Math.random() - 0.5).slice(0, Math.floor(sampleN / 2));
writeFileSync(`${OUT}/${tag}-samples.txt`, [...pickTs, ...pickRs].map((s) => `=== ${s.path}\n${s.text}`).join("\n\n"));
console.log(JSON.stringify(summary, null, 2));
