// #region 🧲️Header
/** @emoji 🔍️ `@semio-tech/cad-js/query` — Cypher-inspired `construct` language: `MATCH (Object {typology: '…'})`, `KernelIndex` on `Model`, `defaultConstructRunner` for `InteractionRuntime.query` (view `CALL` removed). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { CstParser, createToken, Lexer } from "chevrotain";
import type { CstNode, IToken } from "chevrotain";
import {
  type ActionRegistry,
  runRegisteredAction,
  modelDefinitionActionRegistry,
  type ActionResult,
  Model,
  applyModelDiff,
  buildTypologyToEntityKindMapForModelDefinition,
  applyTransformation,
  loadTransformation,
  solidRef,
  isSelectionConstructActionId,
  actionAvailableInModelDefinition,
  defaultModelDefinitionId,
  type ConstructQueryContext,
  type ConstructQueryResult,
  type ConstructQueryRow,
  type ConstructRunner,
  type Expr,
  type ExprBinop,
  type ExprField,
  type ExprVar,
  kernelGeometry,
  type SpatialKernel,
  type ModelDiff,
  EMPTY_MODEL_DIFF,
  type ModelEntityKind,
  type ModelEntityRef,
  type Vec3,
  evalExpr,
  type ExprEnv,
} from "../🟦️index.ts";

type SolidRef = kernelGeometry.SolidRef;
type FaceRef = kernelGeometry.FaceRef;
type ShellRef = kernelGeometry.ShellRef;
// #endregion 🔌️Adapters

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

// #region Ast
export interface NodePatternAst {
  readonly kind: "node";
  readonly var?: string;
  readonly label?: string;
  readonly props?: Record<string, unknown>;
}

export interface RelPatternAst {
  readonly kind: "rel";
  readonly types: readonly string[];
  readonly direction: "->" | "<-" | "--";
}

export type PatternElementAst = NodePatternAst | RelPatternAst;

export interface PatternAst {
  readonly elements: readonly PatternElementAst[];
}

export interface MatchClauseAst {
  readonly kind: "match";
  readonly patterns: readonly PatternAst[];
  readonly where?: Expr;
}

export interface WithClauseAst {
  readonly kind: "with";
  readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[];
  readonly where?: Expr;
}

export interface YieldItemAst {
  readonly key: string;
  readonly alias?: string;
}

export interface CallClauseAst {
  readonly kind: "call";
  readonly actionId: string;
  readonly args: Readonly<Record<string, unknown>>;
  readonly yieldItems: readonly YieldItemAst[];
}

export interface UnwindClauseAst {
  readonly kind: "unwind";
  readonly source: Expr;
  readonly alias: string;
  readonly where?: Expr;
}

export interface ReturnClauseAst {
  readonly kind: "return";
  readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[];
  readonly orderBy?: Expr;
  readonly limit?: number;
}

export type ConstructClauseAst = MatchClauseAst | WithClauseAst | CallClauseAst | UnwindClauseAst;

function assertConstructAst(ast: ConstructAst, activeModelDefinitionId?: string | null): void {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const typologyKinds = buildTypologyToEntityKindMapForModelDefinition(mdId);
  for (const cl of ast.clauses) {
    if (cl.kind !== "match") continue;
    for (const pat of cl.patterns) {
      for (const el of pat.elements) {
        if (el.kind !== "node") continue;
        if (el.label && el.label !== "Object") {
          throw new Error(`unknown node label ${el.label}; use Object {typology: '…'}`);
        }
        const typology = el.props?.typology;
        if (typeof typology === "string" && (typology.includes(".derived.") || typology.startsWith("view."))) {
          throw new Error(`${typology} is not a shipped model typology`);
        }
        if (typeof typology === "string" && !(typology in typologyKinds)) {
          throw new Error(`unknown typology ${typology} for model definition ${mdId}; see listTypologiesForModelDefinition()`);
        }
      }
    }
  }
}

/** @emoji 🔍️ Resolves one `YIELD` key (supports dot paths into `data`) from an `ActionResult`. */
export function resolveActionYield(result: ActionResult, key: string): unknown {
  if (!key.includes(".")) {
    if (key === "diff") return result.diff;
    if (key === "data") return result.data;
    if (key === "patch") return result.patch;
    if (key === "targets") {
      const patchSet = result.patch?.set as { targets?: unknown } | undefined;
      if (patchSet?.targets !== undefined) return patchSet.targets;
      const data = result.data as { targets?: unknown } | undefined;
      return data?.targets;
    }
    return (result as Record<string, unknown>)[key];
  }
  const [head, ...rest] = key.split(".");
  let cur: unknown = head === "data" ? result.data : head === "diff" ? result.diff : head === "patch" ? result.patch : (result as Record<string, unknown>)[head];
  for (const seg of rest) {
    if (cur === null || cur === undefined || typeof cur !== "object") return undefined;
    cur = (cur as Record<string, unknown>)[seg];
  }
  return cur;
}

