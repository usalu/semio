/** 🎭️ Makes every docstring of a TS/MJS/MTS file start with an emoji that no other docstring of the file starts with: bun cp-emoji.ts <list of "workpath<TAB>relname"> <report json>. Edits the work files in place; keeps the first docstring of every emoji. */
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
const TS_ROOT = "/Users/ueli/Documents/semio";
const ROOT = process.env.CP_ROOT ?? TS_ROOT;
const ts = createRequire(`${TS_ROOT}/package.json`)("typescript") as typeof import("typescript");
const LEAD = /^(?:\p{Extended_Pictographic}(?:️|\p{Emoji_Modifier})?(?:‍\p{Extended_Pictographic}️?)*|\p{Regional_Indicator}{2}|[0-9#*]️?⃣|\p{So}|\p{Sm})/u;
const strip = (s: string): string => s.replace(/️/gu, "");

const TABLE: readonly (readonly [RegExp, readonly string[]])[] = [
  [/\b(verif|check|validat|prove|assert|audit|confirm|ensure)/iu, ["✅", "🔍", "🔎", "🧐", "🕵", "✔", "🧪"]],
  [/\b(read|reads|reading|load|loads|loading|parse|parses|decode|decodes|fetch)/iu, ["📖", "📚", "🔖", "📜", "🗞", "📰", "📃"]],
  [/\b(writ|record|records|persist|save|saves|store|stores|stamp)/iu, ["✍", "📝", "🖋", "🗒", "🖊", "📒", "🧾"]],
  [/\b(list|lists|enumerat|collect|gather|inventor)/iu, ["📋", "🗂", "🧺", "📑", "🗃", "📇"]],
  [/\b(remov|delet|drop|drops|dropp|prun|clear|sweep|purge|discard|release|retire|evict|clean)/iu, ["🧹", "🗑", "🧽", "✂", "🪓", "🧼", "🪣"]],
  [/\b(publish|emit|send|post|export|report|announce|notice)/iu, ["📤", "📣", "📡", "📢", "📮", "📨"]],
  [/\b(wait|await|block|sleep|delay|retry|retries|backoff|queue|yield|grace)/iu, ["⏳", "⌛", "💤", "🐢", "🚏", "🪑"]],
  [/\b(hash|digest|fingerprint|checksum|key|keys|identity|identif|name|names|label)/iu, ["🔑", "🧬", "🪪", "🗝", "🏷", "📛", "🆔"]],
  [/\b(lock|lease|leases|custody|mutex|exclusive|claim|owner|fence)/iu, ["🔒", "🔐", "🛡", "🚧", "🪤", "🧱", "🚦"]],
  [/\b(time|clock|timestamp|deadline|timeout|millisecond|second|interval|poll|age|aged|boot|uptime)/iu, ["⏱", "🕰", "⏲", "📅", "🕐", "🗓", "⌚"]],
  [/\b(memory|rss|heap|byte|bytes|budget|capacity|size|limit|bound|ceiling|reservation|grow)/iu, ["🧠", "📏", "🧮", "💾", "📦", "🪣", "🏋"]],
  [/\b(path|paths|dir|directory|directories|folder|file|files|tree|walk|walks|root)/iu, ["📁", "📂", "🌳", "🚶", "🛤", "🧭", "🗺"]],
  [/\b(process|processes|spawn|child|children|pid|worker|thread|job|jobs|run|runs)/iu, ["🧵", "🏃", "⚙", "👷", "🚀", "🛠", "🧑"]],
  [/\b(cache|memo|retain|retains|reuse|reuses|reused|keep|keeps|share|shares|shared)/iu, ["🧊", "🗃", "🧳", "♻", "🏺", "🤝", "🪢"]],
  [/\b(error|fail|fails|failed|failure|refus|reject|throw|stale|abort|cancel|trap|crash|broken)/iu, ["🚫", "🛑", "⚠", "🧯", "❌", "🙅", "🔥"]],
  [/\b(map|table|index|row|rows|schema|json|document|documents|record|format|catalog)/iu, ["🗺", "🧾", "📊", "🗄", "🪧", "📰", "🧮"]],
  [/\b(graph|node|nodes|edge|edges|dependency|dependencies|closure|project|projects|inference|infer)/iu, ["🕸", "🌐", "🔗", "🧩", "🧱", "🪡", "🗺"]],
  [/\b(stream|pipe|channel|port|socket|event|events|watch|watcher|journal|journals|observ)/iu, ["🌊", "🚰", "🔌", "📬", "🛰", "👁", "🔭"]],
  [/\b(plan|plans|decid|decision|choose|chooses|select|selects|policy|rule|rules|admit|admission|grant|grants)/iu, ["🧭", "🎯", "⚖", "🗳", "🎟", "🎫", "📐"]],
  [/\b(count|counts|number|metric|measure|stat|statistic|ratio|percent|average|sum)/iu, ["📊", "📈", "🔢", "📐", "🧮", "📉"]],
  [/\b(test|tests|oracle|fixture|fixtures|law|laws|model|simulat|replay)/iu, ["🧪", "🔬", "🧫", "⚗", "🎲", "🎭"]],
  [/\b(platform|windows|linux|macos|os|host|machine|device)/iu, ["🖥", "🪟", "🐧", "🍎", "💻", "🖱"]],
  [/\b(env|environment|variable|variables|setting|settings|config|configuration|option|options)/iu, ["🌿", "🌍", "🌱", "🌤", "🎚", "🎛"]],
  [/\b(build|builds|compile|compiles|compiler|bazel|cargo|rust|bundle)/iu, ["🏗", "🔨", "🛠", "🧱", "🚧", "⛏"]],
  [/\b(start|starts|launch|launches|begin|begins|init|initial|open|opens|first)/iu, ["🚀", "🌅", "🎬", "▶", "🏁", "🪂"]],
  [/\b(stop|stops|close|closes|closed|end|ends|finish|terminate|exit|kill|shutdown|last)/iu, ["🛑", "🔚", "🚪", "🔕", "⏹", "🏴"]],
  [/\b(copy|clone|mirror|link|links|symlink|alias|forward)/iu, ["🪞", "👯", "🔗", "📎", "🧲", "🪆"]],
  [/\b(progress|status|state|phase|step|steps|stage)/iu, ["🪜", "📶", "🚥", "🧗", "🎢", "🔄"]],
  [/\b(text|message|messages|language|english|german|string|line|lines|output|print)/iu, ["💬", "🗣", "🔤", "📣", "🖨", "✉"]],
];
const HEAD: Readonly<Record<string, readonly string[]>> = {
  captures: ["📸", "🪝", "🧲"], capture: ["📸", "🪝"], resolves: ["🧭", "🔎", "🗺"], resolve: ["🧭", "🔎"], validates: ["✅", "🧐", "🛂"], verifies: ["✅", "🔍", "🛂"], checks: ["🔍", "✅", "🩺"],
  names: ["🏷", "📛", "🪪"], says: ["💬", "📢", "🗣"], tells: ["💬", "📢", "🗣"], reports: ["📣", "📊", "📰"], records: ["✍", "📝", "🖊"], reads: ["📖", "📚", "🔖"],
  lists: ["📋", "🗂", "📑"], creates: ["🏗", "🧱", "🛠"], builds: ["🏗", "🔨", "🧱"], keeps: ["🧳", "🗃", "🧊"], drops: ["🧹", "🗑", "🪣"], removes: ["🧹", "🗑", "🧽"],
  runs: ["🏃", "▶", "🚀"], opens: ["🚪", "🔓", "🪟"], closes: ["🔚", "🔒", "🚪"], waits: ["⏳", "⌛", "💤"], refuses: ["🚫", "🛑", "🙅"], admits: ["🎟", "🚪", "🛂"],
  publishes: ["📤", "📮", "📡"], writes: ["✍", "🖋", "📝"], returns: ["↩", "📬", "🎁"], whether: ["❓", "🤔", "⚖"], true: ["✔", "☑", "👍"], the: [], a: [], an: [],
  shares: ["🤝", "🪢", "🔗"], finds: ["🔎", "🧭", "🕵"], walks: ["🚶", "🥾", "🌲"], parses: ["🔬", "🧬", "📐"], hashes: ["🔑", "#", "🧮"], counts: ["🔢", "🧮", "📊"],
  brings: ["🚚", "🧲", "🧭"], swaps: ["🔁", "🔀", "🤝"], marks: ["🔖", "📌", "🏁"], streams: ["🌊", "🚰", "🌀"], collects: ["🧺", "🧲", "📦"], assembles: ["🧩", "🏗", "🔧"],
};
const POOL: string[] = [];
for (const [a, b] of [[0x1f300, 0x1f64f], [0x1f680, 0x1f6c5], [0x1f90c, 0x1f9ff], [0x2600, 0x26ff], [0x2700, 0x27bf]] as const)
  for (let cp = a; cp <= b; cp++) {
    const ch = String.fromCodePoint(cp);
    if (/\p{Emoji}/u.test(ch) && /\p{Extended_Pictographic}/u.test(ch) && !/\p{Emoji_Modifier}|\p{Regional_Indicator}/u.test(ch) && cp !== 0x1f3f4) POOL.push(ch);
  }
const out: Record<string, { replaced: number; prefixed: number; failed: string[] }> = {};
const entries = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t") as [string, string]);
for (const [work, rel] of entries) {
  const text = readFileSync(work, "utf8");
  const kind = /\.m?ts$/u.test(rel) ? ts.ScriptKind.TS : ts.ScriptKind.JS;
  const sf = ts.createSourceFile(rel, text, ts.ScriptTarget.Latest, true, kind);
  const docs: { pos: number; end: number; text: string }[] = [];
  const seen = new Set<number>();
  const add = (ranges: readonly ts.CommentRange[] | undefined): void => {
    for (const r of ranges ?? []) {
      if (seen.has(r.pos)) continue;
      seen.add(r.pos);
      const t = text.slice(r.pos, r.end);
      if (t.startsWith("/**") && !t.startsWith("/**/") && !t.startsWith("/***")) docs.push({ pos: r.pos, end: r.end, text: t });
    }
  };
  const walk = (node: ts.Node): void => {
    for (const child of node.getChildren(sf)) {
      if (child.getChildCount(sf) === 0) { add(ts.getLeadingCommentRanges(text, child.getFullStart())); add(ts.getTrailingCommentRanges(text, child.end)); }
      else walk(child);
    }
  };
  walk(sf);
  docs.sort((a, b) => a.pos - b.pos);
  const info = docs.map((d) => {
    const m = /^(\/\*\*+[ \t]*(?:\r?\n[ \t]*\*?[ \t]?)?)/u.exec(d.text);
    const head = m ? m[1]!.length : 3;
    const body = d.text.slice(head);
    const lead = LEAD.exec(body.trimStart());
    return { ...d, head, body, key: lead ? strip(lead[0]) : null, leadLen: lead ? lead[0].length : 0, indent: body.length - body.trimStart().length };
  });
  const used = new Set<string>();
  for (const d of info) if (d.key) used.add(d.key);
  const first = new Map<string, number>();
  const todo: { index: number; why: "dup" | "none" }[] = [];
  info.forEach((d, i) => {
    if (!d.key) { todo.push({ index: i, why: "none" }); return; }
    if (first.has(d.key)) todo.push({ index: i, why: "dup" }); else first.set(d.key, i);
  });
  const rows = { replaced: 0, prefixed: 0, failed: [] as string[] };
  const edits: { at: number; remove: number; insert: string }[] = [];
  let poolAt = 0;
  const choose = (desc: string): string => {
    const sample = desc.replace(/^[\s*]+/u, "").slice(0, 120);
    const head = sample.split(/[^A-Za-z]+/u)[0]?.toLowerCase() ?? "";
    for (const c of HEAD[head] ?? []) if (!used.has(c)) { used.add(c); return c; }
    const ranked = TABLE.map(([re, list]) => ({ at: sample.search(re), list })).filter((row) => row.at >= 0).sort((x, y) => x.at - y.at);
    for (const row of ranked) for (const c of row.list) if (!used.has(c)) { used.add(c); return c; }
    while (poolAt < POOL.length) { const c = POOL[poolAt++]!; if (!used.has(strip(c))) { used.add(strip(c)); return c; } }
    return "";
  };
  for (const { index, why } of todo) {
    const d = info[index]!;
    const afterLead = d.body.trimStart().slice(d.leadLen);
    const desc = why === "dup" ? afterLead : d.body.trimStart();
    if (why === "none" && /^@/u.test(d.body.trimStart())) { rows.failed.push(`line ${sf.getLineAndCharacterOfPosition(d.pos).line + 1}: starts with a tag, needs a description`); continue; }
    const chosen = choose(desc);
    if (!chosen) { rows.failed.push(`line ${sf.getLineAndCharacterOfPosition(d.pos).line + 1}: emoji pool exhausted`); continue; }
    const glyph = chosen + "️";
    const start = d.pos + d.head + d.indent;
    if (why === "dup") { edits.push({ at: start, remove: d.leadLen, insert: glyph }); rows.replaced++; }
    else { edits.push({ at: start, remove: 0, insert: glyph + " " }); rows.prefixed++; }
  }
  edits.sort((a, b) => b.at - a.at);
  let next = text;
  for (const e of edits) next = next.slice(0, e.at) + e.insert + next.slice(e.at + e.remove);
  if (next !== text) writeFileSync(work, next);
  out[rel] = rows;
}
writeFileSync(process.argv[3]!, JSON.stringify(out, null, 1));
console.error("[DEBUG] emoji pass", entries.length, "files, rss MB", Math.round(process.memoryUsage().rss / 1048576), "pool", POOL.length);
