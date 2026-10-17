// 🔓️ Visibility widening for the c4p5 splits (widenChunk and identifiers copied unchanged from pfb-tools/widen.ts).
import {rustTokenPairs, rustTokens} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";

export const identifiers = (text: string): Set<string> => new Set(rustTokens(text).filter(t => t.kind === "identifier").map(t => t.text));

export function widenChunk(chunk: string, outside: Set<string>, report: string[], label: string): string {
  const tree = parseRustSourceTree(chunk);
  const edits: {at: number; text: string}[] = [];
  const tokens = rustTokens(chunk);
  const pairs = rustTokenPairs(tokens);
  const tokenAt = (offset: number) => tokens.findIndex(t => t.start >= offset);
  for (const item of tree.items) {
    if (item.name === null && item.kind !== "impl") continue;
    if (item.visibility === "private" && ["fn", "const", "static", "struct", "enum", "type", "trait"].includes(item.kind) && outside.has(item.name!)) {
      edits.push({at: item.declStart, text: "pub(super) "});
      report.push(`${label}: ${item.kind} ${item.name}`);
    }
    const itemVisible = item.visibility !== "private" || outside.has(item.name ?? "");
    const first = tokenAt(item.declStart);
    if (item.kind === "struct") {
      let fieldTouched = false;
      let t = first;
      while (t < tokens.length && tokens[t]!.text !== "struct") t += 1;
      const nameIndex = t + 1;
      let open = nameIndex + 1;
      if (tokens[open]?.text === "<") { let depth = 0; for (; open < tokens.length; open += 1) { if (tokens[open]!.text === "<") depth += 1; else if (tokens[open]!.text === ">" || tokens[open]!.text === ">>") { depth -= tokens[open]!.text === ">>" ? 2 : 1; if (depth <= 0) { open += 1; break; } } } }
      if (tokens[open]?.text === "where") while (open < tokens.length && tokens[open]!.text !== "{" && tokens[open]!.text !== ";") open += 1;
      if (tokens[open]?.text === "{" || tokens[open]?.text === "(") {
        const close = pairs.get(open);
        if (close !== undefined) {
          const braces = tokens[open]!.text === "{";
          let segmentStart = open + 1;
          const segments: [number, number][] = [];
          let depth = 0;
          for (let k = open + 1; k < close; k += 1) {
            const text = tokens[k]!.text;
            if (["(", "[", "{", "<"].includes(text)) depth += 1;
            else if ([")", "]", "}", ">"].includes(text)) depth -= 1;
            else if (text === ">>") depth -= 2;
            else if (text === "," && depth === 0) { segments.push([segmentStart, k]); segmentStart = k + 1; }
          }
          if (segmentStart < close) segments.push([segmentStart, close]);
          let position = 0;
          for (const [a, b] of segments) {
            let c = a;
            while (tokens[c]?.text === "#" && tokens[c + 1]?.text === "[") c = (pairs.get(c + 1) ?? c + 1) + 1;
            if (tokens[c]?.text === "pub") { position += 1; continue; }
            if (braces) {
              const field = tokens[c]!.text;
              if (tokens[c + 1]?.text === ":" && outside.has(field)) { edits.push({at: tokens[c]!.start, text: "pub(super) "}); report.push(`${label}: field ${item.name}.${field}`); fieldTouched = true; }
            } else if (tokens[c] && b > c && itemVisible) { edits.push({at: tokens[c]!.start, text: "pub(super) "}); report.push(`${label}: field ${item.name}.${position}`); }
            position += 1;
          }
        }
      }
      if (fieldTouched && item.visibility === "private" && !outside.has(item.name ?? "")) { edits.push({at: item.declStart, text: "pub(super) "}); report.push(`${label}: struct ${item.name}`); }
    }
    if (item.kind === "impl") {
      let t = first;
      while (t < tokens.length && tokens[t]!.text !== "impl") t += 1;
      let open = t + 1;
      let isTraitImpl = false;
      let depth = 0;
      for (; open < tokens.length; open += 1) {
        const text = tokens[open]!.text;
        if (text === "<") depth += 1; else if (text === ">") depth -= 1; else if (text === ">>") depth -= 2;
        else if (depth === 0 && text === "for") isTraitImpl = true;
        else if (depth === 0 && text === "{") break;
      }
      if (isTraitImpl || tokens[open]?.text !== "{") continue;
      const close = pairs.get(open);
      if (close === undefined) continue;
      for (let k = open + 1; k < close; k += 1) {
        const skip = pairs.get(k);
        if (tokens[k]!.text === "#" && tokens[k + 1]?.text === "[") { k = pairs.get(k + 1) ?? k + 1; continue; }
        let head = k;
        let isPub = false;
        if (tokens[head]!.text === "pub") { isPub = true; if (tokens[head + 1]?.text === "(") head = (pairs.get(head + 1) ?? head + 1) + 1; else head += 1; }
        let qual = head;
        while (["async", "unsafe", "const", "extern", "default"].includes(tokens[qual]?.text ?? "") && tokens[qual + 1]?.text !== "(" && !(tokens[qual]!.text === "const" && tokens[qual + 2]?.text === ":")) qual += 1;
        const keyword = tokens[qual]?.text;
        if (keyword === "fn" || (keyword === "const" && tokens[qual + 2]?.text === ":") || keyword === "type") {
          const nameToken = tokens[qual + 1];
          if (!isPub && nameToken && nameToken.kind === "identifier" && outside.has(nameToken.text)) { edits.push({at: tokens[k]!.start, text: "pub(super) "}); report.push(`${label}: method ${item.name ?? "impl"}::${nameToken.text}`); }
        }
        // skip the member body
        let m = qual;
        while (m < close && !(tokens[m]!.text === ";" || tokens[m]!.text === "{")) { const p = pairs.get(m); m = p !== undefined && p > m && ["(", "[", "<"].includes(tokens[m]!.text) && tokens[m]!.text !== "<" ? p + 1 : m + 1; }
        if (tokens[m]?.text === "{") k = (pairs.get(m) ?? m);
        else k = m;
        void skip;
      }
    }
  }
  let out = chunk;
  for (const edit of edits.sort((a, b) => b.at - a.at)) out = out.slice(0, edit.at) + edit.text + out.slice(edit.at);
  return out;
}