export interface ConstructAst {
  readonly clauses: readonly ConstructClauseAst[];
  readonly returnClause?: ReturnClauseAst;
}

function tokenText(t: IToken): string {
  return t.image;
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
    const s = n.children.StringLit?.[0];
    if (s) return { kind: "const", value: parseLiteralToken(s) };
    const il = n.children.IntegerLit?.[0];
    if (il) return { kind: "const", value: parseLiteralToken(il) };
    const fl = n.children.FloatLit?.[0];
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
    const litTok = valArr[i]?.children?.StringLit?.[0] ?? valArr[i]?.children?.IntegerLit?.[0] ?? valArr[i]?.children?.FloatLit?.[0];
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
  if (n.children.StringLit?.[0]) return parseLiteralToken(n.children.StringLit[0]);
  if (n.children.IntegerLit?.[0]) return parseLiteralToken(n.children.IntegerLit[0]);
  if (n.children.FloatLit?.[0]) return parseLiteralToken(n.children.FloatLit[0]);
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
      const parts = (c.children.actionId?.[0]?.children?.Identifier as IToken[] | undefined) ?? [];
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
    const lim = limN?.children.IntegerLit?.[0];
    returnClause = {
      kind: "return",
      projections: cstToProjectList(pl),
      ...(order ? { orderBy: cstToExpr(order) } : {}),
      ...(lim ? { limit: Number.parseInt(lim.image, 10) } : {}),
    };
  }
  return { clauses, ...(returnClause ? { returnClause } : {}) };
}

/** @emoji 🔍️ Parses `construct` source into `ConstructAst` (throws on syntax error). */
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

// #region Index
function typologyToEntityKindForConstruct(activeModelDefinitionId?: string | null): Readonly<Record<string, ModelEntityKind>> {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  return buildTypologyToEntityKindMapForModelDefinition(mdId);
}

function patternEntityKind(node: NodePatternAst, activeModelDefinitionId?: string | null): ModelEntityKind | undefined {
  if (node.label && node.label !== "Object") return undefined;
  const typology = node.props?.typology;
  if (typeof typology !== "string") return undefined;
  return typologyToEntityKindForConstruct(activeModelDefinitionId)[typology];
}

function objectHandlesForTypology(model: Model, typology: string): EntityHandle[] {
  return Object.values(model.objects)
    .filter((row) => row.typology === typology)
    .map((row) => ({ kind: "object" as ModelEntityKind, id: String(row.id) }));
}

export class KernelIndex {
  private revisionAt = -1;
  private readonly byKind = new Map<ModelEntityKind, Set<string>>();
  private readonly faceToSolids = new Map<string, Set<string>>();
  private readonly edgeToFaces = new Map<string, Set<string>>();

  constructor(private readonly model: Model) {}

  private rebuild(): void {
    this.byKind.clear();
    this.faceToSolids.clear();
    this.edgeToFaces.clear();
    const add = (k: ModelEntityKind, id: string) => {
      let s = this.byKind.get(k);
      if (!s) {
        s = new Set();
        this.byKind.set(k, s);
      }
      s.add(id);
    };
    for (const id of Object.keys(this.model.anchors)) add("anchor", id);
    for (const id of Object.keys(this.model.vertices)) add("vertex", id);
    for (const id of Object.keys(this.model.edges)) add("edge", id);
    for (const id of Object.keys(this.model.wires)) add("wire", id);
    for (const id of Object.keys(this.model.faces)) add("face", id);
    for (const id of Object.keys(this.model.shells)) add("shell", id);
    for (const id of Object.keys(this.model.solids)) add("solid", id);
    for (const id of Object.keys(this.model.objects)) add("object", id);
    for (const [sid, solid] of Object.entries(this.model.solids)) {
      for (const shellId of solid.shellIds) {
        const sh = this.model.shells[shellId];
        if (!sh) continue;
        for (const fid of sh.faceIds) {
          let xs = this.faceToSolids.get(fid);
          if (!xs) {
            xs = new Set();
            this.faceToSolids.set(fid, xs);
          }
          xs.add(sid);
        }
      }
    }
    for (const [fid, face] of Object.entries(this.model.faces)) {
      for (const wid of face.wireIds) {
        const w = this.model.wires[wid];
        if (!w) continue;
        for (const eid of w.edgeIds) {
          let fs = this.edgeToFaces.get(eid);
          if (!fs) {
            fs = new Set();
            this.edgeToFaces.set(eid, fs);
          }
          fs.add(fid);
        }
      }
    }
    this.revisionAt = this.model.revision;
  }

  ensure(): void {
    if (this.revisionAt !== this.model.revision) this.rebuild();
  }

