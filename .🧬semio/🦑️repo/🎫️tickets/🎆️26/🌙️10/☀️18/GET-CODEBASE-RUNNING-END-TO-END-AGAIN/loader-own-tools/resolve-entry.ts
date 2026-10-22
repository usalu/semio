import {observedRequireResolve} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
export async function runCapturedBuildSourceV1(){
 const root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules",out:Record<string,string>={};
 const pairs:[string,string][]=[[root+"/📚️library/⚡️caching/🦀️cargo/📜️script.ts","../../🏗️build/🟦️.ts"],[root+"/📚️library/⚡️caching/🦀️cargo/📜️script.ts","typescript"],[root+"/📚️library/🏗️build/👁️observation/🟨️.mjs",root+"/../../🔨️modules/🏃️process/🧭️routing/🟦️.ts"],["/nonexistent/x.ts","./y.ts"]];
 for(const [from,request] of pairs){try{out[from.split("/").pop()+" "+request]="ok "+observedRequireResolve(from,request).split("/").slice(-2).join("/");}catch(e){out[from.split("/").pop()+" "+request]="ERR "+String((e as Error).message).slice(0,160);}}
 console.log(JSON.stringify(out,null,1));
}
