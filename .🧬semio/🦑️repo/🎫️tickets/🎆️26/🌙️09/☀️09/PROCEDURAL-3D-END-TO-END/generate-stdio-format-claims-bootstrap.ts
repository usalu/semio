// Writes the generated stdio format claims the same way the `generate-stdio-format-claims` target does, for a tree whose ignored output was removed.
import {mkdirSync,existsSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
const root=process.argv[2]!;
const {STDIO_FORMAT_CLAIMS_PATH,stdioFormatClaimsTextV1}=await import(resolve(root,"✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧭️format-claims/📜️generate.ts"));
const target=resolve(root,STDIO_FORMAT_CLAIMS_PATH),text=stdioFormatClaimsTextV1(root);
mkdirSync(dirname(target),{recursive:true});if(!existsSync(target)||readFileSync(target,"utf8")!==text)writeFileSync(target,text);console.log("[DEBUG] wrote",target,text.length);
