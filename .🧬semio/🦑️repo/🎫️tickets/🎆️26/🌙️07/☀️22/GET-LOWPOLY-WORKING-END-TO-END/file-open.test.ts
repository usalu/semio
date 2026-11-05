/** 📂 Exercises the public Lowpoly picker path against an independent DOM implementation. */
import {afterEach,expect,test} from "bun:test";
import {JSDOM} from "jsdom";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const owner="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const fixture=JSON.parse(readFileSync(resolve(owner,"🧪️tests/💾️media-export-and-natural-files/file-open.json"),"utf8"));
const schema=JSON.parse(readFileSync(resolve(owner,"🧪️tests/💾️media-export-and-natural-files/file-open.schema.json"),"utf8"));
const validate=new Ajv({strict:true}).compile(schema);
if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const {requestFileOpen}=await import(pathToFileURL(resolve(owner,"🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts")).href);
let dom;
const originalDocument=Object.getOwnPropertyDescriptor(globalThis,"document");
const originalReader=Object.getOwnPropertyDescriptor(globalThis,"FileReader");
afterEach(()=>{dom?.window.close();for(const [key,value]of [["document",originalDocument],["FileReader",originalReader]]){if(value)Object.defineProperty(globalThis,key,value);else delete globalThis[key];}});
function picker(files=[],clickFailure=false){
 dom=new JSDOM("<!doctype html><body></body>");
 Object.defineProperty(globalThis,"document",{configurable:true,value:dom.window.document});
 Object.defineProperty(globalThis,"FileReader",{configurable:true,value:dom.window.FileReader});
 const input=dom.window.document.createElement("input");
 Object.defineProperty(input,"files",{configurable:true,value:files});
 let removed=0;
 const remove=input.remove.bind(input);input.remove=()=>{removed++;remove();};
 input.click=()=>{if(clickFailure)throw Error("fixture picker unavailable");};
 const create=dom.window.document.createElement.bind(dom.window.document);
 dom.window.document.createElement=(tag,...rest)=>tag==="input"?input:create(tag,...rest);
 return {input,removed:()=>removed};
}
for(const terminal of fixture.terminalEvents)test("public picker settles and detaches on "+terminal,async()=>{
 const p=picker([],terminal==="clickFailure");
 const opened=requestFileOpen(fixture.accept,fixture.readAs,fixture.multiple);
 if(terminal!=="clickFailure")p.input.dispatchEvent(new dom.window.Event(terminal==="empty"?"change":"cancel"));
 const observed=await Promise.race([opened.then(files=>({kind:"settled",files})),new Promise(resolve=>setTimeout(()=>resolve({kind:"pending"}),0))]);
 expect(observed).toEqual({kind:"settled",files:[]});
 expect(p.input.onchange).toBeNull();expect(p.input.oncancel).toBeNull();expect(p.removed()).toBe(1);
 p.input.dispatchEvent(new dom.window.Event("cancel"));expect(p.removed()).toBe(1);
});
test("public picker preserves selected file order names and real DOM dataURLs",async()=>{
 const p=picker();
 Object.defineProperty(p.input,"files",{configurable:true,value:fixture.files.map(file=>new dom.window.File([file.text],file.name,{type:file.type}))});
 const opened=requestFileOpen(fixture.accept,fixture.readAs,fixture.multiple);
 expect(p.input.accept).toBe(fixture.accept);expect(p.input.multiple).toBe(true);
 p.input.dispatchEvent(new dom.window.Event("change"));
 expect(await opened).toEqual(fixture.files.map(({name,contents})=>({name,contents})));
 expect(p.input.onchange).toBeNull();expect(p.input.oncancel).toBeNull();expect(p.removed()).toBe(1);
});
