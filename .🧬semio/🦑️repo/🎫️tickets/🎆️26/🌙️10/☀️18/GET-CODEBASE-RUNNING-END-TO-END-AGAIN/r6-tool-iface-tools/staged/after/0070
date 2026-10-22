import { runtimeCommandNameV1, runtimeCommandNamesV1 } from "../../../⚙️runtime/🏷️identity/🟦️.ts";
import { toolForCommandV1, TOOL_REGISTRY_V1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";

/** 🚩️ One place in a source file that starts a registered tool without the registry. */
export type ToolSpawnSiteV1 = Readonly<{ path: string; line: number; callee: string; form: "spawn" | "shell" | "declared"; tool: string | null; argument: string; scope: "prod" | "test" }>;
/** 🌐️ The languages the scanner reads. */
export type ToolScanLanguageV1 = "ts" | "rust" | "go" | "python" | "shell" | "declared";

type Dialect = Readonly<{ line: readonly string[]; block: boolean; nested: boolean; strings: readonly string[]; raw: boolean; template: boolean; rune: boolean }>;
const DIALECTS: Readonly<Record<"ts" | "rust" | "go", Dialect>> = {
  ts: { line: ["//"], block: true, nested: false, strings: ["\"", "'"], raw: false, template: true, rune: false },
  rust: { line: ["//"], block: true, nested: true, strings: ["\""], raw: true, template: false, rune: true },
  go: { line: ["//"], block: true, nested: false, strings: ["\""], raw: true, template: false, rune: true },
};

/** 🫥️ Replaces every comment of a C-family source with spaces (newlines and offsets survive) so a pattern never matches prose; string contents stay unless `blankStrings` asks for them to be blanked too, which finds code positions that are not inside a string. */
export function maskCommentsV1(text: string, language: "ts" | "rust" | "go", blankStrings = false): string {
  const dialect = DIALECTS[language], out = text.split("");
  const blank = (from: number, to: number): void => { for (let index = from; index < to; index++) if (out[index] !== "\n") out[index] = " "; };
  const templates: number[] = [];
  let index = 0, previous = "";
  while (index < text.length) {
    const char = text[index]!, next = text[index + 1] ?? "";
    if (dialect.line.some(token => text.startsWith(token, index))) { let end = text.indexOf("\n", index); if (end < 0) end = text.length; blank(index, end); index = end; continue; }
    if (dialect.block && char === "/" && next === "*") {
      let depth = 1, end = index + 2;
      while (end < text.length && depth > 0) {
        if (text.startsWith("*/", end)) { depth--; end += 2; } else if (dialect.nested && text.startsWith("/*", end)) { depth++; end += 2; } else end++;
      }
      blank(index, end); index = end; continue;
    }
    if (dialect.raw && language === "rust" && (char === "r" && !/[\w]/u.test(previous)) && /^r#*"/u.test(text.slice(index, index + 8))) {
      const hashes = /^r(#*)"/u.exec(text.slice(index, index + 8))![1]!, close = "\"" + hashes, end = text.indexOf(close, index + 2 + hashes.length);
      const stop = end < 0 ? text.length : end + close.length; if (blankStrings) blank(index + 1, stop - 1); index = stop; previous = "\""; continue;
    }
    if (dialect.raw && language === "go" && char === "`") { const end = text.indexOf("`", index + 1), stop = end < 0 ? text.length : end + 1; if (blankStrings) blank(index + 1, stop - 1); index = stop; previous = "`"; continue; }
    if (dialect.template && char === "`") {
      const opened = index;
      index++;
      while (index < text.length) {
        const inner = text[index]!;
        if (inner === "\\") { index += 2; continue; }
        if (inner === "`") { index++; break; }
        if (inner === "$" && text[index + 1] === "{") { templates.push(1); index += 2; let depth = 1; while (index < text.length && depth > 0) { const code = text[index]!; if (code === "{") depth++; else if (code === "}") depth--; else if (code === "\"" || code === "'") { index = skipString(text, index); continue; } else if (code === "`") { const end = skipTemplate(text, index); index = end; continue; } index++; } templates.pop(); continue; }
        index++;
      }
      if (blankStrings) blank(opened + 1, index - 1);
      previous = "`"; continue;
    }
    if (dialect.rune && char === "'") {
      const rune = /^'(?:\\.[^']*|[^'\\])'/u.exec(text.slice(index, index + 12));
      if (rune !== null) { index += rune[0].length; previous = "'"; continue; }
      index++; previous = "'"; continue;
    }
    if (dialect.strings.includes(char)) { const stop = skipString(text, index); if (blankStrings) blank(index + 1, stop - 1); index = stop; previous = char; continue; }
    if (language === "ts" && char === "/" && /[(,=:[!&|?{};]|^$/u.test(previous.trim() === "" ? lastSignificant(text, index) : previous)) {
      let end = index + 1, inClass = false;
      while (end < text.length && text[end] !== "\n") { const code = text[end]!; if (code === "\\") end++; else if (code === "[") inClass = true; else if (code === "]") inClass = false; else if (code === "/" && !inClass) break; end++; }
      if (text[end] === "/") { index = end + 1; previous = "/"; continue; }
    }
    if (!/\s/u.test(char)) previous = char;
    index++;
  }
  return out.join("");
}

const lastSignificant = (text: string, index: number): string => { for (let cursor = index - 1; cursor >= 0; cursor--) if (!/\s/u.test(text[cursor]!)) return text[cursor]!; return ""; };

function skipString(text: string, start: number): number {
  const quote = text[start]!;
  for (let index = start + 1; index < text.length; index++) {
    if (text[index] === "\\") { index++; continue; }
    if (text[index] === quote || text[index] === "\n" && quote !== "\"" ) return index + 1;
  }
  return text.length;
}

function skipTemplate(text: string, start: number): number {
  for (let index = start + 1; index < text.length; index++) {
    if (text[index] === "\\") { index++; continue; }
    if (text[index] === "`") return index + 1;
    if (text[index] === "$" && text[index + 1] === "{") { let depth = 1; index += 2; while (index < text.length && depth > 0) { if (text[index] === "{") depth++; else if (text[index] === "}") depth--; index++; } index--; }
  }
  return text.length;
}

/** ✂️ Reads the argument that starts at `from` up to the next top-level comma or the closing bracket; returns its text and where it ended. */
export function readArgumentV1(text: string, from: number): Readonly<{ text: string; end: number }> {
  let depth = 0, index = from;
  for (; index < text.length; index++) {
    const char = text[index]!;
    if (char === "\"" || char === "'") { index = skipString(text, index) - 1; continue; }
    if (char === "`") { index = skipTemplate(text, index) - 1; continue; }
    if ("([{".includes(char)) depth++;
    else if (")]}".includes(char)) { if (depth === 0) break; depth--; }
    else if (char === "," && depth === 0) break;
  }
  return { text: text.slice(from, index).trim(), end: index };
}

const STRING = /^(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`$]*)`)$/u;
const literalOf = (expression: string): string | undefined => { const match = STRING.exec(expression.trim()); return match === null ? undefined : match[1] ?? match[2] ?? match[3]; };

const IDENTIFIER = /^[A-Za-z_$][\w$]*$/u;
function resolveExpression(expression: string, masked: string, registry: ToolRegistryV1, language: ToolScanLanguageV1): { value?: string; self?: boolean } {
  const trimmed = expression.trim().replace(/^&/u, "").replace(/\.to_string\(\)$|\.as_str\(\)$/u, "");
  const direct = literalOf(trimmed);
  if (direct !== undefined) return { value: direct };
  if (registry.scan.selfExpressions.includes(trimmed)) return { value: runtimeCommandNameV1(), self: true };
  if (IDENTIFIER.test(trimmed)) {
    const declaration = language === "rust" ? new RegExp(`\\b(?:const|static|let)\\s+(?:mut\\s+)?${trimmed.replace(/\$/gu, "\\$")}\\s*(?::[^=]+)?=\\s*([^;\\n]+)`, "u") : language === "go" ? new RegExp(`\\b(?:const|var)?\\s*${trimmed}\\s*(?::=|=)\\s*([^\\n]+)`, "u") : new RegExp(`\\b(?:const|let|var)\\s+${trimmed.replace(/\$/gu, "\\$")}\\s*(?::[^=]+)?=\\s*([^;\\n]+)`, "u");
    const found = declaration.exec(masked);
    if (found !== null) {
      const right = found[1]!.trim();
      if (registry.scan.selfExpressions.includes(right)) return { value: runtimeCommandNameV1(), self: true };
      const first = /"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'/u.exec(right);
      if (first !== null && !/[(]/u.test(right.slice(0, first.index))) return { value: first[1] ?? first[2] };
    }
  }
  return {};
}

const WRAPPERS = new Set(["sudo", "exec", "env", "time", "nice", "command", "nohup", "xargs", "then", "do", "else", "!"]);
/** 🧩️ Answers the tool commands a shell command line runs: every segment of `a && b | c; d` contributes its first word that is not an assignment or a wrapper such as `sudo`. */
export function shellCommandsV1(line: string): readonly string[] {
  const commands: string[] = [];
  for (const segment of line.split(/&&|\|\||;|\||\$\(|`|\(|\)|\n/u)) {
    const words = segment.trim().split(/\s+/u).filter(Boolean);
    let position = 0;
    while (position < words.length) {
      const word = words[position]!;
      if (WRAPPERS.has(word) || /^[A-Za-z_][A-Za-z0-9_]*=/u.test(word)) position++;
      else if (word.startsWith("-") && position > 0 && (WRAPPERS.has(words[position - 1]!) || words[position - 2] === "nice")) position += word === "-n" && words[position - 1] === "nice" ? 2 : 1;
      else break;
    }
    const word = words[position];
    if (word !== undefined) commands.push(word.replace(/^["']|["']$/gu, "").replace(/^\.\/(?=[^/]+$)/u, ""));
  }
  return commands;
}

/** 🧭️ Tells production source from test source by the registry's test path segments. */
export function toolSpawnScopeV1(path: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): "prod" | "test" {
  const segments = path.split("/");
  return segments.some(segment => registry.scan.testSegments.includes(segment)) || /\.(?:test|spec)\.[a-z]+$/u.test(path) || /_test\.go$/u.test(path) ? "test" : "prod";
}

/** 🧱️ Picks the scan language of a file by its extension; undefined for files the scanner does not read. */
export function toolScanLanguageV1(path: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): ToolScanLanguageV1 | undefined {
  const name = path.split("/").pop()!, extension = name.includes(".") ? "." + name.split(".").pop()! : "";
  for (const language of ["ts", "rust", "go", "python", "shell"] as const) if (registry.scan.extensions[language].includes(extension)) return language;
  return registry.scan.declaredNames.includes(name) ? "declared" : undefined;
}

const lineStarts = (text: string): number[] => { const starts = [0]; for (let index = 0; index < text.length; index++) if (text[index] === "\n") starts.push(index + 1); return starts; };
const lineOf = (starts: readonly number[], offset: number): number => { let low = 0, high = starts.length - 1; while (low < high) { const middle = (low + high + 1) >> 1; if (starts[middle]! <= offset) low = middle; else high = middle - 1; } return low + 1; };

function classify(command: string | undefined, registry: ToolRegistryV1, following?: string): string | null {
  if (command === undefined) return null;
  const entry = toolForCommandV1(command, registry);
  if (entry === undefined) return null;
  return runtimeCommandNamesV1().includes(entry.commands[0]!) && following === "repo" ? "repo" : entry.id;
}

function shellOf(command: string, argumentsText: string, registry: ToolRegistryV1): string | undefined {
  if (!registry.scan.shellWrappers.includes(command.split(/[\\/]/u).pop()!)) return undefined;
  const flag = /["'](?:-c|-lc|\/c|\/k|-Command|-command)["']\s*,\s*(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`]*)`)/u.exec(argumentsText);
  return flag === null ? undefined : flag[1] ?? flag[2] ?? flag[3];
}

function scanCallSites(path: string, text: string, language: "ts" | "rust" | "go" | "python", registry: ToolRegistryV1, scope: "prod" | "test"): ToolSpawnSiteV1[] {
  const testAt = language === "rust" ? text.search(/#\[cfg\(test\)\]/u) : -1, masked = language === "python" ? maskPython(text) : maskCommentsV1(text, language), code = language === "python" ? maskPython(text, true) : maskCommentsV1(text, language, true), starts = lineStarts(text), sites: ToolSpawnSiteV1[] = [];
  const add = (callee: string, form: ToolSpawnSiteV1["form"], tool: string | null, argument: string, offset: number): void => { if (tool !== null) sites.push({ path, line: lineOf(starts, offset), callee, form, tool, argument: argument.slice(0, 80), scope: testAt >= 0 && offset >= testAt ? "test" : scope }); };
  for (const callee of registry.scan.callees.filter(row => row.language === language)) {
    const escaped = callee.name.replace(/[.$]/gu, "\\$&");
    const pattern = callee.shape === "template" ? new RegExp("(?<![\\w$])(?:Bun\\.)?\\$`", "gu") : new RegExp(`(?<![\\w$])${callee.name.includes(".") || callee.name.includes("::") ? "" : "(?:[\\w$]+\\.)?"}${escaped}\\s*\\(`, "gu");
    for (const match of code.matchAll(pattern)) {
      if (callee.shape === "template") {
        const end = masked.indexOf("`", match.index + match[0].length), body = masked.slice(match.index + match[0].length, end < 0 ? undefined : end);
        for (const word of shellCommandsV1(body)) { const parts = body.trim().split(/\s+/u); add(callee.name, "shell", classify(word, registry, parts[1]), body, match.index); }
        continue;
      }
      const open = match.index + match[0].length, first = readArgumentV1(masked, open);
      if (language === "rust" && masked[first.end] !== ")") continue;
      if (callee.shape === "shell-string") {
        const text0 = literalOf(first.text) ?? (first.text.startsWith("`") ? first.text.slice(1, first.text.indexOf("${") < 0 ? -1 : first.text.indexOf("${")) : undefined);
        if (text0 !== undefined) for (const word of shellCommandsV1(text0)) add(callee.name, "shell", classify(word, registry, text0.trim().split(/\s+/u)[1]), text0, match.index);
        continue;
      }
      let expression = first.text, rest = masked.slice(first.end, first.end + 400);
      if (callee.shape === "array" || callee.shape === "optionsCmd") {
        const options = /^\{[\s\S]*?\bcmd\s*:\s*\[/u.exec(first.text);
        if (options !== null) { expression = readArgumentV1(first.text, options[0].length).text; rest = first.text.slice(options[0].length + expression.length); }
        else if (first.text.startsWith("[")) { expression = readArgumentV1(first.text, 1).text; rest = first.text.slice(1 + expression.length); }
        else if (language === "python" && STRING.test(first.text)) { const direct = literalOf(first.text)!; for (const word of shellCommandsV1(direct)) add(callee.name, "shell", classify(word, registry, direct.trim().split(/\s+/u)[1]), direct, match.index); continue; }
        else continue;
      }
      if (language === "go" && callee.name === "exec.CommandContext") { const second = readArgumentV1(masked, first.end + 1); expression = second.text; rest = masked.slice(second.end, second.end + 400); }
      const resolved = resolveExpression(expression, masked, registry, language);
      if (resolved.value === undefined) continue;
      const inner = shellOf(resolved.value, rest, registry);
      if (inner !== undefined) { for (const word of shellCommandsV1(inner)) add(callee.name, "shell", classify(word, registry, inner.trim().split(/\s+/u)[1]), inner, match.index); continue; }
      const following = /^\s*,?\s*(?:\[\s*)?(?:&?\[?\s*)?["'`]([^"'`]*)["'`]/u.exec(rest);
      add(callee.name, "spawn", classify(resolved.value, registry, following?.[1]), expression, match.index);
    }
  }
  return sites;
}

function maskPython(text: string, blankStrings = false): string {
  const out = text.split("");
  let index = 0;
  while (index < text.length) {
    const char = text[index]!;
    if (char === "#") { let end = text.indexOf("\n", index); if (end < 0) end = text.length; for (let cursor = index; cursor < end; cursor++) out[cursor] = " "; index = end; continue; }
    if (char === "\"" || char === "'") {
      const triple = text.startsWith(char.repeat(3), index);
      if (triple) { const end = text.indexOf(char.repeat(3), index + 3), stop = end < 0 ? text.length : end + 3; for (let cursor = index; cursor < stop; cursor++) if (out[cursor] !== "\n") out[cursor] = " "; index = stop; continue; }
      { const stop = skipString(text, index); if (blankStrings) for (let cursor = index + 1; cursor < stop - 1; cursor++) if (out[cursor] !== "\n") out[cursor] = " "; index = stop; } continue;
    }
    index++;
  }
  return out.join("");
}

function scanShell(path: string, text: string, registry: ToolRegistryV1, scope: "prod" | "test"): ToolSpawnSiteV1[] {
  const sites: ToolSpawnSiteV1[] = [];
  text.split(/\r?\n/u).forEach((raw, index) => {
    const line = raw.replace(/(^|\s)#.*$/u, "$1").replace(/^\s*(?:REM|::)\s.*$/iu, "");
    if (!line.trim() || /^\s*(?:echo|printf|Write-Host|Write-Output|log|die|warn)\b/u.test(line) || /^\s*(?:if|elif)\s+(?:!\s+)?(?:command\s+-v|\[|test|type|which)/u.test(line) && !/\b(?:then)\b.*\b\w+\s/u.test(line)) return;
    for (const word of shellCommandsV1(line.replace(/^\s*(?:if|elif|while|until)\s+/u, ""))) {
      const tool = classify(word, registry, line.trim().split(/\s+/u)[1]);
      if (tool !== null && /^[a-z]/iu.test(word)) sites.push({ path, line: index + 1, callee: "shell", form: "shell", tool, argument: line.trim().slice(0, 80), scope });
    }
  });
  return sites;
}

function scanDeclared(path: string, text: string, registry: ToolRegistryV1, scope: "prod" | "test"): ToolSpawnSiteV1[] {
  let document: unknown;
  try { document = JSON.parse(text); } catch { return []; }
  const sites: ToolSpawnSiteV1[] = [], isPackage = path.endsWith("package.json");
  const visit = (value: unknown, key: string, inScripts: boolean): void => {
    if (typeof value === "string") {
      if (isPackage ? inScripts : key === "command" || key === "commands") for (const word of shellCommandsV1(value)) { const tool = classify(word, registry, value.trim().split(/\s+/u)[1]); if (tool !== null) sites.push({ path, line: 0, callee: key, form: "declared", tool, argument: value.slice(0, 80), scope }); }
    } else if (Array.isArray(value)) for (const item of value) visit(item, key, inScripts);
    else if (typeof value === "object" && value !== null) for (const [name, child] of Object.entries(value)) visit(child, name, inScripts || name === "scripts");
  };
  visit(document, "", false);
  return sites;
}

/** 🔍️ Finds every place a source file starts a registered tool without the registry: process spawns and shell strings in TypeScript, Rust, Go and Python, command lines in shell scripts, and commands declared in project and package manifests. Calls whose command cannot be resolved statically are not reported. */
export function scanToolSpawnsV1(path: string, text: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): readonly ToolSpawnSiteV1[] {
  const language = toolScanLanguageV1(path, registry);
  if (language === undefined) return [];
  const scope = toolSpawnScopeV1(path, registry);
  if (language === "declared") return scanDeclared(path, text, registry, scope);
  if (language === "shell") return scanShell(path, text, registry, scope);
  if (!registry.scan.callees.some(callee => callee.language === language && text.includes(callee.name === "$" ? "$`" : callee.name))) return [];
  return scanCallSites(path, text, language, registry, scope);
}

/** 📞️ One call of the tool interface found in source: which tool id it names and where. */
export type ToolCallSiteV1 = Readonly<{ path: string; line: number; callee: string; tool: string }>;

/** 🧵️ Finds the calls of `runToolV1` and `runToolSyncV1` whose first argument is a string literal, so the law can check the id and the caller against the registry. */
export function scanToolCallsV1(path: string, text: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): readonly ToolCallSiteV1[] {
  if (toolScanLanguageV1(path, registry) !== "ts" || !text.includes("runTool")) return [];
  const code = maskCommentsV1(text, "ts", true), masked = maskCommentsV1(text, "ts"), starts = lineStarts(text), sites: ToolCallSiteV1[] = [];
  for (const match of code.matchAll(/(?<![\w$])(runToolV1|runToolSyncV1)\s*\(/gu)) {
    const literal = literalOf(readArgumentV1(masked, match.index + match[0].length).text);
    if (literal !== undefined) sites.push({ path, line: lineOf(starts, match.index), callee: match[1]!, tool: literal });
  }
  return sites;
}
