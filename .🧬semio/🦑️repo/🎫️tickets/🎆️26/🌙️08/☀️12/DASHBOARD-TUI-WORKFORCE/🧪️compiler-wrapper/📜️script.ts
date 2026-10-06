import { existsSync, readdirSync } from "node:fs";
import { resolve, sep } from "node:path";

const args=process.argv.slice(2), original=process.env.PATH??"", build=process.env.CARGO_BUILD_BUILD_DIR;
let omitted=0;
const root=build?resolve(build)+sep:undefined;
const filtered=original.split(";").filter(path=>{
  if(process.env.SEMIO_TEST_DLL_PATH_FILTER!=="1"||!root)return true;
  try{if(resolve(path).toLowerCase().startsWith(root.toLowerCase())&&existsSync(path)&&!readdirSync(path).some(file=>file.toLowerCase().endsWith(".dll"))){omitted++;return false;}}catch{}
  return true;
}).join(";");
console.error(`[DEBUG] compiler path chars=${original.length} omitted=${omitted} finalChars=${filtered.length}`);
const result=Bun.spawnSync(args,{env:{...process.env,PATH:filtered},stdin:"inherit",stdout:"inherit",stderr:"inherit"});
process.exit(result.exitCode);
