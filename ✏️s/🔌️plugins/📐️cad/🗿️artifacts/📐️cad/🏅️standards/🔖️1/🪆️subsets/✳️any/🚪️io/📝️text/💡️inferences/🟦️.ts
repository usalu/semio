/** 📝️ Text representation for `cad.cad.inference`. */
export type CadInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadInferenceTextGuardReject = (at: string, why: string): never => {
  throw new cadCadInferenceTextGuardRefusal(at, why);
};

type cadCadInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadInferenceTextGuardReject(at, "value is not an object");
export const cadCadInferenceTextGuardArray = (value: unknown, at: string, bounds: cadCadInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadInferenceTextGuardString = (value: unknown, at: string, bounds: cadCadInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadInferenceTextGuardReject(at, "value is not a boolean"));
export const cadCadInferenceTextGuardNumber = (value: unknown, at: string, bounds: cadCadInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadInferenceTextGuardInteger = (value: unknown, at: string, bounds: cadCadInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadInferenceTextGuardNumber(value, at, bounds) : cadCadInferenceTextGuardReject(at, "value is not an integer");
export const cadCadInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCadInferenceText(value: unknown, at = "$"): CadInferenceText {
  return cadCadInferenceTextGuardString(value, at);
}

/** 🔍️ Construct DSL physical text admission; third-party CST stays private. */

import { CstParser, createToken, Lexer } from "chevrotain";
import type { CstElement, CstNode, IToken } from "chevrotain";
import { emptyMeshTransfer, solidRef } from "@semio-tech/framework-3d-js";
import { Model, type Expr, type ExprBinop, type ExprField, type ExprVar } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { applyModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { modelDefinitionActionRegistry, type ConstructQueryContext, type ConstructQueryResult, type ConstructRunner } from "../../../✏️editor/⚙️engine/🎬️actions/🟦️.ts";
import { assertConstructAst, runConstructAst, type ConstructAst, type ConstructClauseAst, type NodePatternAst, type RelPatternAst, type PatternAst, type PatternElementAst, type ReturnClauseAst, type YieldItemAst } from "../../../🧬️schema/💡️inferences/🟦️.ts";

// #region Lexer
const WhiteSpace = createToken({
  name: "WhiteSpace",
  pattern: /\s+/,
  group: Lexer.SKIPPED,
});

const MatchKw = createToken({ name: "MatchKw", pattern: /MATCH/i });
const WhereKw = createToken({ name: "WhereKw", pattern: /WHERE/i });
const ReturnKw = createToken({ name: "ReturnKw", pattern: /RETURN/i });
const CallKw = createToken({ name: "CallKw", pattern: /CALL/i });
const YieldKw = createToken({ name: "YieldKw", pattern: /YIELD/i });
const WithKw = createToken({ name: "WithKw", pattern: /WITH/i });
const OrderKw = createToken({ name: "OrderKw", pattern: /ORDER/i });
const ByKw = createToken({ name: "ByKw", pattern: /BY/i });
const LimitKw = createToken({ name: "LimitKw", pattern: /LIMIT/i });
const UnwindKw = createToken({ name: "UnwindKw", pattern: /UNWIND/i });
const AsKw = createToken({ name: "AsKw", pattern: /AS/i });
const AndKw = createToken({ name: "AndKw", pattern: /AND/i });
const OrKw = createToken({ name: "OrKw", pattern: /OR/i });

const LParen = createToken({ name: "LParen", pattern: /\(/ });
const RParen = createToken({ name: "RParen", pattern: /\)/ });
const LBrace = createToken({ name: "LBrace", pattern: /\{/ });
const RBrace = createToken({ name: "RBrace", pattern: /\}/ });
const LBracket = createToken({ name: "LBracket", pattern: /\[/ });
const RBracket = createToken({ name: "RBracket", pattern: /\]/ });
const Comma = createToken({ name: "Comma", pattern: /,/ });
const Colon = createToken({ name: "Colon", pattern: /:/ });
const Dot = createToken({ name: "Dot", pattern: /\./ });
const Minus = createToken({ name: "Minus", pattern: /-/ });
const Lt = createToken({ name: "Lt", pattern: /</ });
const Gt = createToken({ name: "Gt", pattern: />/ });
const Pipe = createToken({ name: "Pipe", pattern: /\|/ });
const Star = createToken({ name: "Star", pattern: /\*/ });
const EqEq = createToken({ name: "EqEq", pattern: /==/ });
const Neq = createToken({ name: "Neq", pattern: /!=/ });
const Lte = createToken({ name: "Lte", pattern: /<=/ });
const Gte = createToken({ name: "Gte", pattern: />=/ });
const Eq = createToken({ name: "Eq", pattern: /=/ });
const Plus = createToken({ name: "Plus", pattern: /\+/ });
const Slash = createToken({ name: "Slash", pattern: /\// });

const StringLit = createToken({ name: "StringLit", pattern: /"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'/ });
const IntegerLit = createToken({ name: "IntegerLit", pattern: /-?\d+/ });
const FloatLit = createToken({ name: "FloatLit", pattern: /-?\d+\.\d+/ });

const Identifier = createToken({ name: "Identifier", pattern: /[a-zA-Z_][a-zA-Z0-9_]*/ });

const allTokens = [
  WhiteSpace,
  MatchKw,
  WhereKw,
  ReturnKw,
  CallKw,
  YieldKw,
  WithKw,
  OrderKw,
  ByKw,
  LimitKw,
  UnwindKw,
  AsKw,
  AndKw,
  OrKw,
  EqEq,
  Neq,
  Lte,
  Gte,
  FloatLit,
  IntegerLit,
  StringLit,
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Comma,
  Colon,
  Dot,
  Minus,
  Lt,
  Gt,
  Pipe,
  Star,
  Eq,
  Plus,
  Slash,
  Identifier,
];

const constructLexer = new Lexer(allTokens);
// #endregion Lexer

// #region Parser
class ConstructParser extends CstParser {
  constructor() {
    super(allTokens, { recoveryEnabled: false });
    this.performSelfAnalysis();
  }

  readonly query = this.RULE("query", () => {
    this.MANY(() => {
      this.OR([{ ALT: () => this.SUBRULE(this.matchClause) }, { ALT: () => this.SUBRULE(this.withClause) }, { ALT: () => this.SUBRULE(this.callClause) }, { ALT: () => this.SUBRULE(this.unwindClause) }]);
    });
    this.OPTION(() => this.SUBRULE(this.returnClause));
  });

  readonly matchClause = this.RULE("matchClause", () => {
    this.CONSUME(MatchKw);
    this.SUBRULE(this.patternList);
    this.OPTION(() => {
      this.CONSUME(WhereKw);
      this.SUBRULE(this.expr);
    });
  });

  readonly withClause = this.RULE("withClause", () => {
    this.CONSUME(WithKw);
    this.SUBRULE(this.projectList);
    this.OPTION(() => {
      this.CONSUME(WhereKw);
      this.SUBRULE(this.expr);
    });
  });

  readonly callClause = this.RULE("callClause", () => {
    this.CONSUME(CallKw);
    this.SUBRULE(this.actionId);
    this.CONSUME(LParen);
    this.OPTION(() => this.SUBRULE(this.objectLiteralExpr));
    this.CONSUME(RParen);
    this.OPTION1(() => this.SUBRULE(this.yieldClause));
  });

  readonly unwindClause = this.RULE("unwindClause", () => {
    this.CONSUME(UnwindKw);
    this.SUBRULE(this.expr);
    this.CONSUME(AsKw);
    this.CONSUME(Identifier);
    this.OPTION(() => {
      this.CONSUME(WhereKw);
      this.SUBRULE1(this.expr);
    });
  });

  readonly yieldClause = this.RULE("yieldClause", () => {
    this.CONSUME(YieldKw);
    this.SUBRULE(this.yieldItem);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.SUBRULE1(this.yieldItem);
    });
  });

  readonly yieldItem = this.RULE("yieldItem", () => {
    this.SUBRULE(this.yieldKey);
    this.OPTION(() => {
      this.CONSUME(AsKw);
      this.CONSUME(Identifier);
    });
  });

  readonly yieldKey = this.RULE("yieldKey", () => {
    this.CONSUME(Identifier);
    this.MANY(() => {
      this.CONSUME(Dot);
      this.CONSUME1(Identifier);
    });
  });

  readonly returnClause = this.RULE("returnClause", () => {
    this.CONSUME(ReturnKw);
    this.SUBRULE(this.projectList);
    this.OPTION(() => {
      this.CONSUME(OrderKw);
      this.CONSUME(ByKw);
      this.SUBRULE(this.orderExpr);
    });
    this.OPTION1(() => {
      this.CONSUME(LimitKw);
      this.SUBRULE(this.returnLimitLit);
    });
  });

  readonly returnLimitLit = this.RULE("returnLimitLit", () => {
    this.CONSUME(IntegerLit);
  });

  readonly patternList = this.RULE("patternList", () => {
    this.SUBRULE(this.pattern);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.SUBRULE1(this.pattern);
    });
  });

  readonly pattern = this.RULE("pattern", () => {
    this.SUBRULE(this.nodePattern);
    this.MANY(() => {
      this.SUBRULE(this.relPattern);
      this.SUBRULE1(this.nodePattern);
    });
  });

  readonly nodePattern = this.RULE("nodePattern", () => {
    this.CONSUME(LParen);
    this.OPTION(() => this.CONSUME(Identifier));
    this.OPTION1(() => {
      this.CONSUME(Colon);
      this.CONSUME1(Identifier);
    });
    this.OPTION2(() => this.SUBRULE(this.propMap));
    this.CONSUME(RParen);
  });

  readonly relPattern = this.RULE("relPattern", () => {
    this.OR([{ ALT: () => this.SUBRULE(this.relPatternIn) }, { ALT: () => this.SUBRULE(this.relPatternOutOrUndirected) }]);
  });

  readonly relPatternIn = this.RULE("relPatternIn", () => {
    this.CONSUME(Lt);
    this.CONSUME(Minus);
    this.SUBRULE(this.relBracket);
  });

  readonly relPatternOutOrUndirected = this.RULE("relPatternOutOrUndirected", () => {
    this.CONSUME(Minus);
    this.SUBRULE(this.relBracket);
    const t1 = this.LA(1);
    const t2 = this.LA(2);
    if (t1.tokenType === Minus && t2.tokenType === Gt) {
      this.CONSUME(Minus);
      this.CONSUME(Gt);
    } else if (t1.tokenType === Minus) {
      this.CONSUME(Minus);
    } else {
      this.CONSUME(Gt);
    }
  });

  readonly relBracket = this.RULE("relBracket", () => {
    this.CONSUME(LBracket);
    this.CONSUME(Colon);
    this.CONSUME(Identifier);
    this.MANY(() => {
      this.CONSUME(Pipe);
      this.CONSUME1(Identifier);
    });
    this.CONSUME(RBracket);
  });

  readonly propMap = this.RULE("propMap", () => {
    this.CONSUME(LBrace);
    this.CONSUME(Identifier);
    this.CONSUME(Colon);
    this.SUBRULE(this.literal);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.CONSUME1(Identifier);
      this.CONSUME1(Colon);
      this.SUBRULE1(this.literal);
    });
    this.CONSUME(RBrace);
  });

  readonly literal = this.RULE("literal", () => {
    this.OR([{ ALT: () => this.CONSUME(StringLit) }, { ALT: () => this.CONSUME(IntegerLit) }, { ALT: () => this.CONSUME(FloatLit) }]);
  });

  readonly actionId = this.RULE("actionId", () => {
    this.CONSUME(Identifier);
    this.MANY(() => {
      this.CONSUME(Dot);
      this.CONSUME1(Identifier);
    });
  });

  readonly projectList = this.RULE("projectList", () => {
    this.SUBRULE(this.projectItem);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.SUBRULE1(this.projectItem);
    });
  });

  readonly projectItem = this.RULE("projectItem", () => {
    this.SUBRULE(this.expr);
    this.OPTION(() => {
      this.CONSUME(AsKw);
      this.CONSUME(Identifier);
    });
  });

  readonly orderExpr = this.RULE("orderExpr", () => {
    this.SUBRULE(this.expr);
  });

  readonly objectLiteralExpr = this.RULE("objectLiteralExpr", () => {
    this.CONSUME(LBrace);
    this.OPTION(() => {
      this.CONSUME(Identifier);
      this.CONSUME(Colon);
      this.SUBRULE(this.valueLiteral);
      this.MANY(() => {
        this.CONSUME(Comma);
        this.CONSUME1(Identifier);
        this.CONSUME1(Colon);
        this.SUBRULE1(this.valueLiteral);
      });
    });
    this.CONSUME(RBrace);
  });

  readonly valueLiteral = this.RULE("valueLiteral", () => {
    this.OR([{ ALT: () => this.CONSUME(StringLit) }, { ALT: () => this.CONSUME(IntegerLit) }, { ALT: () => this.CONSUME(FloatLit) }, { ALT: () => this.SUBRULE(this.arrayLiteral) }, { ALT: () => this.SUBRULE1(this.objectLiteralExpr) }]);
  });

  readonly arrayLiteral = this.RULE("arrayLiteral", () => {
    this.CONSUME(LBracket);
    this.SUBRULE(this.valueLiteral);
    this.MANY(() => {
      this.CONSUME(Comma);
      this.SUBRULE1(this.valueLiteral);
    });
    this.CONSUME(RBracket);
  });

  readonly expr = this.RULE("expr", () => {
    this.SUBRULE(this.orExpr);
  });

  readonly orExpr = this.RULE("orExpr", () => {
    this.SUBRULE(this.andExpr);
    this.MANY(() => {
      this.CONSUME(OrKw);
      this.SUBRULE1(this.andExpr);
    });
  });

  readonly andExpr = this.RULE("andExpr", () => {
    this.SUBRULE(this.cmpExpr);
    this.MANY(() => {
      this.CONSUME(AndKw);
      this.SUBRULE1(this.cmpExpr);
    });
  });

  readonly cmpExpr = this.RULE("cmpExpr", () => {
    this.SUBRULE(this.addExpr);
    this.OPTION(() => {
      this.OR([
        { ALT: () => this.CONSUME(EqEq) },
        { ALT: () => this.CONSUME(Eq) },
        { ALT: () => this.CONSUME(Neq) },
        { ALT: () => this.CONSUME(Lte) },
        { ALT: () => this.CONSUME(Gte) },
        { ALT: () => this.CONSUME(Lt) },
        { ALT: () => this.CONSUME(Gt) },
      ]);
      this.SUBRULE1(this.addExpr);
    });
  });

  readonly addExpr = this.RULE("addExpr", () => {
    this.SUBRULE(this.mulExpr);
    this.MANY(() => {
      this.OR([{ ALT: () => this.CONSUME(Plus) }, { ALT: () => this.CONSUME(Minus) }]);
      this.SUBRULE1(this.mulExpr);
    });
  });

  readonly mulExpr = this.RULE("mulExpr", () => {
    this.SUBRULE(this.unaryExpr);
    this.MANY(() => {
      this.OR([{ ALT: () => this.CONSUME(Star) }, { ALT: () => this.CONSUME(Slash) }]);
      this.SUBRULE1(this.unaryExpr);
    });
  });

  readonly unaryExpr = this.RULE("unaryExpr", () => {
    this.OPTION(() => this.CONSUME(Minus));
    this.SUBRULE(this.primaryExpr);
  });

  readonly primaryExpr = this.RULE("primaryExpr", () => {
    this.OR([
      { ALT: () => this.CONSUME(StringLit) },
      { ALT: () => this.CONSUME(IntegerLit) },
      { ALT: () => this.CONSUME(FloatLit) },
      {
        ALT: () => {
          this.CONSUME(LParen);
          this.SUBRULE(this.expr);
          this.CONSUME(RParen);
        },
      },
      {
        ALT: () => {
          this.CONSUME1(Identifier);
          this.MANY(() => {
            this.CONSUME(Dot);
            this.CONSUME2(Identifier);
          });
        },
      },
    ]);
  });
}

