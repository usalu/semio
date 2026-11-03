import {expect,test} from "bun:test";
import {stripWasmDebugSectionsV1} from "../🟦️.ts";

/** 🪶️ Target `🪶️strip-debug/🧪️tests/🟦️.ts`. The oracle is the host WebAssembly engine: the custom sections of the stripped core module, its validity, and the exported function still running. */
const varint=(value:number)=>{const out:number[]=[];let rest=value;do{let byte=rest%128;rest=Math.floor(rest/128);if(rest)byte|=128;out.push(byte);}while(rest);return out;},section=(id:number,body:number[])=>[id,...varint(body.length),...body],custom=(name:string,payload:number[])=>section(0,[...varint(name.length),...Buffer.from(name),...payload]);
const core=[0,0x61,0x73,0x6d,1,0,0,0,...section(1,[1,0x60,0,1,0x7f]),...section(3,[1,0]),...section(7,[1,1,0x66,0,0]),...section(10,[1,4,0,0x41,42,0x0b]),...custom(".debug_str",Array(2048).fill(7)),...custom("name",[1,2,3]),...custom(".debug_info",Array(512).fill(9))];
const component=(inner:number[])=>new Uint8Array([0,0x61,0x73,0x6d,0x0d,0,1,0,...section(1,inner),...section(1,inner)]);

test("stripping removes every .debug custom section of a core module, keeps the name section and the runnable code",()=>{
 const output=stripWasmDebugSectionsV1(new Uint8Array(core)),module=new WebAssembly.Module(output);
 expect(output.length).toBeLessThan(core.length-2500);
 expect(WebAssembly.Module.customSections(module,".debug_str")).toHaveLength(0);
 expect(WebAssembly.Module.customSections(module,".debug_info")).toHaveLength(0);
 expect(WebAssembly.Module.customSections(module,"name")).toHaveLength(1);
 expect((new WebAssembly.Instance(module).exports.f as ()=>number)()).toBe(42);
 expect(stripWasmDebugSectionsV1(output)).toEqual(output);
});

test("stripping a component rewrites each embedded core module and leaves the component header and layout intact",()=>{
 const input=component(core),output=stripWasmDebugSectionsV1(input),stripped=stripWasmDebugSectionsV1(new Uint8Array(core)),expected=new Uint8Array([0,0x61,0x73,0x6d,0x0d,0,1,0,...section(1,[...stripped]),...section(1,[...stripped])]);
 expect(output).toEqual(expected);
});
