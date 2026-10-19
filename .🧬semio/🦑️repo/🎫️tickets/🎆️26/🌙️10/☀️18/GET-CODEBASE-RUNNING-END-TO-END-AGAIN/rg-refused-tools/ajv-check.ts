import Ajv from "ajv";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🔣️.json";
import corpus from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🧪️.json";
const a = new Ajv({ strict: true }); console.log(a.validate(schema as any, corpus), JSON.stringify(a.errors)?.slice(0, 800));