const parserSingleton = new ConstructParser();
// #endregion Parser

function tokenText(t: IToken): string {
  return t.image;
}

/** 🪪️ Narrows a `CstElement` to its `IToken` case (a rule's terminal child). */
function asToken(e: CstElement | undefined): IToken | undefined {
  return e !== undefined && "tokenType" in e ? e : undefined;
}

/** 🪪️ Narrows a `CstElement` to its `CstNode` case (a rule's subrule child). */
function asNode(e: CstElement | undefined): CstNode | undefined {
  return e !== undefined && "children" in e ? e : undefined;
}

function unquoteString(s: string): string {
  if (s.startsWith('"')) return JSON.parse(s) as string;
  if (s.startsWith("'")) return s.slice(1, -1).replace(/\\'/g, "'");
  return s;
}

function parseLiteralToken(t: IToken): unknown {
  const im = t.image;
  if (t.tokenType === StringLit) return unquoteString(im);
  if (t.tokenType === IntegerLit) return Number.parseInt(im, 10);
  if (t.tokenType === FloatLit) return Number.parseFloat(im);
  return im;
}

function cstToExpr(n: CstNode | undefined): Expr {
  if (!n?.name) return { kind: "const", value: undefined };
  if (n.name === "expr") {
    const ch = n.children.orExpr?.[0] as CstNode | undefined;
    return cstToExpr(ch);
  }
  if (n.name === "orExpr") {
    const ch = n.children.andExpr;
    const xs = (Array.isArray(ch) ? ch : ch ? [ch] : []) as CstNode[];
    if (xs.length === 1) return cstToExpr(xs[0]);
    let cur = cstToExpr(xs[0]);
    for (let i = 1; i < xs.length; i++) {
      cur = { kind: "any", args: [cur, cstToExpr(xs[i]!)] };
    }
    return cur;
  }
  if (n.name === "andExpr") {
    const ch = n.children.cmpExpr;
    const xs = (Array.isArray(ch) ? ch : ch ? [ch] : []) as CstNode[];
    if (xs.length === 1) return cstToExpr(xs[0]);
    let cur = cstToExpr(xs[0]);
    for (let i = 1; i < xs.length; i++) {
      cur = { kind: "all", args: [cur, cstToExpr(xs[i]!)] };
    }
    return cur;
  }
  if (n.name === "cmpExpr") {
    const adds = n.children.addExpr as CstNode[] | CstNode | undefined;
    const arr = (Array.isArray(adds) ? adds : adds ? [adds] : []) as CstNode[];
    if (arr.length === 1) return cstToExpr(arr[0]);
    const left = cstToExpr(arr[0]!);
    const right = cstToExpr(arr[1]!);
    const opTok = (n.children.EqEq?.[0] ?? n.children.Eq?.[0] ?? n.children.Neq?.[0] ?? n.children.Lte?.[0] ?? n.children.Gte?.[0] ?? n.children.Lt?.[0] ?? n.children.Gt?.[0]) as IToken | undefined;
    const opMap: Record<string, ExprBinop["operation"]> = {
      "==": "==",
      "=": "==",
      "!=": "!=",
      "<=": "<=",
      ">=": ">=",
      "<": "<",
      ">": ">",
    };
    const operation = opTok ? (opMap[opTok.image] ?? "==") : "==";
    return { kind: "binop", operation, left, right };
  }
  if (n.name === "addExpr") {
    const muls = n.children.mulExpr as CstNode[] | CstNode | undefined;
    const arr = (Array.isArray(muls) ? muls : muls ? [muls] : []) as CstNode[];
    if (arr.length === 1) return cstToExpr(arr[0]);
    let cur = cstToExpr(arr[0]!);
    const pluses = (n.children.Plus as IToken[] | undefined) ?? [];
    const minuses = (n.children.Minus as IToken[] | undefined) ?? [];
    const operations: ("+" | "-")[] = [];
    for (const _ of arr.slice(1)) {
      const next = operations.length;
      if (pluses[next]) operations.push("+");
      else operations.push("-");
    }
    for (let i = 1; i < arr.length; i++) {
      const o = operations[i - 1] ?? "+";
      cur = { kind: "binop", operation: o, left: cur, right: cstToExpr(arr[i]!) };
    }
    return cur;
  }
  if (n.name === "mulExpr") {
    const uns = n.children.unaryExpr as CstNode[] | CstNode | undefined;
    const arr = (Array.isArray(uns) ? uns : uns ? [uns] : []) as CstNode[];
    if (arr.length === 1) return cstToExpr(arr[0]);
    let cur = cstToExpr(arr[0]!);
    const stars = (n.children.Star as IToken[] | undefined) ?? [];
    const slashes = (n.children.Slash as IToken[] | undefined) ?? [];
    for (let i = 1; i < arr.length; i++) {
      const isStar = Boolean(stars[i - 1]);
      cur = { kind: "binop", operation: isStar ? "*" : "/", left: cur, right: cstToExpr(arr[i]!) };
    }
    return cur;
  }
  if (n.name === "unaryExpr") {
    const prim = (n.children.primaryExpr?.[0] ?? n.children.primaryExpr) as CstNode | undefined;
    const neg = n.children.Minus?.[0];
    const inner = cstToExpr(prim);
    if (neg) return { kind: "binop", operation: "-", left: { kind: "const", value: 0 }, right: inner };
    return inner;
  }
  if (n.name === "primaryExpr") {
    const s = asToken(n.children.StringLit?.[0]);
    if (s) return { kind: "const", value: parseLiteralToken(s) };
    const il = asToken(n.children.IntegerLit?.[0]);
    if (il) return { kind: "const", value: parseLiteralToken(il) };
    const fl = asToken(n.children.FloatLit?.[0]);
    if (fl) return { kind: "const", value: parseLiteralToken(fl) };
    const inner = n.children.expr?.[0] as CstNode | undefined;
    if (inner) return cstToExpr(inner);
    const ids = n.children.Identifier as IToken[] | undefined;
    if (ids && ids.length) {
      let cur: Expr = { kind: "var", name: tokenText(ids[0]!) };
      for (let i = 1; i < ids.length; i++) {
        cur = { kind: "field", object: cur, name: tokenText(ids[i]!) };
      }
      return cur;
    }
  }
  return { kind: "const", value: undefined };
}

function cstToPropMap(n: CstNode | undefined): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  if (!n?.children.Identifier) return out;
  const keys = n.children.Identifier as IToken[];
  const vals = n.children.literal as CstNode[] | CstNode | undefined;
  const valArr = (Array.isArray(vals) ? vals : vals ? [vals] : []) as CstNode[];
  for (let i = 0; i < keys.length; i++) {
    const litTok = asToken(valArr[i]?.children?.StringLit?.[0] ?? valArr[i]?.children?.IntegerLit?.[0] ?? valArr[i]?.children?.FloatLit?.[0]);
    if (litTok) out[tokenText(keys[i]!)] = parseLiteralToken(litTok);
  }
  return out;
}