  idsForKind(k: ModelEntityKind): readonly string[] {
    this.ensure();
    return [...(this.byKind.get(k) ?? [])];
  }

  lookupById(id: string): { kind: ModelEntityKind; id: string } | null {
    this.ensure();
    for (const [k, s] of this.byKind) {
      if (s.has(id)) return { kind: k, id };
    }
    return null;
  }

  selectivityScore(node: NodePatternAst): number {
    if (node.props?.id !== undefined) return 0;
    if (patternEntityKind(node)) return 1;
    return 2;
  }

  adjacentSolidIds(solidId: string): Set<string> {
    this.ensure();
    const out = new Set<string>();
    const solid = this.model.solids[solidId];
    if (!solid) return out;
    const faces = new Set<string>();
    for (const shellId of solid.shellIds) {
      const sh = this.model.shells[shellId];
      if (!sh) continue;
      for (const f of sh.faceIds) faces.add(f);
    }
    for (const f of faces) {
      for (const other of this.faceToSolids.get(f) ?? []) {
        if (other !== solidId) out.add(other);
      }
    }
    return out;
  }

  edgeIncidentFaceCount(edgeId: string): number {
    this.ensure();
    return this.edgeToFaces.get(edgeId)?.size ?? 0;
  }

  facesForEdge(edgeId: string): readonly string[] {
    this.ensure();
    return [...(this.edgeToFaces.get(edgeId) ?? [])];
  }
}
// #endregion Index

// #region Traversers
export type EntityHandle = ModelEntityRef;

function* iterateBoundedBy(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "face") {
    const f = model.faces[from.id];
    if (!f) return;
    for (const w of f.wireIds) yield { kind: "wire", id: w };
  } else if (from.kind === "solid") {
    const c = model.solids[from.id];
    if (!c) return;
    for (const s of c.shellIds) yield { kind: "shell", id: s };
  } else if (from.kind === "shell") {
    const s = model.shells[from.id];
    if (!s) return;
    for (const f of s.faceIds) yield { kind: "face", id: f };
  } else if (from.kind === "wire") {
    const w = model.wires[from.id];
    if (!w) return;
    for (const e of w.edgeIds) yield { kind: "edge", id: e };
  } else if (from.kind === "edge") {
    const e = model.edges[from.id];
    if (!e) return;
    for (const v of e.vertexIds) yield { kind: "vertex", id: v };
  }
}

function* iterateContainsInverse(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "face") {
    for (const [sid, sh] of Object.entries(model.shells)) {
      if (sh.faceIds.includes(from.id as FaceRef)) yield { kind: "shell", id: sid };
    }
  } else if (from.kind === "shell") {
    for (const [cid, c] of Object.entries(model.solids)) {
      if (c.shellIds.includes(from.id)) yield { kind: "solid", id: cid };
    }
  } else if (from.kind === "wire") {
    for (const [fid, fa] of Object.entries(model.faces)) {
      if (fa.wireIds.includes(from.id)) yield { kind: "face", id: fid };
    }
  } else if (from.kind === "edge") {
    for (const [wid, w] of Object.entries(model.wires)) {
      if (w.edgeIds.includes(from.id)) yield { kind: "wire", id: wid };
    }
  } else if (from.kind === "vertex") {
    for (const [eid, e] of Object.entries(model.edges)) {
      if (e.vertexIds.includes(from.id)) yield { kind: "edge", id: eid };
    }
  }
}

function* iterateContainsForward(model: Model, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "solid") {
    const c = model.solids[from.id];
    if (!c) return;
    for (const shellId of c.shellIds) yield { kind: "shell", id: shellId };
  } else if (from.kind === "shell") {
    const s = model.shells[from.id];
    if (!s) return;
    for (const fid of s.faceIds) yield { kind: "face", id: fid };
  } else if (from.kind === "face") {
    const f = model.faces[from.id];
    if (!f) return;
    for (const wid of f.wireIds) yield { kind: "wire", id: wid };
  } else if (from.kind === "wire") {
    const w = model.wires[from.id];
    if (!w) return;
    for (const eid of w.edgeIds) yield { kind: "edge", id: eid };
  }
}

function lookupAnyEntity(model: Model, id: string): EntityHandle | null {
  if (model.anchors[id]) return { kind: "anchor", id };
  if (model.vertices[id]) return { kind: "vertex", id };
  if (model.edges[id]) return { kind: "edge", id };
  if (model.wires[id]) return { kind: "wire", id };
  if (model.faces[id]) return { kind: "face", id };
  if (model.shells[id]) return { kind: "shell", id };
  if (model.solids[id]) return { kind: "solid", id };
  return null;
}

