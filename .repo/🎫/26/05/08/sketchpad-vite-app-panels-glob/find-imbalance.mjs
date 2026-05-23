import t from "typescript";
import { readFileSync } from "node:fs";

const file = process.argv[2];
const src = readFileSync(file, "utf8");
const sf = t.createSourceFile(file, src, t.ScriptTarget.Latest, true);
const lineStarts = sf.getLineStarts();
function lineOf(p) {
  let lo = 0, hi = lineStarts.length - 1;
  while (lo < hi) { const m = (lo + hi + 1) >> 1; if (lineStarts[m] <= p) lo = m; else hi = m - 1; }
  return lo + 1;
}

const sc = t.createScanner(t.ScriptTarget.Latest, false, t.LanguageVariant.Standard, src);
const stack = [];
let tok = sc.scan();
let watch = false;
while (tok !== t.SyntaxKind.EndOfFileToken) {
  const ln = lineOf(sc.getTokenStart());
  const before = stack.length;
  switch (tok) {
    case t.SyntaxKind.OpenParenToken: stack.push(["(", ln]); break;
    case t.SyntaxKind.CloseParenToken: {
      const top = stack[stack.length - 1];
      if (top && top[0] === "(") stack.pop();
      else console.log("orphan ) at", ln, "top was", top);
      break;
    }
    case t.SyntaxKind.OpenBracketToken: stack.push(["[", ln]); break;
    case t.SyntaxKind.CloseBracketToken: {
      const top = stack[stack.length - 1];
      if (top && top[0] === "[") stack.pop();
      else console.log("orphan ] at", ln);
      break;
    }
    case t.SyntaxKind.OpenBraceToken: stack.push(["{", ln]); break;
    case t.SyntaxKind.CloseBraceToken: {
      const top = stack[stack.length - 1];
      if (top && top[0] === "{") stack.pop();
      else if (top && top[0] === "${") {
        stack.pop();
        const reTok = sc.reScanTemplateToken(false);
        if (reTok === t.SyntaxKind.TemplateMiddle) stack.push(["${", ln]);
      } else console.log("orphan } at", ln, "top was", top);
      break;
    }
    case t.SyntaxKind.TemplateHead: stack.push(["${", ln]); break;
  }
  // turn watch on once we are around line 7740
  if (ln >= 7740 && ln <= 7748) {
    if (tok === t.SyntaxKind.OpenParenToken || tok === t.SyntaxKind.CloseParenToken || tok === t.SyntaxKind.OpenBraceToken || tok === t.SyntaxKind.CloseBraceToken) {
      console.log("line", ln, "tok", t.SyntaxKind[tok], "before depth", before, "after depth", stack.length, "top now:", stack[stack.length-1]);
    }
  }
  tok = sc.scan();
}
console.log("FINAL stack tail:", stack.slice(-15).map(x => `${x[0]}@${x[1]}`));
