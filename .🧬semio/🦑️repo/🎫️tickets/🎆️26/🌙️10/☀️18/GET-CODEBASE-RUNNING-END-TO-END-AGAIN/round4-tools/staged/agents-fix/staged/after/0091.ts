import { type RustToken, rustFindTopLevel, rustTokenPairs, rustTokenSegments, rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";

/** 🚱️ Proves a `.join()` call chain's ROOT can never resolve to a manifest-relative path at plan
 * time — `env::temp_dir()`, an `env::var`/`env::var_os` runtime lookup (optionally routed through
 * `std::env::args()`-style CLI access), or a bare `fn` parameter this file never itself binds to a
 * `CARGO_MANIFEST_DIR` proof — so literal segments joined onto it name a freshly synthesized runtime
 * path, not a rewritable repository reference. Distinguishes PROVEN-non-repo from merely-unproven:
 * an unrecognized root is left alone so the caller still treats it as unresolved. One bounded
 * same-file helper-function hop (depth 1) lets a `fn returning_a_path() -> PathBuf { ... }` whose own
 * tail expression is proven non-repo count for its callers too, without unbounded recursion.
 *
 * There is no whole-file unpaired-bracket bail (unlike the CARGO_MANIFEST_DIR provers): this function only ever SUPPRESSES a block and never emits a rewrite, and `inspectRustJoinArgumentSpans`, which shares this exact tokenizer/pairer and already runs unguarded on every file this feeds, proves the risk is contained; every match still requires the exact adjacent-token shape to fire. */
export function inspectRustNonRepoJoinBaseSpans(source: string): ReadonlySet<number> {
  if (!/\.\s*join\s*\(/u.test(source)) return new Set();
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
  const rows = new Set<number>();
  type Binding = { readonly kind: "nonrepo" };
  const literal = (token: RustToken | undefined): token is RustToken & { readonly kind: "string" } => token?.kind === "string" && token.text.startsWith('"') && token.text.endsWith('"') && !token.text.includes("\\");
  const tokensEqual = (start: number, end: number, expected: readonly string[]): boolean => end - start === expected.length && expected.every((text, offset) => tokens[start + offset]?.text === text);
  const matchCall = (cursor: number, segments: readonly string[]): { readonly openParen: number; readonly close: number } | null => {
    let index = cursor;
    if (tokens[index]?.text === "::") index++;
    for (let step = 0; step < segments.length; step++) {
      if (tokens[index]?.text !== segments[step]) return null;
      index++;
      if (step + 1 < segments.length) { if (tokens[index]?.text !== "::") return null; index++; }
    }
    if (tokens[index]?.text !== "(") return null;
    const close = pairs.get(index);
    return close === undefined ? null : { openParen: index, close };
  };
  /** 🧫️ Consumes a bounded whitelist of pass-through combinators that never smuggle a fresh,
   * possibly-manifest-rooted value into the chain — only exact-matched fallbacks are trusted.
   * `parent`/`to_path_buf`/`clone`/`as_path`/`to_owned` are included because each is a pure identity
   * or ancestor-of-self operation on the SAME filesystem path: none of them can turn a path rooted
   * outside the repo into one rooted inside it. `parent` in particular only ever walks UP the tree —
   * a temp directory's ancestor is still under the same non-repo root, however many hops are taken —
   * so it stays sound with no bound on repetition, unlike e.g. `join`, which is handled separately by
   * the caller precisely because it CAN introduce a fresh, potentially manifest-relative segment. */
  const passThroughSuffix = (start: number, end: number): number => {
    let cursor = start;
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 2]?.text === "(") {
      const name = tokens[cursor + 1]?.text, close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      const empty = close === cursor + 3;
      if ((name === "unwrap" || name === "ok" || name === "next" || name === "unwrap_or_default" || name === "parent" || name === "to_path_buf" || name === "clone" || name === "as_path" || name === "to_owned") && empty) { cursor = close + 1; continue; }
      if ((name === "expect" || name === "nth" || name === "skip") && close > cursor + 2) { cursor = close + 1; continue; }
      if (name === "unwrap_or_else" && (tokensEqual(cursor + 3, close, ["std", "::", "env", "::", "temp_dir"]) || tokensEqual(cursor + 3, close, ["env", "::", "temp_dir"]))) { cursor = close + 1; continue; }
      if (name === "map" && (tokensEqual(cursor + 3, close, ["PathBuf", "::", "from"]) || tokensEqual(cursor + 3, close, ["std", "::", "path", "::", "PathBuf", "::", "from"]))) { cursor = close + 1; continue; }
      break;
    }
    return cursor;
  };
  const helperCache = new Map<string, boolean>(), helperInProgress = new Set<string>();
  let functionDeclarations: Map<string, number[]> | undefined;
  /** 🪜️ One bounded hop into a same-file free function's own tail expression. Bails (never proves) on
   * anything but exactly one textual `fn NAME` match, keeping this sound without call-site tracing. */
  const helperReturnsNonRepo = (name: string, depth: number): boolean => {
    if (helperCache.has(name)) return helperCache.get(name)!;
    if (helperInProgress.has(name)) return false;
    helperInProgress.add(name);
    let result = false;
    if (!functionDeclarations) {
      functionDeclarations = new Map();
      for (let index = 0; index + 1 < tokens.length; index++) if (tokens[index]!.text === "fn") {
        const declared = tokens[index + 1]!.text, rows = functionDeclarations.get(declared);
        if (rows) rows.push(index);
        else functionDeclarations.set(declared, [index]);
      }
    }
    const matches = functionDeclarations.get(name) ?? [];
    if (matches.length === 1) {
      const cursor = matches[0]! + 2, paramsClose = tokens[cursor]?.text === "(" ? pairs.get(cursor) : undefined;
      const braceStart = paramsClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, paramsClose + 1, tokens.length, new Set(["{", ";"]));
      const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
      if (braceEnd !== undefined) result = bodyReturnsNonRepo(braceStart + 1, braceEnd, depth);
    }
    helperInProgress.delete(name);
    helperCache.set(name, result);
    return result;
  };
  /** 🌱️ Recognizes one non-repo root/base expression starting at `cursor`, returning the index just
   * past it (before any `.join()` chain), or null if unrecognized — callers must then still block.
   *
   * `<any path>::test_support::tempdir()` is such a root: a project-owned wrapper, uniquely defined once (`🏪️store/🦀️.rs` `pub fn tempdir()`) as `std::env::temp_dir().join(...)` and called via varying aliases/qualifiers (`store::test_support::tempdir()`, `crate::os_store::test_support::tempdir()`, …). It is matched by SUFFIX so any qualifying prefix path is accepted, but a BARE `tempdir()` with no `test_support::` qualifier ahead of it is never trusted. */
  const rootEnd = (cursor: number, end: number, bindings: ReadonlyMap<string, Binding>, depth: number): number | null => {
    if (tokens[cursor]?.text === "(") {
      const close = pairs.get(cursor);
      if (close === undefined || close >= end) return null;
      const inner = rootEnd(cursor + 1, close, bindings, depth);
      return inner === close ? close + 1 : null;
    }
    if (tokens[cursor]?.kind === "identifier" && tokens[cursor + 1]?.text !== "::" && bindings.get(tokens[cursor]!.text)?.kind === "nonrepo") return passThroughSuffix(cursor + 1, end);
    for (const segments of [["std", "env", "temp_dir"], ["env", "temp_dir"], ["std", "env", "args"], ["env", "args"]]) {
      const call = matchCall(cursor, segments);
      if (call && call.close === call.openParen + 1) return passThroughSuffix(call.close + 1, end);
    }
    if (tokens[cursor]?.kind === "identifier") {
      let suffixCursor = cursor;
      while (tokens[suffixCursor]?.kind === "identifier" && tokens[suffixCursor + 1]?.text === "::") suffixCursor += 2;
      if (suffixCursor > cursor && tokens[suffixCursor - 2]?.text === "test_support" && tokens[suffixCursor]?.text === "tempdir" && tokens[suffixCursor + 1]?.text === "(") {
        const close = pairs.get(suffixCursor + 1);
        if (close === suffixCursor + 2) return passThroughSuffix(close + 1, end);
      }
    }
    for (const segments of [["std", "env", "var"], ["env", "var"], ["std", "env", "var_os"], ["env", "var_os"]]) {
      const call = matchCall(cursor, segments);
      if (call && call.close > call.openParen) return passThroughSuffix(call.close + 1, end);
    }
    for (const segments of [["std", "fs", "canonicalize"], ["fs", "canonicalize"]]) {
      const call = matchCall(cursor, segments);
      if (call && rootEnd(call.openParen + 1, call.close, bindings, depth) === call.close) return passThroughSuffix(call.close + 1, end);
    }
    for (const segments of [["Path", "new"], ["std", "path", "Path", "new"], ["PathBuf", "from"], ["std", "path", "PathBuf", "from"]]) {
      const call = matchCall(cursor, segments);
      if (call && rootEnd(call.openParen + 1, call.close, bindings, depth) === call.close) return passThroughSuffix(call.close + 1, end);
    }
    if (depth > 0 && tokens[cursor]?.kind === "identifier" && tokens[cursor + 1]?.text === "(") {
      const close = pairs.get(cursor + 1);
      if (close !== undefined && close < end && helperReturnsNonRepo(tokens[cursor]!.text, depth - 1)) return passThroughSuffix(close + 1, end);
    }
    return null;
  };
  /** 🔗️ Walks a maximal run of `.join(...)` calls off an already-proven non-repo root, recording only
   * bare string-literal arguments — non-literal steps don't break the chain's proven status. */
  const walkJoinChain = (start: number, end: number): number => {
    let cursor = start;
    while (cursor + 2 < end && tokens[cursor]?.text === "." && tokens[cursor + 1]?.text === "join" && tokens[cursor + 2]?.text === "(") {
      const close = pairs.get(cursor + 2);
      if (close === undefined || close >= end) break;
      if (close === cursor + 4 && literal(tokens[cursor + 3])) rows.add(tokens[cursor + 3]!.start + 1);
      cursor = close + 1;
    }
    return cursor;
  };
  /** 🧮️ Sequentially threads `let` bindings through one function body's statements, then classifies
   * only whether its tail (implicit-return) expression's root is proven non-repo — no row recording,
   * since the outer `visit` below independently walks this exact same body for its own literal joins. Each `let` proves its right-hand side BEFORE deleting the old binding: `let x = f(x)` (re-binding shadowing, e.g. `PathBuf::from(out_dir)`) must still see the prior `x`. */
  const bodyReturnsNonRepo = (start: number, end: number, depth: number): boolean => {
    const bindings = new Map<string, Binding>();
    const segments = rustTokenSegments(tokens, pairs, start, end, ";");
    const hasTrailingSemicolon = end > start && tokens[end - 1]?.text === ";";
    for (let index = 0; index < segments.length; index++) {
      const [segStart, segEnd] = segments[index]!, isTail = index === segments.length - 1 && !hasTrailingSemicolon;
      if (!isTail) {
        if (tokens[segStart]?.text === "let") {
          const equal = rustFindTopLevel(tokens, pairs, segStart + 1, segEnd, new Set(["="]));
          const nameIndex = tokens[segStart + 1]?.text === "mut" ? segStart + 2 : segStart + 1, nameToken = tokens[nameIndex];
          if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
            const proven = rootEnd(equal + 1, segEnd, bindings, depth) !== null;
            bindings.delete(nameToken.text);
            if (proven) bindings.set(nameToken.text, { kind: "nonrepo" });
          }
        }
        continue;
      }
      return rootEnd(segStart, segEnd, bindings, depth) !== null;
    }
    return false;
  };
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
  /** 🧭️ Walks one token range, threading `let` bindings and descending into nested `fn` bodies. A tuple-pattern destructure `let (a, _, c) = qualifier::fn(..);` binds each named (non-`_`) position whose slot is independently proven non-repo by `tupleCallNonRepoPositions`, and any unrecognized shape just binds nothing (the prior, always-safe default). A plain `let` proves its right-hand side against the OLD binding of the name (self-shadowing, e.g. `let out_dir = PathBuf::from(out_dir)`) before clearing it, because deleting first would make the reference to the shadowed value unprovable. */
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        index = (pairs.get(index + 2) ?? end - 1) + 1;
        continue;
      }
      if (token.text === "fn") {
        const paramsOpen = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["("]));
        const paramsClose = paramsOpen < 0 ? undefined : pairs.get(paramsOpen);
        const braceStart = paramsClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, paramsClose + 1, end, new Set(["{", ";"]));
        const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
        if (paramsClose !== undefined && braceEnd !== undefined) {
          const fresh = new Map<string, Binding>();
          for (const [first, last] of rustTokenSegments(tokens, pairs, paramsOpen + 1, paramsClose, ",")) {
            const nameIndex = tokens[first]?.text === "mut" ? first + 1 : first;
            if (tokens[nameIndex]?.kind === "identifier" && tokens[nameIndex + 1]?.text === ":" && nameIndex < last) fresh.set(tokens[nameIndex]!.text, { kind: "nonrepo" });
          }
          visit(braceStart + 1, braceEnd, fresh);
        }
        index = braceStart < 0 ? (paramsClose === undefined ? index + 1 : paramsClose + 1) : (braceEnd ?? braceStart) + 1;
        continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"])), stop = boundary < 0 ? end : boundary;
        if (tokens[index + 1]?.text === "(") {
          const patternClose = pairs.get(index + 1);
          const elements = patternClose === undefined ? [] : rustTokenSegments(tokens, pairs, index + 2, patternClose, ",");
          const names: (string | null)[] = [];
          let patternOk = patternClose !== undefined && patternClose < stop && tokens[patternClose + 1]?.text === "=" && elements.length > 0;
          if (patternOk) for (const [first, last] of elements) {
            if (last === first + 1 && tokens[first]?.text === "_") { names.push(null); continue; }
            if (last === first + 1 && tokens[first]?.kind === "identifier") { names.push(tokens[first]!.text); continue; }
            if (last === first + 2 && tokens[first]?.text === "mut" && tokens[first + 1]?.kind === "identifier") { names.push(tokens[first + 1]!.text); continue; }
            patternOk = false; break;
          }
          if (patternOk) {
            for (const name of names) if (name) bindings.delete(name);
            const positions = tupleCallNonRepoPositions(patternClose! + 2, stop, names.length);
            if (positions) for (let position = 0; position < names.length; position++) { const name = names[position]; if (name && positions[position]) bindings.set(name, { kind: "nonrepo" }); }
          }
          index = boundary < 0 ? end : boundary + 1;
          continue;
        }
        const equal = rustFindTopLevel(tokens, pairs, index + 1, stop, new Set(["="]));
        const nameIndex = tokens[index + 1]?.text === "mut" ? index + 2 : index + 1, nameToken = tokens[nameIndex];
        if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
          const rootAfter = rootEnd(equal + 1, stop, bindings, 1);
          bindings.delete(nameToken.text);
          if (rootAfter !== null) { bindings.set(nameToken.text, { kind: "nonrepo" }); walkJoinChain(rootAfter, stop); }
        } else if (nameToken?.kind === "identifier") bindings.delete(nameToken.text);
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      const rootAfter = rootEnd(index, end, bindings, 1);
      if (rootAfter !== null) { index = walkJoinChain(rootAfter, end); continue; }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
  return rows;
}