function cstToNodePattern(n: CstNode): NodePatternAst {
  const ids = (n.children.Identifier as IToken[] | undefined) ?? [];
  const colons = (n.children.Colon as IToken[] | undefined) ?? [];
  const pm = n.children.propMap?.[0] as CstNode | undefined;
  const props = pm ? cstToPropMap(pm) : undefined;
  const propsOpt = props && Object.keys(props).length ? { props } : {};
  if (ids.length === 0) return { kind: "node", ...propsOpt };
  if (colons.length === 0) return { kind: "node", var: tokenText(ids[0]!), ...propsOpt };
  if (ids.length >= 2) return { kind: "node", var: tokenText(ids[0]!), label: tokenText(ids[1]!), ...propsOpt };
  return { kind: "node", label: tokenText(ids[0]!), ...propsOpt };
}

function cstToRelPattern(n: CstNode): RelPatternAst {
  const inn = n.children.relPatternIn?.[0] as CstNode | undefined;
  const outu = n.children.relPatternOutOrUndirected?.[0] as CstNode | undefined;
  const body = inn ?? outu;
  if (!body) return { kind: "rel", types: [], direction: "->" };
  const rb = body.children.relBracket?.[0] as CstNode | undefined;
  const types = ((rb?.children.Identifier as IToken[]) ?? []).map((t) => tokenText(t));
  if (inn) return { kind: "rel", types, direction: "<-" };
  const hasGt = Boolean(outu?.children.Gt?.[0]);
  return { kind: "rel", types, direction: hasGt ? "->" : "--" };
}

