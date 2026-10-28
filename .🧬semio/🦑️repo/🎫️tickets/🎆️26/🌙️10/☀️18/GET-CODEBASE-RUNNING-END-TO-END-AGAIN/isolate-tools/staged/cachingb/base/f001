import{test,expect}from"bun:test";
import Ajv from"ajv";
import ts from"typescript";
import{Database}from"bun:sqlite";
import{createHash}from"node:crypto";
import{readFile,writeFile}from"node:fs/promises";
import{join,resolve,dirname}from"node:path";
import{readRustUnitCodeReceiptV1,withRepositoryRustUnitCodeReaderV1}from"./../../📖️read/🟦️.ts";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1}from"@semio-tech/nx-bootstrap-tools/authorizers";
import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{withIssuedResourceControlV1}from"@semio-tech/process/routing";
import fixture from"./../🔣️.json";
import receiptSchema from"./../../🧬️schema/📜️receipt.json";
import corpus from"./🔣️.json";
import schema from"./🧬️schema.json";
const hash=(value:string)=>createHash("sha256").update(value).digest("hex");
test("sealed Rust code reader preserves neutral receipt identity and refusals",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const sourceSha256=hash(JSON.stringify(fixture.source)+"\n"),receipt={version:1,source:fixture.source,sourceSha256,sourceDigest:hash(JSON.stringify({source:sourceSha256,compiler:fixture.compiler})),compiler:fixture.compiler,programDigest:"1".repeat(64),programBytes:1},validate=new Ajv({strict:true}).compile(receiptSchema),db=new Database(":memory:");
 try{expect(sourceSha256).toBe(corpus.sourceSha256);expect(receipt.sourceDigest).toBe(corpus.sourceDigest);expect(validate(receipt)).toBe(true);expect(JSON.stringify(readRustUnitCodeReceiptV1(receipt))).toBe(JSON.stringify(receipt));expect((db.query("select json_array_length(?,'$.source.sources') n").get(JSON.stringify(receipt))as{n:number}).n).toBe(1);
 for(const field of fixture.invalidations){const changed=structuredClone(receipt)as any;if(field==="sourceDigest")changed.sourceDigest="2".repeat(64);else if(field==="sourceSha256")changed.sourceSha256="3".repeat(64);else if(field==="parent-entry")changed.source.entry="../compiler.ts";else if(field==="duplicate-source")changed.source.sources.push(...changed.source.sources);else changed.grant=true;expect(()=>readRustUnitCodeReceiptV1(changed),field).toThrow();}
 const missing=structuredClone(receipt)as any;delete missing.source.resolutions;expect(validate(missing)).toBe(false);expect(()=>readRustUnitCodeReceiptV1(missing)).toThrow();
 }finally{db.close();}
});
registerBuildTestLawV1("sealed Rust code reader ownership excludes publication and compiler capture","sdk-fence",original=>withIssuedResourceControlV1(original,"rust-code:reader-source",0,async control=>{
 const root=process.cwd(),text=await readFile(join(root,corpus.consumer),"utf8"),tree=ts.createSourceFile("history.ts",text,ts.ScriptTarget.Latest,true),owner=tree.statements.find(node=>ts.isImportDeclaration(node)&&node.importClause?.namedBindings?.getText(tree).includes("withRepositoryRustUnitCodeReaderV1"));
 if(!owner||!ts.isImportDeclaration(owner)||!ts.isStringLiteral(owner.moduleSpecifier))throw Error("Actual code-reader consumer required");
 expect(resolve(dirname(join(root,corpus.consumer)),owner.moduleSpecifier.text)).toBe(join(root,corpus.entry));
 let callbacks=0;await expect(withRepositoryRustUnitCodeReaderV1(root,original,control,{},async()=>{callbacks++;})).rejects.toThrow("Closed Rust compiler code receipt required");expect(callbacks).toBe(0);
 const captured=await captureNxAuthorizerSourcesV1(root,corpus.entry,control.signal),contents=nxAuthorizerCapturedBytesV1(captured),paths=captured.manifest.sources.map(row=>row.path),bytes=[...contents.values()].reduce((n,row)=>n+row.byteLength,0),db=new Database(":memory:");
 try{expect(paths.length).toBeLessThanOrEqual(corpus.maximumFiles);expect(bytes).toBeLessThanOrEqual(corpus.maximumBytes);for(const forbidden of corpus.forbidden){expect(paths).not.toContain(forbidden);expect((db.query("select count(*) n from json_each(?) where value=?").get(JSON.stringify(paths),forbidden)as{n:number}).n).toBe(0);}
 const mode=Object.getOwnPropertyDescriptor(original.capabilities,"buildMode")?.value;if(mode!=="prototype"&&mode!=="release")throw Error("Original build mode required");const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Original ticket artifact sink required");await writeFile(join(sink,"rust-code-reader-source-"+mode+".json"),JSON.stringify({version:1,nativeBuild:false,files:paths.length,bytes,manifest:captured.manifest,digest:captured.digest},null,2)+"\n");
 }finally{db.close();}
}));
