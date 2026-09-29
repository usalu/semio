
/** 🧯️ Repo-relative path of the framework fault catalog — the en/de text of every code a framework crate raises.
 * @see ../../../../../../../🔨️modules/⚠️diagnostic/🗂️catalog/🧬️schema/🔣️.json */
export const FAULT_CATALOG_REL_PATH = "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json";
/** 🧯️ The module that defines the fault constructors (`FaultCode::new`, `app_fault`, the `fault_from_error!` macro) —
 * the one source whose constructors take the code as a parameter. */
export const FAULT_DEFINITION_MODULE = "🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs";
const FAULT_PLUGIN_ROOT = "✏️s/🔌️plugins/";

/** 🧯️ Who owns the codes of one source: an app crate of one plugin (declares in its app definitions) or the framework
 * (declares in {@link FAULT_CATALOG_REL_PATH}). */
export type FaultOwner = Readonly<{ kind: "app"; plugin: string }> | Readonly<{ kind: "framework" }>;
/** 🧯️ One raised code and the parameter names the raising call chains (`.with_parameter("name", …)`). */
export type FaultRaise = Readonly<{ path: string; line: number; code: string; parameters: readonly string[] }>;
/** 🧯️ One `FaultCode::new(CONST)` the census resolves against the `const CONST: &str = "…";` it scanned. */
export type FaultConstRaise = Readonly<{ path: string; line: number; name: string; parameters: readonly string[] }>;
/** 🧯️ One `.fault("code", LocalizedLabel::native("en", "de"))` declaration. */
export type FaultDeclarationSite = Readonly<{ path: string; line: number; code: string; en: string; de: string }>;
/** 🧯️ One framework catalog entry. */
export type FaultCatalogEntry = Readonly<{ code: string; en: string; de: string }>;
/** 🚨️ The rules of the fault law. */
export type FaultRule =
  | "fault-from" | "computed-code" | "app-raise-form" | "framework-app-fault" | "code-tuple" | "received-code" | "computed-parameter"
  | "declaration-form" | "framework-declaration" | "code-grammar" | "parameter-grammar" | "unresolved-const" | "undeclared" | "unraised-declaration"
  | "declaration-conflict" | "uncatalogued" | "unraised-catalog-entry" | "catalog-duplicate" | "catalog-shape" | "text-empty" | "text-untranslated"
  | "placeholder-mismatch" | "parameter-mismatch";
/** 🚨️ One breach of the fault law at one source line (line 0: the catalog). */
export type FaultViolation = Readonly<{ path: string; line: number; rule: FaultRule; detail: string }>;
/** 🧯️ What one source raises, declares and binds, and how it breaks the law on its own. */
export type FaultFacts = Readonly<{ raises: FaultRaise[]; constRaises: FaultConstRaise[]; consts: [string, string][]; declarations: FaultDeclarationSite[]; violations: FaultViolation[] }>;

const FAULT_CODE = /^[A-Za-z0-9._/-]+$/u;
const FAULT_PARAMETER_NAME = /^[a-z][A-Za-z0-9]*$/u;
const FAULT_CODE_BYTES = 64;
const FAULT_PARAMETERS_MAXIMUM = 8;
const FAULT_PARAMETER_NAME_BYTES = 32;
const utf8Bytes = (text: string): number => Buffer.byteLength(text, "utf8");

/** 🧯️ The owner of one repo-relative source path. */
export function faultOwnerOfPath(path: string): FaultOwner {
  return path.startsWith(FAULT_PLUGIN_ROOT) ? { kind: "app", plugin: path.slice(FAULT_PLUGIN_ROOT.length).split("/")[0]! } : { kind: "framework" };
}

/** 🔤️ The Rust string literal starting at `index` (`"…"`, `r"…"`, `r#"…"#`), unescaped, and the index after it; `null`
 * when none starts there. */
