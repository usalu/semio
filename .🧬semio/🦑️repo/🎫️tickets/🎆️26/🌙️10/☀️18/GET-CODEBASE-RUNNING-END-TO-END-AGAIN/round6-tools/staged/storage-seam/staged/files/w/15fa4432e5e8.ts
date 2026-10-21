import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {join,resolve} from "node:path";
import schema from "../../🧬️schema/🔣️.json";
import platformSchema from "../../../🌐️platform/🧬️schema/🔣️.json";
import directorySchema from "../../../📁️directory/🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {nativeStorageSelectionV1,selectedNativeStorageDeclarationV1,withNativeStorageSelectionV1,nativeStorageDispatchKeyV1,nativeStorageHostMountsVolumesV1} from "../../📋️contract/🟦️.ts";
import {nativeStorageBackendsV1,selectNativeStorageBackendV1} from "../../🧭️selection/🟦️.ts";

const refusal=(key:string)=>(fixture.refusalText as Record<string,string>)[key]!;
const protocol=schema.definitions.Protocol.const;

test("the declared backends validate against their schema, the vocabularies match the shared platform schema and the directory protocol",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);
 const backend=ajv.compile({$ref:schema.$id+"#/definitions/Backend"});
 for(const row of protocol.backends)expect(backend(row),JSON.stringify(backend.errors)).toBe(true);
 expect(new Set(protocol.backends.map(row=>row.id)).size).toBe(protocol.backends.length);
 expect(schema.definitions.Platform.enum).toEqual(platformSchema.enum);
 const directory=protocol.backends.find(row=>row.id==="directory-partition")!;
 expect(directory.backingFilename).toBe(directorySchema.definitions.Protocol.const.anchor);
 const selection=ajv.compile({$ref:schema.$id+"#/definitions/Selection"});
 for(const row of fixture.rows)if(row.selection.platform!==undefined&&!["freebsd"].includes(row.selection.platform)&&!((row.selection as {require?:string[]}).require?.includes("root-privilege")))expect(selection(row.selection),row.id+JSON.stringify(selection.errors)).toBe(true);
});

test("the selection table gives the same answer in TypeScript and in the independent Python oracle",()=>{
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle.py"),resolve(import.meta.dir,"../../🧬️schema/🔣️.json"),resolve(import.meta.dir,"🧫️fixtures/🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as Record<string,string>;
 for(const row of fixture.rows){
  let actual:string;try{actual=selectNativeStorageBackendV1(row.selection as never).id;}catch(error){const message=(error as Error).message,key=Object.keys(fixture.refusalText).find(name=>message.startsWith(refusal(name)));actual="!"+key;}
  const expected:string="expect"in row?(row as {expect:string}).expect:"!"+(row as {refusal:string}).refusal;
  expect(actual,row.id).toBe(expected);expect(oracle[row.id],row.id+" (oracle)").toBe(expected);
 }
});

test("a scoped selection, the declared environment and the host platform feed the same pure function in that order",()=>{
 expect(nativeStorageSelectionV1({},"linux")).toEqual({platform:"linux"});
 expect(nativeStorageSelectionV1({[protocol.environment.backend]:"directory-partition",[protocol.environment.require]:"unprivileged, atomic-exchange"},"darwin")).toEqual({platform:"darwin",backend:"directory-partition",require:["unprivileged","atomic-exchange"]});
 expect(selectedNativeStorageDeclarationV1({platform:"darwin"}).id).toBe("mounted-image");
 expect(withNativeStorageSelectionV1({platform:"darwin",backend:"directory-partition"},()=>selectedNativeStorageDeclarationV1().id)).toBe("directory-partition");
 expect(withNativeStorageSelectionV1({platform:"darwin"},()=>withNativeStorageSelectionV1({backend:"directory-partition"},()=>nativeStorageSelectionV1()))).toMatchObject({platform:"darwin",backend:"directory-partition"});
 expect(withNativeStorageSelectionV1({platform:"darwin",backend:"directory-partition"},()=>nativeStorageDispatchKeyV1())).toBe("linux");
 expect(withNativeStorageSelectionV1({platform:"darwin"},()=>nativeStorageDispatchKeyV1())).toBe("darwin");
 expect(withNativeStorageSelectionV1({platform:"win32"},()=>nativeStorageDispatchKeyV1())).toBe("win32");
 expect(nativeStorageHostMountsVolumesV1("darwin")).toBe(true);expect(nativeStorageHostMountsVolumesV1("linux")).toBe(false);expect(nativeStorageHostMountsVolumesV1("win32")).toBe(false);
});

test("concurrent scoped selections stay isolated across awaits",async()=>{
 const read=(backend:string)=>withNativeStorageSelectionV1({platform:"darwin",backend},async()=>{await Bun.sleep(5);const before=selectedNativeStorageDeclarationV1().id;await Bun.sleep(5);return[before,selectedNativeStorageDeclarationV1().id];});
 const [first,second]=await Promise.all([read("mounted-image"),read("directory-partition")]);
 expect(first).toEqual(["mounted-image","mounted-image"]);expect(second).toEqual(["directory-partition","directory-partition"]);
 expect(nativeStorageBackendsV1.length).toBe(protocol.backends.length);
});
