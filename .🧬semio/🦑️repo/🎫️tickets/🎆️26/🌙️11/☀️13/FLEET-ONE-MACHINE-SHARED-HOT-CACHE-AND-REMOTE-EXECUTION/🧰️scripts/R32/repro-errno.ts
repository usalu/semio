import {dlopen,FFIType as f,toArrayBuffer} from "bun:ffi";
import {openSync} from "node:fs";
const lib=dlopen("libSystem.B.dylib",{fstatat:{args:[f.i32,f.ptr,f.ptr,f.i32],returns:f.i32},__error:{args:[],returns:f.ptr}});
const fd=openSync(process.argv[2]!,0);
const name=Buffer.from(".reset.json\0"),buf=Buffer.alloc(144);
const seen=new Map<number,number>();
let junk:any[]=[];
const N=Number(process.argv[3]??200000);
for(let i=0;i<N;i++){
 if(i%7===0){junk.push(new Array(2000).fill(i));if(junk.length>200)junk=[];}
 const s=Number(lib.symbols.fstatat(fd,name,buf,0x20));
 if(s<0){const p=lib.symbols.__error()!;const n=new DataView(toArrayBuffer(p,0,4)).getInt32(0,true);seen.set(n,(seen.get(n)??0)+1);}
}
console.log(JSON.stringify([...seen]));