function* iterateShares(model: Model, index: KernelIndex, from: EntityHandle): Generator<EntityHandle> {
  if (from.kind === "edge") {
    for (const fid of index.facesForEdge(from.id)) yield { kind: "face", id: fid };
  } else if (from.kind === "vertex") {
    for (const e of iterateContainsInverse(model, from)) yield e;
  }
}

function* traverseRel(model: Model, _kernel: SpatialKernel, index: KernelIndex, from: EntityHandle, relTypes: readonly string[], direction: RelPatternAst["direction"]): Generator<EntityHandle> {
  const rel = relTypes[0] ?? "BOUNDED_BY";
  const both = direction === "--";
  const out = direction === "->" || both;
  const inn = direction === "<-" || both;
  const forward =
    rel === "BOUNDED_BY"
      ? iterateBoundedBy(model, from)
      : rel === "CONTAINS"
        ? out
          ? iterateContainsForward(model, from)
          : iterateContainsInverse(model, from)
        : rel === "SHARES"
          ? iterateShares(model, index, from)
          : rel === "ADJACENT_TO" && from.kind === "solid"
            ? (function* () {
                for (const id of index.adjacentSolidIds(from.id)) yield { kind: "solid", id };
              })()
            : rel === "HAS_VERTEX"
              ? (function* () {
                  let frontier: EntityHandle[] = [from];
                  for (let depth = 0; depth < 8; depth++) {
                    const next: EntityHandle[] = [];
                    for (const x of frontier) {
                      for (const y of iterateBoundedBy(model, x)) {
                        if (y.kind === "vertex") yield y;
                        else next.push(y);
                      }
                    }
                    frontier = next;
                  }
                })()
              : (function* () {})();
  if (out && !inn) return yield* forward;
  if (inn && !out) {
    if (rel === "CONTAINS") return yield* iterateContainsInverse(model, from);
    return yield* forward;
  }
  yield* forward;
}
// #endregion Traversers

// #region Planner
export type PlanStepAst =
  | { readonly kind: "match"; readonly pattern: PatternAst; readonly where?: Expr }
  | { readonly kind: "with"; readonly projections: readonly { readonly expr: Expr; readonly alias?: string }[]; readonly where?: Expr }
  | { readonly kind: "call"; readonly actionId: string; readonly args: Readonly<Record<string, unknown>>; readonly yieldItems: readonly YieldItemAst[] }
  | { readonly kind: "unwind"; readonly source: Expr; readonly alias: string; readonly where?: Expr };

export interface ExecutionPlan {
  readonly steps: readonly PlanStepAst[];
  readonly returnClause?: ReturnClauseAst;
}

/** @emoji 🧭️ Flattens `MATCH` comma patterns into sequential steps (cartesian product handled in executor). */
export function planConstruct(ast: ConstructAst): ExecutionPlan {
  const steps: PlanStepAst[] = [];
  for (const cl of ast.clauses) {
    if (cl.kind === "match") {
      for (const p of cl.patterns) steps.push({ kind: "match", pattern: p, ...(cl.where ? { where: cl.where } : {}) });
    } else if (cl.kind === "with") steps.push({ kind: "with", projections: cl.projections, ...(cl.where ? { where: cl.where } : {}) });
    else if (cl.kind === "call") steps.push({ kind: "call", actionId: cl.actionId, args: cl.args, yieldItems: cl.yieldItems });
    else if (cl.kind === "unwind") steps.push({ kind: "unwind", source: cl.source, alias: cl.alias, ...(cl.where ? { where: cl.where } : {}) });
  }
  return { steps, ...(ast.returnClause ? { returnClause: ast.returnClause } : {}) };
}
// #endregion Planner

// #region Executor
type Row = Record<string, ModelEntityRef | unknown>;

function rowVarsToEnv(row: Row, model: Model, meta: import("../🟦️index.ts").AttributeStore, preview: SpatialKernel, activeModelDefinitionId?: string | null): ExprEnv {
  const vars: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(row)) {
    if (v && typeof v === "object" && "kind" in (v as object) && "id" in (v as object)) vars[k] = v;
    else vars[k] = v;
  }
  return { context: {}, vars, model, metadata: meta, activeModelDefinitionId, preview };
}

/** @emoji 🔄️ Runs a shipped transformation against the construct context model. */
function runTransformationCall(actionId: string, ctx: ConstructQueryContext): ActionResult {
  const spec = loadTransformation(actionId);
  if (!spec) throw new Error(`unknown transformation ${actionId}`);
  const model = applyTransformation(spec, ctx.model, ctx.preview);
  const objects = Object.keys(model.objects)
    .sort()
    .map((key) => model.objects[key]!);
  return { diff: EMPTY_MODEL_DIFF, data: { model, objects } };
}

