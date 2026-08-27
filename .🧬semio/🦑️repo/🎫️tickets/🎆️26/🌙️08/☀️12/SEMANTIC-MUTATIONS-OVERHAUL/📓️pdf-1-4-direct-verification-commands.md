# PDF 1.4 Direct Verification Commands

All commands ran from `/Users/ueli/Documents/semio`. These are ticket evidence, not new permanent runners. No Cargo command was started in this lane; the coordinator owns the registered Nx runtime gate. None of these commands reads or traverses `compose/**`.

## Fixture-First Structural Red

Nine direct inverse fixtures were authored before the Rust owners. This command returned exit 1 with `required=9, missing=9`, then exit 0 with `missing=0` after implementation.

```sh
bun -e 'import{existsSync}from"node:fs";const paths=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-page/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-page/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️move-page/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-page/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️replace-page-text/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/📝️set-page-text/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/🧹️clear-page-text/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations/📐️set-page-size/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations/📉️collapse-page-size/🦀️component.rs"];const missing=paths.filter(p=>!existsSync(p));console.log(JSON.stringify({required:paths.length,missing:missing.length,paths:missing}));process.exit(missing.length?1:0);'
```

## Ajv Descriptor, Payload, and Wire

```sh
bun -e 'import Ajv from "ajv";import {readFileSync,existsSync} from "node:fs";const defs=[{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📥️insert-page","kind":"insert-page","name":"InsertPage","tag":0,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🗑️remove-page","kind":"remove-page","name":"RemovePage","tag":1,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🔀️move-page","kind":"move-page","name":"MovePage","tag":2,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📐️resize-page","kind":"resize-page","name":"ResizePage","tag":3,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📝️replace-page-text","kind":"replace-page-text","name":"ReplacePageText","tag":4,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"📝️set-page-text","kind":"set-page-text","name":"SetPageText","tag":null,"subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"🧹️clear-page-text","kind":"clear-page-text","name":"ClearPageText","tag":null,"subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📐️set-page-size","kind":"set-page-size","name":"SetPageSize","tag":null,"subset":"x"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📉️collapse-page-size","kind":"collapse-page-size","name":"CollapsePageSize","tag":null,"subset":"x"}];const schema=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json","utf8"));const ajv=new Ajv({allErrors:true,strict:false});const validate=ajv.compile(schema);const errors=[];let descriptors=0,payloads=0,wireChecks=0;for(const d of defs){const owner=d.root+"/"+d.folder;const descriptor=JSON.parse(readFileSync(owner+"/🔣️component.json","utf8"));if(!validate(descriptor))errors.push({owner,type:"descriptor",errors:validate.errors});else descriptors++;if(descriptor.owner!==owner||descriptor.semanticKind!==d.kind||descriptor.aggregateVariant!==d.name||descriptor.binaryTag!==d.tag)errors.push({owner,type:"identity"});if(descriptor.requiredLanguageSurfaces.includes("json-schema")){const payloadSchema=JSON.parse(readFileSync(owner+"/🔣️payload.schema.json","utf8"));ajv.addSchema(payloadSchema,owner+"/🔣️payload.schema.json");const v=ajv.compile(payloadSchema);const f=JSON.parse(readFileSync(owner+"/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json","utf8"));if(!v(f.mutation.payload))errors.push({owner,type:"fixture",errors:v.errors});else payloads++;if(v({...f.mutation.payload,unexpected:true}))errors.push({owner,type:"additional-property"});wireChecks+=2;}}const root=defs[0].root;const rootSchema=JSON.parse(readFileSync(root+"/🔣️component.json","utf8"));delete rootSchema.$id;for(const branch of rootSchema.oneOf)branch.properties.payload.$ref=JSON.parse(readFileSync(root+"/"+branch.properties.payload.$ref,"utf8")).$id;const wire=ajv.compile(rootSchema);for(const d of defs.filter(d=>d.subset==="any")){const f=JSON.parse(readFileSync(d.root+"/"+d.folder+"/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json","utf8"));for(const op of[f.mutation,...f.inverse]){if(!wire(op))errors.push({type:"wire",op,errors:wire.errors});wireChecks++;}for(const bad of[{...f.mutation,payload:{phase:"apply",value:f.mutation.payload}},{...f.mutation,mutation:d.name[0].toLowerCase()+d.name.slice(1)},{...f.mutation,extra:true}]){if(wire(bad))errors.push({type:"bad-wire",bad});wireChecks++;}}console.log(JSON.stringify({descriptors,payloads,wireChecks,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "descriptors": 9,
  "payloads": 5,
  "wireChecks": 35,
  "errors": []
}
```

