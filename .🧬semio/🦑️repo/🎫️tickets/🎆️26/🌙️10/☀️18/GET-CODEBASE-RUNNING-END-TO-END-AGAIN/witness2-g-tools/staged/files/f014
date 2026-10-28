import {join} from "node:path";
import {symlinkedDirectoriesV1} from "../🔭️world/🟦️.ts";
import {canonicalRootV1} from "../../📨️protocol/📍️endpoint/🟨️.mjs";
import {VOUCH_POLICY_V1,openVouchV1,type VouchV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import type {ChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../🟦️.ts";

/** 🪪️ Opens the vouch of one verification of a workspace against its change journal: the record lives in the private runtime directory of the journal, so it dies with the journal's epoch and never reaches the checkout. */
export function openJournalVouchV1(root:string,consumer:string,key:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal,ports:Readonly<{client?:ChangeJournalClientV1;directory?:string}>={}):Promise<VouchV1>{
 const canonical=canonicalRootV1(root);
 return openVouchV1({client:ports.client??connectChangeJournalV1(canonical,environment),directory:ports.directory??join(changeJournalDirectoryV1(canonical,environment),VOUCH_POLICY_V1.directory),consumer,key,discoverLinked:async paths=>symlinkedDirectoriesV1(canonical,[...paths].map(path=>join(canonical,path))),...(signal?{signal}:{})});
}
