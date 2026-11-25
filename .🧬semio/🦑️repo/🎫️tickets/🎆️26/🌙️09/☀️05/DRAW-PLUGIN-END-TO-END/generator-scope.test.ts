import { test, expect } from 'bun:test';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import Ajv from 'ajv';
import { validateGeneratorContractsAgainstWorkspace } from '../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts';
const root=process.cwd();
const taxonomy=JSON.parse(readFileSync(resolve(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'),'utf8'));
test('scoped validation retains the complete owner authority',()=>{
 expect(new Ajv().validate({type:'object',required:['assets-build'],properties:{'assets-build':{type:'object'}}},taxonomy.generatorContracts)).toBe(true);
 const changed={...taxonomy,generatorContracts:{...taxonomy.generatorContracts, 'absent-test-owner':{target:'absent:test',ownerPath:'absent',outputRoots:[]}}};
 expect(validateGeneratorContractsAgainstWorkspace(root,changed,new Set())).toEqual([]);
 expect(validateGeneratorContractsAgainstWorkspace(root,changed,new Set(['absent-test-owner']))).toHaveLength(1);
});
