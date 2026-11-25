/** 🔎 Prints the Ajv errors of the wasm browser compiler fixture against its schema. */
import Ajv from "ajv";
import cases from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/🧪️tests/🧫️fixtures/🔣️.json";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/🧪️tests/🧫️fixtures/schema.json";
const ajv=new Ajv({strict:true});
console.log(ajv.validate(schema,cases), JSON.stringify(ajv.errors?.slice(0,5),null,1));
