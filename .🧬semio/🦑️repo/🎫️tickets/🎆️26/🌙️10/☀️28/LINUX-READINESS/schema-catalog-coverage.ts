import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {schemaScopeModuleOwnerPath,schemaScopeOwnerLevel,schemaScopeCollectionPath,schemaScopeIdFromDocumentId} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import {loadCatalogTaxonomy,canonicalPrimaryFilenameForKind} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const output=join(import.meta.dir,"🗑️generated"),taxonomy=loadCatalogTaxonomy(),catalog=JSON.parse(readFileSync(taxonomy.schemaExportResolution.catalogPath,"utf8")),diagnostics=JSON.parse(readFileSync(join(output,"schema-diagnostics-catalog-current.json"),"utf8")).diagnostics;
const format=taxonomy.schemaFacetKinds![taxonomy.schemaDefaultFacetKind]!.normativeFormat,leaf=canonicalPrimaryFilenameForKind(taxonomy.schemaFormats[format]!.fileKindId,taxonomy),counts:Record<string,number>={},rows=[];
for(const row of diagnostics.filter((row:{code:string})=>row.code==="schema-catalog-stale")){
 const owner=schemaScopeModuleOwnerPath(row.path,taxonomy),level=schemaScopeOwnerLevel(owner,taxonomy);let id:unknown;
 try{id=JSON.parse(readFileSync(join(row.path,leaf),"utf8")).$id;}catch{}
 const scope=typeof id==="string"?schemaScopeIdFromDocumentId(id,taxonomy):null,indexed=scope?catalog.scopes[scope]:undefined;
 const kind=schemaScopeCollectionPath(row.path,taxonomy)?"collection":!level?"ineligible-owner":typeof id!=="string"?"missing-root-id":!scope?"invalid-root-id":indexed&&indexed.path!==row.path?"duplicate-scope-at-other-owner":!indexed?"scope-not-indexed":"indexed";
 counts[kind]=(counts[kind]??0)+1;rows.push({path:row.path,owner,level,id:id??null,scope,indexedAt:indexed?.path??null,kind});
}
writeFileSync(join(output,"schema-catalog-coverage.json"),JSON.stringify({counts,rows},null,2)+"\n");console.log(JSON.stringify(counts));

