import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
await Parser.init();
const wasm=join(dirname(createRequire("/Users/ueli/Documents/semio/package.json").resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const p=new Parser();p.setLanguage(await Parser.Language.load(wasm));
for(const t of ["#![a]\n#![b]\nuse a;\nuse b;\nfn f() {}\n","#![allow(x)]\n#![allow(y)]\nuse a;\n"]){const r=p.parse(t).rootNode;console.log(r.namedChildren.map(c=>c.type+":"+JSON.stringify(c.text)));}
