/** 🔮️ Independent Node Buffer UTF8 encoding and Ajv validate bounded fragment publication. */
import {createRequire} from 'node:module';const require=createRequire(import.meta.url),Ajv=require('ajv/dist/2020.js');
export function fragmentOracle(){const ajv=new Ajv({strict:false});return{validate:(schema,value)=>ajv.validate(schema,value),bytes:value=>Buffer.from(value,'utf8')};}
