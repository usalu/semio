import {readFileSync,writeFileSync} from "node:fs";
const root="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/";
for(const part of ["🩺️preflight","📦️dependencies","💾️cache","💾️cache/📋️ledger","🦀️cargo","⚙️runtime"]){
 const file=root+part+"/🟦️.ts",prefix=part.includes("/")?"../../":"../",before=readFileSync(file,"utf8"),needle="export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{";
 if(!before.includes(needle)||before.includes("answerKernelCheckV1"))throw Error("Unexpected kernel source: "+file);
 const after=`import {answerKernelCheckV1} from "${prefix}📥️entry/✅️check/🟦️.ts";\n`+before.replace(needle,needle+"if(answerKernelCheckV1(args,proof))return;");writeFileSync(file,after);
}
