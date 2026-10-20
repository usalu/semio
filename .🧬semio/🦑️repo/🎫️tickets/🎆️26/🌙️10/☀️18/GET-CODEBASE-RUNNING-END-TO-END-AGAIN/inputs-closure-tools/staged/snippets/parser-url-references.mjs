/** 🔗️ Retains the literal paths a script resolves against itself with `new URL("…", import.meta.url)`, which name the test files it runs and the data it reads. */
export function commandUrlReferences(path, source) {
  if (!/new\s+URL\s*\(/u.test(source) || !source.includes("import.meta.url") || /\.json$|\.d\.[cm]?ts$/u.test(path)) return [];
  const key = createHash("sha256").update(JSON.stringify([path, source, "urls"])).digest("hex"), previous = facts.get(key);
  if (previous) { hits++; facts.delete(key); facts.set(key, previous); return [...previous.imports]; }
  misses++;
  const compiler = require("typescript"), urls = new Set(), visit = node => {
    if (compiler.isNewExpression(node) && compiler.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.length >= 2 && compiler.isStringLiteralLike(node.arguments[0]) && compiler.isPropertyAccessExpression(node.arguments[1]) && node.arguments[1].name.text === "url" && compiler.isMetaProperty(node.arguments[1].expression) && node.arguments[1].expression.keywordToken === compiler.SyntaxKind.ImportKeyword) urls.add(node.arguments[0].text);
    compiler.forEachChild(node, visit);
  };
  visit(compiler.createSourceFile(path, source, compiler.ScriptTarget.Latest, false));
  const result = Object.freeze([...urls]), size = Buffer.byteLength(key) + Buffer.byteLength(JSON.stringify(result)) + 192;
  if (size <= limit) {
    while (bytes + size > limit) { const oldest = facts.keys().next().value; bytes -= facts.get(oldest).bytes; facts.delete(oldest); }
    facts.set(key, { imports: result, bytes: size }); bytes += size;
  }
  return [...result];
}

