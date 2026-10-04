# TextError Actual TypeScript Owner Production

2026-10-03T00:04:43.571Z

The exact three owned TypeScript sources are authored and absent-before captured. This follows strict missing-owner TS2307 RED and native missing-kind compiler RED10, neither claiming law runtime. Rust kind production remains absent during downstream tests-first work. New own APIs require explicit actual Value-owned kind and exact copied span, with no external runtime dependencies, string classification or implicit source position.

## 🧰️framework/🔨️modules/⚠️diagnostic/🚧️text-error/🟦️.ts

Before / inverse: absent; remove only this new source.

Full production source:

```
import { ValueError, type ValueRefusalKind } from "../../🌱️value/⚠️refusal/🟦️.ts";
import type { TextSpan } from "../📍️span/🟦️.ts";
/** 🧾️ Closed wire projection of a typed source-positioned error. */
export interface TextErrorWire { readonly kind: ValueRefusalKind; readonly message: string; readonly span: TextSpan; readonly expected?: string }
/** 🚧️ An owned error retains refusal authority independently from its display prose. */
export class TextError extends Error {
  readonly span: TextSpan;
  constructor(public readonly kind: ValueRefusalKind, message: string, span: TextSpan, public readonly expected?: string) { super(message); this.name = "TextError"; this.span = { line: span.line, column: span.column, length: span.length }; }
  /** 🧭️ Requires the caller's source position while retaining the original refusal authority. */
  static fromValueError(error: ValueError, span: TextSpan): TextError { return new TextError(error.kind, error.message, span); }
  /** 🧩️ Adds explicit expected syntax without inferring a refusal kind. */
  static expected(kind: ValueRefusalKind, message: string, span: TextSpan, expected: string): TextError { return new TextError(kind, message, span, expected); }
  /** 🧬️ Publishes the mandatory owned kind and exact optional expected syntax. */
  toWire(): TextErrorWire { return { kind: this.kind, message: this.message, span: { line: this.span.line, column: this.span.column, length: this.span.length }, ...(this.expected === undefined ? {} : { expected: this.expected }) }; }
  /** 🔤️ Matches native source-positioned error display. */
  override toString(): string { return `${this.message} at ${this.span.line}:${this.span.column}`; }
}

```

## 🧰️framework/🔨️modules/⚠️diagnostic/📍️span/🟦️.ts

Before / inverse: absent; remove only this new source.

Full production source:

```
/** 📍️ The owned diagnostic source position declared by its closed schema. */
export interface TextSpan { readonly line: number; readonly column: number; readonly length: number }

```

## 🧰️framework/🔨️modules/⚠️diagnostic/🟦️.ts

Before / inverse: absent; remove only this new source.

Full production source:

```
export { TextError, type TextErrorWire } from "./🚧️text-error/🟦️.ts";
export type { TextSpan } from "./📍️span/🟦️.ts";

```

Actual registered strict/noUnchecked TypeScript0diagnostics then Bun2/2/0fail GREEN,80ms runtime/3.9s Nx. DEBUG confirms eight typed source kinds, exact copied span and dotted message. 32 scalar wire/display projections compare independent Ajv and SQLite; four malformed wire cases are schema assertions. The actual copied span is verified after mutating the caller's original source cursor. Eleven exact relevant source/hash rows are captured; three mounted owner hashes remain unchanged, zero gaps. Rust production remains absent, so this does not claim native GREEN or a TS controlled Diagnostic codec. [Full receipt](🗑️generated/value-refusal/text-error-typescript-production-green-1-receipt.json).