## Internal Validator Agreement

```sh
bun -e 'import Ajv from "ajv";import {readFileSync} from "node:fs";import {validateJsonSchemaSubset} from "./📜️script.ts";const schema=JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json","utf8"));const ajv=new Ajv({allErrors:true,strict:false});const validate=ajv.compile(schema);const defs=[{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📥️insert-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🗑️remove-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🔀️move-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📐️resize-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📝️replace-page-text","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"📝️set-page-text","subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"🧹️clear-page-text","subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📐️set-page-size","subset":"x"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📉️collapse-page-size","subset":"x"}];let checks=0;const errors=[];for(const d of defs){const owner=d.root+"/"+d.folder;const valid=JSON.parse(readFileSync(owner+"/🔣️component.json","utf8"));for(const [expected,value]of[[true,valid],[false,{...valid,outcomeClasses:[]}],[false,{...valid,invertibility:"unclassified"}],[false,{...valid,diffParticipation:"unclassified"}],[false,{...valid,binaryTag:-1}]]){const actual=validate(value),internal=validateJsonSchemaSubset(schema,value).length===0;checks++;if(actual!==expected||internal!==expected)errors.push({owner,expected,actual,internal});}if(d.subset==="any"){const schema=JSON.parse(readFileSync(owner+"/🔣️payload.schema.json","utf8"));const v=ajv.compile(schema);const fixture=JSON.parse(readFileSync(owner+"/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json","utf8"));for(const[expected,value]of[[true,fixture.mutation.payload],[false,{...fixture.mutation.payload,extra:true}]]){const actual=v(value),internal=validateJsonSchemaSubset(schema,value).length===0;checks++;if(actual!==expected||internal!==expected)errors.push({owner,expected,actual,internal});}}}console.log(JSON.stringify({checks,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "checks": 55,
  "errors": []
}
```

## Independent Page Vector Oracle

```sh
bun -e 'import _ from "lodash";import {readFileSync} from "node:fs";const defs=[{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📥️insert-page","kind":"insert-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🗑️remove-page","kind":"remove-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🔀️move-page","kind":"move-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📐️resize-page","kind":"resize-page","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📝️replace-page-text","kind":"replace-page-text","subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"📝️set-page-text","kind":"set-page-text","subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"🧹️clear-page-text","kind":"clear-page-text","subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📐️set-page-size","kind":"set-page-size","subset":"x"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📉️collapse-page-size","kind":"collapse-page-size","subset":"x"}];const apply=(base,op)=>{let next=_.cloneDeep(base);const p=op.payload;switch(op.mutation){case"insert-page":next.pages=_.concat(_.take(next.pages,p.index),[p.page],_.drop(next.pages,p.index));break;case"remove-page":_.pullAt(next.pages,p.index);break;case"move-page":{const moved=_.pullAt(next.pages,p.from);next.pages=_.concat(_.take(next.pages,p.to),moved,_.drop(next.pages,p.to));break;}case"resize-page":_.set(next,["pages",p.index,"width"],p.width);_.set(next,["pages",p.index,"height"],p.height);break;case"replace-page-text":_.set(next,["pages",p.index,"text"],p.text);break;case"set-page-text":_.set(next,["pages",0,"text"],p.text);break;case"clear-page-text":_.set(next,["pages",0,"text"],"");break;case"set-page-size":_.set(next,["pages",0,"width"],p.width);_.set(next,["pages",0,"height"],p.height);break;case"collapse-page-size":_.set(next,["pages",0,"width"],0);break;default:throw Error("Unknown kind");}return next;};let checks=0;const errors=[];for(const d of defs){const f=JSON.parse(readFileSync(d.root+"/"+d.folder+"/🧪️tests/round-trips-the-concrete-inverse/🔣️component.json","utf8"));const actual=apply(f.base,f.mutation);checks++;if(!_.isEqual(actual,f.expected))errors.push({kind:d.kind,law:"forward"});const restored=f.inverse.reduce(apply,actual);checks++;if(!_.isEqual(restored,f.base))errors.push({kind:d.kind,law:"inverse"});}console.log(JSON.stringify({oracle:"lodash",version:_.VERSION,checks,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "oracle": "lodash",
  "version": "4.18.1",
  "checks": 18,
  "errors": []
}
```

