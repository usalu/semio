import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/";
test("Board public descriptors own first-party values and semantic visibility",()=>{
 for(const file of ["🦀️.rs","🧬️schema/🦀️.rs","🔌️ports/🦀️.rs","🔌️ports/🧬️schema/🦀️.rs","🔌️ports/➡️directed/🦀️.rs","🔌️ports/➡️directed/🧬️schema/🦀️.rs"]){const source=readFileSync(root+file,"utf8");expect(source).not.toContain("pub user_data: Option<serde_json::Value>");expect(source).not.toContain("pub nodes: Vec<serde_json::Value>");expect(source).not.toContain("pub fn board_json_visible");}
});
test("Canvas palette is a typed overlay and JSON belongs explicit IO",()=>{
 const source=readFileSync(root+"🔌️ports/➡️directed/🦀️.rs","utf8");expect(source).not.toContain("pub fn merge_from_json");expect(source).toContain("apply_overlay");
});

import fixtures from "./🔣️.json";
import Ajv from "ajv";
test("48 typed palette fields agree with independent closed JSON Schema",async()=>{
 const semantic=await import(join(process.cwd(),root,"🧬️schema/🎨️palette/🟦️.ts"));
 const io=await import(join(process.cwd(),root,"🚪️io/📝️text/🎨️palette/🟦️.ts"));
 const schema=JSON.parse(readFileSync(join(root,"🧬️schema/🎨️palette/🧬️overlay/🔣️.json"),"utf8"));
 const validate=new Ajv({strict:true}).compile(schema);
 const control=()=>({maximumBytes:16384,maximumNodes:512,maximumDepth:8,chunk:16,cancelled:()=>false,progress:()=>{},yield:async()=>{}});
 for(const overlay of fixtures.overlays){expect(validate(overlay)).toBe(true);const admitted=await io.decodeBoardPaletteOverlayJson(JSON.stringify(overlay),control());expect(admitted).toEqual(overlay);expect(semantic.applyBoardPaletteOverlay(fixtures.base,admitted)).toEqual({...fixtures.base,...overlay});}
 for(const row of fixtures.paletteSources){expect(validate(JSON.parse(row.source))).toBe(true);expect(await io.decodeBoardPaletteOverlayJson(row.source,control())).toEqual(row.expected);}
 for(const value of fixtures.invalidOverlays){expect(validate(value)).toBe(false);await expect(io.decodeBoardPaletteOverlayJson(JSON.stringify(value),control())).rejects.toThrow();}
 await expect(io.decodeBoardPaletteOverlayJson('{"nodeFill":[0,0,0,255],"nodeFill":[1,2,3,255]}',control())).rejects.toThrow();
 await expect(io.decodeBoardPaletteOverlayJson('{}',{...control(),cancelled:()=>true})).rejects.toThrow();
 await expect(io.decodeBoardPaletteOverlayJson('{"nodeFill":[0,0,0,255]}',{...control(),maximumBytes:1})).rejects.toThrow();
 console.log("[DEBUG] Board palettes 48fields+10shape/componentrefusals+duplicate+budget+cancellation");
});
test("pure owned visibility agrees with independent object precedence",async()=>{const {boardVisibleOption,boardVisibleOrTrue,boardLockedOption}=await import(join(process.cwd(),root,"🧬️schema/👁️visibility/🟦️.ts"));for(const row of fixtures.visibility){expect(boardVisibleOption(row.flags)??null).toEqual(row.option);expect(boardVisibleOrTrue(row.flags)).toBe(row.visible);expect(boardLockedOption(row.flags)??null).toEqual(row.locked);expect(boardVisibleOrTrue(row.flags)).toBe(typeof row.flags.hidden==="boolean"?!row.flags.hidden:row.flags.visible??true);}console.log("[DEBUG] Board visibility",fixtures.visibility.length,"independent cases");});

test("all direct Board consumers use canonical typed symbols",()=>{for(const owner of ["🦀️.rs","🔌️ports/🦀️.rs","🔌️ports/➡️directed/🦀️.rs","🔌️ports/➡️directed/➕️normal/🦀️.rs","🔌️ports/➡️directed/🕸️dag/🦀️.rs"]){const source=readFileSync(root+owner,"utf8");expect(source).not.toMatch(/pub (?:fn|[a-z_]+:)[^\n]*serde_json::/);expect(source).not.toContain("set_canvas_theme_from_json");expect(source).not.toContain("board_json_visible");expect(source).not.toContain("property_bag_from_value(&semio_framework_value::DslValue::from(v))");}});

test("typed edge tip catalog facts agree with independent schema and neutral defaults",async()=>{const {edgeTipFromCatalogEntry}=await import(join(process.cwd(),root,"🧬️schema/🔺️edge-tip/🟦️.ts"));const schema=JSON.parse(readFileSync(join(root,"🧬️schema/🔺️edge-tip/🔣️.json"),"utf8"));const validate=new Ajv({strict:true}).compile(schema);for(const row of fixtures.edgeTips){expect(validate(row.entry)).toBe(true);expect(edgeTipFromCatalogEntry(row.entry)).toEqual(row.expected);}console.log("[DEBUG] Board edge tips",fixtures.edgeTips.length,"independent schema/default vectors");});

import Parser from "web-tree-sitter";
import {dirname,resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {pathToFileURL} from "node:url";
test("actual Board schema graph validates all four neutral userdata families",()=>{const ajv=new Ajv({strict:true});const owners=[root,root+"🔌️ports/",root+"🔌️ports/➡️directed/"];for(const owner of owners){const path=resolve(owner,"🧬️schema/🔣️.json");ajv.addSchema(JSON.parse(readFileSync(path,"utf8")),pathToFileURL(path).href);}for(const payload of Object.values(fixtures.userdata)){for(const [owner,name,value]of [[owners[0],"NodeDescriptor",{id:"n",x:0,y:0,userData:payload}],[owners[1],"HandleDescriptor",{id:"h",nodeId:"n",angle:0,userData:payload}],[owners[2],"EdgeDescriptor",{id:"e",source:"h",target:"j",userData:payload}],[owners[2],"WireDescriptor",{id:"w",source:"h",userData:payload}]]as const){const validate=ajv.getSchema(pathToFileURL(resolve(owner,"🧬️schema/🔣️.json")).href+"#/$defs/"+name)!;expect(validate(value)).toBe(true);expect(validate({...value,id:42})).toBe(false);}}console.log("[DEBUG] Board descriptor graph 4families x5 userdata +20 mandatory scalar refusals");});
test("independent Rust parser accepts all Board and direct receiving sources",async()=>{await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));const files=new Set(execFileSync("rg",["--files",root,"-g","*.rs"],{encoding:"utf8"}).trim().split("\n"));const manifest=JSON.parse(readFileSync(join(import.meta.dir,"source-owners.json"),"utf8"));for(const path of manifest.receivers)files.add(path);files.add("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs");for(const file of files){const tree=parser.parse(readFileSync(file,"utf8"))!;expect(tree.rootNode.hasError(),file).toBe(false);tree.delete();}parser.delete();console.log("[DEBUG] Board independent Rust syntax",files.size,"sources; compilation not claimed");});
