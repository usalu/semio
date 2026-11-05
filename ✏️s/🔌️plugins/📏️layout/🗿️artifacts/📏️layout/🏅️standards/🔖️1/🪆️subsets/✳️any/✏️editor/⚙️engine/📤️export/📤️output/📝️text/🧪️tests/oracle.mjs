/** 🔮️ Independent Node Buffer UTF8 encoding and Ajv validate bounded fragment publication. */
import {createRequire} from 'node:module';const require=createRequire(import.meta.url),{semioSchemaAjvV1}=require('@semio-tech/framework/schema/oracles/validator');
export function fragmentOracle(){const ajv=semioSchemaAjvV1();return{validate:(schema,value)=>ajv.validate(schema,value),bytes:value=>Buffer.from(value,'utf8')};}
