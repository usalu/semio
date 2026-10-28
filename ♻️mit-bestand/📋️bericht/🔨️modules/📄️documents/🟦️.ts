import { BundleScript, createIssuedResourceControlV1 } from "@semio-tech/process/routing";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{withPrintExecutionV1}from"@semio-tech/print/artifacts/operation";
import{publishPrintArtifact,withPublishedPrintDocumentV1}from"@semio-tech/print/modules/tectonic-template-compilation";
import{readPrintMaterialFileV1}from"@semio-tech/print/artifacts/material/producer";

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


export class BuildScript extends BundleScript{async run(args:string[]):Promise<void>{const[id]=args;if(args.length!==1||!id)throw new Error("Report compilation requires exactly one document id");const document=reportDocument(id),phaseControl=createIssuedResourceControlV1(this.invocation,"print:report:build",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},execution=>publishPrintArtifact(execution,{version:1,kind:"document",namespace:"report",id:document.id,producer:`@semio-tech/mit-bestand-bericht:build-${document.id}`,sourceRoot:product,texPath:document.texPath,sources:document.sources,dark:false}));}}
export class CompleteScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Report collection completion accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.invocation,"print:report:read",0);await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"document"},async execution=>{for(const document of catalog.documents)await withPublishedPrintDocumentV1(execution,"report",document.id,`@semio-tech/mit-bestand-bericht:build-${document.id}`,async artifact=>{const bytes=await readPrintMaterialFileV1(join(artifact.path,document.id+".pdf"),67108864,artifact.control);if(new TextDecoder().decode(bytes.subarray(0,5))!=="%PDF-")throw new Error("Published report PDF required");await artifact.recheck();});});console.log(`[print] ${catalog.documents.length} retained report PDFs ready`);}}