## Direct Surface and Catalog Parity

```sh
bun -e 'import{readFileSync,readdirSync,existsSync}from"node:fs";import YAML from"yaml";const defs=[{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📥️insert-page","mod":"insert_page","name":"InsertPage","kind":"insert-page","tag":0,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🗑️remove-page","mod":"remove_page","name":"RemovePage","kind":"remove-page","tag":1,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"🔀️move-page","mod":"move_page","name":"MovePage","kind":"move-page","tag":2,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📐️resize-page","mod":"resize_page","name":"ResizePage","kind":"resize-page","tag":3,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","folder":"📝️replace-page-text","mod":"replace_page_text","name":"ReplacePageText","kind":"replace-page-text","tag":4,"subset":"any"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"📝️set-page-text","mod":"set_page_text","name":"SetPageText","kind":"set-page-text","tag":null,"subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","folder":"🧹️clear-page-text","mod":"clear_page_text","name":"ClearPageText","kind":"clear-page-text","tag":null,"subset":"a"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📐️set-page-size","mod":"set_page_size","name":"SetPageSize","kind":"set-page-size","tag":null,"subset":"x"},{"root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","folder":"📉️collapse-page-size","mod":"collapse_page_size","name":"CollapsePageSize","kind":"collapse-page-size","tag":null,"subset":"x"}];const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const read=p=>readFileSync(p,"utf8");const errors=[];let descriptors=0,surfaces=0,vectors=0,registryCallbacks=0;const surfaceFiles={rust:"🦀️component.rs",typescript:"🟦️component.ts",graphql:"🔗️component.graphql",protobuf:"🛰️component.proto","json-schema":"🔣️payload.schema.json",text:"📝️text/🦀️component.rs",binary:"💾️binary/🦀️component.rs"};for(const d of defs){const owner=d.root+"/"+d.folder;const descriptor=JSON.parse(read(owner+"/🔣️component.json"));descriptors++;for(const surface of descriptor.requiredLanguageSurfaces){surfaces++;if(!existsSync(owner+"/"+surfaceFiles[surface]))errors.push({owner,surface});}const rootSource=read(d.root+"/🦀️component.rs");if(!rootSource.includes(d.name+"("+d.name+")")||!rootSource.includes(d.folder+"/🦀️component.rs"))errors.push({owner,kind:"aggregate"});const catalog=JSON.parse(read(d.root+"/../../🧪️oracle/🔣️.json")).mutationCatalogs[0];const vector=catalog.vectors.find(v=>v.mutationId===d.kind);if(!vector||vector.mutationDirectoryName!==d.folder)errors.push({owner,kind:"vector"});else for(const scenario of vector.scenarios){vectors++;if(!existsSync(owner+"/🧪️tests/"+scenario.directoryName+"/🔣️component.json"))errors.push({owner,kind:"fixture"});}if(d.subset==="any"){for(const facet of["text","binary"]){const component=facet==="text"?"📝️text":"💾️binary";const source=read(d.root+"/"+component+"/🦀️component.rs");for(const callback of facet==="text"?["print","parse"]:["encode","decode"]){registryCallbacks++;if(!source.includes("super::"+d.mod+"::"+facet+"::"+callback))errors.push({owner,kind:"registry",facet,callback});}const leaf=read(owner+"/"+component+"/🦀️component.rs");if(!leaf.includes("PdfMutation::"+d.name))errors.push({owner,kind:"codec-owner",facet});}for(const file of["🔗️component.graphql","🛰️component.proto"]){if(!read(d.root+"/"+file).includes(d.name))errors.push({owner,kind:"root-schema",file});}if(!read(d.root+"/📝️text/📖️component.grammar.semio").includes(d.kind+" = "))errors.push({owner,kind:"grammar"});}}
const ksy=YAML.parse(read(roots[0]+"/💾️binary/🥋️component.ksy"));for(const d of defs.filter(d=>d.subset==="any"))if(ksy.seq[2].type.cases[d.tag]!==d.mod)errors.push({kind:"ksy-tag",d});for(const root of roots){const rows=defs.filter(d=>d.root===root);const catalog=JSON.parse(read(root+"/../../🧪️oracle/🔣️.json")).mutationCatalogs[0];if(JSON.stringify(catalog.kinds)!==JSON.stringify(rows.map(d=>d.kind)))errors.push({root,kind:"catalog-order"});const source=read(root+"/🦀️component.rs");if(source.includes("pub const KINDS"))errors.push({root,kind:"manual-kinds"});if(!source.includes("::kinds()"))errors.push({root,kind:"derived-kinds"});}console.log(JSON.stringify({roots:roots.length,descriptors,surfaces,vectors,registryCallbacks,ksyTags:Object.keys(ksy.seq[2].type.cases).length,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "roots": 3,
  "descriptors": 9,
  "surfaces": 39,
  "vectors": 9,
  "registryCallbacks": 20,
  "ksyTags": 5,
  "errors": []
}
```

