import { parseFile, sccs, ts, type Parsed, type Unit } from "./lib.ts";
import { readFileSync, writeFileSync, mkdirSync, existsSync, rmSync } from "node:fs";
import { dirname, join, relative, resolve, basename, isAbsolute } from "node:path";
import { diagnose } from "./check.ts";
import { rewriteFile, lookupFrom } from "./rewrite.ts";
import { readdirSync, statSync, copyFileSync } from "node:fs";

export interface ChunkPlan { dir: string; phase?: "pre" | "core" | "post"; parts?: string[]; leaf?: string; ranges?: [number, number][]; units?: number[]; wrapRegion?: string; }
export interface Plan {
  file: string;
  dir?: string;
  chunks: ChunkPlan[];
  pins?: Record<string, string>;
  edits?: { chunk: string; find: string; replace: string }[];
  sideEffectChunk?: string;
  allowCycles?: boolean;
  keepParentIn?: string;
  cap?: number;
  unwrapDescribe?: string;
  sourceFrom?: string;
  supportChunk?: string;
  aggregator?: boolean;
  metaDir?: { support: string };
  minSize?: number;
  anchor?: string;
  noMerge?: boolean;
}

export interface Assigned {
  parsed: Parsed;
  plan: Plan;
  unitChunk: number[];
  chunkFiles: string[];
  hoists: { unit: number; from: number; to: number }[];
  sccOf: number[];
  comps: number[][];
}

export function chunkFile(plan: Plan, c: ChunkPlan): string {
  const base = plan.dir ?? dirname(plan.file);
  return join(base, c.dir, c.leaf ?? "🟦️.ts");
}