function* expandPattern(model: Model, kernel: SpatialKernel, index: KernelIndex, pat: PatternAst, activeModelDefinitionId?: string | null): Generator<Row> {
  const els = pat.elements;
  if (!els.length) return;
  const first = els[0] as NodePatternAst;
  const startVar = first.var ?? "__n0";
  const startKind = patternEntityKind(first, activeModelDefinitionId);
  const startTypology = first.props?.typology;
  const idProp = first.props?.id;
  index.ensure();
  let seeds: EntityHandle[] = [];
  if (typeof idProp === "string" && startKind) seeds = [{ kind: startKind, id: idProp }];
  else if (typeof idProp === "string") {
    const lk = index.lookupById(idProp);
    if (lk) seeds = [lk];
  } else if (typeof startTypology === "string" && startKind === "object") {
    seeds = objectHandlesForTypology(model, startTypology);
  } else if (startKind) seeds = index.idsForKind(startKind).map((id) => ({ kind: startKind, id }));
  else seeds = [];

  function* expandFrom(j: number, row: Row): Generator<Row> {
    if (j >= els.length) {
      yield { ...row };
      return;
    }
    const el = els[j]!;
    if (el.kind === "node") {
      const nm = el.var ?? `__n${j}`;
      const kind = patternEntityKind(el, activeModelDefinitionId);
      const pid = el.props?.id;
      const typology = el.props?.typology;
      if (j === 0) {
        for (const s of seeds) {
          if (kind && s.kind !== kind) continue;
          if (typeof pid === "string" && s.id !== pid) continue;
          if (typeof typology === "string" && s.kind === "object") {
            const obj = model.objects[s.id];
            if (!obj || obj.typology !== typology) continue;
          }
          yield* expandFrom(j + 1, { ...row, [nm]: s });
        }
      }
      return;
    }
    const rel = el as RelPatternAst;
    const prevNode = els[j - 1] as NodePatternAst;
    const prevName = prevNode.var ?? `__n${j - 1}`;
    const from = row[prevName] as EntityHandle;
    const nextNode = els[j + 1] as NodePatternAst;
    const nm = nextNode.var ?? `__n${j + 1}`;
    const kind = patternEntityKind(nextNode, activeModelDefinitionId);
    const pid = nextNode.props?.id;
    const typology = nextNode.props?.typology;
    for (const x of traverseRel(model, kernel, index, from, rel.types, rel.direction)) {
      if (kind && x.kind !== kind) continue;
      if (typeof pid === "string" && x.id !== pid) continue;
      if (typeof typology === "string" && x.kind === "object") {
        const obj = model.objects[x.id];
        if (!obj || obj.typology !== typology) continue;
      }
      yield* expandFrom(j + 2, { ...row, [nm]: x });
    }
  }
  yield* expandFrom(0, {});
}