## Mount Existence

```sh
bun -e 'import{readFileSync}from"node:fs";const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const errors=[];for(const root of roots){const s=readFileSync(root+"/🦀️component.rs","utf8");for(const m of s.matchAll(/#\[path = "([^"]+)"\]/g)){const path=root+"/"+m[1];try{readFileSync(path)}catch{errors.push(path)}}}console.log(JSON.stringify({mountedRoots:roots.length,errors}));'
```

Exit 0:

```text
{"mountedRoots":3,"errors":[]}
```

## Bun TypeScript Parser

```sh
bun -e 'import{readdirSync,readFileSync}from"node:fs";const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const walk=p=>readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]);const files=roots.flatMap(walk).filter(p=>p.endsWith(".ts"));const errors=[];const transpiler=new Bun.Transpiler({loader:"ts"});for(const file of files){try{transpiler.transformSync(readFileSync(file,"utf8"));}catch(error){errors.push({file,error:String(error)});}}console.log(JSON.stringify({files:files.length,parsed:files.length-errors.length,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "files": 8,
  "parsed": 8,
  "errors": []
}
```

## Independent TypeScript AST

```sh
bun -e 'import ts from "typescript";import{readFileSync,readdirSync}from"node:fs";const root="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations";const walk=p=>readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]);let checked=0;const errors=[];for(const path of walk(root).filter(p=>p.endsWith(".ts"))){const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);checked++;for(const d of file.parseDiagnostics)errors.push({path,code:d.code,message:ts.flattenDiagnosticMessageText(d.messageText," ")});}console.log(JSON.stringify({parser:"typescript",version:ts.version,checked,errors},null,2));process.exit(errors.length?1:0);'
```

Exit 0:

```text
{
  "parser": "typescript",
  "version": "5.9.3",
  "checked": 8,
  "errors": []
}
```

## Pinned Nightly Parse

```sh
bun -e 'import{readdirSync}from"node:fs";import{spawnSync}from"node:child_process";const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const walk=p=>readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]);const files=[...roots.flatMap(walk).filter(p=>p.endsWith(".rs")),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs"];const failed=[];for(const path of files){const r=spawnSync("rustc",["+nightly","-Z","parse-crate-root-only","--edition=2021","--crate-type=lib",path],{encoding:"utf8"});if(r.status!==0)failed.push({path,code:r.status,stderr:r.stderr});}console.log(JSON.stringify({files:files.length,parsed:files.length-failed.length,failed},null,2));process.exit(failed.length?1:0);'
```

Exit 0:

```text
{
  "files": 30,
  "parsed": 30,
  "failed": []
}
```

## Rustfmt Check

