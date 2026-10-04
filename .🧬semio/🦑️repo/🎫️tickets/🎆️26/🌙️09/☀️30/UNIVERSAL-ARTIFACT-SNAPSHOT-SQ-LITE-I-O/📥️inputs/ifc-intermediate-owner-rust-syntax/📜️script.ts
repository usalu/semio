/** 🔎️ Parses held IFC provider and mounted owning law syntax without type or runtime credit. */
import {readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const pair=JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/root-ifc-complete-intermediate-literal-reference-semantic-schema-provider-pairs.json","utf8"));
const files=pair.files.filter(file=>file.path.endsWith("🦀️.rs")).map(file=>({path:file.path,source:file.after,scope:"held"}));
for(const path of ["/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs","/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs"])files.push({path,source:readFileSync(path,"utf8"),scope:"mounted-test-only"});
const rows=files.map(file=>{const result=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true"],{input:file.source,encoding:"utf8"});return{path:file.path,scope:file.scope,exitCode:result.status,diagnostics:result.stderr??"",error:result.error?.message??null};});
writeFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated/root-ifc-intermediate-reference-held-provider-native-law-rust-syntax-receipt.json",JSON.stringify({scope:"Syntax-only; no typecheck or native assertions",rows},null,2)+"\n");
console.log(JSON.stringify({files:rows.length,errors:rows.filter(row=>row.exitCode!==0)}));process.exit(rows.every(row=>row.exitCode===0)?0:1);
