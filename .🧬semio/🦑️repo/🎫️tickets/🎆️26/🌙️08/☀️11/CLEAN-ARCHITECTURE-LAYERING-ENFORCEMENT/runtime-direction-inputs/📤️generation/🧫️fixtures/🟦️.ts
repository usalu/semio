/** 📤️ A token generator defers its emitted include expression to the consuming compilation. */
export type RustGeneratedTokenOutput = Readonly<{ macro: string; start: number; end: number; line: number; origin: Readonly<{ provider: "quote"; symbol: "quote"; kind: "direct-import" | "qualified"; importStart?: number; importEnd?: number }>; inputs: readonly Readonly<{ kind: "include" | "include_str" | "include_bytes"; expression: string; start: number; end: number; line: number }>[] }>;

/** 🪪️ Retains proven token-output syntax separately from the current crate's authored inputs. */
function rustGeneratedTokenOutputs(source: string, tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): readonly Readonly<{ first: number; close: number; output: RustGeneratedTokenOutput }>[] {
  const imports = new Map<string, Readonly<{ start: number; end: number }>>(), ambiguous = new Set<string>();
  const enclosing = (index: number) => [...pairs].some(([open, close]) => open < index && close > index && tokens[open]?.text === "{");
  let qualifiedClosed = true;
  for (let index = 0; index < tokens.length; index++) {
    if ((tokens[index]?.text === "mod" || tokens[index]?.text === "extern" && tokens[index + 1]?.text === "crate") && rustIdentifierSymbol(tokens[index + (tokens[index]?.text === "mod" ? 1 : 2)]) === "quote") qualifiedClosed = false;
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    if (end < 0) continue;
    const body = tokens.slice(index + 1, end), direct = body.length === 3 || body.length === 5 && body[3]?.text === "as";
    const fromQuote = body[0]?.text === "quote" && body[1]?.text === "::" && body[2]?.text === "quote", alias = direct && fromQuote ? rustIdentifierSymbol(body.at(-1)) : null;
    const attributes = [...pairs].some(([open, close]) => close === index - 1 && tokens[open]?.text === "[" && tokens[open - 1]?.text === "#");
    if (direct && fromQuote && alias && !enclosing(index) && !attributes && tokens[index - 1]?.text !== "pub") {
      if (imports.has(alias)) ambiguous.add(alias);
      else imports.set(alias, { start: tokens[index]!.start, end: tokens[end]!.end });
    } else if (body.some(token => token.text === "*")) { qualifiedClosed = false; ambiguous.add("*"); }
    else for (const token of body) if (rustIdentifierSymbol(token) === "quote") { qualifiedClosed = false; ambiguous.add("quote"); }
    index = end;
  }
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    if (end < 0) continue;
    for (const [alias, origin] of imports) if (tokens[index]!.start !== origin.start && tokens.slice(index + 1, end).some(token => rustIdentifierSymbol(token) === alias)) ambiguous.add(alias);
    index = end;
  }
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!") { const name = rustIdentifierSymbol(tokens[index + 2]); if (name) ambiguous.add(name); }
    if (tokens[index]?.text === "macro" || tokens[index]?.text === "mod") { const name = rustIdentifierSymbol(tokens[index + 1]); if (name) ambiguous.add(name); }
  }
  const ranges: Readonly<{ first: number; close: number; output: RustGeneratedTokenOutput }>[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index + 1]?.text !== "!") continue;
    let first = index;
    while (first >= 2 && tokens[first - 1]?.text === "::" && tokens[first - 2]?.kind === "identifier") first -= 2;
    const name = tokens.slice(first, index + 1).map(token => token.text).join(""), close = pairs.get(index + 2);
    if (close === undefined || name !== "quote::quote" && !imports.has(name) && name !== "quote") continue;
    const inputs: RustGeneratedTokenOutput["inputs"][number][] = [];
    for (let at = index + 3; at < close; at++) {
      const kind = rustIdentifierSymbol(tokens[at]);
      if (!["include", "include_str", "include_bytes"].includes(kind ?? "") || tokens[at + 1]?.text !== "!") continue;
      const end = pairs.get(at + 2);
      if (end === undefined || end > close) throw Error("Unsupported Rust compile token-output delimiter");
      inputs.push({ kind: kind as RustGeneratedTokenOutput["inputs"][number]["kind"], expression: source.slice(tokens[at + 2]!.end, tokens[end]!.start), start: tokens[at]!.start, end: tokens[end]!.end, line: source.slice(0, tokens[at]!.start).split("\n").length });
      at = end;
    }
    if (!inputs.length) continue;
    const qualified = name === "quote::quote", imported = imports.get(name);
    if (!qualifiedClosed || ambiguous.has("*") || ambiguous.has(name) || ambiguous.has("quote") || !qualified && !imported) throw Error("Unsupported Rust compile token-output origin: " + name);
    const origin: RustGeneratedTokenOutput["origin"] = { provider: "quote", symbol: "quote", kind: qualified ? "qualified" : "direct-import", ...(imported && !qualified ? { importStart: imported.start, importEnd: imported.end } : {}) };
    ranges.push({ first, close, output: { macro: name, start: tokens[first]!.start, end: tokens[close]!.end, line: source.slice(0, tokens[first]!.start).split("\n").length, origin, inputs } });
    index = close;
  }
  return ranges;
}
