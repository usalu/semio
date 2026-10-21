import type { RuntimeImportRecordV1, RuntimeLoaderV1, RuntimeTranspilerOptionsV1, RuntimeTranspilerV1 } from "../../📋️contract/🟦️.ts";

const REGEX_PREVIOUS = /[(,=:[!&|?{};+\-*%<>~^]$/u;
const REGEX_KEYWORDS = /(?:^|[^\w$])(?:return|typeof|case|do|else|in|of|void|delete|throw|new|yield|await)$/u;

/** 🔬️ One masked source: code with comments, string contents, regex bodies and template text blanked to spaces, plus the string and template literals that were blanked. */
export interface MaskedSourceV1 { readonly code: string; readonly strings: readonly { readonly start: number; readonly end: number; readonly value: string }[]; readonly templates: readonly { readonly start: number; readonly end: number; readonly body: string }[] }

/** 🔬️ Masks comments and literals so structural searches see code only; positions are preserved. */
export function maskSourceV1(source: string): MaskedSourceV1 {
  const output = source.split("");
  const strings: { start: number; end: number; value: string }[] = [], templates: { start: number; end: number; body: string }[] = [];
  const blank = (from: number, to: number): void => { for (let at = from; at < to; at++) if (output[at] !== "\n" && output[at] !== "\r") output[at] = " "; };
  let index = 0;
  const scanCode = (stopAtBrace: boolean): void => {
    let depth = 0, previous = "";
    while (index < source.length) {
      const character = source[index]!, next = source[index + 1];
      if (character === "/" && next === "/") { const end = source.indexOf("\n", index); const stop = end < 0 ? source.length : end; blank(index, stop); index = stop; continue; }
      if (character === "/" && next === "*") { const end = source.indexOf("*/", index + 2); const stop = end < 0 ? source.length : end + 2; blank(index, stop); index = stop; continue; }
      if (character === '"' || character === "'") {
        const start = index++;
        while (index < source.length && source[index] !== character && source[index] !== "\n") { if (source[index] === "\\") index++; index++; }
        strings.push({ start, end: index + 1, value: source.slice(start + 1, index) });
        blank(start + 1, index); index++; previous = "x"; continue;
      }
      if (character === "`") {
        const start = index++;
        let bodyStart = index;
        while (index < source.length && source[index] !== "`") {
          if (source[index] === "\\") { index += 2; continue; }
          if (source[index] === "$" && source[index + 1] === "{") { blank(bodyStart, index); index += 2; scanCode(true); bodyStart = index; continue; }
          index++;
        }
        blank(bodyStart, index);
        templates.push({ start, end: index + 1, body: source.slice(start + 1, index) });
        index++; previous = "x"; continue;
      }
      if (character === "/" && (previous === "" || REGEX_PREVIOUS.test(previous) || REGEX_KEYWORDS.test(previous))) {
        const start = index++;
        let inClass = false;
        while (index < source.length && source[index] !== "\n") { const c = source[index]!; if (c === "\\") { index += 2; continue; } if (c === "[") inClass = true; else if (c === "]") inClass = false; else if (c === "/" && !inClass) break; index++; }
        blank(start + 1, index); index++; previous = "x"; continue;
      }
      if (stopAtBrace) { if (character === "{") depth++; else if (character === "}") { if (depth === 0) { index++; return; } depth--; } }
      if (!/\s/u.test(character)) previous = (previous + character).slice(-24);
      index++;
    }
  };
  scanCode(false);
  return { code: output.join(""), strings, templates };
}

function literalAfter(masked: MaskedSourceV1, from: number): { readonly value: string; readonly end: number } | undefined {
  const row = masked.strings.find(candidate => candidate.start >= from && /^\s*$/u.test(masked.code.slice(from, candidate.start)));
  return row === undefined ? undefined : { value: row.value, end: row.end };
}

/** 🔬️ Lists the module specifiers a source names: static imports and re-exports, dynamic imports, `require` and `require.resolve`; type-only imports are not imports. */
export function scanImportsV1(source: string): RuntimeImportRecordV1[] {
  const masked = maskSourceV1(source), { code } = masked, records: { at: number; row: RuntimeImportRecordV1 }[] = [];
  for (const match of code.matchAll(/(?<![\w$.])(import|export)\b(?!\s*[:(=])/gu)) {
    const keyword = match[1]!, start = match.index! + match[0].length;
    const head = code.slice(start, start + 4000);
    if (keyword === "import") {
      const direct = /^\s*$/u.test(code.slice(start, masked.strings.find(row => row.start >= start)?.start ?? start));
      if (direct) { const literal = literalAfter(masked, start); if (literal !== undefined) { records.push({ at: match.index!, row: { path: literal.value, kind: "import-statement" } }); continue; } }
      if (/^\s*type\b(?!\s*,|\s+from\b)/u.test(head)) continue;
      const clause = /^([^;]*?)\bfrom\b/su.exec(head);
      if (clause === null) continue;
      const names = clause[1]!;
      const braces = /^\s*(?:[\w$]+\s*,\s*)?\{([^}]*)\}\s*$/su.exec(names);
      if (braces !== null && braces[1]!.trim() !== "" && braces[1]!.split(",").map(part => part.trim()).filter(Boolean).every(part => /^type\s+/u.test(part)) && !/^\s*[\w$]+\s*,/u.test(names)) continue;
      const literal = literalAfter(masked, start + clause[0].length);
      if (literal !== undefined) records.push({ at: match.index!, row: { path: literal.value, kind: "import-statement" } });
    } else {
      if (/^\s*type\b/u.test(head)) continue;
      const clause = /^([^;]*?)\bfrom\b/su.exec(head);
      if (clause === null || /[=(]/u.test(clause[1]!.replace(/\{[^}]*\}/gu, ""))) continue;
      const literal = literalAfter(masked, start + clause[0].length);
      if (literal !== undefined) records.push({ at: match.index!, row: { path: literal.value, kind: "import-statement" } });
    }
  }
  for (const match of code.matchAll(/(?<![\w$.])import\s*\(\s*/gu)) { if (/\btypeof\s*$/u.test(code.slice(Math.max(0, match.index! - 12), match.index!))) continue; const literal = literalAfter(masked, match.index! + match[0].length); if (literal !== undefined && /^\s*[,)]/u.test(code.slice(literal.end, literal.end + 8))) records.push({ at: match.index!, row: { path: literal.value, kind: "dynamic-import" } }); }
  const localRequire = /(?:\b(?:const|let|var)\s+require\b|\brequire\s*=(?!=)|\{[^}]*\brequire\b[^}]*\}\s*=)/u.test(code);
  if (!localRequire) for (const match of code.matchAll(/(?<![\w$.])require\s*(\.\s*resolve\s*)?\(\s*/gu)) { const literal = literalAfter(masked, match.index! + match[0].length); if (literal !== undefined) records.push({ at: match.index!, row: { path: literal.value, kind: match[1] === undefined ? "require-call" : "require-resolve" } }); }
  return records.sort((left, right) => left.at - right.at).map(row => row.row);
}

/** 🔬️ Lists the top-level exported names of a source. */
export function scanExportsV1(source: string): string[] {
  const { code } = maskSourceV1(source), names = new Set<string>();
  let depth = 0;
  for (let index = 0; index < code.length; index++) {
    const character = code[index]!;
    if (character === "{" || character === "(" || character === "[") depth++;
    else if (character === "}" || character === ")" || character === "]") depth--;
    else if (depth === 0 && code.startsWith("export", index) && !/[\w$]/u.test(code[index - 1] ?? " ") && !/[\w$]/u.test(code[index + 6] ?? " ")) {
      const open = code.indexOf("{", index + 6), close = open < 0 ? -1 : code.indexOf("}", open), rest = code.slice(index + 6, Math.max(index + 6 + 400, /^\s*(?:type\s+)?$/u.test(code.slice(index + 6, Math.max(open, 0))) && close > 0 ? close + 1 : 0));
      let match: RegExpExecArray | null;
      if ((match = /^\s*(?:declare\s+)?(?:async\s+)?(?:function\s*\*?|class|abstract\s+class|const\s+enum|enum)\s+([\w$]+)/u.exec(rest)) !== null) names.add(match[1]!);
      else if ((match = /^\s*(?:declare\s+)?(?:const|let|var)\s+([\w$]+)/u.exec(rest)) !== null) names.add(match[1]!);
      else if ((match = /^\s*(?:type\s+)?\{([^}]*)\}/su.exec(rest)) !== null) { if (!/^\s*type\b/u.test(rest)) for (const part of match[1]!.split(",")) { if (/^\s*type\s+/u.test(part)) continue; const name = part.trim().split(/\s+as\s+/u).at(-1)?.trim(); if (name) names.add(name); } }
      else if (/^\s*default\b/u.test(rest)) names.add("default");
      else if ((match = /^\s*\*\s+as\s+([\w$]+)/u.exec(rest)) !== null) names.add(match[1]!);
    }
  }
  return [...names];
}

