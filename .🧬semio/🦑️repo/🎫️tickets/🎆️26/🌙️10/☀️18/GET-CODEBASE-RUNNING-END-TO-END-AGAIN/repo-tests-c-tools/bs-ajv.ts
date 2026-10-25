import Ajv from "ajv";
import {readFileSync} from "node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const fixture=JSON.parse(readFileSync(L+"🧪️tests/🧫️fixtures/🔧️build-surfaces/🔣️.json","utf8")),schema=JSON.parse(readFileSync(L+"🧬️schema/🔧️build-surfaces/🔣️.json","utf8"));
const v=new Ajv({strict:true,allErrors:true}).compile(schema);
console.log("[prof]",v(fixture),JSON.stringify(v.errors?.slice(0,12),null,1));
