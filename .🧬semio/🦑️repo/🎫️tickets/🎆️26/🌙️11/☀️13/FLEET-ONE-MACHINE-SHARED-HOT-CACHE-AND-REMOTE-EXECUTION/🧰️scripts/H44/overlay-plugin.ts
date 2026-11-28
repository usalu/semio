import {plugin} from "bun";
import {existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
const live="/Users/ueli/Documents/semio/",overlay="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H44/overlay/";
const toLive=(path:string):string=>path.startsWith(overlay)?live+path.slice(overlay.length):path;
plugin({name:"h44-overlay",setup(build){
 build.onResolve({filter:/^\.{1,2}\//},args=>{
  const target=resolve(dirname(toLive(args.importer)),args.path);
  if(target.startsWith(live)){const mapped=overlay+target.slice(live.length);if(existsSync(mapped))return{path:mapped};}
  return{path:target};
 });
 build.onResolve({filter:/^[^./]/},args=>{
  if(args.path.startsWith("node:")||args.path.startsWith("bun:")||false)return undefined;
  try{return{path:Bun.resolveSync(args.path,dirname(toLive(args.importer)))};}catch{return undefined;}
 });
}});
