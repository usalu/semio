import Ajv from "ajv";import{readFileSync}from"node:fs";
const[schemaPath,fixturePath]=process.argv.slice(2);
const v=new Ajv({strict:false}).compile(JSON.parse(readFileSync(schemaPath,"utf8")));
console.log(v(JSON.parse(readFileSync(fixturePath,"utf8"))),JSON.stringify(v.errors));
