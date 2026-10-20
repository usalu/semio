import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync} from "node:fs";
const src=readFileSync(process.argv[2],"utf8");
const sf=ts.createSourceFile("x.ts",src,ts.ScriptTarget.Latest,true);
const out:string[]=[];
const visit=(n:ts.Node)=>{
  if((ts.isStringLiteral(n)||ts.isNoSubstitutionTemplateLiteral(n))&&/^\.\.?(\/|$)/.test(n.text)){
    const line=sf.getLineAndCharacterOfPosition(n.getStart()).line+1;
    const p=n.parent;const ctx=p.getText(sf).slice(0,60).replace(/\n/g," ");
    const kind=ts.isImportDeclaration(p)?"IMPORT":ts.isCallExpression(p)?"CALL:"+p.expression.getText(sf).slice(0,30):ts.isNewExpression(p)?"NEW:"+p.expression.getText(sf):ts.isLiteralTypeNode(p)?"TYPE":ts.SyntaxKind[p.kind];
    out.push(`${line}\t${kind}\t${n.text}`);
  }
  ts.forEachChild(n,visit);
};
visit(sf);
console.log(out.join("\n"));