```sh
bun -e 'import{readdirSync}from"node:fs";import{spawnSync}from"node:child_process";const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const walk=p=>readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]);const files=[...roots.flatMap(walk).filter(p=>p.endsWith(".rs")),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs"];const r=spawnSync("rustfmt",["--check","--edition","2021","--config","skip_children=true",...files],{encoding:"utf8"});console.log(JSON.stringify({files:files.length,status:r.status,stderr:r.stderr}));process.exit(r.status??1);'
```

Exit 0:

```text
{"files":30,"status":0,"stderr":""}
```

## Owned Scope Hygiene

```sh
bun -e 'import{readFileSync,readdirSync,statSync}from"node:fs";const scopes=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/component.feature","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/component.feature","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/component.feature","✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️component.graphql","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs"];const walk=p=>statSync(p).isDirectory()?readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]):[p];const files=scopes.flatMap(walk);const rust=files.filter(p=>p.endsWith(".rs"));const source=files.filter(p=>/\.(rs|ts|json|proto|graphql|semio|feature|ksy|abnf|ebnf|g4|spicy)$/.test(p));const debug=source.filter(p=>readFileSync(p,"utf8").includes("[DEBUG]"));const roots=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"];const markers=["NoMutation","SetSnapshot","no-mutation","set-snapshot","Restore(","Undo(","🦠️mutation/","🧾️payload.schema.json"];const forbidden=roots.flatMap(walk).flatMap(path=>markers.filter(m=>readFileSync(path,"utf8").includes(m)).map(marker=>({path,marker})));console.log(JSON.stringify({currentOwnedFiles:files.length,rust:rust.length,debug,forbidden},null,2));process.exit(debug.length||forbidden.length?1:0);'
```

Exit 0:

```text
{
  "currentOwnedFiles": 96,
  "rust": 34,
  "debug": [],
  "forbidden": []
}
```

## Scoped Diff Hygiene

```sh
git diff --check -- '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🔣️.json' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/component.feature' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🔣️.json' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/component.feature' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🔣️.json' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/component.feature' '✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️component.graphql' '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs'
```

Exit 0, no output.

## Exact File Digests

```sh
bun -e 'import{readFileSync,readdirSync,statSync}from"node:fs";import{createHash}from"node:crypto";const scopes=["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/component.feature","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/component.feature","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🔣️.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/component.feature","✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️component.graphql","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🧪️tests/🦀️test.rs"];const walk=p=>statSync(p).isDirectory()?readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(p+"/"+e.name):[p+"/"+e.name]):[p];const current=[...new Set(scopes.flatMap(walk))].sort().map(path=>({path,sha256:createHash("sha256").update(readFileSync(path)).digest("hex")}));console.log(JSON.stringify({current,removed:["✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🟦️component.ts","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🦠️mutation/🔣️component.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🎯️outcome/🔣️component.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/📸️snapshot/⬅️before/🔣️component.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/📸️snapshot/➡️after/🔣️component.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🔺️diff/🔣️component.json","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🟦️component.ts","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🟦️component.ts","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]},null,2));'
```

Output preserved in `🔣️pdf-1-4-direct-files.json`: 96 current explicitly owned paths, 12 removed legacy paths. The glue digest is a point-in-time shared-file digest; this lane owns only its PDF1.4 mutation mount hunk.

## Known Red Checks and Corrections

- The first Ajv aggregate check failed because the harness registered Unicode filesystem paths as URI keys. The corrected check resolves each local `$ref` through the corresponding leaf's canonical `$id`; descriptors and payloads did not change for that harness correction.
- The first Bun TypeScript parse found five direct interfaces missing a closing brace, while the other three files parsed. The coordinator authorized only those five TS closing-brace repairs during the Rust/schema/fixture freeze. Both final parsers pass 8/8.
- The first independent structural query found one manual `KINDS` roster in each root. The constants were deleted; structural catalog assertions derive identities from `SemanticMutation::kinds()`. The coordinator reran all three exact roots and obtained zero violations. See `🧪️pdf-1-4-independent-policy-final.log`.
- Two interim census expressions had syntax mistakes before producing the successful census. They were inspection commands only and never wrote source.
