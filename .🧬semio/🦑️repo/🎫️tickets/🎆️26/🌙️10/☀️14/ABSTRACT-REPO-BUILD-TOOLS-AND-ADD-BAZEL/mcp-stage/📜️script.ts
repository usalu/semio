import * as tests from "bun:test";
import * as filesystem from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";

const root=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),ticket=resolve(import.meta.dir,".."),directory=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"),source=join(directory,"🟦️.ts"),subject=await import(source),{registerTests1}=await import(join(directory,"🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts"));
await registerTests1(tests as never,{...subject,...filesystem,join,tmpdir:()=>join(ticket,"🗑️generated")},{directory,url:pathToFileURL(source).href});
