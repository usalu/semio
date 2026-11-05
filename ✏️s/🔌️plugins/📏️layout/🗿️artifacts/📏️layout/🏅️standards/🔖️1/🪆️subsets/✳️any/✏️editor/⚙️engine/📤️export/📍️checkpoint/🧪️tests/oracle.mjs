/** 🔮️ Lodash independently counts adjacent publication/stage changes; Ajv validates their neutral schema. */
import {createRequire} from 'node:module';const require=createRequire(import.meta.url),{isEqual}=require('lodash'),{semioSchemaAjvV1}=require('@semio-tech/framework/schema/oracles/validator');
export function milestoneOracle(){const ajv=semioSchemaAjvV1();return{validate:(schema,value)=>ajv.validate(schema,value),changes:marks=>marks.slice(1).filter((mark,index)=>!isEqual(mark,marks[index])).length};}
