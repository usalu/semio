const [moduleRoot,worktree,which]=process.argv.slice(2) as [string,string,string];
const git=await import(`${moduleRoot}/🌿️git/🟦️.ts`);
const tracked=(await git.trackedEntriesV1(worktree))!;
const mb=()=>Math.round(process.memoryUsage().rss/1048576);
const ILLEGAL=/[<>:"|?*\\\u0000-\u001f\u007f-\u009f]/u;
const RESERVED=/^(?:CON|PRN|AUX|NUL|CONIN\$|CONOUT\$|CLOCK\$|COM[1-9¹²³]|LPT[1-9¹²³])$/u;
console.log("base",mb());
let t=performance.now(),n=0;
for(const e of tracked){const path=e.path;
 if(which==="nfc"&&path!==path.normalize("NFC"))n++;
 if(which==="split"){const parts=path.split("/");if(parts.some(part=>Buffer.byteLength(part)>255))n++;}
 if(which==="illegal"&&ILLEGAL.test(path))n++;
 if(which==="trail"){const parts=path.split("/");if(parts.some(part=>part.endsWith(".")||part.endsWith(" ")))n++;}
 if(which==="reserved"){const parts=path.split("/");if(parts.some(part=>RESERVED.test(part.split(".")[0].replace(/ +$/u,"").toUpperCase())))n++;}
 if(which==="reserved2"){const parts=path.split("/");if(parts.some(part=>/^[cpanl]/i.test(part)&&RESERVED.test(part.split(".")[0].replace(/ +$/u,"").toUpperCase())))n++;}
}
console.log(which,"ms",(performance.now()-t).toFixed(0),"rss",mb(),n);
