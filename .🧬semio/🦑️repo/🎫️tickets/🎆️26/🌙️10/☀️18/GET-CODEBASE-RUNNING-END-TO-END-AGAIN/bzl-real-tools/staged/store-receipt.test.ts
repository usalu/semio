// 🧾 Language-agnostic rows for the staged receipt guard: a target that declares no receipt output is never name-validated; a declaring target must match the closed pattern (Python re.fullmatch oracle).
import {expect,test} from "bun:test";
import {declaredStoreReceiptV1} from "./store-receipt.mjs";
const pattern="^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$",rows=[
 {target:"build",outputs:["{projectRoot}/.🧾️store/build.json"],receipt:true},
 {target:"🏭️generator",outputs:[],receipt:false},
 {target:"🏭️generator-manifests",outputs:["{projectRoot}/dist"],receipt:false},
 {target:"🏭️generator",outputs:["{projectRoot}/.🧾️store/🏭️generator.json"],receipt:"refused"},
 {target:"x".repeat(129),outputs:["{projectRoot}/.🧾️store/"+"x".repeat(129)+".json"],receipt:"refused"},
 {target:"test-e2e",outputs:undefined,receipt:false},
] as const;
const oracle=(target:string)=>Bun.spawnSync(["python3","-I","-c","import re,sys;print(int(bool(re.fullmatch(sys.argv[1],sys.argv[2]))))",pattern,target]).stdout.toString().trim()==="1";
for(const row of rows)test("receipt guard "+row.target.slice(0,16)+" "+String(row.receipt),()=>{
 const call=()=>declaredStoreReceiptV1({workspaceRoot:"/w",projectRoot:"p",target:row.target,outputs:row.outputs as unknown as string[]});
 if(row.receipt==="refused"){expect(call).toThrow("Closed store receipt target required");expect(oracle(row.target)).toBe(row.target.length<=128&&!row.target.includes("🏭"));}
 else if(row.receipt){expect(call()).toBe("/w/p/.🧾️store/"+row.target+".json");expect(oracle(row.target)).toBe(true);}
 else expect(call()).toBeUndefined();
});