export function unwrapDescribeText(text: string, name: string): { text: string; start: number; end: number } {
  const sf = ts.createSourceFile("x.ts", text, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
  for (const st of sf.statements) {
    if (!ts.isExpressionStatement(st) || !ts.isCallExpression(st.expression)) continue;
    const c = st.expression;
    if (!ts.isIdentifier(c.expression) || c.expression.text !== "describe" || c.arguments.length < 2) continue;
    const a0 = c.arguments[0]!, a1 = c.arguments[1]!;
    if (!ts.isStringLiteralLike(a0) || a0.text !== name || !(ts.isArrowFunction(a1) || ts.isFunctionExpression(a1)) || !ts.isBlock(a1.body)) continue;
    const body = a1.body;
    const inner = text.slice(body.getStart(sf) + 1, body.getEnd() - 1);
    const before = text.slice(0, st.getStart(sf));
    return { text: before + inner + text.slice(st.getEnd()), start: before.length, end: before.length + inner.length };
  }
  throw new Error(`describe ${name} not found`);
}

export function assign(plan0: Plan): Assigned {
  let override: string | undefined;
  let wrapRange: { start: number; end: number } | undefined;
  if (plan0.unwrapDescribe) {
    const u = unwrapDescribeText(readFileSync(plan0.file, "utf8"), plan0.unwrapDescribe);
    override = u.text;
    wrapRange = { start: u.start, end: u.end };
  }
  if (!override && plan0.sourceFrom) override = readFileSync(plan0.sourceFrom, "utf8");
  const parsed = parseFile(plan0.file, override);
  (parsed as any).wrapRange = wrapRange;
  const n = parsed.units.length;
  const cap = plan0.cap ?? 900;
  const chunkIndex = new Map<string, number>();
  plan0.chunks.forEach((c, i) => chunkIndex.set(c.dir, i));
  const phaseOf: ("pre" | "core" | "post")[] = new Array(n).fill("pre");
  if (plan0.anchor) {
    const seed = parsed.units.find((u) => u.names.has(plan0.anchor!));
    if (!seed) throw new Error(`anchor ${plan0.anchor}`);
    const cs = sccs(n, (i) => parsed.units[i].deps).find((c) => c.includes(seed.id))!;
    const coreSet = new Set(cs);
    cs.forEach((u) => (phaseOf[u] = "core"));
    const rev: number[][] = parsed.units.map(() => []);
    parsed.units.forEach((u) => u.deps.forEach((d) => rev[d].push(u.id)));
    const st = [...cs];
    while (st.length) { const x = st.pop()!; for (const y of rev[x]) if (!coreSet.has(y) && phaseOf[y] !== "post") { phaseOf[y] = "post"; st.push(y); } }
  }
  const direct: number[] = new Array(n).fill(-1);
  plan0.chunks.forEach((c, ci) => {
    for (const [a, b] of c.ranges ?? []) for (let i = a; i <= b && i < n; i++) if (direct[i] < 0 && (!c.phase || c.phase === phaseOf[i]) && !(plan0.supportChunk && !parsed.units[i].effect)) direct[i] = ci;
    for (const u of c.units ?? []) direct[u] = ci;
  });
  for (const [u, d] of Object.entries(plan0.pins ?? {})) {
    if (!chunkIndex.has(d)) throw new Error(`pin chunk ${d}`);
    direct[Number(u)] = chunkIndex.get(d)!;
  }
  if (plan0.supportChunk) {
    if (!chunkIndex.has(plan0.supportChunk)) throw new Error("supportChunk");
    const sup = chunkIndex.get(plan0.supportChunk)!;
    const users = parsed.units.map(() => new Set<number>());
    const rdeps: number[][] = parsed.units.map(() => []);
    parsed.units.forEach((u) => u.deps.forEach((d) => rdeps[d].push(u.id)));
    const effects = parsed.units.filter((u) => u.effect && direct[u.id] >= 0);
    for (const e of effects) {
      const st = [e.id], seen = new Set<number>([e.id]);
      while (st.length) { const x = st.pop()!; for (const d of parsed.units[x].deps) if (!seen.has(d)) { seen.add(d); users[d].add(direct[e.id]); st.push(d); } }
    }
    for (const u of parsed.units) {
      if (u.effect || direct[u.id] >= 0) continue;
      direct[u.id] = users[u.id].size === 1 ? [...users[u.id]][0]! : sup;
    }
  }
  const comps = sccs(n, (i) => parsed.units[i].deps);
  const sccOf: number[] = new Array(n).fill(-1);
  comps.forEach((c, ci) => c.forEach((u) => (sccOf[u] = ci)));
  const pinnedUnits = new Set(Object.keys(plan0.pins ?? {}).map(Number));
  const sccUser: number[] = comps.map((c) => {
    const pinned = c.filter((u) => pinnedUnits.has(u));
    const base = (pinned.length ? pinned : c).slice().sort((a, b) => a - b);
    for (const u of base) if (direct[u] >= 0) return direct[u];
    return -1;
  });
  const missing = comps.map((c, i) => (sccUser[i] < 0 ? c[0] : -1)).filter((i) => i >= 0);
  if (missing.length) throw new Error(`unassigned units: ${missing.slice(0, 20).join(",")} (${missing.length})`);
  const nodeLines = comps.map((c) => c.reduce((s, u) => s + parsed.units[u].lines, 0));
  const nodeMin = comps.map((c) => Math.min(...c));
  const deps = comps.map((c) => { const s = new Set<number>(); c.forEach((u) => parsed.units[u].deps.forEach((d) => { if (sccOf[d] !== sccOf[u]) s.add(sccOf[d]); })); return s; });
  const remaining: number[][] = plan0.chunks.map(() => []);
  comps.forEach((_, i) => remaining[sccUser[i]].push(i));
  remaining.forEach((r) => r.sort((x, y) => nodeMin[x] - nodeMin[y]));
  const done = new Set<number>();
  const runs: { user: number; nodes: number[]; lines: number }[] = [];
  const pushRun = (user: number, nodes: number[]) => {
    let cur: number[] = [];
    let lines = 0;
    const flush = () => { if (cur.length) runs.push({ user, nodes: cur, lines }); cur = []; lines = 0; };
    for (const x of nodes) {
      if (cur.length && lines + nodeLines[x] > cap) flush();
      cur.push(x);
      lines += nodeLines[x];
    }
    flush();
    nodes.forEach((x) => done.add(x));
  };
  let left = comps.length;
  while (left > 0) {
    let progressed = false;
    for (let ci = 0; ci < remaining.length; ci++) {
      const X = remaining[ci];
      if (!X.length) continue;
      const set = new Set(X);
      if (X.every((x) => [...deps[x]].every((d) => done.has(d) || set.has(d)))) {
        pushRun(ci, X);
        left -= X.length;
        remaining[ci] = [];
        progressed = true;
        break;
      }
    }
    if (progressed) continue;
    let bestCi = -1, bestS: number[] = [], bestLines = -1;
    for (let ci = 0; ci < remaining.length; ci++) {
      const X = remaining[ci];
      if (!X.length) continue;
      let S = new Set(X);
      let ch = true;
      while (ch) {
        ch = false;
        for (const x of S) if ([...deps[x]].some((d) => !done.has(d) && !S.has(d))) { S.delete(x); ch = true; }
      }
      const lines = [...S].reduce((acc, x) => acc + nodeLines[x], 0);
      if (S.size && lines > bestLines) { bestLines = lines; bestCi = ci; bestS = X.filter((x) => S.has(x)); }
    }
    if (bestCi < 0) throw new Error("no schedulable chunk");
    pushRun(bestCi, bestS);
    left -= bestS.length;
    const sset = new Set(bestS);
    remaining[bestCi] = remaining[bestCi].filter((x) => !sset.has(x));
  }
  type Cl = { users: Map<number, number>; nodes: number[]; lines: number; order: number };
  let cls: Cl[] = runs.map((r, i) => ({ users: new Map([[r.user, r.lines]]), nodes: r.nodes.slice(), lines: r.lines, order: i }));
  const nodeCl = new Map<number, number>();
  const rebuild = () => { nodeCl.clear(); cls.forEach((c, i) => c.nodes.forEach((x) => nodeCl.set(x, i))); };
  const edgeCounts = () => {
    const m = new Map<string, number>();
    comps.forEach((_, x) => deps[x].forEach((d) => { const a = nodeCl.get(x)!, b = nodeCl.get(d)!; if (a !== b) { const k = a < b ? `${a},${b}` : `${b},${a}`; m.set(k, (m.get(k) ?? 0) + 1); } }));
    return m;
  };
  const succ = () => {
    const sc: Set<number>[] = cls.map(() => new Set());
    comps.forEach((_, x) => deps[x].forEach((d) => { const a = nodeCl.get(x)!, b = nodeCl.get(d)!; if (a !== b) sc[a].add(b); }));
    return sc;
  };
  const reach = (sc: Set<number>[], from: number, to: number, skipDirect: boolean) => {
    const seen = new Set<number>();
    const st: number[] = [];
    for (const y of sc[from]) { if (skipDirect && y === to) continue; st.push(y); }
    while (st.length) { const y = st.pop()!; if (y === to) return true; if (seen.has(y)) continue; seen.add(y); sc[y].forEach((z) => st.push(z)); }
    return false;
  };
  const mergeLoop = (accept: (a: Cl, b: Cl, edges: number) => number) => {
    for (;;) {
      rebuild();
      const ec = edgeCounts();
      const sc = succ();
      let best = -1, ba = -1, bb = -1;
      for (let i = 0; i < cls.length; i++) for (let j = i + 1; j < cls.length; j++) {
        const e = ec.get(`${i},${j}`) ?? 0;
        const score = accept(cls[i], cls[j], e);
        if (score <= best) continue;
        if (cls[i].lines + cls[j].lines > cap) continue;
        if (reach(sc, i, j, true) || reach(sc, j, i, true)) continue;
        best = score; ba = i; bb = j;
      }
      if (best <= 0) break;
      const A = cls[ba], B = cls[bb];
      const users = new Map(A.users);
      B.users.forEach((v, k) => users.set(k, (users.get(k) ?? 0) + v));
      cls[ba] = { users, nodes: [...A.nodes, ...B.nodes], lines: A.lines + B.lines, order: Math.min(A.order, B.order) };
      cls.splice(bb, 1);
    }
  };
  if (!plan0.noMerge) mergeLoop((a, b, e) => { const shared = [...a.users.keys()].some((k) => b.users.has(k)); return shared ? 1000 + e : 0; });
  const minSize = plan0.minSize ?? 0;
  if (!plan0.noMerge && minSize > 0) mergeLoop((a, b, e) => (e > 0 && (a.lines < minSize || b.lines < minSize) ? 10 + e : 0));
  rebuild();
  const sc2 = succ();
  const indeg2 = cls.map(() => 0);
  sc2.forEach((set) => set.forEach((d) => indeg2[d]++));
  void indeg2;
  const orderIdx: number[] = [];
  const placed = new Set<number>();
  const rsucc: Set<number>[] = cls.map(() => new Set());
  sc2.forEach((set, a) => set.forEach((d) => rsucc[a].add(d)));
  while (orderIdx.length < cls.length) {
    let pick = -1;
    for (let i = 0; i < cls.length; i++) {
      if (placed.has(i)) continue;
      if ([...sc2[i]].every((d) => placed.has(d))) { if (pick < 0 || cls[i].order < cls[pick].order) pick = i; }
    }
    if (pick < 0) throw new Error("cluster cycle");
    placed.add(pick);
    orderIdx.push(pick);
  }
  cls = orderIdx.map((i) => cls[i]);
  rebuild();
  const seen = new Map<number, number>();
  const resolved: ChunkPlan[] = cls.map((c) => {
    let user = -1, ul = -1;
    c.users.forEach((v, k) => { if (v > ul) { ul = v; user = k; } });
    const k = seen.get(user) ?? 0;
    seen.set(user, k + 1);
    const base = plan0.chunks[user];
    const names = base.parts ?? [];
    const dir = k === 0 ? base.dir : names[k - 1] ?? `${base.dir}~part${k + 1}`;
    return { dir, leaf: base.leaf };
  });
  {
    let progress = true;
    const skipped = new Set<string>();
    while (progress) {
      progress = false;
      const first = new Map<string, number>();
      for (let i = 0; i < resolved.length && !progress; i++) {
        const nm = resolved[i].dir;
        if (!first.has(nm)) { first.set(nm, i); continue; }
        const t = first.get(nm)!;
        rebuild();
        const sc = succ();
        if (reach(sc, t, i, true) || reach(sc, i, t, true) || cls[t].lines + cls[i].lines > cap + 400) {
          const alt = `${nm}~x${i}`;
          console.error(`merge infeasible: ${nm} (cluster ${i}: ${cls[i].nodes.slice(0, 3).map((x) => [...parsed.units[comps[x][0]].names][0]).join(",")}) -> kept as ${alt}`);
          resolved[i] = { ...resolved[i], dir: alt };
          skipped.add(alt);
          progress = true;
          break;
        }
        cls[t] = { users: cls[t].users, nodes: [...cls[t].nodes, ...cls[i].nodes], lines: cls[t].lines + cls[i].lines, order: Math.min(cls[t].order, cls[i].order) };
        cls.splice(i, 1);
        resolved.splice(i, 1);
        progress = true;
      }
    }
    rebuild();
    const s3 = succ();
    const placed2 = new Set<number>();
    const ord: number[] = [];
    while (ord.length < cls.length) {
      let pick = -1;
      for (let i = 0; i < cls.length; i++) {
        if (placed2.has(i)) continue;
        if ([...s3[i]].every((d) => placed2.has(d))) { if (pick < 0 || cls[i].order < cls[pick].order) pick = i; }
      }
      if (pick < 0) throw new Error("cycle after same-dir merge");
      placed2.add(pick);
      ord.push(pick);
    }
    const c2 = ord.map((i) => cls[i]);
    const r2 = ord.map((i) => resolved[i]);
    cls = c2;
    resolved.length = 0;
    resolved.push(...r2);
    rebuild();
  }
  const unitChunk: number[] = new Array(n).fill(-1);
  cls.forEach((c, ri) => c.nodes.forEach((x) => comps[x].forEach((u) => (unitChunk[u] = ri))));
  if (plan0.aggregator) { const k = plan0.chunks.find((c) => c.dir === ".")!; resolved.push({ dir: ".", leaf: k.leaf }); }
  const plan: Plan = { ...plan0, chunks: resolved };
  const hoists: Assigned["hoists"] = [];
  return { parsed, plan, unitChunk, chunkFiles: resolved.map((c) => chunkFile(plan, c)), hoists, sccOf, comps };
}

export function report(a: Assigned) {
  const { parsed, plan, unitChunk } = a;
  const lines = plan.chunks.map(() => 0), counts = plan.chunks.map(() => 0);
  parsed.units.forEach((u) => { lines[unitChunk[u.id]] += u.lines; counts[unitChunk[u.id]]++; });
  plan.chunks.forEach((c, i) => {
    const ids = parsed.units.filter((u) => unitChunk[u.id] === i);
    const first = ids.slice(0, 3).map((u) => [...u.names][0] ?? u.kind).join(",");
    console.log(`${String(i).padStart(2)} ${String(lines[i]).padStart(5)}L ${String(counts[i]).padStart(3)}u ${c.dir.padEnd(34)} ${process.env.CONTENTS ? ids.map((u) => [...u.names][0] ?? u.kind).join(" ") : first}`);
  });
  if (a.hoists.length) {
    console.log(`hoists=${a.hoists.length}`);
    const by = new Map<string, number>();
    for (const h of a.hoists) { const k = `${plan.chunks[h.from].dir} -> ${plan.chunks[h.to].dir}`; by.set(k, (by.get(k) ?? 0) + 1); }
    for (const [k, v] of by) console.log(`  ${v}\t${k}`);
  }
}

function pureTypeDecl(p: Parsed, u: Unit): boolean {
  return u.stmts.every((s) => { const k = p.stmtInfo[s].kind; return k === "InterfaceDeclaration" || k === "TypeAliasDeclaration"; });
}

function stripRegions(text: string): string {
  return text.split("\n").filter((l) => !/^\s*\/\/#(end)?region\b/.test(l)).join("\n");
}

function chunkOf(_abs: string, a: Assigned, u: Unit): string {
  return a.chunkFiles[a.unitChunk[u.id]];
}

function unitText(a: Assigned, u: Unit, needExport: boolean, keepIndent = false): string {
  const { parsed } = a;
  const { text, sf } = parsed;
  const parts: string[] = [];
  for (const s of u.stmts) {
    const node = parsed.stmtInfo[s].node;
    const full = node.getFullStart();
    const leading = ts.getLeadingCommentRanges(text, full) ?? [];
    const kept = leading.filter((r) => !/^\/\/#(end)?region\b/.test(text.slice(r.pos, r.end)));
    const startPos = kept.length ? kept[0].pos : node.getStart(sf);
    const trailing = ts.getTrailingCommentRanges(text, node.getEnd()) ?? [];
    const endPos = trailing.length ? trailing[trailing.length - 1].end : node.getEnd();
    let body = text.slice(startPos, endPos);
    const spots: { start: number; end: number; text: string }[] = [];
    const scan = (n: import("typescript").Node) => {
      if (a.plan.metaDir && ts.isPropertyAccessExpression(n) && ts.isMetaProperty(n.expression) && n.expression.keywordToken === ts.SyntaxKind.ImportKeyword && (n.name.text === "dir" || n.name.text === "url")) {
        const usage = ((a as any).metaUsage ??= new Map<string, Set<string>>()) as Map<string, Set<string>>;
        const set = usage.get(chunkOf("", a, u)) ?? usage.set(chunkOf("", a, u), new Set()).get(chunkOf("", a, u))!;
        set.add(n.name.text);
        spots.push({ start: n.getStart(sf) - startPos, end: n.getEnd() - startPos, text: n.name.text === "dir" ? "contractRoot" : "contractUrl" });
        return;
      }
      if (ts.isCallExpression(n) && n.arguments.length >= 1 && ts.isStringLiteralLike(n.arguments[0]!) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require")) && n.arguments[0]!.text.startsWith(".")) {
        const lit = n.arguments[0]!;
        const abs = resolve(dirname(a.plan.file), lit.text);
        const q = lit.getText(sf)[0];
        spots.push({ start: lit.getStart(sf) - startPos, end: lit.getEnd() - startPos, text: q + relSpec(chunkOf(abs, a, u), abs) + q });
      }
      ts.forEachChild(n, scan);
    };
    scan(node);
    spots.sort((x, y) => y.start - x.start).forEach((sp) => { body = body.slice(0, sp.start) + sp.text + body.slice(sp.end); });
    if (needExport && !parsed.stmtInfo[s].exported) {
      const rel = node.getStart(sf) - startPos;
      body = body.slice(0, rel) + "export " + body.slice(rel);
    }
    const stmtStart = node.getStart(sf);
    const lineStart = text.lastIndexOf("\n", stmtStart - 1) + 1;
    const indent = /^[ \t]*$/.test(text.slice(lineStart, stmtStart)) ? text.slice(lineStart, stmtStart) : "";
    const leadLength = stmtStart - startPos;
    let cleaned = (stripRegions(body.slice(0, leadLength)) + body.slice(leadLength)).replace(/^\s*\n+/, "").replace(/\s+$/, "");
    if (indent) {
      if (keepIndent) cleaned = indent + cleaned;
      else cleaned = cleaned.split("\n").map((l, i) => (i > 0 && l.startsWith(indent) ? l.slice(indent.length) : l)).join("\n");
    }
    parts.push(cleaned);
  }
  return parts.join("\n\n");
}

function relSpec(fromFile: string, toAbs: string): string {
  let r = relative(dirname(fromFile), toAbs).split("\\").join("/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
}

export interface Emitted { file: string; text: string; chunk: number; exportsNeeded: Set<string>; }

export function emit(a: Assigned): Emitted[] {
  const { parsed, plan, unitChunk } = a;
  const nChunks = plan.chunks.length;
  const needExportUnit = new Set<number>();
  const crossNames: Map<number, Map<number, Set<string>>> = new Map();
  for (const u of parsed.units) {
    const cu = unitChunk[u.id];
    for (const [dep, names] of u.refNames) {
      const cd = unitChunk[dep];
      if (cd === cu) continue;
      needExportUnit.add(dep);
      const byChunk = crossNames.get(cu) ?? crossNames.set(cu, new Map()).get(cu)!;
      const set = byChunk.get(cd) ?? byChunk.set(cd, new Set()).get(cd)!;
      names.forEach((nm) => set.add(nm));
    }
  }
  const nameKind = new Map<string, "type" | "value">();
  for (const u of parsed.units) for (const nm of u.names) nameKind.set(nm, pureTypeDecl(parsed, u) ? "type" : "value");
  const out: Emitted[] = [];
  for (let ci = 0; ci < nChunks; ci++) {
    const file = a.chunkFiles[ci];
    const units = parsed.units.filter((u) => unitChunk[u.id] === ci).sort((x, y) => x.id - y.id);
    if (plan.aggregator && plan.chunks[ci].dir === ".") {
      const header = parsed.text.match(/\/\/#region 🧲️Header[\s\S]*?\/\/#endregion 🧲️Header\n/u)?.[0] ?? "";
      const sideEffects = parsed.importStmts.filter((st) => !st.importClause).map((st) => {
        const spec = (st.moduleSpecifier as import("typescript").StringLiteral).text;
        return `import ${JSON.stringify(spec)};`;
      });
      const lines0 = sideEffects;
      const lines = [...lines0, ...plan.chunks.map((c, j) => ({ c, j })).filter(({ c, j }) => c.dir !== "." && parsed.units.some((u) => unitChunk[u.id] === j && u.effect)).map(({ j }) => `import ${JSON.stringify(relSpec(file, a.chunkFiles[j]))};`)];
      out.push({ file, text: header + lines.join("\n") + "\n", chunk: ci, exportsNeeded: new Set() });
      continue;
    }
    if (!units.length) { out.push({ file, text: "", chunk: ci, exportsNeeded: new Set() }); continue; }
    let bodies: string[];
    const wr = (parsed as any).wrapRange as { start: number; end: number } | undefined;
    if (wr && plan.unwrapDescribe) {
      const inside = (u: Unit) => u.effect && u.start >= wr.start && u.start < wr.end;
      const helpers = units.filter((u) => !inside(u) && !u.effect), wrapped = units.filter(inside), tail = units.filter((u) => u.effect && !inside(u));
      bodies = helpers.map((u) => unitText(a, u, needExportUnit.has(u.id)));
      if (wrapped.length) bodies.push(`describe(${JSON.stringify(plan.unwrapDescribe)}, () => {\n${wrapped.map((u) => unitText(a, u, false, true)).join("\n\n")}\n});`);
      bodies.push(...tail.map((u) => unitText(a, u, false)));
    } else bodies = units.map((u) => unitText(a, u, needExportUnit.has(u.id)));
    const importLines: string[] = [];
    const bySpec = new Map<string, { clauseType: boolean; default?: string; ns?: string; named: string[] }>();
    const used = new Set<string>();
    units.forEach((u) => u.usesImportLocals.forEach((l) => used.add(l)));
    if (wr && plan.unwrapDescribe && units.some((u) => u.effect && u.start >= wr.start && u.start < wr.end)) used.add("describe");
    const keyOf = (spec: string, typeOnly: boolean) => `${typeOnly ? "T" : "V"}\u0000${spec}`;
    const order: string[] = [];
    const stmtOrder = parsed.importStmts.map((st) => parsed.sf.statements.indexOf(st));
    for (const b of [...parsed.bindings.values()].sort((x, y) => x.stmt - y.stmt)) {
      if (!used.has(b.local)) continue;
      const stmt = parsed.sf.statements[b.stmt] as import("typescript").ImportDeclaration;
      const clauseType = !!stmt.importClause?.isTypeOnly;
      const abs = b.spec.startsWith(".") ? resolve(dirname(plan.file), b.spec) : undefined;
      const spec = abs ? relSpec(file, abs) : b.spec;
      const key = keyOf(spec, clauseType);
      if (!bySpec.has(key)) { bySpec.set(key, { clauseType, named: [] }); order.push(key); }
      const e = bySpec.get(key)!;
      if (b.kind === "default") e.default = b.local;
      else if (b.kind === "namespace") e.ns = b.local;
      else e.named.push((b.typeOnly && !clauseType ? "type " : "") + (b.imported === b.local ? b.local : `${b.imported} as ${b.local}`));
    }
    void stmtOrder;
    for (const key of order) {
      const spec = key.split("\u0000")[1];
      const e = bySpec.get(key)!;
      const head = e.clauseType ? "import type " : "import ";
      const bits: string[] = [];
      if (e.default) bits.push(e.default);
      if (e.ns) bits.push(`* as ${e.ns}`);
      if (e.named.length) bits.push(`{ ${e.named.join(", ")} }`);
      if (e.ns && e.named.length) {
        importLines.push(`${head}${[e.default, `* as ${e.ns}`].filter(Boolean).join(", ")} from ${JSON.stringify(spec)};`);
        importLines.push(`${head}{ ${e.named.join(", ")} } from ${JSON.stringify(spec)};`);
      } else importLines.push(`${head}${bits.join(", ")} from ${JSON.stringify(spec)};`);
    }
    if (plan.sideEffectChunk === plan.chunks[ci].dir) {
      for (const st of parsed.importStmts) if (!st.importClause) {
        const spec = (st.moduleSpecifier as import("typescript").StringLiteral).text;
        const abs = spec.startsWith(".") ? resolve(dirname(plan.file), spec) : undefined;
        importLines.unshift(`import ${JSON.stringify(abs ? relSpec(file, abs) : spec)};`);
      }
    }
    const cross = crossNames.get(ci);
    if (cross) {
      for (const cd of [...cross.keys()].sort((x, y) => x - y)) {
        const names = [...cross.get(cd)!].sort();
        const allType = names.every((nm) => nameKind.get(nm) === "type");
        const spec = relSpec(file, a.chunkFiles[cd]);
        const items = names.map((nm) => (!allType && nameKind.get(nm) === "type" ? `type ${nm}` : nm));
        importLines.push(`import ${allType ? "type " : ""}{ ${items.join(", ")} } from ${JSON.stringify(spec)};`);
      }
    }
    const metaUse = ((a as any).metaUsage as Map<string, Set<string>> | undefined)?.get(file);
    if (plan.metaDir && metaUse && plan.chunks[ci].dir !== plan.metaDir.support) {
      const sup = plan.chunks.findIndex((c) => c.dir === plan.metaDir!.support);
      importLines.push(`import { ${["contractRoot", "contractUrl"].filter((n) => metaUse.has(n === "contractRoot" ? "dir" : "url")).join(", ")} } from ${JSON.stringify(relSpec(file, a.chunkFiles[sup]))};`);
    }
    if (plan.metaDir && plan.chunks[ci].dir === plan.metaDir.support) {
      importLines.push('import * as contractPath from "node:path";', 'import * as contractUrlModule from "node:url";');
      bodies.unshift('export const contractRoot = contractPath.resolve(import.meta.dir, "..");\nexport const contractUrl = contractUrlModule.pathToFileURL(contractPath.join(contractRoot, "🟦️.ts")).href;');
    }
    let textOut = (a.chunkFiles[ci] === plan.file && parsed.text.startsWith("#!") ? parsed.text.slice(0, parsed.text.indexOf("\n") + 1) : "") + (a.chunkFiles[ci] === plan.file ? (parsed.text.match(/\/\/#region 🧲️Header[\s\S]*?\/\/#endregion 🧲️Header\n/u)?.[0] ?? "") : "") + (importLines.length ? importLines.join("\n") + "\n\n" : "") + bodies.join("\n\n") + "\n";
    for (const ed of plan.edits ?? []) {
      if (ed.chunk !== plan.chunks[ci].dir) continue;
      const idx = textOut.indexOf(ed.find);
      if (idx < 0) throw new Error(`edit not found in ${ed.chunk}: ${ed.find.slice(0, 60)}`);
      if (textOut.indexOf(ed.find, idx + 1) >= 0) throw new Error(`edit ambiguous in ${ed.chunk}: ${ed.find.slice(0, 60)}`);
      textOut = textOut.replace(ed.find, () => ed.replace);
    }
    const exportsNeeded = new Set<string>();
    units.forEach((u) => { if (needExportUnit.has(u.id)) u.names.forEach((nm) => exportsNeeded.add(nm)); });
    out.push({ file, text: textOut, chunk: ci, exportsNeeded });
  }
  return out;
}

export function exportTable(a: Assigned): Map<string, { chunk: number } | { reexport: { spec: string; imported: string; typeOnly: boolean } }> {
  const { parsed, unitChunk } = a;
  const table = new Map<string, any>();
  for (const u of parsed.units) {
    if (!u.exported && !parsed.localExportLists.length) {
      continue;
    }
    for (const s of u.stmts) {
      if (!parsed.stmtInfo[s].exported) continue;
      for (const nm of parsed.stmtInfo[s].names) table.set(nm, { chunk: unitChunk[u.id] });
    }
  }
  for (const ex of parsed.reexports) {
    const spec = (ex.moduleSpecifier as import("typescript").StringLiteral).text;
    const clauseType = ex.isTypeOnly;
    if (!ex.exportClause) throw new Error("export * from in parent");
    if (!ts.isNamedExports(ex.exportClause)) throw new Error("export * as ns");
    for (const el of ex.exportClause.elements) table.set(el.name.text, { reexport: { spec, imported: (el.propertyName ?? el.name).text, typeOnly: clauseType || el.isTypeOnly } });
  }
  for (const ex of parsed.localExportLists) {
    if (!ex.exportClause || !ts.isNamedExports(ex.exportClause)) continue;
    for (const el of ex.exportClause.elements) {
      const local = (el.propertyName ?? el.name).text;
      const b = parsed.bindings.get(local);
      if (b) table.set(el.name.text, { reexport: { spec: b.spec, imported: b.imported, typeOnly: b.typeOnly || ex.isTypeOnly || el.isTypeOnly } });
      else {
        const u = parsed.units.find((u) => u.names.has(local));
        if (!u) throw new Error(`local export ${local} not found`);
        table.set(el.name.text, { chunk: unitChunk[u.id], localAlias: local });
      }
    }
  }
  return table;
}

export function checkAcyclic(a: Assigned): string[] {
  const { parsed, unitChunk, plan } = a;
  const n = plan.chunks.length;
  const adj: Set<number>[] = Array.from({ length: n }, () => new Set());
  for (const u of parsed.units) u.deps.forEach((d) => { if (unitChunk[d] !== unitChunk[u.id]) adj[unitChunk[u.id]].add(unitChunk[d]); });
  const cs = sccs(n, (i) => adj[i]).filter((c) => c.length > 1);
  return cs.map((c) => c.map((i) => plan.chunks[i].dir).join(" <-> "));
}

if (import.meta.main) {
  const cmd = process.argv[2];
  const plan: Plan = JSON.parse(readFileSync(process.argv[3], "utf8"));
  const a = assign(plan);
  if (cmd === "report") {
    report(a);
    const cyc = checkAcyclic(a);
    console.log(cyc.length ? `CYCLES: ${cyc.join(" | ")}` : "chunk graph acyclic");
  } else if (cmd === "apply" || cmd === "check") {
    const sitesFile = process.argv[4];
    const em = emit(a);
    const bad = em.filter((e) => !e.text.trim());
    if (bad.length) throw new Error(`empty chunks: ${bad.map((e) => e.file).join(",")}`);
    const REPO = "/Users/ueli/Documents/semio";
    const seg = (name: string) => leadingEmoji(name);
    const seen = new Map<string, string>();
    const parentDir = plan.dir ?? dirname(plan.file);
    for (const c of a.plan.chunks) {
      if (c.dir === ".") continue;
      const e = seg(c.dir);
      if (!e) throw new Error(`chunk dir without emoji ${c.dir}`);
      if (seen.has(e)) throw new Error(`emoji clash ${c.dir} vs ${seen.get(e)}`);
      seen.set(e, c.dir);
    }
    if (existsSync(parentDir)) for (const nm of readdirSync(parentDir)) {
      if (nm.startsWith(".")) continue;
      const e = seg(nm);
      if (seen.has(e) && !a.plan.chunks.some((c) => c.dir === nm)) throw new Error(`emoji clash with existing sibling ${nm} vs ${seen.get(e)}`);
    }
    for (const e of em) {
      const rel = relative(REPO, e.file);
      if (Buffer.byteLength(rel) > 238) throw new Error(`path too long (${Buffer.byteLength(rel)}): ${rel}`);
      if (rel.length > 250) throw new Error(`path chars too long: ${rel}`);
      if (cmd === "apply" && existsSync(e.file) && e.file !== plan.file) throw new Error(`exists: ${e.file}`);
    }
    const overlay = new Map(em.map((e) => [e.file, e.text] as [string, string]));
    const hidden = new Set([plan.file]);
    const base = diagnose([plan.file], new Set());
    const ignore = new Set(base.map((d) => d.msg));
    const res = diagnose(em.map((e) => e.file), ignore, overlay, hidden);
    for (const d of res.slice(0, 100)) console.log(`${relative(REPO, d.file)}:${d.line} TS${d.code} ${d.msg}`);
    console.log(`virtual check: ${res.length} diagnostics`);
    if (res.length) process.exit(1);
    if (cmd === "check") process.exit(0);
    const backup = join("/Users/ueli/.cache/c6repo2/backup", basename(dirname(plan.file)) + "-" + Date.now());
    const save = (f: string) => { const d = join(backup, relative(REPO, f)); mkdirSync(dirname(d), { recursive: true }); copyFileSync(f, d); };
    save(plan.file);
    console.log(`backup: ${backup}`);
    for (const e of em) {
      mkdirSync(dirname(e.file), { recursive: true });
      writeFileSync(e.file, e.text);
    }
    console.log(`wrote ${em.length} chunk files`);
    const table = exportTable(a);
    const parents = new Map([[plan.file, lookupFrom(table, plan.file, a.chunkFiles)]]);
    const sites: any[] = JSON.parse(readFileSync(sitesFile, "utf8"));
    const files = [...new Set(sites.map((s) => s.file as string))].filter((f) => f !== plan.file && !a.chunkFiles.includes(f));
    const manual: string[] = [];
    let edited = 0;
    for (const f of files) {
      save(f);
      const r = rewriteFile(f, parents, true);
      if (r.edits) edited++;
      manual.push(...r.manual);
    }
    console.log(`importers edited: ${edited}/${files.length}; manual: ${manual.length}`);
    if (plan.keepParentIn === undefined) rmSync(plan.file);
    writeFileSync(process.argv[5] ?? "/dev/null", manual.join("\n") + "\n");
  } else if (cmd === "emit") {
    const outDir = process.argv[4];
    const em = emit(a);
    mkdirSync(outDir, { recursive: true });
    for (const e of em) {
      const p = join(outDir, relative(dirname(plan.file), e.file));
      mkdirSync(dirname(p), { recursive: true });
      writeFileSync(p, e.text);
    }
    console.log("emitted", em.length);
  }
}

export function leadingEmoji(name: string): string {
  const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
  const first = [...seg.segment(name)][0]?.segment ?? "";
  return first.replace(/️/g, "");
}