function cstToPattern(n: CstNode): PatternAst {
  const nodes = (n.children.nodePattern as CstNode[] | undefined) ?? [];
  const rels = (n.children.relPattern as CstNode[] | undefined) ?? [];
  const elements: PatternElementAst[] = [];
  for (let i = 0; i < nodes.length; i++) {
    elements.push(cstToNodePattern(nodes[i]!));
    if (i < rels.length) elements.push(cstToRelPattern(rels[i]!));
  }
  return { elements };
}

function cstToYieldKey(n: CstNode | undefined): string {
  const parts = (n?.children.Identifier as IToken[] | undefined) ?? [];
  return parts.map((t) => tokenText(t)).join(".");
}

function cstToYieldItems(n: CstNode | undefined): YieldItemAst[] {
  if (!n?.children.yieldItem) return [];
  const items = (Array.isArray(n.children.yieldItem) ? n.children.yieldItem : [n.children.yieldItem]) as CstNode[];
  const out: YieldItemAst[] = [];
  for (const it of items) {
    const keyN = it.children.yieldKey?.[0] as CstNode | undefined;
    const key = cstToYieldKey(keyN);
    if (!key) continue;
    const aliasTok = (it.children.Identifier as IToken[] | undefined)?.[0];
    const alias = aliasTok ? tokenText(aliasTok) : undefined;
    out.push(alias ? { key, alias } : { key });
  }
  return out;
}

