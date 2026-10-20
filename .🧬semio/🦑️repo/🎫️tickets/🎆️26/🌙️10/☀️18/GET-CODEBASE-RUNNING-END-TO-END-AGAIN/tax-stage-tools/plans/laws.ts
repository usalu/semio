// ⚖️ Source-shape laws that encoded "the router script exports its command classes": the classes now live in the lifted command module, the router only registers them.
import {LIB, Vfs} from "../vfs.ts";

const E = (cp: number): string => String.fromCodePoint(cp) + "️";

export function planLaws(v: Vfs, log: (row: Record<string, unknown>) => void): void {
  const caching = E(0x26A1) + "caching", boot = E(0x1F680) + "bootstrap", deps = E(0x1F4E6) + "dependencies", TS = E(0x1F7E6) + ".ts", script = E(0x1F4DC) + "script.ts";
  const test = `${LIB}/${E(0x1F3D7)}build/${E(0x1F9EA)}tests/${E(0x1F578)}graph/${TS}`, fixture = `${LIB}/${E(0x1F3D7)}build/${E(0x1F9EA)}tests/${E(0x1F9EB)}fixtures/${E(0x1F578)}graph/${E(0x1F523)}.json`;
  const commandModule = `${caching}/${boot}/${deps}/${E(0x1F9EA)}tests/${E(0x1F52C)}script-commands/${TS}`;
  if (!v.exists(`${LIB}/${commandModule}`)) throw new Error("dependency command module missing");
  v.edit(test, "router=join(library,row.router),parser=", "router=join(library,row.router),commandModule=join(library,row.commandModule),parser=");
  v.edit(test, "commandSyntax=parser.createSourceFile(router,commands,parser.ScriptTarget.Latest,true),", "commandSyntax=parser.createSourceFile(router,commands,parser.ScriptTarget.Latest,true),moduleSyntax=parser.createSourceFile(commandModule,readFileSync(commandModule,\"utf8\"),parser.ScriptTarget.Latest,true),");
  v.edit(test, "expect(commandSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text)).toEqual(row.commands);", "expect(commandSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)).map((node:any)=>node.name.text)).toEqual([]);expect(moduleSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text)).toEqual(row.commands);for(const name of row.commands)expect(commands).toContain(name);");
  v.edit(fixture, `"router": "${caching}/${boot}/${deps}/${script}",`, `"router": "${caching}/${boot}/${deps}/${script}",\n    "commandModule": "${commandModule}",`);
  log({kind: "law", test: test.replace(LIB, "L"), fixture: fixture.replace(LIB, "L")});
}
