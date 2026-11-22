/** 📏 Verifies the Browser catalog destination and longest physical native journal sibling fit together. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { localArtifactDestinationV1 } from "./🗑️generated/browser-first-archive-publication/artifact.ts";

const documentId = "pdf:fixture";
const identity = {documentId,schema:"stdio.pdf.1.7",name:"two-pages.pdf",surface:{pluginId:"stdio-pdf",appId:"pdf"}};
const root = "/Users/ueli/Documents/semio";
const tail = "/.semio/.genesis-4294967295-ffffffffffffffff.stage";
const end = "/os/local-documents/" + createHash("sha256").update(documentId).digest("hex");
const max = 256 - tail.length - end.length;
for (const path of [root.padEnd(max,"x"),"C:\\Users\\native".padEnd(max,"y")]) {
  const admitted = await localArtifactDestinationV1(identity,path,0);
  assert(!("refusal" in admitted));
  assert.equal(admitted.binding.path,path+end);
  assert.equal((admitted.binding.path+tail).length,256);
  assert.deepEqual(await localArtifactDestinationV1(identity,path+"x",0),{refusal:"invalid-request"});
}
console.log("[DEBUG] Actual SHA-256 folder and longest physical journal path admit 256 UTF-16 units and refuse 257.");
