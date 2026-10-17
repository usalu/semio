// 🧲️ Shared helpers of the FIXTURE-READER-DEDUPE migration tools.

/** Offset just after the inner documentation and inner attributes that open an inline module body that starts at `bodyStart` (right after its `{`). */
export function afterModHeader(text: string, bodyStart: number): number {
  let at = bodyStart;
  let last = bodyStart;
  for (;;) {
    let probe = at;
    while (probe < text.length && /\s/u.test(text[probe]!)) probe += 1;
    if (text.startsWith("//!", probe)) { const eol = text.indexOf("\n", probe); at = eol < 0 ? text.length : eol; last = at; continue; }
    if (text.startsWith("#![", probe)) { const close = text.indexOf("]", probe); if (close < 0) break; const eol = text.indexOf("\n", close); at = eol < 0 ? text.length : eol; last = at; continue; }
    break;
  }
  return last;
}

/** True when a file path names test support: a tests folder, testkit, fixtures, oracle, companion, laws or testing crate. */
export function testLikePath(rel: string): boolean {
  return ["/🧪️test", "/🧰️testkit", "/🧰️fixtures", "/🔮️oracles", "companion", "/⚖️laws", "/🧰️support", "/🧰️test-support", "/🏭️generator"].some(marker => rel.includes(marker));
}

type Tok = { kind: string; text: string };

/** Gate of a token index: "test" when an enclosing item carries `#[cfg(test)]` or `cfg(all(test, ..))`, "testkit" for `any(test, feature = "testkit")`, "ungated" otherwise. */
export function gateAt(tokens: readonly Tok[], pairs: ReadonlyMap<number, number>, index: number): "test" | "testkit" | "ungated" {
  const open: number[] = [];
  for (let i = 0; i < index; i += 1) {
    if (tokens[i]!.text === "}") open.pop();
    if (tokens[i]!.text === "{") open.push(i);
  }
  for (const brace of open.reverse()) {
    let head = brace - 1;
    while (head >= 0 && !(["mod", "fn", "impl", "const", "static", "trait", "struct", "enum"].includes(tokens[head]!.text) && tokens[head - 1]?.text !== "'")) { head -= 1; if (brace - head > 200) break; }
    if (head < 0 || brace - head > 200) continue;
    let start = head;
    while (start > 0 && ["pub", "unsafe", "async", "extern", "const"].includes(tokens[start - 1]!.text)) start -= 1;
    while (tokens[start - 1]?.text === ")") { const o = pairs.get(start - 1); if (o === undefined) break; start = o; if (tokens[start - 1]?.text === "pub") start -= 1; }
    let k = start - 1;
    while (k >= 0 && tokens[k]!.text === "]") {
      const o = pairs.get(k);
      if (o === undefined || tokens[o - 1]?.text !== "#") break;
      const attr = tokens.slice(o + 1, k).map(t => t.text).join(" ");
      if (/^cfg \( test \)$/u.test(attr) || /^cfg \( all \( test\b/u.test(attr)) return "test";
      if (/^cfg \( any \( test , feature = "testkit" \) \)$/u.test(attr) || /^cfg \( any \( feature = "testkit" , test \) \)$/u.test(attr)) return "testkit";
      k = o - 2;
    }
  }
  return "ungated";
}