function cstToProjectList(n: CstNode | undefined): { expr: Expr; alias?: string }[] {
  if (!n?.children.projectItem) return [];
  const items = (Array.isArray(n.children.projectItem) ? n.children.projectItem : [n.children.projectItem]) as CstNode[];
  const out: { expr: Expr; alias?: string }[] = [];
  for (const it of items) {
    const ex = it.children.expr?.[0] as CstNode | undefined;
    const expr = ex ? cstToExpr(ex) : ({ kind: "const", value: undefined } as Expr);
    const ids = (it.children.Identifier as IToken[] | undefined) ?? [];
    const alias = ids.length ? tokenText(ids[0]!) : undefined;
    out.push(alias ? { expr, alias } : { expr });
  }
  return out;
}

function cstToLiteralObject(n: CstNode | undefined): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  if (!n?.children.Identifier) return out;
  const keys = n.children.Identifier as IToken[];
  const vals = (n.children.valueLiteral as CstNode[] | undefined) ?? [];
  for (let i = 0; i < keys.length; i++) {
    out[tokenText(keys[i]!)] = cstValueLiteralToValue(vals[i]);
  }
  return out;
}

function cstValueLiteralToValue(n: CstNode | undefined): unknown {
  if (!n) return undefined;
  const str = asToken(n.children.StringLit?.[0]);
  if (str) return parseLiteralToken(str);
  const int = asToken(n.children.IntegerLit?.[0]);
  if (int) return parseLiteralToken(int);
  const flt = asToken(n.children.FloatLit?.[0]);
  if (flt) return parseLiteralToken(flt);
  const arr = n.children.arrayLiteral?.[0] as CstNode | undefined;
  if (arr) {
    const vs = (arr.children.valueLiteral as CstNode[] | undefined) ?? [];
    return vs.map((x) => cstValueLiteralToValue(x));
  }
  const ob = n.children.objectLiteralExpr?.[0] as CstNode | undefined;
  if (ob) return cstToLiteralObject(ob);
  return undefined;
}

