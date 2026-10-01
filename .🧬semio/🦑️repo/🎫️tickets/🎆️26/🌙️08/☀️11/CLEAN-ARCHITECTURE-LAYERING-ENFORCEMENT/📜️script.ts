import ts from "typescript";
import { readFileSync } from "node:fs";
const file = process.argv[2]!;
const program=ts.createProgram([file],{target:ts.ScriptTarget.ESNext,noResolve:true});
const sf=program.getSourceFile(file)!;const checker=program.getTypeChecker();
const source=readFileSync(file,"utf8");const start=source.indexOf("//#region 💡️Inference");const end=source.indexOf("//#endregion 💡️Inference",start);
const names=new Map<string,string>();
function visit(node:ts.Node){if(node.pos<end&&node.end>start){if(ts.isIdentifier(node)){const sym=checker.getSymbolAtLocation(node);const dec=sym?.declarations?.[0];if(dec&& (dec.getSourceFile()!==sf||dec.pos<start||dec.end>end))names.set(node.text,ts.SyntaxKind[dec.kind]);}ts.forEachChild(node,visit);}}visit(sf);
console.log([...names].sort(([a],[b])=>a.localeCompare(b)));

