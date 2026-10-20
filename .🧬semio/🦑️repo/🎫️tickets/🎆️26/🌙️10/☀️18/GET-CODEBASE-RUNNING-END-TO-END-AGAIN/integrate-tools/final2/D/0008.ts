import {lstatSync} from "node:fs";

/** 🔖️ A journal revision: every stat field an append, rewrite or replacement must change. */
export type JournalRevisionV1=string;
/** 🧾️ Remembers the one journal state read under a revision. */
export type JournalReceiptV1<S>=Readonly<{revision():JournalRevisionV1|undefined;recall(revision:JournalRevisionV1):S|undefined;remember(revision:JournalRevisionV1,state:S):void}>;

/** 🔖️ Observes the revision of a plain single-link journal file, or nothing when it is absent, linked or not a regular file. */
export function observeJournalRevisionV1(path:string):JournalRevisionV1|undefined{
 try{const row=lstatSync(path,{bigint:true});return row.isFile()&&row.nlink===1n?[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":"):undefined;}catch{return undefined;}
}

/** 🧾️ A one-slot receipt keyed by journal revision: a state read while the revision was stable before and after the read stays valid for exactly that revision. */
export function createJournalReceiptV1<S>(path:string):JournalReceiptV1<S>{
 let held:Readonly<{revision:JournalRevisionV1;state:S}>|undefined;
 return Object.freeze({revision:()=>observeJournalRevisionV1(path),recall:(revision:JournalRevisionV1)=>held?.revision===revision?held.state:undefined,remember:(revision:JournalRevisionV1,state:S)=>{held=Object.freeze({revision,state});}});
}
