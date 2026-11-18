import{readFileSync}from"node:fs";
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/v2/";
export const PN=L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",GM=L+"👁️observation/🚧️read-parent/🟨️.mjs",GT=L+"👁️observation/🚧️read-parent/🧪️tests/🟦️.ts",PT=L+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🚧️read-parent/🟦️.ts";
const FS=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json",FT=L+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0)throw Error("anchor missing: "+from.slice(0,80));if(text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,80));return text.replace(from,()=>to);};
export const all=[
 {file:GM,apply:()=>readFileSync(SC+"guard.mjs","utf8")},
 {file:GT,apply:()=>readFileSync(SC+"guard-test.ts","utf8")},
 {file:PT,apply:()=>readFileSync(SC+"pn-test.ts","utf8")},
 {file:PN,apply:edit(`import { assertNotReadParentV1, markReadParentV1 } from`,`import { assertNotReadParentV1, withReadParentScopeV1 } from`)},
 {file:PN,apply:edit(`export async function emojiProjectJsonNodes(configFiles, options, context) {`,`export function emojiProjectJsonNodes(configFiles, options, context) { return withReadParentScopeV1(readScope=>emojiProjectJsonNodesBodyV1(configFiles, options, context, readScope)); }

/** 🛡️ The read of {@link emojiProjectJsonNodes} inside the scope of its parent: the computations of a farm child are refused only in what this read started, and only while its farm exists. */
async function emojiProjectJsonNodesBodyV1(configFiles, options, context, readScope) {`)},
 {file:PN,apply:edit(`markReadParentV1(()=>farm.exists())`,`readScope.arm(()=>farm.exists())`)},
 {file:FS,apply:edit("holds them for the workers for at most {seconds} s","holds them for the workers; a project that waits longer than {seconds} s says why")},
 {file:FS,apply:edit("hält sie höchstens {seconds} s für die Worker zurück","hält sie für die Worker zurück; ein Projekt, das länger als {seconds} s wartet, nennt den Grund")},
 {file:FT,apply:edit("holds them for the workers for at most 190 s","holds them for the workers; a project that waits longer than 190 s says why")},
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
