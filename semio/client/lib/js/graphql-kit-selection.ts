//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — kit command selection + Response payload wiring for scoped mutations.
//#endregion 🧲Header

/** @emoji 📬 Selection for golden {@code Response} on command mutation leaves. */
export const GQL_RESPONSE_SELECTION =
  "ok errors { kind message requestId } result { ... on IdResult { value } }";

type GqlScan = { paren: number; inString: boolean; escape: boolean };

function advanceGqlScan(ch: string, st: GqlScan): void {
  if (st.inString) {
    if (st.escape) {
      st.escape = false;
      return;
    }
    if (ch === "\\") {
      st.escape = true;
      return;
    }
    if (ch === '"') st.inString = false;
    return;
  }
  if (ch === '"') {
    st.inString = true;
    return;
  }
  if (ch === "(") st.paren++;
  else if (ch === ")") st.paren--;
}

function findMatchingCloseBrace(s: string, openIdx: number): number {
  if (s[openIdx] !== "{") return -1;
  const st: GqlScan = { paren: 0, inString: false, escape: false };
  let depth = 0;
  for (let i = openIdx; i < s.length; i++) {
    const ch = s[i]!;
    if (st.inString) {
      advanceGqlScan(ch, st);
      continue;
    }
    if (ch === '"') {
      st.inString = true;
      continue;
    }
    if (ch === "{") depth++;
    else if (ch === "}") {
      depth--;
      if (depth === 0) return i;
    }
  }
  return -1;
}

function lastArgListCloseParen(s: string): number {
  const st: GqlScan = { paren: 0, inString: false, escape: false };
  let last = -1;
  for (let i = 0; i < s.length; i++) {
    const ch = s[i]!;
    advanceGqlScan(ch, st);
    if (!st.inString && st.paren === 0 && ch === ")") last = i;
  }
  return last;
}

function hasTopLevelSelectionBrace(s: string): boolean {
  const st: GqlScan = { paren: 0, inString: false, escape: false };
  for (let i = 0; i < s.length; i++) {
    const ch = s[i]!;
    advanceGqlScan(ch, st);
    if (!st.inString && st.paren === 0 && ch === "{") return true;
  }
  return false;
}

function appendResponseAfterArgs(fieldWithArgs: string): string {
  const t = fieldWithArgs.trim();
  if (t.includes(GQL_RESPONSE_SELECTION)) return t;
  const closeParen = lastArgListCloseParen(t);
  if (closeParen === -1) return `${t} { ${GQL_RESPONSE_SELECTION} }`;
  const after = t.slice(closeParen + 1).trim();
  if (after.startsWith("{")) {
    const open = closeParen + 1 + t.slice(closeParen + 1).indexOf("{");
    const close = findMatchingCloseBrace(t, open);
    if (close === -1) return `${t} { ${GQL_RESPONSE_SELECTION} }`;
    const head = t.slice(0, open).trimEnd();
    const inner = t.slice(open + 1, close).trim();
    const tail = t.slice(close + 1).trim();
    return `${head} { ${transformKitSelectionBlock(inner)} }${tail === "" ? "" : ` ${tail}`}`;
  }
  return `${t.slice(0, closeParen + 1)} { ${GQL_RESPONSE_SELECTION} }`;
}

function transformKitSelectionBlock(inner: string): string {
  if (!hasTopLevelSelectionBrace(inner)) return appendResponseAfterArgs(inner);
  const open = inner.indexOf("{");
  const close = findMatchingCloseBrace(inner, open);
  if (close === -1) return appendResponseAfterArgs(inner);
  const head = inner.slice(0, open).trimEnd();
  const body = inner.slice(open + 1, close).trim();
  const tail = inner.slice(close + 1).trim();
  return `${head} { ${transformKitSelectionBlock(body)} }${tail === "" ? "" : ` ${transformKitSelectionBlock(tail)}`}`;
}

/** @emoji 📬 Appends {@link GQL_RESPONSE_SELECTION} after kit command args, not inside input objects. */
export function withResponseSelection(kitSelection: string): string {
  const trimmed = kitSelection.trim();
  const open = trimmed.indexOf("{");
  if (open === -1) return appendResponseAfterArgs(trimmed);
  const close = findMatchingCloseBrace(trimmed, open);
  if (close === -1) return appendResponseAfterArgs(trimmed);
  const head = trimmed.slice(0, open).trimEnd();
  const inner = trimmed.slice(open + 1, close).trim();
  const tail = trimmed.slice(close + 1).trim();
  const result = `${head} { ${transformKitSelectionBlock(inner)} }`;
  return tail === "" ? result : `${result} ${withResponseSelection(tail)}`;
}
