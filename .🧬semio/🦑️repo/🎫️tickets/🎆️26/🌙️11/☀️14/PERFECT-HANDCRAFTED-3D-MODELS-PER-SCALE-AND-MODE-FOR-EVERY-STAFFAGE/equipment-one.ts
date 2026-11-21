/** 🧰 Run one public product check on one document, reading only its referenced alternatives. */
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {validateAsset,validateRanges} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import {checkVolumes,type VolumeRule} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import {loadEnvironments} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import {DEFAULT_PRINT_PROFILE} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import {STAFFAGE} from "./volumes-lib.ts";
import {referencedAssets} from "./r3-validate.ts";

const [path,mode,den,ruleList]=process.argv.slice(2);
const doc=JSON.parse(readFileSync(path,"utf8"));
const asset=doc.asset;
let issues: unknown[];
if(mode==="document"){
  const schema=JSON.parse(readFileSync(join(STAFFAGE,"🧬️schema/🔣️.json"),"utf8"));
  const ajv=new Ajv({strict:true,allErrors:true});
  ajv.addSchema(schema);
  const matches=ajv.compile({$ref:schema.$id+"#/$defs/AssetDocument"});
  const schemaIssues=matches(doc)?[]:matches.errors??[];
  const assets=referencedAssets(asset);
  const environments=new Set((await loadEnvironments()).presets.map(p=>p.id));
  issues=[...schemaIssues,...validateAsset(asset,{assets,environments})];
}else if(mode==="paper"){
  issues=[...validateRanges(asset,DEFAULT_PRINT_PROFILE)];
}else if(mode==="volumes"){
  issues=[...checkVolumes(asset,{
    ...(den?{scale:Number(den) as 50|100|200|500}:{}),
    ...(ruleList?{rules:ruleList.split(",") as VolumeRule[]}:{}),
  })];
}else{
  throw Error("Supply document, paper or volumes [scale [comma-separated rules]]");
}
console.log(JSON.stringify({id:asset.id,mode,scale:den,rules:ruleList,issues}));
if(issues.length) process.exitCode=1;
