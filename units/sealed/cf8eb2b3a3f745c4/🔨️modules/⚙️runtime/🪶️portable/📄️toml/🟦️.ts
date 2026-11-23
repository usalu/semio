import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
const BARE = /[A-Za-z0-9_-]/u;
const ESCAPES: Readonly<Record<string, string>> = { b: "\b", t: "\t", n: "\n", f: "\f", r: "\r", '"': '"', "\\": "\\" };
const DATE_TIME = /^(\d{4}-\d{2}-\d{2})(?:[Tt ](\d{2}:\d{2}:\d{2}(?:\.\d+)?)([Zz]|[+-]\d{2}:\d{2})?)?$/u;
const LOCAL_TIME = /^\d{2}:\d{2}:\d{2}(?:\.\d+)?$/u;

type TableV1 = Record<string, unknown>;

class TomlParserV1 {
  private index = 0;
  private readonly explicit = new WeakSet<object>();
  private readonly closed = new WeakSet<object>();
  private readonly dotted = new WeakSet<object>();
  private readonly text: string;
  constructor(text: string) { this.text = text; }

  private fail(id: string, parameters: Readonly<Record<string, string | number>> = {}): never {
    let line = 1, column = 1;
    for (let at = 0; at < this.index && at < this.text.length; at++) { if (this.text[at] === "\n") { line++; column = 1; } else column++; }
    throw new SyntaxError(runtimeMessageV1("syntax", { format: "TOML", detail: id, position: `${line}:${column}`, ...parameters }));
  }

  private peek(offset = 0): string | undefined { return this.text[this.index + offset]; }
  private blank(): void { while (this.peek() === " " || this.peek() === "\t") this.index++; }
  private comment(): void { if (this.peek() === "#") while (this.index < this.text.length && this.peek() !== "\n") { if (this.peek() === "\r" && this.peek(1) === "\n") break; this.index++; } }
  private skipSpace(): void {
    for (;;) {
      this.blank(); this.comment();
      const character = this.peek();
      if (character === "\n") this.index++;
      else if (character === "\r" && this.peek(1) === "\n") this.index += 2;
      else return;
    }
  }
  private endOfLine(): void {
    this.blank(); this.comment();
    const character = this.peek();
    if (character === undefined) return;
    if (character === "\n") { this.index++; return; }
    if (character === "\r" && this.peek(1) === "\n") { this.index += 2; return; }
    this.fail("toml.expectedEndOfLine");
  }

  parse(): TableV1 {
    const root: TableV1 = {};
    let current = root;
    for (;;) {
      this.skipSpace();
      if (this.index >= this.text.length) return root;
      if (this.peek() === "[") {
        const array = this.peek(1) === "[";
        this.index += array ? 2 : 1;
        this.blank();
        const path = this.keyPath();
        this.blank();
        if (array ? !(this.peek() === "]" && this.peek(1) === "]") : this.peek() !== "]") this.fail("toml.expectedBracket");
        this.index += array ? 2 : 1;
        current = array ? this.arrayTable(root, path) : this.table(root, path);
        this.endOfLine();
        continue;
      }
      const path = this.keyPath();
      this.blank();
      if (this.peek() !== "=") this.fail("toml.expectedEquals");
      this.index++;
      this.blank();
      this.assign(current, path, this.value());
      this.endOfLine();
    }
  }

  private keyPath(): string[] {
    const path: string[] = [];
    for (;;) {
      this.blank();
      path.push(this.key());
      this.blank();
      if (this.peek() === ".") { this.index++; continue; }
      return path;
    }
  }

  private key(): string {
    const character = this.peek();
    if (character === '"') return this.basicString();
    if (character === "'") return this.literalString();
    const start = this.index;
    while (this.index < this.text.length && BARE.test(this.text[this.index]!)) this.index++;
    if (start === this.index) this.fail("toml.expectedKey");
    return this.text.slice(start, this.index);
  }

  private child(parent: TableV1, name: string): TableV1 {
    const existing = Object.hasOwn(parent, name) ? parent[name] : undefined;
    if (existing === undefined) { const created: TableV1 = {}; Object.defineProperty(parent, name, { value: created, enumerable: true, writable: true, configurable: true }); return created; }
    if (Array.isArray(existing)) { const last = existing.at(-1); if (last !== null && typeof last === "object" && !Array.isArray(last)) return last as TableV1; }
    if (existing !== null && typeof existing === "object" && !Array.isArray(existing) && !(existing instanceof Date)) return existing as TableV1;
    return this.fail("toml.notATable", { name });
  }

  private table(root: TableV1, path: string[]): TableV1 {
    let current = root;
    for (const name of path.slice(0, -1)) current = this.child(current, name);
    const name = path.at(-1)!, existed = Object.hasOwn(current, name), target = this.child(current, name);
    if (existed && (this.explicit.has(target) || this.dotted.has(target) || this.closed.has(target))) this.fail("toml.tableDefined", { name: path.join(".") });
    this.explicit.add(target);
    return target;
  }

