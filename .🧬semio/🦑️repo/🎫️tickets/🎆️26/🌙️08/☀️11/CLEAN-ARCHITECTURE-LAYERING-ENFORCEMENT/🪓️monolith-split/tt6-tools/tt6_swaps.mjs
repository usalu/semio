// 🔀 Finds `expect(A).matcher(B)` sites whose HEAD form was `expect(B).matcher(A)` (swapped actual/expected) and matchers that changed (toEqual -> toMatchObject).
// usage: node tt6_swaps.mjs <list-file> [--all]   (list: repo-relative paths, one per line)
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const require = createRequire(root + "/package.json");
const ts = require("typescript");
const MATCHERS = new Set(["toBe", "toEqual", "toStrictEqual", "toContain", "toContainEqual", "toMatchObject", "toBeCloseTo", "toHaveLength", "toBeGreaterThan", "toBeLessThan", "toBeGreaterThanOrEqual", "toBeLessThanOrEqual", "toMatchInlineSnapshot", "toHaveProperty", "toBeInstanceOf"]);
const norm = (s) => s.replace(/\s+/g, "").replace(/,\)/g, ")").replace(/,\]/g, "]").replace(/,\}/g, "}");

function calls(text, file) {
  const kind = file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, kind);
  const out = [];
  const visit = (n) => {
    if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression) && MATCHERS.has(n.expression.name.text)) {
      let recv = n.expression.expression;
      while (ts.isPropertyAccessExpression(recv) && ["not", "resolves", "rejects"].includes(recv.name.text)) recv = recv.expression;
      if (ts.isCallExpression(recv) && ts.isIdentifier(recv.expression) && recv.expression.text === "expect" && recv.arguments.length === 1) {
        const line = sf.getLineAndCharacterOfPosition(n.getStart()).line + 1;
        const chain = n.expression.expression.getText().slice(recv.getText().length);
        out.push({ start: n.getStart(), end: n.getEnd(), line, chain, m: n.expression.name.text, a: norm(recv.arguments[0].getText()), b: norm(n.arguments.map((x) => x.getText()).join(",")), at: recv.arguments[0].getText(), bt: n.arguments.map((x) => x.getText()).join(", ") });
      }
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return out;
}

const files = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
let swapped = 0, changedMatcher = 0, nohead = 0, applied = 0;
const APPLY = process.argv.includes('--apply');
for (const file of files) {
  if (!existsSync(root + "/" + file)) continue;
  const head = spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "show", "HEAD:" + file], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
  if (head.status !== 0) { nohead++; console.log(`NOHEAD ${file}`); continue; }
  const now = calls(readFileSync(root + "/" + file, "utf8"), file);
  const before = calls(head.stdout, file);
  const key = (c) => `${c.chain}|${c.m}|${c.a}|${c.b}`;
  const beforeKeys = new Map();
  for (const c of before) beforeKeys.set(key(c), (beforeKeys.get(key(c)) ?? 0) + 1);
  const used = new Map();
  const unmatched = [];
  for (const c of now) {
    const k = key(c);
    if ((beforeKeys.get(k) ?? 0) > (used.get(k) ?? 0)) used.set(k, (used.get(k) ?? 0) + 1);
    else unmatched.push(c);
  }
  const unmatchedBefore = [];
  const usedB = new Map();
  const nowKeys = new Map();
  for (const c of now) nowKeys.set(key(c), (nowKeys.get(key(c)) ?? 0) + 1);
  for (const c of before) {
    const k = key(c);
    if ((nowKeys.get(k) ?? 0) > (usedB.get(k) ?? 0)) usedB.set(k, (usedB.get(k) ?? 0) + 1);
    else unmatchedBefore.push(c);
  }
  const edits = [];
  for (const c of unmatched) {
    const sw = unmatchedBefore.find((o) => !o.taken && o.a === c.b && o.b === c.a);
    if (sw) sw.taken = true;
    if (sw) { edits.push({ c, text: `expect(${c.bt})${c.chain}.${sw.m}(${c.at})` }); swapped++; console.log(`SWAP ${file}:${c.line} now expect(${c.at.slice(0, 90)}).${c.m}(${c.bt.slice(0, 90)})  was expect(${sw.at.slice(0, 90)}).${sw.m}(${sw.bt.slice(0, 90)})`); continue; }
    const sm = unmatchedBefore.find((o) => !o.taken && o.a === c.a && o.b === c.b && o.m !== c.m);
    if (sm) sm.taken = true;
    if (sm) { edits.push({ c, text: `expect(${c.at})${c.chain}.${sm.m}(${c.bt})` }); changedMatcher++; console.log(`MATCHER ${file}:${c.line} ${sm.m} -> ${c.m}  expect(${c.at.slice(0, 80)})`); continue; }
    if (process.argv.includes("--fuzzy")) {
      const toks = (s) => new Set(s.match(/[A-Za-z_$][\w$]*/g) ?? []);
      const sim = (x, y) => { const a = toks(x), b = toks(y); if (!a.size || !b.size) return 0; let i = 0; for (const k of a) if (b.has(k)) i++; return i / (a.size + b.size - i); };
      let best = null;
      for (const o of unmatchedBefore) {
        if (o.taken) continue;
        const straight = sim(c.at, o.at) + sim(c.bt, o.bt), crossed = sim(c.at, o.bt) + sim(c.bt, o.at);
        if (crossed > straight && crossed > 0.5 && (!best || crossed - straight > best.d)) best = { o, d: crossed - straight };
      }
      if (best) { console.log(`FUZZY ${file}:${c.line} now expect(${c.at.slice(0, 100)}).${c.m}(${c.bt.slice(0, 100)})  was expect(${best.o.at.slice(0, 100)}).${best.o.m}(${best.o.bt.slice(0, 100)})`); continue; }
    }
    if (process.argv.includes("--all")) console.log(`OTHER ${file}:${c.line} ${c.m} expect(${c.at.slice(0, 70)}) (${c.bt.slice(0, 70)})`);
  }
  if (process.argv.includes("--before")) for (const o of unmatchedBefore) if (!o.taken) console.log(`BEFORE ${file}:${o.line} ${o.m} expect(${o.at.slice(0, 100)}) (${o.bt.slice(0, 100)})`);
  if (APPLY && edits.length) {
    let text = readFileSync(root + "/" + file, "utf8");
    const sorted = edits.sort((x, y) => y.c.start - x.c.start);
    let last = Infinity;
    for (const e of sorted) { if (e.c.end > last) { console.log(`SKIP-OVERLAP ${file}:${e.c.line}`); continue; } text = text.slice(0, e.c.start) + e.text + text.slice(e.c.end); last = e.c.start; applied++; }
    writeFileSync(root + "/" + file, text);
  }
}
console.error(`files=${files.length} swapped=${swapped} matcherChanged=${changedMatcher} nohead=${nohead} applied=${applied}`);
