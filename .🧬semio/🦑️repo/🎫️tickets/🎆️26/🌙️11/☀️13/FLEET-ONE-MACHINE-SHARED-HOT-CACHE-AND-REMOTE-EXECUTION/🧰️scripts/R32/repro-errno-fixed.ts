import {dlopen,FFIType as f,toArrayBuffer} from "bun:ffi";
import {openSync} from "node:fs";
import {getSystemErrorName} from "node:util";
const {settleNativeCallV1}=await import(process.env.R32_DESCRIPTOR??"/Users/Shared/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts");
const lib=dlopen("libSystem.B.dylib",{fstatat:{args:[f.i32,f.ptr,f.ptr,f.i32],returns:f.i32},__error:{args:[],returns:f.ptr}});
const fd=openSync(process.argv[2]!,0),name=Buffer.from(".reset.json\0"),buf=Buffer.alloc(144),N=Number(process.argv[3]??1000000);
const readErrno=()=>new DataView(toArrayBuffer(lib.symbols.__error()!,0,4)).getInt32(0,true),transient=(n:number)=>n===4||getSystemErrorName(-n).startsWith("Unknown");
const raw=new Map<number,number>(),settled=new Map<string,number>();let junk:any[]=[];
for(let i=0;i<N;i++){
  if(i%7===0){junk.push(new Array(2000).fill(i));if(junk.length>200)junk=[];}
  const call=()=>Number(lib.symbols.fstatat(fd,name,buf,0x20));
  if(call()<0){const n=readErrno();raw.set(n,(raw.get(n)??0)+1);}
  const r=settleNativeCallV1(call,readErrno,transient,4,()=>{});
  const key=r.number+"/reads"+r.reads;settled.set(key,(settled.get(key)??0)+1);
}
console.log("raw single read (old):",JSON.stringify([...raw]),"settled (new):",JSON.stringify([...settled]));
