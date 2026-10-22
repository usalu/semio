import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {aggregateScope} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
import {hoistAggregateSupport} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🧰️support-hoist/🟦️.ts";
const cases=JSON.parse(readFileSync(new URL("u46_cases.json",import.meta.url),"utf8"));
const out:any={hoist:[],impl:[]};
for(const row of cases.hoists){
 const root=mkdtempSync(join(tmpdir(),"semio-dump46-")),directory=join(root,"agg");mkdirSync(directory);
 for(const name of row.taken)mkdirSync(join(directory,name));
 const r=hoistAggregateSupport({aggregateFile:join(directory,"mod.rs"),aggregateText:row.aggregate,aggregateName:"Agg",leafTexts:row.leaves,scope:aggregateScope(row.aggregate,row.eff,row.isComponent)});
 out.hoist.push(r===null?null:{aggregateText:r.aggregateText,supportFile:r.supportFile.slice(directory.length+1).split("\\").join("/"),supportText:r.supportText,moved:[...r.moved]});rmSync(root,{recursive:true,force:true});
}
writeFileSync(new URL("../gen/u46_goldens.json",import.meta.url),JSON.stringify(out,null,1));
