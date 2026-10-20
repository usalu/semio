import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
export function syntaxErrors(path, text) {
  const kind = path.endsWith(".mjs") ? ts.ScriptKind.JS : path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, kind);
  return sf.parseDiagnostics.map((d) => `${sf.getLineAndCharacterOfPosition(d.start).line + 1}: ${ts.flattenDiagnosticMessageText(d.messageText, "\n")}`);
}
export function docEmojiDuplicates(text) {
  const counts = new Map();
  for (const m of text.matchAll(/\/\*\*\s*(?:\*\s*)?(\p{Extended_Pictographic}️?)/gu)) counts.set(m[1], (counts.get(m[1]) ?? 0) + 1);
  return counts;
}
