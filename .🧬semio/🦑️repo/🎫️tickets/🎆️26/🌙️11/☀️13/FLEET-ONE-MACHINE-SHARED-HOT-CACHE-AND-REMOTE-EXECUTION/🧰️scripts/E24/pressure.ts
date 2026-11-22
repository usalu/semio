import Ajv from "/Users/ueli/Documents/semio/node_modules/ajv";
import {readFileSync} from "node:fs";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧬️schema/🔣️.json";
const policy=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json","utf8")),root=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🔣️.json","utf8"));
const a=new Ajv({strict:false});console.log("root",a.validate(root,policy),JSON.stringify(a.errors)?.slice(0,300));
console.log("eq",JSON.stringify(root.properties.storage.properties.pressure)===JSON.stringify(schema.definitions.Policy));
const b=new Ajv({strict:true});try{console.log("pol",b.validate(schema.definitions.Policy,policy.storage.pressure),JSON.stringify(b.errors)?.slice(0,300));}catch(e){console.log("throw",String(e).slice(0,300))}