async function* executeConstruct(plan: ExecutionPlan, ctx: ConstructQueryContext): AsyncIterable<ConstructQueryRow> {
  const index = new KernelIndex(ctx.model);
  let rows: Row[] = [{}];
  for (const st of plan.steps) {
    if (st.kind === "match") {
      const next: Row[] = [];
      for (const r of rows) {
        for (const row of expandPattern(ctx.model, ctx.kernel, index, st.pattern, ctx.activeModelDefinitionId)) {
          const merged = { ...r, ...row };
          if (st.where) {
            const ok = evalExpr(st.where, rowVarsToEnv(merged, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
            if (!ok) continue;
          }
          next.push(merged);
        }
      }
      rows = next;
    } else if (st.kind === "with") {
      const next: Row[] = [];
      for (const r of rows) {
        const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
        const out: Row = { ...r };
        for (const p of st.projections) {
          const v = evalExpr(p.expr, env);
          if (p.alias) out[p.alias] = v;
        }
        if (st.where) {
          const ok = evalExpr(st.where, rowVarsToEnv(out, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
          if (!ok) continue;
        }
        next.push(out);
      }
      rows = next;
    } else if (st.kind === "call") {
      const transformation = loadTransformation(st.actionId);
      const modelDefinitionId = ctx.activeModelDefinitionId ?? defaultModelDefinitionId();
      if (!transformation && !ctx.actions.get(st.actionId)) throw new Error(`unknown action ${st.actionId}`);
      if (!transformation && !actionAvailableInModelDefinition(st.actionId, modelDefinitionId)) {
        throw new Error(`action ${st.actionId} is not available in model definition ${modelDefinitionId}`);
      }
      const next: Row[] = [];
      for (const r of rows) {
        const paramBag: Record<string, unknown> = { __context: {}, __event: { kind: "construct.call" }, ...st.args };
        if (isSelectionConstructActionId(st.actionId) && paramBag.seedTargets === undefined && ctx.selectionTargets && ctx.selectionTargets.length > 0) {
          paramBag.seedTargets = ctx.selectionTargets;
        }
        const res = transformation
          ? runTransformationCall(st.actionId, ctx)
          : await runRegisteredAction(ctx.actions, st.actionId, paramBag, {
              kernel: ctx.kernel,
              preview: ctx.kernel,
              model: ctx.model,
              activeModelDefinitionId: ctx.activeModelDefinitionId ?? null,
            });
        const nr = { ...r };
        for (const y of st.yieldItems) {
          const v = resolveActionYield(res, y.key);
          if (v !== undefined) nr[y.alias ?? y.key] = v;
        }
        next.push(nr);
      }
      rows = next;
    } else if (st.kind === "unwind") {
      const next: Row[] = [];
      for (const r of rows) {
        const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
        const src = evalExpr(st.source, env);
        if (!Array.isArray(src)) continue;
        for (const item of src) {
          const merged: Row = { ...r, [st.alias]: item };
          if (st.where) {
            const ok = evalExpr(st.where, rowVarsToEnv(merged, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId));
            if (!ok) continue;
          }
          next.push(merged);
        }
      }
      rows = next;
    }
  }
  const ret = plan.returnClause;
  if (!ret) {
    for (const r of rows) yield { ...r };
    return;
  }
  let out = rows;
  if (ret.limit !== undefined) out = out.slice(0, ret.limit);
  for (const r of out) {
    const env = rowVarsToEnv(r, ctx.model, ctx.model.metadata, ctx.kernel, ctx.activeModelDefinitionId);
    const o: ConstructQueryRow = {};
    for (let i = 0; i < ret.projections.length; i++) {
      const p = ret.projections[i]!;
      const k = p.alias ?? `c${i}`;
      o[k] = evalExpr(p.expr, env);
    }
    yield o;
  }
}
// #endregion Executor

// #region Api
/** @emoji 🔍️ Materializes `executeConstruct` into `ConstructQueryResult`. */
export async function runConstruct(text: string, ctx: ConstructQueryContext): Promise<ConstructQueryResult> {
  const ast = parseConstruct(text, ctx.activeModelDefinitionId);
  const plan = planConstruct(ast);
  const rows: ConstructQueryRow[] = [];
  let data: unknown;
  let diff: ModelDiff | undefined;
  for await (const row of executeConstruct(plan, ctx)) {
    rows.push(row);
    if (row.data !== undefined) data = row.data;
    if (row.diff !== undefined) diff = row.diff as ModelDiff;
  }
  return { rows, ...(data !== undefined ? { data } : {}), ...(diff !== undefined ? { diff } : {}) };
}

/** @emoji 🔍️ Default `InteractionRuntimeOptions.query` bridge (`@semio-tech/cad-js/core`). */
export const defaultConstructRunner: ConstructRunner = (text, ctx) => runConstruct(text, ctx);

/** @emoji 🔍️ Cached `KernelIndex` wrapper for repeated `construct` scripts on one document revision. */
export class ConstructEngine {
  private index: KernelIndex | null = null;
  private rev = -1;

  constructor(private readonly model: Model) {}

  private ix(): KernelIndex {
    if (!this.index || this.rev !== this.model.revision) {
      this.index = new KernelIndex(this.model);
      this.rev = this.model.revision;
    }
    return this.index;
  }

  /** @emoji 🧭️ Ensures `KernelIndex` matches current `model.revision` (side-effect on cache). */
  warmIndex(): void {
    this.ix().ensure();
  }
}
// #endregion Api

// #region 🧪️Tests
const __spatialQueryTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️component.ts") : null;
const __spatialQueryTestKernel = import.meta.vitest ? await import("../📐️brepjs/🟦️component.ts") : null;

if (import.meta.vitest) {
  __spatialQueryTestRuntime!.bootstrapCadModules();
  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialQueryTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  class QueryTestKernel extends BrepjsKernel {
    override readonly id = "stub-k";
    override readonly operations = [] as const;
    override async createBoxFromCorners() {
      return "c0" as SolidRef;
    }
    override async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }) {
      const cell = await this.createBoxFromCorners(input);
      return { solid: cell, diff: this.boxModelDiff(input, cell) };
    }
    override async volume() {
      return 0;
    }
    override async tessellate() {
      return { positions: new Float32Array(), indices: new Uint32Array() };
    }
  }

  function mkKernelStub(): SpatialKernel {
    return new QueryTestKernel();
  }

  function seedSolidShellFaces(model: Model): { solid: string; shell: string; faces: string[] } {
    const f0 = "f0" as FaceRef;
    const f1 = "f1" as FaceRef;
    const sh = "s0" as ShellRef;
    const c0 = "c0" as SolidRef;
    model.faces[f0] = { id: f0, wireIds: [] };
    model.faces[f1] = { id: f1, wireIds: [] };
    model.shells[sh] = { id: sh, faceIds: [f0, f1] };
    model.solids[c0] = { id: c0, shellIds: [sh] };
    return { solid: c0, shell: sh, faces: [f0, f1] };
  }

  describe("@semio-tech/cad-js/query parse", () => {
    it("parses MATCH RETURN with property access", () => {
      const a = parseConstruct("MATCH (f:Object {typology: 'spatial.shape.kernel.face', id: 'f0'}) RETURN f.id");
      expect(a.clauses[0]?.kind).toBe("match");
      expect(a.returnClause?.projections.length).toBe(1);
    });
    it("parses RETURN LIMIT without ORDER BY", () => {
      const a = parseConstruct("MATCH (v:Object {typology: 'spatial.shape.kernel.vertex'}) RETURN v.id LIMIT 3");
      expect(a.returnClause?.limit).toBe(3);
    });
    it("parses CALL with object literal and YIELD", () => {
      const a = parseConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [2,3,0], height: 4 }) YIELD diff");
      const c = a.clauses[0];
      expect(c?.kind).toBe("call");
      if (c?.kind === "call") {
        expect(c.actionId).toBe("primitive.createBoxFromCorners");
        expect(c.args.height).toBe(4);
        expect(c.yieldItems[0]?.key).toBe("diff");
      }
    });
    it("parses CALL transformation YIELD with AS alias", () => {
      const a = parseConstruct("CALL aec.building.energy.from_geometry({}) YIELD objects AS energyObjects");
      const c = a.clauses[0];
      expect(c?.kind).toBe("call");
      if (c?.kind === "call") {
        expect(c.actionId).toBe("aec.building.energy.from_geometry");
        expect(c.yieldItems[0]).toEqual({ key: "objects", alias: "energyObjects" });
      }
    });
    it("parses UNWIND with WHERE", () => {
      const a = parseConstruct("UNWIND surfaces AS s WHERE s.exposure = 'external' RETURN s.id");
      expect(a.clauses[0]?.kind).toBe("unwind");
    });
    it("rejects unknown node labels", () => {
      expect(() => parseConstruct("MATCH (f:Face {id: 'f0'}) RETURN f.id")).toThrow(/unknown node label Face/);
      expect(() => parseConstruct("MATCH (m:Model) RETURN m.id")).toThrow(/unknown node label Model/);
      expect(() => parseConstruct("MATCH (s:Surface) RETURN s.id")).toThrow(/unknown node label Surface/);
    });
    it("rejects MATCH on non-shipped typology ids", () => {
      expect(() => parseConstruct("MATCH (o:Object {typology: 'energy.derived.hull'}) RETURN o.id")).toThrow(/not a shipped model typology/);
      expect(() => parseConstruct("MATCH (o:Object {typology: 'unknown.typology.foo'}) RETURN o.id")).toThrow(/unknown typology/);
    });
  });

  describe("@semio-tech/cad-js/query execute", () => {
    it("MATCH solid shell face chain returns face ids", async () => {
      const model = new Model();
      seedSolidShellFaces(model);
      const q = `MATCH (c:Object {typology: 'spatial.shape.kernel.solid'})-[:BOUNDED_BY]->(:Object {typology: 'spatial.shape.kernel.shell'})-[:CONTAINS]->(f:Object {typology: 'spatial.shape.kernel.face'}) RETURN f.id`;
      const res = await runConstruct(q, {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      const ids = res.rows.map((r) => r.c0).sort();
      expect(ids).toEqual(["f0", "f1"]);
    });

    it("ADJACENT_TO finds solids sharing a face", async () => {
      const model = new Model();
      const fShared = "fs" as FaceRef;
      const sh0 = "s0" as ShellRef;
      const sh1 = "s1" as ShellRef;
      model.faces[fShared] = { id: fShared, wireIds: [] };
      model.shells[sh0] = { id: sh0, faceIds: [fShared] };
      model.shells[sh1] = { id: sh1, faceIds: [fShared] };
      model.solids["c0" as SolidRef] = { id: "c0" as SolidRef, shellIds: [sh0] };
      model.solids["c1" as SolidRef] = { id: "c1" as SolidRef, shellIds: [sh1] };
      const res = await runConstruct("MATCH (a:Object {typology: 'spatial.shape.kernel.solid'})-[:ADJACENT_TO]-(b:Object {typology: 'spatial.shape.kernel.solid'}) RETURN a.id, b.id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.rows.length).toBeGreaterThan(0);
      const pair = res.rows.find((r) => String(r.c0) === "c0" && String(r.c1) === "c1");
      expect(pair).toBeDefined();
    });

    it("CALL view.* is unknown action", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL view.energy.energy.hull({}) YIELD data", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
        }),
      ).rejects.toThrow(/unknown action/i);
    });

    it("unknown CALL action throws", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL no.such.action({}) YIELD data", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
        }),
      ).rejects.toThrow(/unknown action/i);
    });

    it("CALL createBoxFromCorners yields diff and data.solid", async () => {
      const model = new Model();
      const res = await runConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [2,3,0], height: 4 }) YIELD diff, data.solid AS solid", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.diff).toBeDefined();
      expect(res.diff?.solids?.added?.length).toBeGreaterThan(0);
      expect(res.rows[0]?.solid).toBeDefined();
    });

    it("CALL selection.selectAll YIELD targets returns every box model kind", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const actions = modelDefinitionActionRegistry();
      expect(actions.get("selection.selectAll")).not.toBeNull();
      const res = await runConstruct("CALL selection.selectAll({}) YIELD targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions,
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(Array.isArray(targets)).toBe(true);
      expect(targets!.length).toBeGreaterThan(8);
      expect(targets!.some((t) => t.kind === "solid" && t.id === "box")).toBe(true);
    });

    it("CALL selection.apply invert uses construct selectionTargets seed", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const seed = [{ kind: "solid", id: "box", editable: true }];
      const res = await runConstruct("CALL selection.apply({ operation: 'invert' }) YIELD data.targets AS targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
        selectionTargets: seed,
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(targets!.some((t) => t.kind === "solid" && t.id === "box")).toBe(false);
      expect(targets!.length).toBeGreaterThan(0);
    });

    it("CALL selection.selectVertices YIELD targets lists only vertices", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const res = await runConstruct("CALL selection.selectVertices({}) YIELD targets", {
        model: model,
        kernel: new QueryTestKernel(),
        actions: modelDefinitionActionRegistry(),
      });
      const targets = res.rows[0]?.targets as { kind: string; id: string }[] | undefined;
      expect(targets?.length).toBe(8);
      expect(targets?.every((t) => t.kind === "vertex")).toBe(true);
    });

    it("rejects actions outside active model definition", async () => {
      const model = new Model();
      await expect(
        runConstruct("CALL primitive.createBoxFromCorners({ cornerA: [0,0,0], cornerB: [1,1,0], height: 1 })", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });

    it("rejects geometry selection commands under energy model definition", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      await expect(
        runConstruct("CALL selection.selectVertices({}) YIELD targets", {
          model: model,
          kernel: new QueryTestKernel(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/not available in model definition aec\.building\.energy/);
    });

    it("rejects geometry typology MATCH under energy model definition", async () => {
      const model = new Model();
      await expect(
        runConstruct("MATCH (o:Object {typology: 'spatial.shape.primitive.box'}) RETURN o.id", {
          model: model,
          kernel: mkKernelStub(),
          actions: modelDefinitionActionRegistry(),
          activeModelDefinitionId: "aec.building.energy",
        }),
      ).rejects.toThrow(/unknown typology spatial\.shape\.primitive\.box for model definition aec\.building\.energy/);
    });

    it("MATCH energy typology resolves to object rows", async () => {
      const model = new Model();
      const r = M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box"));
      applyModelDiff(model, r);
      model.objects["energy-hull"] = {
        id: "energy-hull" as import("../🟦️index.ts").ObjectRef,
        typology: "energy.energy.hull",
        primitives: { solid: String(r.solid ?? "box") },
      };
      const res = await runConstruct("MATCH (o:Object {typology: 'energy.energy.hull'}) RETURN o.id AS id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
        activeModelDefinitionId: "aec.building.energy",
      });
      expect(res.rows.some((row) => row.id === "energy-hull")).toBe(true);
    });

    it("MATCH spatial.shape.primitive.box typology resolves to solids", async () => {
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box")));
      const res = await runConstruct("MATCH (s:Object {typology: 'spatial.shape.primitive.box'}) RETURN s.id", {
        model: model,
        kernel: mkKernelStub(),
        actions: modelDefinitionActionRegistry(),
      });
      expect(res.rows.length).toBeGreaterThanOrEqual(1);
      expect(res.rows.some((r) => typeof r.c0 === "string")).toBe(true);
    });

    it("CALL selection.selectObjects YIELD targets from model objects", async () => {
      const model = new Model();
      const r = M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box"));
      applyModelDiff(model, r);
      model.objects["object-box"] = {
        id: "object-box" as import("../🟦️index.ts").ObjectRef,
        typology: "spatial.shape.primitive.box",
        primitives: { solid: String(r.solid ?? "box") },
      };
      const kernel = new QueryTestKernel();
      const res = await runConstruct("CALL selection.selectObjects({}) YIELD targets", {
        model: model,
        kernel,
        actions: modelDefinitionActionRegistry(),
      });
      const targets = res.rows[0]?.targets as { kind: string }[] | undefined;
      expect(targets!.length).toBeGreaterThan(0);
      expect(targets!.every((t) => t.kind === "object")).toBe(true);
    });
  });
}
// #endregion 🧪️Tests
