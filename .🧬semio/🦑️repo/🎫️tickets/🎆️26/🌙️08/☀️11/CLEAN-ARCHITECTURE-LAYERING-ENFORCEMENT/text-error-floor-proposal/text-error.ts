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