function rustStringAt(code: string, index: number): { value: string; end: number } | null {
  const raw = /^r(#*)"/u.exec(code.slice(index, index + 40));
  if (raw) {
    const close = `"${raw[1]}`;
    const start = index + raw[0].length;
    const stop = code.indexOf(close, start);
    return stop < 0 ? null : { value: code.slice(start, stop), end: stop + close.length };
  }
  if (code[index] !== '"') return null;
  let value = "";
  for (let at = index + 1; at < code.length; at += 1) {
    const char = code[at]!;
    if (char === '"') return { value, end: at + 1 };
    if (char !== "\\") {
      value += char;
      continue;
    }
    const next = code[at + 1] ?? "";
    if (next === "u" && code[at + 2] === "{") {
      const close = code.indexOf("}", at);
      value += String.fromCodePoint(Number.parseInt(code.slice(at + 3, close), 16));
      at = close;
    } else if (next === "\n") {
      at += 1;
      while (/\s/u.test(code[at + 1] ?? "")) at += 1;
    } else {
      value += ({ n: "\n", t: "\t", r: "\r", "0": "\0" } as Record<string, string>)[next] ?? next;
      at += 1;
    }
  }
  return null;
}

const CHAR_LITERAL = /^'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'/u;

/** 🧭️ Walks `code` from `index` to the bracket that closes the one opened just before `index`, skipping string and char
 * literals; the index of that closing bracket (or `code.length`). */
function closingBracket(code: string, index: number): number {
  let depth = 0;
  for (let at = index; at < code.length; at += 1) {
    const char = code[at]!;
    if (char === '"' || (char === "r" && /^r#*"/u.test(code.slice(at, at + 40)) && !/\w/u.test(code[at - 1] ?? ""))) {
      const literal = rustStringAt(code, at);
      if (literal) {
        at = literal.end - 1;
        continue;
      }
    }
    if (char === "'") {
      const literal = CHAR_LITERAL.exec(code.slice(at, at + 16));
      if (literal) at += literal[0].length - 1;
      continue;
    }
    if (char === "(" || char === "[" || char === "{") depth += 1;
    else if (char === ")" || char === "]" || char === "}") {
      if (depth === 0) return at;
      depth -= 1;
    }
  }
  return code.length;
}

/** 🧩️ The top-level arguments of the call whose `(` is at `open`, and the index after its `)`. */
function callArguments(code: string, open: number): { args: string[]; end: number } {
  const close = closingBracket(code, open + 1);
  const args: string[] = [];
  let start = open + 1;
  let depth = 0;
  for (let at = open + 1; at < close; at += 1) {
    const char = code[at]!;
    if (char === '"' || char === "r") {
      const literal = (char === '"' || (/^r#*"/u.test(code.slice(at, at + 40)) && !/\w/u.test(code[at - 1] ?? ""))) ? rustStringAt(code, at) : null;
      if (literal) {
        at = literal.end - 1;
        continue;
      }
    }
    if (char === "'") {
      const literal = CHAR_LITERAL.exec(code.slice(at, at + 16));
      if (literal) at += literal[0].length - 1;
      continue;
    }
    if (char === "(" || char === "[" || char === "{") depth += 1;
    else if (char === ")" || char === "]" || char === "}") depth -= 1;
    else if (char === "," && depth === 0) {
      args.push(code.slice(start, at).trim());
      start = at + 1;
    }
  }
  const last = code.slice(start, close).trim();
  if (last.length > 0) args.push(last);
  return { args, end: close + 1 };
}

/** 🔤️ The value of an argument that is exactly one string literal, else `null`. */
function literalArgument(argument: string | undefined): string | null {
  if (argument === undefined) return null;
  const literal = rustStringAt(argument, 0);
  return literal && literal.end === argument.length ? literal.value : null;
}

/** ⛓️ The method calls chained after index `end` (`.name(args)` …), stopping at the first non-call. */
function chainedCalls(code: string, end: number): { method: string; args: string[] }[] {
  const calls: { method: string; args: string[] }[] = [];
  let at = end;
  for (;;) {
    const step = /^\s*\.\s*([A-Za-z_]\w*)\s*\(/u.exec(code.slice(at, at + 200));
    if (!step) return calls;
    const open = at + step[0].length - 1;
    const call = callArguments(code, open);
    calls.push({ method: step[1]!, args: call.args });
    at = call.end;
  }
}

/** ✂️ The end of the item or statement a `#[cfg(test)]` ending at `from` gates: through the `}` closing its first
 * top-level `{`, or through its `;` — whichever comes first outside string and char literals. */
function testItemEnd(code: string, from: number): number {
  for (let at = from; at < code.length; at += 1) {
    const char = code[at]!;
    if (char === '"' || (char === "r" && /^r#*"/u.test(code.slice(at, at + 40)) && !/\w/u.test(code[at - 1] ?? ""))) {
      const literal = rustStringAt(code, at);
      if (literal) {
        at = literal.end - 1;
        continue;
      }
    }
    if (char === "'") {
      const literal = CHAR_LITERAL.exec(code.slice(at, at + 16));
      if (literal) at += literal[0].length - 1;
      continue;
    }
    if (char === ";") return at + 1;
    if (char === "{") return closingBracket(code, at + 1) + 1;
  }
  return code.length;
}

/** ✂️ The code of a whole Rust source: comments removed line by line ({@link rustCodeOfLine}) and every outermost
 * `#[cfg(test)]` item blanked — spans chosen forward on the literal-masked code, a nested `#[cfg(test)]` inside a chosen
 * span skipped (newlines kept, so indices keep their line). */
export function rustProductionCode(text: string): string {
  const inBlock = { open: false };
  let code = text.split("\n").map((line) => rustCodeOfLine(line, inBlock)).join("\n");
  const spans: [number, number][] = [];
  for (const match of maskLiterals(code).matchAll(/#\[cfg\(test\)\]/gu)) {
    if (spans.length > 0 && match.index! < spans.at(-1)![1]) continue;
    spans.push([match.index!, testItemEnd(code, match.index! + match[0].length)]);
  }
  for (const [start, end] of spans.reverse()) code = code.slice(0, start) + code.slice(start, end).replace(/[^\n]/g, " ") + code.slice(end);
  return code;
}

/** 🙈️ `code` with the inside of every string and char literal blanked (delimiters and newlines kept), so a pattern
 * matched on it is code, never prose — indices are unchanged. */
function maskLiterals(code: string): string {
  let masked = "";
  for (let at = 0; at < code.length; at += 1) {
    const char = code[at]!;
    const opensString = char === '"' || (char === "r" && /^r#*"/u.test(code.slice(at, at + 40)) && !/\w/u.test(code[at - 1] ?? ""));
    const literal = opensString ? rustStringAt(code, at) : char === "'" ? CHAR_LITERAL.exec(code.slice(at, at + 16)) : null;
    if (literal === null) {
      masked += char;
      continue;
    }
    const end = "end" in literal ? literal.end : at + literal[0].length;
    const text = code.slice(at, end);
    const open = text.indexOf(char === "'" ? "'" : '"') + 1;
    const close = char === "'" ? text.length - 1 : text.lastIndexOf('"');
    masked += text.slice(0, open) + text.slice(open, close).replace(/[^\n]/g, " ") + text.slice(close);
    at = end - 1;
  }
  return masked;
}

/** 🧯️ The fault facts of one Rust source (see {@link FaultFacts}), CODE-BASED: every code literal a source can raise —
 * `app_fault("…")` and every `FaultCode::new("…")` wherever it sits (a helper taking a `FaultCode` is called with one, so
 * its literal is read at the call site) — is a raise of the source's owner, with the parameters the enclosing call's
 * chain names; every computed code (`app_fault(expr)`, `FaultCode::new(expr)` outside the definitions module), every
 * `Fault::from` and an app's own `Fault::new` is a violation. */
export function faultFactsOfText(path: string, text: string): FaultFacts {
  const code = rustProductionCode(text);
  const masked = maskLiterals(code);
  const owner = faultOwnerOfPath(path);
  const definitions = path === FAULT_DEFINITION_MODULE;
  const lineStarts = [0, ...[...code.matchAll(/\n/gu)].map((match) => match.index! + 1)];
  const lineOf = (index: number): number => {
    let low = 0;
    let high = lineStarts.length - 1;
    while (low < high) {
      const middle = (low + high + 1) >> 1;
      if (lineStarts[middle]! <= index) low = middle;
      else high = middle - 1;
    }
    return low + 1;
  };
  const facts: FaultFacts = { raises: [], constRaises: [], consts: [], declarations: [], violations: [] };
  const violate = (index: number, rule: FaultRule, detail: string): void => void facts.violations.push({ path, line: lineOf(index), rule, detail });
  const parametersAfter = (index: number, end: number): string[] => {
    const names: string[] = [];
    for (const call of chainedCalls(code, end)) {
      if (call.method !== "with_parameter") continue;
      const name = literalArgument(call.args[0]);
      if (name === null) violate(index, "computed-parameter", `with_parameter(${call.args[0] ?? ""}) names no literal`);
      else names.push(name);
    }
    return names;
  };
  const opens: number[] = [];
  const enclosing = new Map<number, number | null>();
  const events: number[] = [...masked.matchAll(/FaultCode::new\s*\(/gu)].map((match) => match.index!);
  let cursor = 0;
  for (const event of events) {
    for (; cursor < event; cursor += 1) {
      const char = code[cursor]!;
      if (char === '"' || (char === "r" && /^r#*"/u.test(code.slice(cursor, cursor + 40)) && !/\w/u.test(code[cursor - 1] ?? ""))) {
        const literal = rustStringAt(code, cursor);
        if (literal && literal.end <= event) {
          cursor = literal.end - 1;
          continue;
        }
      }
      if (char === "'") {
        const literal = CHAR_LITERAL.exec(code.slice(cursor, cursor + 16));
        if (literal) cursor += literal[0].length - 1;
        continue;
      }
      if (char === "(" || char === "[" || char === "{") opens.push(cursor);
      else if (char === ")" || char === "]" || char === "}") opens.pop();
    }
    const top = opens.at(-1);
    enclosing.set(event, top !== undefined && code[top] === "(" ? top : null);
  }
  for (const match of masked.matchAll(/(?<![\w])((?:[A-Za-z_]\w*::)*)app_fault\s*\(/gu)) {
    if (/\bfn\s+$/u.test(masked.slice(Math.max(0, match.index! - 8), match.index!))) continue;
    const open = match.index! + match[0].length - 1;
    const call = callArguments(code, open);
    const literal = literalArgument(call.args[0]);
    if (owner.kind === "framework" && !definitions) violate(match.index!, "framework-app-fault", "a framework crate raises framework codes, declared in the catalog");
    else if (literal === null) violate(match.index!, "computed-code", `app_fault(${call.args[0] ?? ""})`);
    else facts.raises.push({ path, line: lineOf(match.index!), code: literal, parameters: parametersAfter(match.index!, call.end) });
  }
  for (const event of events) {
    const open = code.indexOf("(", event);
    const call = callArguments(code, open);
    const argument = call.args[0] ?? "";
    const literal = literalArgument(argument);
    const parent = enclosing.get(event) ?? null;
    const parameters = parent === null ? [] : parametersAfter(event, callArguments(code, parent).end);
    if (literal !== null) facts.raises.push({ path, line: lineOf(event), code: literal, parameters });
    else if (/^(?:[A-Za-z_]\w*::)*[A-Z][A-Z0-9_]*$/u.test(argument)) facts.constRaises.push({ path, line: lineOf(event), name: argument.split("::").at(-1)!, parameters });
    else if (!definitions) violate(event, "computed-code", `FaultCode::new(${argument})`);
  }
  for (const match of masked.matchAll(/FaultCode::received\s*\(/gu)) {
    const argument = callArguments(code, match.index! + match[0].length - 1).args[0] ?? "";
    if (owner.kind === "app" || !/^(?:[A-Za-z_][\w.]*\.)?code(?:\.clone\(\))?$/u.test(argument)) violate(match.index!, "received-code", `FaultCode::received(${argument}) — only a decoded record's own code (\`code\`, \`<record>.code\`)`);
  }
  for (const match of masked.matchAll(/(?<![\w])(?:[A-Za-z_]\w*::)*Fault::from\b/gu)) violate(match.index!, "fault-from", "Fault::from (called or passed as a path) answers no declared code");
  if (owner.kind === "app") for (const match of masked.matchAll(/(?<![\w])(?:[A-Za-z_]\w*::)*Fault::new\s*\(/gu)) violate(match.index!, "app-raise-form", "Fault::new — an app refuses through app_fault");
  if (!definitions) for (const match of masked.matchAll(/(?<![\w:])FaultCode\s*\(/gu)) violate(match.index!, "code-tuple", "FaultCode(…) builds a code the census cannot read");
  for (const match of masked.matchAll(/fault_from_error!\s*\(/gu)) {
    if (definitions) continue;
    const call = callArguments(code, match.index! + match[0].length - 1);
    const literal = literalArgument(call.args[2]);
    if (literal === null) violate(match.index!, "computed-code", `fault_from_error!(… ${call.args[2] ?? ""})`);
    else facts.raises.push({ path, line: lineOf(match.index!), code: literal, parameters: [] });
  }
  for (const match of masked.matchAll(/\bconst\s+([A-Z][A-Z0-9_]*)\s*:\s*&\s*(?:'static\s+)?str\s*=\s*/gu)) {
    const literal = rustStringAt(code, match.index! + match[0].length);
    if (literal) facts.consts.push([match[1]!, literal.value]);
  }
  for (const match of masked.matchAll(/\.fault\s*\(/gu)) {
    const call = callArguments(code, match.index! + match[0].length - 1);
    const declared = literalArgument(call.args[0]);
    if (declared === null) continue;
    if (owner.kind === "framework") {
      violate(match.index!, "framework-declaration", `.fault("${declared}", …) — a framework code is declared in the catalog`);
      continue;
    }
    const label = /^(?:[A-Za-z_]\w*::)*LocalizedLabel::native\s*\(/u.exec(call.args[1] ?? "");
    const texts = label && call.args.length === 2 ? callArguments(call.args[1]!, label[0].length - 1) : null;
    const en = texts && texts.args.length === 2 && texts.end === call.args[1]!.length ? literalArgument(texts.args[0]) : null;
    const de = texts && texts.args.length === 2 && texts.end === call.args[1]!.length ? literalArgument(texts.args[1]) : null;
    if (en === null || de === null) violate(match.index!, "declaration-form", `.fault("${declared}", ${call.args[1] ?? ""}) — the text is LocalizedLabel::native("en", "de") of two literals`);
    else facts.declarations.push({ path, line: lineOf(match.index!), code: declared, en, de });
  }
  return facts;
}

/** 🧩️ The placeholder names of a fault text in order of first appearance (`{name}`, `[a-z][A-Za-z0-9]*`) and every
 * brace group that is no placeholder. */
export function faultTextPlaceholders(text: string): { names: string[]; malformed: string[] } {
  const names: string[] = [];
  const malformed: string[] = [];
  for (const match of text.matchAll(/\{([^{}]*)\}|[{}]/gu)) {
    const name = match[1];
    if (name !== undefined && FAULT_PARAMETER_NAME.test(name) && utf8Bytes(name) <= FAULT_PARAMETER_NAME_BYTES) {
      if (!names.includes(name)) names.push(name);
    } else malformed.push(match[0]);
  }
  return { names, malformed };
}

/** 🗣️ The text rules of one declaration or catalog entry: both cells non-empty, different, with the same well-formed
 * placeholders, at most {@link FAULT_PARAMETERS_MAXIMUM}. */
function faultTextViolations(path: string, line: number, code: string, en: string, de: string): FaultViolation[] {
  const found: FaultViolation[] = [];
  if (en.trim().length === 0 || de.trim().length === 0) found.push({ path, line, rule: "text-empty", detail: `${code}: an empty en or de text` });
  else if (en === de) found.push({ path, line, rule: "text-untranslated", detail: `${code}: the de text is the en text` });
  const english = faultTextPlaceholders(en);
  const german = faultTextPlaceholders(de);
  if (english.malformed.length + german.malformed.length > 0 || [...english.names].sort().join(",") !== [...german.names].sort().join(",") || english.names.length > FAULT_PARAMETERS_MAXIMUM) {
    found.push({ path, line, rule: "placeholder-mismatch", detail: `${code}: en {${english.names.join(",")}} vs de {${german.names.join(",")}}${english.malformed.length + german.malformed.length > 0 ? `, malformed ${[...english.malformed, ...german.malformed].join(" ")}` : ""}` });
  }
  return found;
}

/** ⚖️ The global fault law over the facts of every production source and the framework catalog: every raised code is
 * well-formed and declared — by its plugin's app definitions, or in the catalog for a framework crate — with texts whose
 * placeholders are exactly the parameters every raise chains; every declaration and catalog entry is raised; a plugin
 * declares one code one way. */
export function faultLawViolations(sources: readonly Readonly<{ path: string; facts: FaultFacts }>[], catalog: unknown): FaultViolation[] {
  const violations: FaultViolation[] = sources.flatMap((source) => source.facts.violations);
  const consts = new Map<string, Set<string>>();
  for (const source of sources) for (const [name, value] of source.facts.consts) consts.set(name, (consts.get(name) ?? new Set()).add(value));
  const raises: FaultRaise[] = sources.flatMap((source) => source.facts.raises);
  for (const source of sources) {
    for (const reference of source.facts.constRaises) {
      const values = consts.get(reference.name);
      if (!values) violations.push({ path: reference.path, line: reference.line, rule: "unresolved-const", detail: `FaultCode::new(${reference.name}) binds no &str literal the census scanned` });
      else for (const value of values) raises.push({ path: reference.path, line: reference.line, code: value, parameters: reference.parameters });
    }
  }
  for (const raise of raises) {
    if (!FAULT_CODE.test(raise.code) || utf8Bytes(raise.code) > FAULT_CODE_BYTES) violations.push({ path: raise.path, line: raise.line, rule: "code-grammar", detail: `"${raise.code}" is not [A-Za-z0-9._/-] within ${FAULT_CODE_BYTES} bytes` });
    const bad = raise.parameters.filter((name) => !FAULT_PARAMETER_NAME.test(name) || utf8Bytes(name) > FAULT_PARAMETER_NAME_BYTES);
    if (bad.length > 0 || raise.parameters.length > FAULT_PARAMETERS_MAXIMUM) violations.push({ path: raise.path, line: raise.line, rule: "parameter-grammar", detail: `${raise.code}: parameters ${raise.parameters.join(",")}` });
  }
  const declared = new Map<string, Map<string, FaultDeclarationSite>>();
  for (const source of sources) {
    for (const declaration of source.facts.declarations) {
      const owner = faultOwnerOfPath(declaration.path);
      if (owner.kind !== "app") continue;
      const plugin = declared.get(owner.plugin) ?? new Map<string, FaultDeclarationSite>();
      declared.set(owner.plugin, plugin);
      const first = plugin.get(declaration.code);
      if (!FAULT_CODE.test(declaration.code) || utf8Bytes(declaration.code) > FAULT_CODE_BYTES) violations.push({ path: declaration.path, line: declaration.line, rule: "code-grammar", detail: `"${declaration.code}" is not [A-Za-z0-9._/-] within ${FAULT_CODE_BYTES} bytes` });
      if (first && (first.en !== declaration.en || first.de !== declaration.de)) violations.push({ path: declaration.path, line: declaration.line, rule: "declaration-conflict", detail: `${declaration.code} is declared otherwise at ${first.path}:${first.line}` });
      if (!first) {
        plugin.set(declaration.code, declaration);
        violations.push(...faultTextViolations(declaration.path, declaration.line, declaration.code, declaration.en, declaration.de));
      }
    }
  }
  const entries = new Map<string, FaultCatalogEntry>();
  const catalogShape = catalog !== null && typeof catalog === "object" && (catalog as { schema?: unknown }).schema === "semio.fault-catalog.v1" && Array.isArray((catalog as { faults?: unknown }).faults);
  if (!catalogShape) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-shape", detail: "no semio.fault-catalog.v1 document" });
  for (const entry of catalogShape ? ((catalog as { faults: unknown[] }).faults) : []) {
    const { code, en, de } = (entry ?? {}) as Record<string, unknown>;
    if (typeof code !== "string" || typeof en !== "string" || typeof de !== "string") {
      violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-shape", detail: `entry ${JSON.stringify(entry).slice(0, 120)}` });
      continue;
    }
    if (entries.has(code)) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "catalog-duplicate", detail: code });
    entries.set(code, { code, en, de });
    violations.push(...faultTextViolations(FAULT_CATALOG_REL_PATH, 0, code, en, de));
  }
  const raisedByPlugin = new Map<string, Set<string>>();
  const raisedByFramework = new Set<string>();
  for (const raise of raises) {
    const owner = faultOwnerOfPath(raise.path);
    const text = owner.kind === "app" ? declared.get(owner.plugin)?.get(raise.code) : entries.get(raise.code);
    if (owner.kind === "app") raisedByPlugin.set(owner.plugin, (raisedByPlugin.get(owner.plugin) ?? new Set()).add(raise.code));
    else raisedByFramework.add(raise.code);
    if (!text) {
      violations.push({ path: raise.path, line: raise.line, rule: owner.kind === "app" ? "undeclared" : "uncatalogued", detail: owner.kind === "app" ? `${raise.code} is declared by no app definition of ${owner.plugin}` : `${raise.code} is not in the framework fault catalog` });
      continue;
    }
    const placeholders = faultTextPlaceholders(text.en).names;
    if ([...placeholders].sort().join(",") !== [...raise.parameters].sort().join(",")) violations.push({ path: raise.path, line: raise.line, rule: "parameter-mismatch", detail: `${raise.code} raises {${raise.parameters.join(",")}} but its text shows {${placeholders.join(",")}}` });
  }
  for (const [plugin, codes] of declared) for (const declaration of codes.values()) if (!raisedByPlugin.get(plugin)?.has(declaration.code)) violations.push({ path: declaration.path, line: declaration.line, rule: "unraised-declaration", detail: `${declaration.code} is raised by no source of ${plugin}` });
  for (const entry of entries.values()) if (!raisedByFramework.has(entry.code)) violations.push({ path: FAULT_CATALOG_REL_PATH, line: 0, rule: "unraised-catalog-entry", detail: `${entry.code} is raised by no framework source` });
  return violations;
}

/** 🔢️ The raw occurrences of the tokens the fault census reads, per source — the unit the `git grep -o` oracle counts. */
export const FAULT_CENSUS_TOKENS = ["app_fault(", "Fault::from(", "FaultCode::new(", ".fault(\""] as const;

/** 📊️ The fault census over every production Rust source (tracked or untracked, never ignored; outside the ticket tree
 * and test paths) plus the framework catalog, the file coverage cross-checked against `git grep -o` (the oracle): every
 * source's count of each {@link FAULT_CENSUS_TOKENS} token equals the scanner's. Progress every 2000 files; the signal
 * stops the walk. Runs on an overlay work tree through `GIT_DIR` + `GIT_WORK_TREE`. */
export function runFaultCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const pathspec = ["--", "*.rs", ":!.🧬semio"];
  const listed = spawnSync("git", ["-c", "core.quotePath=false", "ls-files", "-z", "-c", "-o", "--exclude-standard", ...pathspec], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  const paths = [...new Set(listed.stdout.split("\0").filter((path) => path && !TEST_SEGMENT.test(path)))];
  const sources: { path: string; facts: FaultFacts }[] = [];
  const scanned = new Map<string, number>();
  for (const [index, path] of paths.entries()) {
    if (signal.aborted) throw new Error("fault census cancelled");
    if (index % 2000 === 0) onProgress(`${index}/${paths.length} Rust sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, path), "utf8");
    } catch {
      continue;
    }
    for (const token of FAULT_CENSUS_TOKENS) {
      const count = text.split(token).length - 1;
      if (count > 0) scanned.set(`${path}\0${token}`, count);
    }
    if (/app_fault|Fault|\.fault\s*\(|fault_from_error!|const\s+[A-Z]/u.test(text)) sources.push({ path, facts: faultFactsOfText(path, text) });
  }
  let catalog: unknown = null;
  try {
    catalog = JSON.parse(readFileSync(join(repoRoot, FAULT_CATALOG_REL_PATH), "utf8"));
  } catch {
    catalog = null;
  }
  const violations = faultLawViolations(sources, catalog);
  const production = new Set(paths);
  const oracle = new Map<string, number>();
  for (const token of FAULT_CENSUS_TOKENS) {
    const rows = spawnSync("git", ["-c", "core.quotePath=false", "grep", "--untracked", "-o", "-F", "-e", token, ...pathspec], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\n").filter(Boolean);
    for (const row of rows) {
      const path = row.slice(0, row.length - token.length - 1);
      if (production.has(path)) oracle.set(`${path}\0${token}`, (oracle.get(`${path}\0${token}`) ?? 0) + 1);
    }
  }
  const disagreements = [...new Set([...oracle.keys(), ...scanned.keys()])].filter((key) => (oracle.get(key) ?? 0) !== (scanned.get(key) ?? 0)).map((key) => key.replace("\0", " "));
  onProgress(`${paths.length}/${paths.length} Rust sources scanned`);
  const raises = sources.flatMap((source) => source.facts.raises);
  const declarations = sources.flatMap((source) => source.facts.declarations);
  return { files: paths.length, raises, declarations, catalogEntries: Array.isArray((catalog as { faults?: unknown } | null)?.faults) ? (catalog as { faults: unknown[] }).faults.length : 0, violations, disagreements };
}
