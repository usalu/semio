import {readFileSync,statfsSync} from "node:fs";
import {dirname,join} from "node:path";

/** 🔄️ The partitions whose image is sized elastically when it is created (`elastic` of the policy). */
export type ElasticPartitionIdV1="graph"|"products"|"rust";
/** 📐️ The elastic sizing rule of an image (`elastic.<partition>` of the policy). */
export type ElasticRuleV1=Readonly<{needBytes:number;headroomPercent:number;hostFreeReservePercent:number;alignBytes:number;maximumBytes:number;automatic:boolean;pressureFreeBytes:number}>;
/** 🖼️ Sizes a new image from its working set and the host: need times headroom, capped by the rule's maximum and by what the host volume may give (its free bytes minus the reserve), aligned down, never below the declared capacity. */
export function elasticCapacityV1(rule:ElasticRuleV1,minimumBytes:number,hostFreeBytes:number):number{
 const wanted=Math.ceil(rule.needBytes*(100+rule.headroomPercent)/100),ceiling=Math.min(rule.maximumBytes,Math.floor(hostFreeBytes*(100-rule.hostFreeReservePercent)/100)),target=Math.min(wanted,Math.max(ceiling,minimumBytes));
 return Math.max(minimumBytes,Math.floor(target/rule.alignBytes)*rule.alignBytes);
}
/** 🪪️ The capacity of an elastic image: the one its receipt pins when an image exists, else the elastic size a new image would get; any unreadable state keeps the declared capacity. */
export function effectiveCapacityV1(directory:string,minimumBytes:number,rule:ElasticRuleV1):number{
 try{
  const receipt=JSON.parse(readFileSync(join(directory,"image.json"),"utf8")) as {maximumPhysicalBytes?:unknown};
  return Number.isSafeInteger(receipt.maximumPhysicalBytes)&&(receipt.maximumPhysicalBytes as number)%1048576===0&&(receipt.maximumPhysicalBytes as number)>=minimumBytes?receipt.maximumPhysicalBytes as number:minimumBytes;
 }catch(error){
  if((error as NodeJS.ErrnoException)?.code!=="ENOENT")return minimumBytes;
  try{const host=statfsSync(dirname(directory));return elasticCapacityV1(rule,minimumBytes,host.bavail*host.bsize);}catch{return minimumBytes;}
 }
}