function warningFree<T>(body: () => T): T {
  const original = process.emitWarning;
  process.emitWarning = ((warning: unknown, ...rest: unknown[]) => { if ((typeof warning === "string" ? warning : (warning as Error)?.message ?? "").includes("stripTypeScriptTypes")) return; return (original as (...arguments_: unknown[]) => void).call(process, warning, ...rest); }) as typeof process.emitWarning;
  try { return body(); } finally { process.emitWarning = original; }
}

function replaceDefines(code: string, define: Readonly<Record<string, string>>): string {
  const masked = maskSourceV1(code);
  const edits: { start: number; end: number; text: string }[] = [];
  for (const [key, value] of Object.entries(define)) {
    const pattern = new RegExp(`(?<![\\w$.])${key.split(".").map(part => part.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("\\s*\\.\\s*")}(?![\\w$])`, "gu");
    for (const match of masked.code.matchAll(pattern)) edits.push({ start: match.index!, end: match.index! + match[0].length, text: value });
  }
  let result = code;
  for (const edit of edits.sort((left, right) => right.start - left.start)) result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);
  return result;
}

function removeDeadBranches(code: string): string {
  let current = code;
  for (let round = 0; round < 64; round++) {
    const { code: masked } = maskSourceV1(current);
    const match = /(?<![\w$.])if\s*\(\s*(?:undefined|false|0|void 0|null)\s*\)\s*\{/u.exec(masked);
    if (match === null) return current;
    const open = match.index + match[0].length - 1;
    let depth = 0, close = -1;
    for (let at = open; at < masked.length; at++) { if (masked[at] === "{") depth++; else if (masked[at] === "}") { depth--; if (depth === 0) { close = at; break; } } }
    if (close < 0) return current;
    let end = close + 1, replacement = "";
    const tail = /^\s*else\s*/u.exec(masked.slice(end));
    if (tail !== null) {
      const after = end + tail[0].length;
      if (masked[after] === "{") { let nested = 0, stop = -1; for (let at = after; at < masked.length; at++) { if (masked[at] === "{") nested++; else if (masked[at] === "}") { nested--; if (nested === 0) { stop = at; break; } } } if (stop >= 0) { replacement = current.slice(after, stop + 1); end = stop + 1; } }
    }
    current = current.slice(0, match.index) + replacement + current.slice(end);
  }
  return current;
}

/** 🔬️ A portable transpiler: type stripping by the runtime's own stripper, define replacement and dead-branch removal by our scanner, import and export scanning by our scanner; TypeScript only, JSX loaders are refused. */
export function createPortableTranspilerV1(options: RuntimeTranspilerOptionsV1 = {}): RuntimeTranspilerV1 {
  const stripper = (process.getBuiltinModule("node:module") as { stripTypeScriptTypes?: (code: string, options?: { mode?: string }) => string } | undefined)?.stripTypeScriptTypes;
  const transformSync = (source: string, loader: RuntimeLoaderV1 = options.loader ?? "js"): string => {
    if (loader === "jsx" || loader === "tsx") throw new Error(`The portable transpiler does not support the ${loader} loader`);
    let code = source;
    if (loader === "ts") { if (stripper === undefined) throw new Error("The host provides no TypeScript stripper"); code = warningFree(() => stripper(code, { mode: "transform" })); }
    if (options.define !== undefined) code = replaceDefines(code, options.define);
    if (options.deadCodeElimination === true) code = removeDeadBranches(code);
    return code;
  };
  return { transformSync, scan: source => ({ exports: scanExportsV1(source), imports: scanImportsV1(source) }), scanImports: scanImportsV1 };
}
