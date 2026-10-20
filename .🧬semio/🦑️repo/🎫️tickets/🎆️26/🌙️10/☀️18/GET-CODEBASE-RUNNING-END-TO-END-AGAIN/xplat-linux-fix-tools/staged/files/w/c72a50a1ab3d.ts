import {expect,test} from "bun:test";
import {concurrentFreeBytesV1,concurrentFreeFractionV1,type ConcurrentVolumeReaderV1} from "../../💾️storage/🟦️.ts";
import {concurrentTextV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const unreadable:ConcurrentVolumeReaderV1=async()=>{throw Object.assign(Error("ENOSYS: function not implemented, statfs"),{code:"ENOSYS"});};
const readable:ConcurrentVolumeReaderV1=async()=>({blocks:1000,bavail:250,bsize:4096});
const withLanguage=async<T>(language:string,body:()=>Promise<T>)=>{const before=process.env.SEMIO_LANGUAGE;process.env.SEMIO_LANGUAGE=language;try{return await body();}finally{if(before===undefined)delete process.env.SEMIO_LANGUAGE;else process.env.SEMIO_LANGUAGE=before;}};

test("a readable volume reports its free fraction and free bytes",async()=>{
 expect(await concurrentFreeFractionV1("/volume",readable)).toBe(0.25);expect(await concurrentFreeBytesV1("/volume",readable)).toBe(250*4096);
 expect(await concurrentFreeFractionV1("/volume",async()=>({blocks:0,bavail:0,bsize:4096}))).toBe(1);
});

test("an unreadable volume is refused in English and German with the path and the cause, never reported as empty or unlimited",async()=>{
 for(const language of["en","de"]){
  const expectedHead=concurrentTextV1(language,"freeSpaceUnreadable",{path:"/volume",reason:"ENOSYS: function not implemented, statfs"});
  for(const call of[()=>concurrentFreeBytesV1("/volume",unreadable),()=>concurrentFreeFractionV1("/volume",unreadable)]){
   const error=await withLanguage(language,()=>call()).then(()=>undefined,failure=>failure as Error);
   expect(error,language).toBeDefined();expect(error!.message).toBe(expectedHead);expect(error!.message).toContain("/volume");expect(error!.message).toContain("ENOSYS");expect((error as Error&{cause?:{code?:string}}).cause?.code).toBe("ENOSYS");
  }
 }
 expect(concurrentTextV1("en","freeSpaceUnreadable",{path:"p",reason:"r"})).not.toBe(concurrentTextV1("de","freeSpaceUnreadable",{path:"p",reason:"r"}));
 expect(Object.keys(schema.$defs.Texts.const.en).sort()).toEqual(Object.keys(schema.$defs.Texts.const.de).sort());
});