  private arrayTable(root: TableV1, path: string[]): TableV1 {
    let current = root;
    for (const name of path.slice(0, -1)) current = this.child(current, name);
    const name = path.at(-1)!, existing = Object.hasOwn(current, name) ? current[name] : undefined;
    if (existing === undefined) { Object.defineProperty(current, name, { value: [], enumerable: true, writable: true, configurable: true }); }
    else if (!Array.isArray(existing) || this.closed.has(existing)) this.fail("toml.notAnArrayOfTables", { name });
    const entry: TableV1 = {};
    (current[name] as unknown[]).push(entry);
    this.explicit.add(entry);
    return entry;
  }

  private assign(table: TableV1, path: string[], value: unknown): void {
    let current = table;
    for (const name of path.slice(0, -1)) {
      const existing = Object.hasOwn(current, name) ? current[name] : undefined;
      if (existing === undefined) { const created: TableV1 = {}; Object.defineProperty(current, name, { value: created, enumerable: true, writable: true, configurable: true }); this.dotted.add(created); current = created; continue; }
      if (existing === null || typeof existing !== "object" || Array.isArray(existing) || existing instanceof Date || this.closed.has(existing)) this.fail("toml.notATable", { name });
      current = existing as TableV1;
    }
    const name = path.at(-1)!;
    if (Object.hasOwn(current, name)) this.fail("toml.keyDefined", { name });
    Object.defineProperty(current, name, { value, enumerable: true, writable: true, configurable: true });
  }

  private value(): unknown {
    const character = this.peek();
    if (character === '"') return this.peek(1) === '"' && this.peek(2) === '"' ? this.multilineBasic() : this.basicString();
    if (character === "'") return this.peek(1) === "'" && this.peek(2) === "'" ? this.multilineLiteral() : this.literalString();
    if (character === "[") return this.array();
    if (character === "{") return this.inlineTable();
    return this.scalar();
  }

  private array(): unknown[] {
    this.index++;
    const items: unknown[] = [];
    for (;;) {
      this.skipSpace();
      if (this.peek() === "]") { this.index++; this.closed.add(items); return items; }
      items.push(this.value());
      this.skipSpace();
      if (this.peek() === ",") { this.index++; continue; }
      if (this.peek() !== "]") this.fail("toml.expectedCommaOrBracket");
    }
  }

  private inlineTable(): TableV1 {
    this.index++;
    const table: TableV1 = {};
    this.blank();
    if (this.peek() === "}") { this.index++; this.closed.add(table); return table; }
    for (;;) {
      this.blank();
      const path = this.keyPath();
      this.blank();
      if (this.peek() !== "=") this.fail("toml.expectedEquals");
      this.index++;
      this.blank();
      this.assign(table, path, this.value());
      this.blank();
      if (this.peek() === ",") { this.index++; continue; }
      if (this.peek() === "}") { this.index++; this.closed.add(table); return table; }
      this.fail("toml.expectedCommaOrBrace");
    }
  }

  private basicString(): string {
    this.index++;
    let output = "";
    for (;;) {
      const character = this.peek();
      if (character === undefined || character === "\n") this.fail("toml.unterminatedString");
      this.index++;
      if (character === '"') return output;
      if (character === "\\") { output += this.escape(); continue; }
      if (character < " " && character !== "\t" || character === "\u007f") this.fail("toml.controlCharacter");
      output += character;
    }
  }

  private escape(): string {
    const code = this.peek();
    this.index++;
    if (code !== undefined && code in ESCAPES) return ESCAPES[code]!;
    if (code === "u" || code === "U") {
      const width = code === "u" ? 4 : 8, digits = this.text.slice(this.index, this.index + width);
      if (digits.length !== width || !/^[0-9A-Fa-f]+$/u.test(digits)) this.fail("toml.invalidUnicodeEscape");
      const point = Number.parseInt(digits, 16);
      if (point > 0x10ffff || point >= 0xd800 && point <= 0xdfff) this.fail("toml.invalidUnicodeScalar");
      this.index += width;
      return String.fromCodePoint(point);
    }
    return this.fail("toml.invalidEscape");
  }

  private literalString(): string {
    this.index++;
    const start = this.index;
    while (this.index < this.text.length && this.text[this.index] !== "'" && this.text[this.index] !== "\n") this.index++;
    if (this.peek() !== "'") this.fail("toml.unterminatedLiteral");
    return this.text.slice(start, this.index++);
  }

