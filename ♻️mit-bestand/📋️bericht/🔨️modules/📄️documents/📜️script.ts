import{BundleScript,ScriptRouter,receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readFileSync}from"node:fs";
import{basename,join}from"node:path";
import{withPrintExecutionV1}from"../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/🎛️operation/🟦️.ts";
import{publishPrintArtifact,withPublishedPrintDocumentV1}from"../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import{readPrintMaterialFileV1}from"../../../../🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts";
type ReportDocument = { id: string; texPath: string; sources: string[]; actorNetwork: boolean };
const catalog: { version: number; documents: ReportDocument[] } = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const product = "♻️mit-bestand/📋️bericht";

/** 📋️ Resolves a declared report without allowing arbitrary source or output paths. */
export function reportDocument(id: string): ReportDocument {
  const document = catalog.documents.find(document => document.id === id);
  if (catalog.version !== 1 || !document || !/^[a-z]+$/.test(id)) throw new Error(`Unknown report document: ${id}`);
  for (const path of [document.texPath, ...document.sources]) if (!path || path.includes("\\") || path.startsWith("/") || path.split("/").some(part => ["..", ".", ""].includes(part))) throw new Error(`Invalid report source: ${path}`);
  return document;
}


class BuildScript extends BundleScript{async run(args:string[]):Promise<void>{const[id]=args;if(args.length!==1||!id)throw Error("Report compilation requires exactly one document id");const document=reportDocument(id),phaseControl=createIssuedResourceControlV1(this.invocation,"print:report:build",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},execution=>publishPrintArtifact(execution,{version:1,kind:"document",namespace:"report",id:document.id,sourceRoot:product,texPath:document.texPath,sources:document.sources,dark:false}));}}
class CompleteScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("Report collection completion accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.invocation,"print:report:read",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},async execution=>{for(const document of catalog.documents)await withPublishedPrintDocumentV1(execution,"report",document.id,async artifact=>{const bytes=await readPrintMaterialFileV1(join(artifact.path,basename(document.texPath,".tex")+".pdf"),67108864,artifact.control);if(new TextDecoder().decode(bytes.subarray(0,5))!=="%PDF-")throw Error("Published report PDF required");await artifact.recheck();});});console.log(`[print] ${catalog.documents.length} retained report PDFs ready`);}}
const router=new ScriptRouter(import.meta.dir).register("build",BuildScript).register("complete",CompleteScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
