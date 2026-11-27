import Ajv from "ajv";import{readFileSync}from"node:fs";
const d="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/";
const [s,v]=process.argv.slice(2);const a=new Ajv({strict:true});const f=a.compile(JSON.parse(readFileSync(d+s,"utf8")));console.log(f(JSON.parse(readFileSync(d+v,"utf8"))),JSON.stringify(f.errors));
