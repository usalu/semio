import Ajv from'ajv';
import{readFileSync,writeFileSync}from'node:fs';
const physical=JSON.parse(readFileSync(new URL('./🗑️generated/type-value-select-oracle.json',import.meta.url),'utf8')),metadata=JSON.parse(readFileSync(new URL('./🗑️generated/type-value-select-metadata-oracle.json',import.meta.url),'utf8'));
const ajv=new Ajv({strict:true}),rows=[];
for(const row of physical.rows){if(!row.physical)continue;const aggregate=metadata.aggregates[row.member];let cardinality=true;if(aggregate){const lower=aggregate.kind==='array'?aggregate.upper-aggregate.lower+1:aggregate.lower,upper=aggregate.kind==='array'?lower:aggregate.upper;cardinality=ajv.compile({type:'array',minItems:lower,maxItems:upper})(row.value.value)as boolean;}rows.push({id:row.id,rawAdmitted:row.admitted,rawExact:row.exact,metadataCardinality:cardinality,composite:row.exact&&row.admitted&&cardinality,expected:row.expected});}
writeFileSync(new URL('./🗑️generated/type-value-select-cardinality-oracle.json',import.meta.url),JSON.stringify({oracle:'IfcOpenShell physical+independent metadata+Ajv cardinality',metadata:metadata.aggregates,nativeExecuted:false,rows},null,2)+'\n');
console.log('[DEBUG]',JSON.stringify({physical:rows.length,raw:rows.filter(row=>row.rawExact&&row.rawAdmitted===row.expected).length,composite:rows.filter(row=>row.composite===row.expected).length,retainedRawGap:rows.filter(row=>row.rawAdmitted!==row.expected).map(row=>row.id)}));
if(!rows.every(row=>row.composite===row.expected))throw Error('composite cardinality oracle mismatch');