  private multilineBasic(): string {
    this.index += 3;
    if (this.peek() === "\r" && this.peek(1) === "\n") this.index += 2; else if (this.peek() === "\n") this.index++;
    let output = "";
    for (;;) {
      if (this.index >= this.text.length) this.fail("toml.unterminatedMultiline");
      const character = this.text[this.index]!;
      if (character === '"' && this.peek(1) === '"' && this.peek(2) === '"') {
        let run = 3;
        while (this.peek(run) === '"') run++;
        if (run > 5) this.fail("toml.tooManyQuotes");
        this.index += run;
        return output + '"'.repeat(run - 3);
      }
      this.index++;
      if (character === "\\") {
        const next = this.peek();
        if (next === "\n" || next === "\r" || next === " " || next === "\t") {
          let probe = this.index;
          while (this.text[probe] === " " || this.text[probe] === "\t") probe++;
          if (this.text[probe] === "\r" && this.text[probe + 1] === "\n") probe++;
          if (this.text[probe] === "\n") { this.index = probe + 1; while (/[\s]/u.test(this.text[this.index] ?? "x")) this.index++; continue; }
        }
        output += this.escape();
        continue;
      }
      if (character === "\r" && this.peek() === "\n") continue;
      output += character;
    }
  }

  private multilineLiteral(): string {
    this.index += 3;
    if (this.peek() === "\r" && this.peek(1) === "\n") this.index += 2; else if (this.peek() === "\n") this.index++;
    const start = this.index;
    for (;;) {
      if (this.index >= this.text.length) this.fail("toml.unterminatedMultilineLiteral");
      if (this.text[this.index] === "'" && this.text[this.index + 1] === "'" && this.text[this.index + 2] === "'") {
        let run = 3;
        while (this.peek(run) === "'") run++;
        if (run > 5) this.fail("toml.tooManyQuotes");
        const content = this.text.slice(start, this.index) + "'".repeat(run - 3);
        this.index += run;
        return content.replace(/\r\n/gu, "\n");
      }
      this.index++;
    }
  }

  private scalar(): unknown {
    const start = this.index;
    while (this.index < this.text.length && !/[\s,\]}#]/u.test(this.text[this.index]!)) this.index++;
    let token = this.text.slice(start, this.index);
    if (token === "true") return true;
    if (token === "false") return false;
    if (/^\d{4}-\d{2}-\d{2}$/u.test(token) && this.peek() === " " && /^\d{2}:/u.test(this.text.slice(this.index + 1, this.index + 4))) {
      const timeStart = this.index + 1;
      let end = timeStart;
      while (end < this.text.length && !/[\s,\]}#]/u.test(this.text[end]!)) end++;
      token = `${token} ${this.text.slice(timeStart, end)}`;
      this.index = end;
    }
    const date = DATE_TIME.exec(token);
    if (date !== null) {
      if (date[2] === undefined) return token;
      if (date[3] === undefined) return token;
      const parsed = new Date(`${date[1]}T${date[2]}${date[3].toUpperCase()}`);
      if (Number.isNaN(parsed.getTime())) this.fail("toml.invalidDateTime");
      return parsed;
    }
    if (LOCAL_TIME.test(token)) return token;
    if (/^[+-]?(?:inf|nan)$/u.test(token)) return token.endsWith("nan") ? Number.NaN : token.startsWith("-") ? Number.NEGATIVE_INFINITY : Number.POSITIVE_INFINITY;
    const radix = /^0([xob])([0-9A-Fa-f](?:_?[0-9A-Fa-f])*)$/u.exec(token);
    if (radix !== null) {
      const base = radix[1] === "x" ? 16 : radix[1] === "o" ? 8 : 2, digits = radix[2]!.replaceAll("_", "");
      if (!new RegExp(`^[${base === 16 ? "0-9A-Fa-f" : base === 8 ? "0-7" : "01"}]+$`, "u").test(digits)) this.fail("toml.invalidInteger");
      const value = BigInt((radix[1] === "x" ? "0x" : radix[1] === "o" ? "0o" : "0b") + digits);
      return value <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(value) : value;
    }
    if (/^[+-]?(?:0|[1-9](?:_?\d)*)$/u.test(token)) {
      const value = BigInt(token.replaceAll("_", ""));
      return value >= BigInt(Number.MIN_SAFE_INTEGER) && value <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(value) : value;
    }
    if (/^[+-]?(?:0|[1-9](?:_?\d)*)(?:\.\d(?:_?\d)*)?(?:[eE][+-]?\d(?:_?\d)*)?$/u.test(token) && /[.eE]/u.test(token)) return Number(token.replaceAll("_", ""));
    return this.fail("toml.invalidValue", { token: JSON.stringify(token) });
  }
}

/** 📄️ Parses TOML 1.0 text into plain objects, arrays, numbers (bigint beyond 53 bits), booleans, strings and dates; the portable counterpart of a runtime's native parser, with `@iarna/toml` as test oracle. */
export function parseTomlV1(text: string): any {
  return new TomlParserV1(text).parse();
}