function cstToAst(cst: CstNode): ConstructAst {
  const clauses: ConstructClauseAst[] = [];
  const mc = cst.children.matchClause as CstNode[] | undefined;
  if (mc) {
    for (const m of mc) {
      const plist = m.children.patternList?.[0] as CstNode | undefined;
      const pats = (plist?.children.pattern as CstNode[] | undefined) ?? [];
      const patterns = pats.map((p) => cstToPattern(p));
      const whereN = m.children.expr?.[0] as CstNode | undefined;
      const where = whereN ? cstToExpr(whereN) : undefined;
      clauses.push({ kind: "match", patterns, ...(where ? { where } : {}) });
    }
  }
  const wc = cst.children.withClause as CstNode[] | undefined;
  if (wc) {
    for (const w of wc) {
      const pl = w.children.projectList?.[0] as CstNode | undefined;
      const whereN = w.children.expr?.[0] as CstNode | undefined;
      clauses.push({
        kind: "with",
        projections: cstToProjectList(pl),
        ...(whereN ? { where: cstToExpr(whereN) } : {}),
      });
    }
  }
  const cc = cst.children.callClause as CstNode[] | undefined;
  if (cc) {
    for (const c of cc) {
      const parts = (asNode(c.children.actionId?.[0])?.children.Identifier as IToken[] | undefined) ?? [];
      const actionId = parts.map((t) => tokenText(t)).join(".");
      const obj = c.children.objectLiteralExpr?.[0] as CstNode | undefined;
      const args = obj ? cstToLiteralObject(obj) : {};
      const yc = c.children.yieldClause?.[0] as CstNode | undefined;
      clauses.push({ kind: "call", actionId, args, yieldItems: cstToYieldItems(yc) });
    }
  }
  const uc = cst.children.unwindClause as CstNode[] | undefined;
  if (uc) {
    for (const u of uc) {
      const src = u.children.expr?.[0] as CstNode | undefined;
      const aliasTok = (u.children.Identifier as IToken[] | undefined)?.[0];
      const whereN = u.children.expr?.[1] as CstNode | undefined;
      if (!src || !aliasTok) continue;
      clauses.push({
        kind: "unwind",
        source: cstToExpr(src),
        alias: tokenText(aliasTok),
        ...(whereN ? { where: cstToExpr(whereN) } : {}),
      });
    }
  }
  const ret = cst.children.returnClause?.[0] as CstNode | undefined;
  let returnClause: ReturnClauseAst | undefined;
  if (ret) {
    const pl = ret.children.projectList?.[0] as CstNode | undefined;
    const order = ret.children.orderExpr?.[0] as CstNode | undefined;
    const limN = ret.children.returnLimitLit?.[0] as CstNode | undefined;
    const lim = asToken(limN?.children.IntegerLit?.[0]);
    returnClause = {
      kind: "return",
      projections: cstToProjectList(pl),
      ...(order ? { orderBy: cstToExpr(order) } : {}),
      ...(lim ? { limit: Number.parseInt(lim.image, 10) } : {}),
    };
  }
  return { clauses, ...(returnClause ? { returnClause } : {}) };
}

