/** 🧪 Verifies journal-before-attach ordering and rollback in the staged catalog opening gate. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import { decodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { keepArtifactOpeningV1, keepFirstArtifactOpeningV1 } from "./🗑️generated/browser-first-archive-publication/opening.ts";
import laws from "./browser-first-archive-opening.json";
import schema from "./browser-first-archive-opening.schema.json";

const validator = new Ajv({strict:true}).compile(schema);
assert(validator(laws), JSON.stringify(validator.errors));
const documentId = "local-pdf:signed-first";
const directory = "/catalog-test";
const target = directory + "/os/local-documents/" + createHash("sha256").update(documentId).digest("hex");
const archive = {parent_pack: [65,66], parent_spr: [67,68], members: []};
const admissionCheck = (admission:{document:{documentId:string,target:string},binding:{kind:string,path:string}}) => {
  assert.equal(admission.document.documentId, documentId);
  assert.equal(admission.document.target, target);
  assert.deepEqual(admission.binding, {kind:"folder",dataClass:"persistedLocalOnly",path:target});
};
for (const test of laws) {
  const observed: string[] = [], progress: number[] = [];
  const cancellation = new AbortController();
  const receipt = {runtimeKey:"pdf",clientInstanceId:"exact-instance"};
  const port = {
    name:"two-pages.pdf", schema:"stdio.pdf.1.7", surface:{pluginId:"stdio-pdf",appId:"pdf"}, nowMs:100,
    dataDir:test.fault === "folder" ? undefined : directory,
    signal:cancellation.signal, current:()=>true,
    progress:(completed:number,total:number)=>{assert.equal(total,5);progress.push(completed);},
    identity:async()=>{observed.push("identity");return {parent_document_id:test.fault === "identity" ? null : documentId};},
    attach:async(admission:Parameters<typeof admissionCheck>[0])=>{
      observed.push("attach");admissionCheck(admission);return test.fault === "attach" ? null : receipt;
    },
    record:async()=>{observed.push("record");return test.fault !== "catalog";},
    publish:()=>{observed.push("publish");return test.fault !== "panel";},
    close:async(closed:typeof receipt)=>{observed.push("close");assert.equal(closed, receipt);},
    unlist:async(document:{documentId:string})=>{observed.push("unlist");assert.equal(document.documentId,documentId);},
    report:(error:unknown)=>{throw error;},
  };
  const opening = test.mode === "ordinary"
    ? keepArtifactOpeningV1({...port, archive:async()=>{observed.push("archive");return archive;}, async write(admission) {
      assert.equal(this.signal,cancellation.signal);
      observed.push("persist");admissionCheck(admission);
      assert.deepEqual([...admission.archive],[1,2,65,66,2,67,68,0]);
      assert.deepEqual(decodeDocumentArchiveBytes(admission.archive), archive);
      return true;
    }})
    : keepFirstArtifactOpeningV1({...port, async persist(destination) {
      assert.equal(this.signal,cancellation.signal);
      observed.push("persist");admissionCheck(destination);
      assert(!("archive" in destination));
      if (test.fault === "cancel") cancellation.abort(new Error("cancelled exactly after sealing"));
      return test.fault !== "persist";
    }});
  const expectedFailure = ({
    identity:"natural-file.document-identity-unavailable", folder:"natural-file.no-data-folder", persist:"natural-file.archive-not-persisted",
    cancel:"cancelled exactly after sealing", attach:"natural-file.attachment-refused", catalog:"natural-file.catalog-not-persisted",
    panel:"natural-file.panel-unavailable",
  } as Record<string,string>)[test.fault];
  if (expectedFailure) await assert.rejects(opening,{message:expectedFailure});
  else assert.equal(await opening,true);
  assert.deepEqual(observed,test.expected,test.name);
  assert.equal(progress.at(-1) === 5,!expectedFailure,test.name);
}
console.log("[DEBUG] Nine catalog opening orders verified with AJV, independent SHA-256, and the exact archive codec.");
