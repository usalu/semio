/** 🔮️ Lodash independently counts adjacent publication/stage changes; Ajv validates their neutral schema. */
import {createRequire} from 'node:module';const require=createRequire(import.meta.url),{isEqual}=require('lodash'),Ajv=require('ajv/dist/2020.js');
export function milestoneOracle(){const ajv=new Ajv({strict:false});return{validate:(schema,value)=>ajv.validate(schema,value),changes:marks=>marks.slice(1).filter((mark,index)=>!isEqual(mark,marks[index])).length};}
