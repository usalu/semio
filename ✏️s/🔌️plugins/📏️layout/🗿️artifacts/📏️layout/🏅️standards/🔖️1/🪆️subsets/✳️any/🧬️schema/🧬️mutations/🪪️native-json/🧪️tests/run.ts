import{checkLayoutNullableProfiles}from'../../../🔺️diff/🧪️tests/🫙️nullable/🟦️.ts';
import{profileFacetOracle}from'../../../🔺️diff/🧪️tests/🫙️nullable/oracle.mjs';
import {checkDataFieldsDecision,checkDataFieldsPlan,checkDataFieldsOutput} from '../../🧳️retained/🧾️data/🧪️tests/🟦️.ts';
import {dataOracle} from '../../🧳️retained/🧾️data/🧪️tests/oracle.mjs';
import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import Ajv from 'ajv';
import { testLayoutMutationTransport } from './🟦️.ts';
import { testLayoutGraphValidation } from '../../../✅️validation/🧪️tests/🟦️.ts';
import { graphValid } from '../../🧪️tests/✅️validation/oracle.mjs';
import { testLayoutMutationOracle } from '../../🧪️tests/⚙️semantic/🟦️.ts';
import {projectPatch} from '../../🧪️tests/⚙️semantic/oracle.mjs';
import {verifyLayoutSimpleDecision} from '../../🧳️retained/🎚️simple/🧪️tests/🟦️.ts';

/** 🧪️ Runs real canonical production paths against independently compiled primary schemas. */
export function runLayoutCanonicalTests(repositoryRoot: string) {
  const root = join(repositoryRoot, '✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any');
  const mutations = join(root, '🧬️schema/🧬️mutations');
  const schemaFiles = [
    join(root, '🧬️schema/🔣️.json'), join(mutations, '🔣️.json'),
    join(repositoryRoot, '🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🔣️.json'),
    ...['🔣️.json', '🔗️link/🧬️schema/🔣️.json', '🪆️child/🧬️schema/🔣️.json', '📦️blob/🧬️schema/🔣️.json'].map(file => join(repositoryRoot, '🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference', file)),
  ];
  for (const entry of readdirSync(mutations, { withFileTypes: true })) if (entry.isDirectory()) {
    const file = join(mutations, entry.name, '🧬️schema/🔣️.json'); if (existsSync(file)) schemaFiles.push(file);
  }
  const ajv = new Ajv({ strict: false, strictNumbers: true, allErrors: true, validateFormats: false });
  for (const file of schemaFiles) ajv.addSchema(JSON.parse(readFileSync(file, 'utf8')));
  const mutationSchema = ajv.getSchema('https://json.schemas.assets.semio-tech.com/s/layout/layout/mutations.json');
  if (!mutationSchema) throw new Error('Layout mutation schema is unregistered');
  return { nullableProfiles: checkLayoutNullableProfiles(profileFacetOracle(repositoryRoot)), schemas: schemaFiles.length, graph: testLayoutGraphValidation(graphValid), transport: testLayoutMutationTransport(value => mutationSchema(value) === true), semantic: testLayoutMutationOracle(), dataFieldsDecision: checkDataFieldsDecision(dataOracle()), dataFieldsPlan: checkDataFieldsPlan(dataOracle()), dataFieldsOutput: checkDataFieldsOutput(dataOracle()), retainedDecision: verifyLayoutSimpleDecision({apply<T>(before:T,ops:readonly unknown[]):T{return projectPatch(before,ops) as T;}}) };
}
