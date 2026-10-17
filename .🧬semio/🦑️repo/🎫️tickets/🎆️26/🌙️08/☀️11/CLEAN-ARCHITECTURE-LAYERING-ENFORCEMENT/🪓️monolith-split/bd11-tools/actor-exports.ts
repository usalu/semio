import {types} from "@bytecodealliance/jco";
import ts from "typescript";
const wit = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit";
const generated = await types(wit, {name: "actor-contract", worldName: "actor", instantiation: "async", asyncMode: "jspi"});
const parsed = ts.createSourceFile("actor.d.ts", new TextDecoder().decode(generated["actor-contract.d.ts"]!), ts.ScriptTarget.Latest, true);
const actor = parsed.statements.find(node => ts.isInterfaceDeclaration(node) && node.name.text === "Actor") as ts.InterfaceDeclaration;
const expected: Record<string, string[]> = {};
for (const member of actor.members) {
  const name = member.name!.getText(parsed); if (!/^[a-z]+$/.test(name)) continue;
  const source = ts.createSourceFile(name + ".ts", new TextDecoder().decode(generated["interfaces/semio-framework-" + name + ".d.ts"]!), ts.ScriptTarget.Latest, true);
  expected[name] = source.statements.filter(ts.isFunctionDeclaration).map(node => node.name!.text);
}
console.log(JSON.stringify(expected));