/** 🔍️ Parses `construct` source into `ConstructAst` (throws on syntax error). */
export function parseConstruct(text: string, activeModelDefinitionId?: string | null): ConstructAst {
  const lex = constructLexer.tokenize(text);
  if (lex.errors.length) throw new Error(lex.errors.map((e) => e.message).join("; "));
  parserSingleton.input = lex.tokens;
  const cst = parserSingleton.query();
  const errs = parserSingleton.errors;
  if (errs.length > 0) throw new Error(errs.map((e) => e.message).join("; "));
  const ast = cstToAst(cst as unknown as CstNode);
  assertConstructAst(ast, activeModelDefinitionId);
  return ast;
}
// #endregion Ast


/** 🚪️ Admits Construct text before running the typed semantic program. */
export async function runConstruct(text: string, ctx: ConstructQueryContext): Promise<ConstructQueryResult> {
  return runConstructAst(parseConstruct(text, ctx.activeModelDefinitionId), ctx);
}

/** 🌉️ Connects the interaction text boundary to typed Construct execution. */
export const defaultConstructRunner: ConstructRunner = (text, ctx) => runConstruct(text, ctx);

// #region 🧪️Tests
const __spatialQueryTestRuntime = import.meta.vitest ? await import("../../../✏️editor/⚙️engine/🏃️runtime/🟦️.ts") : null;
const __spatialQueryTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts") : null;

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts`. */
export type InferencesTestDependencies = {
  readonly Model: typeof Model;
  readonly __spatialQueryTestKernel: typeof __spatialQueryTestKernel;
  readonly __spatialQueryTestRuntime: typeof __spatialQueryTestRuntime;
  readonly applyModelDiff: typeof applyModelDiff;
  readonly emptyMeshTransfer: typeof emptyMeshTransfer;
  readonly modelDefinitionActionRegistry: typeof modelDefinitionActionRegistry;
  readonly parseConstruct: typeof parseConstruct;
  readonly runConstruct: typeof runConstruct;
  readonly runConstructAst: typeof runConstructAst;
  readonly solidRef: typeof solidRef;
};

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts");
  await registerTests1(import.meta.vitest, { Model, __spatialQueryTestKernel, __spatialQueryTestRuntime, applyModelDiff, emptyMeshTransfer, modelDefinitionActionRegistry, parseConstruct, runConstruct, runConstructAst, solidRef }, { url: import.meta.url });
}
// #endregion 🧪️Tests
