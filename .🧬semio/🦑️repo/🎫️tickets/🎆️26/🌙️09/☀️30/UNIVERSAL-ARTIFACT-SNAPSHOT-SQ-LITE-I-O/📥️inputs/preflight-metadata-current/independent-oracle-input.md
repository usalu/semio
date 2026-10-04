The initial independent oracle computation is retained here as source text because only 📜️script.ts is executable.

```ts
/** 🔬️ Independent canonical bytes and BLAKE3 oracle for the authored long metadata recipe. */
import {blake3} from "@noble/hashes/blake3.js";
const prefix="m".repeat(131072),texts=[prefix+"a",prefix+"z"];
const bytes:number[]=[];const integer=(n:number)=>{while(n>=128){bytes.push((n&127)|128);n=Math.floor(n/128);}bytes.push(n);};const text=(s:string)=>{const b=Buffer.from(s);integer(b.length);for(const byte of b)bytes.push(byte);};
integer(1);integer(1);integer(0);text("label");bytes.push(0,7);integer(2);for(const s of texts){integer(0);text(s);}
console.log(Buffer.from(blake3(Uint8Array.from(bytes))).toString("hex"));

```
