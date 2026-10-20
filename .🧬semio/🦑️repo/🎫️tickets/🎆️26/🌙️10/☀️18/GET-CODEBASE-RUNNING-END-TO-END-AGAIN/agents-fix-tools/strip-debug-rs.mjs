const MACROS = new Set(["println", "eprintln", "print", "eprint", "dbg"]);
export function rustTokens(s) {
  const t = [];
  let i = 0;
  const n = s.length;
  while (i < n) {
    const c = s[i];
    if (/\s/.test(c)) { i++; continue; }
    if (c === "/" && s[i + 1] === "/") { const e = s.indexOf("\n", i); const end = e < 0 ? n : e; t.push({ k: "comment", s: i, e: end }); i = end; continue; }
    if (c === "/" && s[i + 1] === "*") { let depth = 1, j = i + 2; while (j < n && depth) { if (s[j] === "/" && s[j + 1] === "*") { depth++; j += 2; } else if (s[j] === "*" && s[j + 1] === "/") { depth--; j += 2; } else j++; } t.push({ k: "comment", s: i, e: j }); i = j; continue; }
    const raw = /^(b?r)(#*)"/.exec(s.slice(i, i + 12));
    if (raw) { const close = '"' + raw[2]; const e = s.indexOf(close, i + raw[0].length); const end = e < 0 ? n : e + close.length; t.push({ k: "str", s: i, e: end }); i = end; continue; }
    if (c === '"' || (c === "b" && s[i + 1] === '"')) { let j = i + (c === "b" ? 2 : 1); while (j < n && s[j] !== '"') j += s[j] === "\\" ? 2 : 1; t.push({ k: "str", s: i, e: j + 1 }); i = j + 1; continue; }
    if (c === "'" || (c === "b" && s[i + 1] === "'")) {
      const o = c === "b" ? i + 1 : i;
      const m = /^'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^\\'])'/u.exec(s.slice(o, o + 16));
      if (m) { t.push({ k: "char", s: i, e: o + m[0].length }); i = o + m[0].length; continue; }
      const l = /^'[A-Za-z_][A-Za-z0-9_]*/.exec(s.slice(i, i + 40));
      if (l) { t.push({ k: "life", s: i, e: i + l[0].length }); i += l[0].length; continue; }
    }
    const id = /^[A-Za-z_][A-Za-z0-9_]*/.exec(s.slice(i, i + 80));
    if (id) { t.push({ k: "id", s: i, e: i + id[0].length }); i += id[0].length; continue; }
    t.push({ k: "p", s: i, e: i + 1 }); i++;
  }
  return t;
}
export function stripDebugRs(text, name, notes) {
  const tk = rustTokens(text).filter((x) => x.k !== "comment");
  const txt = (x) => text.slice(x.s, x.e);
  const pair = new Map();
  const stack = [];
  tk.forEach((x, idx) => {
    if (x.k !== "p") return;
    const c = txt(x);
    if ("([{".includes(c)) stack.push(idx);
    else if (")]}".includes(c)) { const o = stack.pop(); pair.set(o, idx); pair.set(idx, o); }
  });
  const line = (pos) => text.slice(0, pos).split("\n").length;
  const cuts = [];
  tk.forEach((x, idx) => {
    if (x.k !== "str" || !txt(x).includes("[DEBUG]")) return;
    let k = idx, open = -1;
    const chain = [];
    let cur = idx;
    for (;;) {
      let depth = 0, j = cur - 1;
      while (j >= 0) { const c = tk[j].k === "p" ? txt(tk[j]) : ""; if (")]}".includes(c) && c) depth++; else if ("([{".includes(c) && c) { if (depth === 0) break; depth--; } j--; }
      if (j < 0) break;
      chain.push(j);
      if (txt(tk[j]) === "(" && tk[j - 1] && txt(tk[j - 1]) === "!" && tk[j - 2] && tk[j - 2].k === "id") { open = j; break; }
      cur = j;
    }
    if (open < 0) { notes.push("NO-MACRO at line " + line(x.s)); return; }
    const name = txt(tk[open - 2]);
    if (!MACROS.has(name)) { notes.push("OTHER-MACRO " + name + "! at line " + line(x.s)); return; }
    let start = open - 2;
    while (start >= 2 && txt(tk[start - 1]) === ":" && txt(tk[start - 2]) === ":") start -= 3;
    let end = pair.get(open);
    const semi = tk[end + 1] && txt(tk[end + 1]) === ";" ? end + 1 : end;
    cuts.push({ start, end: semi, hadSemi: semi !== end, tk, line: line(tk[start].s) });
  });
  const prevTok = (c) => tk[c.start - 1] && txt(tk[c.start - 1]);
  const nextTok = (c) => tk[c.end + 1] && txt(tk[c.end + 1]);
  const ranges = [];
  for (const c of cuts) {
    let s = tk[c.start].s, e = tk[c.end].e;
    const p = prevTok(c), nx = nextTok(c);
    if (c.hadSemi === false) {
      if (p === "=>") { notes.push("MATCH-ARM-EXPR at line " + c.line); ranges.push([s, e, "()"]); continue; }
      if (nx === "}" && p === "{") { notes.push("TAIL-ONLY-BLOCK line " + c.line); }
      else if (nx === "}") { notes.push("TAIL-EXPR line " + c.line + " prev=" + p); }
      else notes.push("NO-SEMI line " + c.line + " next=" + nx);
    }
    if (p === "{" && (nx === "}" || (c.hadSemi && nx === "}"))) notes.push("EMPTY-BLOCK-AFTER at line " + c.line + " (block head: " + text.slice(Math.max(0, tk[c.start - 1].s - 60), tk[c.start - 1].s).replace(/\n/g, " ") + ")");
    ranges.push([s, e, ""]);
  }
  ranges.sort((a, b) => a[0] - b[0]);
  let out = "", pos = 0;
  for (const [s, e, rep] of ranges) {
    const ls = text.lastIndexOf("\n", s - 1) + 1;
    const le = text.indexOf("\n", e);
    const before = text.slice(ls, s), after = text.slice(e, le < 0 ? text.length : le);
    if (rep === "" && /^\s*$/.test(before) && /^\s*$/.test(after)) { out += text.slice(pos, ls); pos = le < 0 ? text.length : le + 1; }
    else { out += text.slice(pos, s) + rep; pos = e; }
  }
  return out + text.slice(pos);
}
