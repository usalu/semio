/** 🔸️ The resolve qualified function phase of inspectRustNonRepoJoinBaseSpans. */
import { rustFindTopLevel, rustTokenSegments } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import type { buildTokensState } from "../🔶️tokens/🟦️.ts";
import type { Binding } from "../🟢️shared/🟦️.ts";

/** 🔹 Builds resolveQualifiedFunctionBody, tupleTailPositions, tupleCallNonRepoPositions of inspectRustNonRepoJoinBaseSpans. */
export function buildResolveQualifiedFunctionState({ tokens, pairs, rootEnd, literal }: Pick<ReturnType<typeof buildTokensState>, "tokens" | "pairs" | "rootEnd" | "literal">) {
  /** 🔎️ Locates one free function's body and positional parameter names — qualified to a specific
   * `mod NAME { .. }` block when `qualifier` is given, else uniquely by bare name — refusing on any
   * ambiguity (more than one candidate) exactly like `helperReturnsNonRepo`'s same-file uniqueness
   * requirement, just scoped one level narrower so two same-named functions in different modules
   * (e.g. two sibling test modules each declaring their own `materialize`) never collide. */
  const resolveQualifiedFunctionBody = (qualifier: string | null, name: string): { readonly open: number; readonly close: number; readonly params: readonly string[] } | null => {
    let searchStart = 0, searchEnd = tokens.length;
    if (qualifier !== null) {
      const modMatches: number[] = [];
      for (let index = 0; index < tokens.length; index++) if (tokens[index]?.text === "mod" && tokens[index + 1]?.text === qualifier && tokens[index + 2]?.text === "{") modMatches.push(index);
      if (modMatches.length !== 1) return null;
      const modOpen = modMatches[0]! + 2, modClose = pairs.get(modOpen);
      if (modClose === undefined) return null;
      searchStart = modOpen + 1; searchEnd = modClose;
    }
    const fnMatches: number[] = [];
    for (let index = searchStart; index < searchEnd; index++) if (tokens[index]?.text === "fn" && tokens[index + 1]?.text === name) fnMatches.push(index);
    if (fnMatches.length !== 1) return null;
    const fnIndex = fnMatches[0]!;
    if (tokens[fnIndex + 2]?.text !== "(") return null;
    const paramsOpen = fnIndex + 2, paramsClose = pairs.get(paramsOpen);
    if (paramsClose === undefined) return null;
    const braceStart = rustFindTopLevel(tokens, pairs, paramsClose + 1, searchEnd, new Set(["{", ";"]));
    const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
    if (braceEnd === undefined) return null;
    const params = rustTokenSegments(tokens, pairs, paramsOpen + 1, paramsClose, ",").map(([first, last]) => {
      const nameIndex = tokens[first]?.text === "mut" ? first + 1 : first;
      return tokens[nameIndex]?.kind === "identifier" && tokens[nameIndex + 1]?.text === ":" && nameIndex < last ? tokens[nameIndex]!.text : "";
    });
    return { open: braceStart + 1, close: braceEnd, params };
  };
  /** 🧩️ Evaluates one function body's TUPLE tail into per-position proven-non-repo flags, given the
   * literal string arguments (if any) its caller passed for named parameters. A leading top-level
   * `match <param> { "lit" => .., .., _ => {} }` — reachable only when the body's tail has no
   * separating `;` from a preceding brace-terminated statement — is eliminated (treated as a no-op
   * and skipped) ONLY when every non-wildcard arm is a single string literal that provably does not
   * equal the known argument, and the trailing `_` arm is empty: that proves, for THIS SPECIFIC
   * call's actual argument, only the wildcard arm can run, without evaluating any other arm's effects
   * (sound constant-propagation, not "assume the happy path" — any arm that could match, or a match
   * whose scrutinee isn't a known literal, refuses instead of guessing). Bounded to exactly one same-
   * file hop for the callee's OWN internal lets (`depth: 1` below), matching the one-hop convention
   * used throughout this function. */
  const tupleTailPositions = (bodyOpen: number, bodyClose: number, literalBindings: ReadonlyMap<string, string>): readonly boolean[] | null => {
    const bindings = new Map<string, Binding>();
    const segments = rustTokenSegments(tokens, pairs, bodyOpen, bodyClose, ";");
    const hasTrailingSemicolon = bodyClose > bodyOpen && tokens[bodyClose - 1]?.text === ";";
    for (let index = 0; index < segments.length; index++) {
      const [segStart, segEnd] = segments[index]!, isTail = index === segments.length - 1 && !hasTrailingSemicolon;
      if (!isTail) {
        if (tokens[segStart]?.text === "let") {
          const equal = rustFindTopLevel(tokens, pairs, segStart + 1, segEnd, new Set(["="]));
          const nameIndex = tokens[segStart + 1]?.text === "mut" ? segStart + 2 : segStart + 1, nameToken = tokens[nameIndex];
          if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
            const proven = rootEnd(equal + 1, segEnd, bindings, 1) !== null;
            bindings.delete(nameToken.text);
            if (proven) bindings.set(nameToken.text, { kind: "nonrepo" });
          }
        }
        continue;
      }
      let tailStart = segStart;
      if (tokens[tailStart]?.text === "match") {
        const braceIndex = rustFindTopLevel(tokens, pairs, tailStart + 1, segEnd, new Set(["{"]));
        const matchClose = braceIndex < 0 ? undefined : pairs.get(braceIndex);
        if (braceIndex < 0 || matchClose === undefined) return null;
        const scrutinee = tokens.slice(tailStart + 1, braceIndex);
        if (scrutinee.length !== 1 || scrutinee[0]?.kind !== "identifier") return null;
        const literalValue = literalBindings.get(scrutinee[0]!.text);
        if (literalValue === undefined) return null;
        const arms = rustTokenSegments(tokens, pairs, braceIndex + 1, matchClose, ",");
        if (arms.length === 0) return null;
        for (let armIndex = 0; armIndex < arms.length; armIndex++) {
          const [armFirst, armLast] = arms[armIndex]!;
          const arrow = rustFindTopLevel(tokens, pairs, armFirst, armLast, new Set(["=>"]));
          if (arrow < 0) return null;
          const patternLength = arrow - armFirst, isLastArm = armIndex === arms.length - 1;
          if (isLastArm) {
            if (patternLength !== 1 || tokens[armFirst]?.text !== "_") return null;
            const bodyTokens = tokens.slice(arrow + 1, armLast);
            const emptyBody = bodyTokens.length === 0 || (bodyTokens.length === 2 && (bodyTokens[0]!.text === "{" && bodyTokens[1]!.text === "}" || bodyTokens[0]!.text === "(" && bodyTokens[1]!.text === ")"));
            if (!emptyBody) return null;
          } else {
            if (patternLength !== 1 || tokens[armFirst]?.kind !== "string") return null;
            if (tokens[armFirst]!.text.slice(1, -1) === literalValue) return null;
          }
        }
        tailStart = matchClose + 1;
        if (tailStart >= segEnd) return null;
      }
      if (tokens[tailStart]?.text === "(" && pairs.get(tailStart) === segEnd - 1) {
        const elements = rustTokenSegments(tokens, pairs, tailStart + 1, segEnd - 1, ",");
        if (elements.length < 2) return null;
        return elements.map(([elFirst, elLast]) => rootEnd(elFirst, elLast, bindings, 1) === elLast);
      }
      return [rootEnd(tailStart, segEnd, bindings, 1) === segEnd];
    }
    return null;
  };
  /** 🪢️ Resolves a tuple-pattern `let`'s right-hand-side call (`qualifier::name(..)` or `name(..)`,
   * consuming the ENTIRE span up to the statement's `;` — no trailing suffix chain trusted) into per-
   * position proven-non-repo flags for its returned tuple, threading any literal string arguments
   * into the callee by parameter name. */
  const tupleCallNonRepoPositions = (start: number, end: number, arity: number): readonly boolean[] | null => {
    let cursor = start, qualifier: string | null = null;
    if (tokens[cursor]?.kind !== "identifier") return null;
    if (tokens[cursor + 1]?.text === "::" && tokens[cursor + 2]?.kind === "identifier") { qualifier = tokens[cursor]!.text; cursor += 2; }
    const nameToken = tokens[cursor];
    if (nameToken?.kind !== "identifier" || tokens[cursor + 1]?.text !== "(") return null;
    const argOpen = cursor + 1, argClose = pairs.get(argOpen);
    if (argClose === undefined || argClose + 1 !== end) return null;
    const resolved = resolveQualifiedFunctionBody(qualifier, nameToken.text);
    if (!resolved || resolved.params.length === 0) return null;
    const argumentSpans = rustTokenSegments(tokens, pairs, argOpen + 1, argClose, ",");
    const literalBindings = new Map<string, string>();
    for (let argIndex = 0; argIndex < argumentSpans.length && argIndex < resolved.params.length; argIndex++) {
      const [argFirst, argLast] = argumentSpans[argIndex]!, parameterName = resolved.params[argIndex];
      if (argLast === argFirst + 1 && literal(tokens[argFirst]) && parameterName) literalBindings.set(parameterName, tokens[argFirst]!.text.slice(1, -1));
    }
    const positions = tupleTailPositions(resolved.open, resolved.close, literalBindings);
    return positions && positions.length === arity ? positions : null;
  };
  return { tupleCallNonRepoPositions };
}
