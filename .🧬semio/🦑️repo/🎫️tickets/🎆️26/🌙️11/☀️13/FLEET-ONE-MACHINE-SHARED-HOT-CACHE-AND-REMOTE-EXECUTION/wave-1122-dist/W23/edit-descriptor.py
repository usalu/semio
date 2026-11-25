import os
path="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
src=open(path,encoding="utf8").read()
typeLine='type ArtifactWindowsRelativeFactsV1={identity:ArtifactReadFileIdentityV1;kind:ArtifactRelativeEntryV1["kind"];volumeSerial:string;fileId:string;attributes:number;security:string;streams:string;target?:string};\n'
top=r'''/** 🪪️ The complete NTFS identity of one open Windows handle as the relative layer observes it. */
export type ArtifactWindowsRelativeFactsV1={identity:ArtifactReadFileIdentityV1;kind:ArtifactRelativeEntryV1["kind"];volumeSerial:string;fileId:string;attributes:number;security:string;streams:string;target?:string};
const artifactWindowsFactsChangeV1=exportedSchema.definitions.RelativeWindowsFactsChange.const;
/** 🔎️ One compared fact of a Windows handle that differs between two observations: its field (prefixed by the role of the handle), the expected and the actual value, rendered bounded. */
export type ArtifactWindowsRelativeFactsDifferenceV1=Readonly<{field:string;expected:string;actual:string}>;
/** 🗝️ The declared Windows refusals that name the facts that changed. */
export type ArtifactWindowsRelativeFactsRefusalV1=keyof typeof artifactWindowsFactsChangeV1.refusals;
/** 🟰️ Whether two observations of one Windows handle carry the same complete identity: the seven identity fields, kind, volume, file id, attributes, security, streams and reparse target. */
export function sameArtifactWindowsRelativeFactsV1(a:ArtifactWindowsRelativeFactsV1,b:ArtifactWindowsRelativeFactsV1):boolean{return sameArtifactRelativeIdentityV1(a.identity,b.identity)&&a.kind===b.kind&&a.volumeSerial===b.volumeSerial&&a.fileId===b.fileId&&a.attributes===b.attributes&&a.security===b.security&&a.streams===b.streams&&a.target===b.target;}
const artifactWindowsFactsTextV1=(value:string,points:number,tail:boolean):string=>{const clean=[...value.replace(/[\u0000-\u001f\u007f]/gu,"?")];return clean.length<=points?clean.join(""):tail?"…"+clean.slice(clean.length-points+1).join(""):clean.slice(0,points-1).join("")+"…";};
/** 🔬️ Names every fact `sameArtifactWindowsRelativeFactsV1` compares that differs, in schema order, with both values; empty exactly when the two observations are the same. */
export function artifactWindowsRelativeFactsDifferencesV1(expected:ArtifactWindowsRelativeFactsV1,actual:ArtifactWindowsRelativeFactsV1,role?:string):readonly ArtifactWindowsRelativeFactsDifferenceV1[]{
 const p=artifactWindowsFactsChangeV1,read=(facts:ArtifactWindowsRelativeFactsV1,field:string):unknown=>((p.identityFields.includes(field)?facts.identity:facts) as unknown as Record<string,unknown>)[field],show=(field:string,value:unknown)=>value===undefined?p.absent:p.hexFields.includes(field)?"0x"+(Number(value)>>>0).toString(16):artifactWindowsFactsTextV1(String(value),p.maximumValuePoints,false),rows:ArtifactWindowsRelativeFactsDifferenceV1[]=[];
 for(const field of[...p.identityFields,...p.fields]){const before=read(expected,field),after=read(actual,field);if(before!==after)rows.push(Object.freeze({field:role===undefined?field:role+"."+field,expected:show(field,before),actual:show(field,after)}));}
 return Object.freeze(rows);
}
/** 📝️ Renders a Windows facts refusal in English and German from the schema texts: the refusal, the directory (its tail kept when long) and every differing fact. */
export function artifactWindowsRelativeFactsChangeTextsV1(refusal:ArtifactWindowsRelativeFactsRefusalV1,directory:string,differences:readonly ArtifactWindowsRelativeFactsDifferenceV1[]):Readonly<{en:string;de:string}>{
 const p=artifactWindowsFactsChangeV1,fill=(template:string,values:Readonly<Record<string,string>>)=>template.replace(/\{(\w+)\}/gu,(_,key:string)=>values[key]??""),text=(language:"en"|"de")=>fill(p.texts[language].message,{refusal:p.refusals[refusal][language],directory:artifactWindowsFactsTextV1(directory,p.maximumDirectoryPoints,true),differences:differences.map(row=>fill(p.texts[language].difference,row)).join(p.separator)});
 return Object.freeze({en:text("en"),de:text("de")});
}
/** 🧾️ A Windows relative handle whose compared facts moved between two observations: the declared refusal, the directory and exactly which facts differ, so a fail-closed refusal says what changed. */
export class ArtifactWindowsRelativeFactsChangedErrorV1 extends Error{
 readonly refusal:string;readonly directory:string;readonly differences:readonly ArtifactWindowsRelativeFactsDifferenceV1[];readonly texts:Readonly<{en:string;de:string}>;
 constructor(refusal:ArtifactWindowsRelativeFactsRefusalV1,directory:string,differences:readonly ArtifactWindowsRelativeFactsDifferenceV1[]){const texts=artifactWindowsRelativeFactsChangeTextsV1(refusal,directory,differences);super(texts.en);this.name="WindowsRelativeFactsChanged";this.refusal=artifactWindowsFactsChangeV1.refusals[refusal].en;this.directory=directory;this.differences=differences;this.texts=texts;}
}
/** 🏷️ The refusal sentence a retry policy classifies: the declared refusal of a typed Windows facts change (its directory and values never decide a retry), the message of anything else. */
export function artifactRelativeRefusalV1(error:unknown):string{return error instanceof ArtifactWindowsRelativeFactsChangedErrorV1?error.refusal:String((error as Error)?.message??error);}
'''
reps=[
(typeLine,top),
('same=(a:ArtifactWindowsRelativeFactsV1,b:ArtifactWindowsRelativeFactsV1)=>sameArtifactRelativeIdentityV1(a.identity,b.identity)&&a.kind===b.kind&&a.volumeSerial===b.volumeSerial&&a.fileId===b.fileId&&a.attributes===b.attributes&&a.security===b.security&&a.streams===b.streams&&a.target===b.target,anchor=',
 'same=sameArtifactWindowsRelativeFactsV1,refuseFacts=(refusal:ArtifactWindowsRelativeFactsRefusalV1,directory:string,expected:ArtifactWindowsRelativeFactsV1,actual:ArtifactWindowsRelativeFactsV1)=>new ArtifactWindowsRelativeFactsChangedErrorV1(refusal,directory,artifactWindowsRelativeFactsDifferencesV1(expected,actual)),anchor='),
('const directory=<R>(owned:ArtifactWindowsRelativeHandleV1,parent:ArtifactRelativeFrameV1|undefined,run:(value:ArtifactRelativeDirectoryV1)=>Promise<R>,preservationPathUnits?:number):Promise<R>=>{',
 'const directory=<R>(owned:ArtifactWindowsRelativeHandleV1,parent:ArtifactRelativeFrameV1|undefined,framePath:()=>string,run:(value:ArtifactRelativeDirectoryV1)=>Promise<R>,preservationPathUnits?:number):Promise<R>=>{'),
('checkpoint();if(!same(expected,facts(owned)))throw Error("Original Windows relative directory facts changed");}',
 'checkpoint();const current=facts(owned);if(!same(expected,current))throw refuseFacts("directory",framePath(),expected,current);}'),
('if(!same(before,facts(child)))throw Error("Windows relative entry changed while observed");',
 'const again=facts(child);if(!same(before,again))throw refuseFacts("entry",artifactRelativeWindowsPathV1.join(framePath(),name),before,again);'),
('if(!same(expected,before))throw Error("Exact Windows enumeration parent required");',
 'if(!same(expected,before))throw refuseFacts("enumerationParent",framePath(),expected,before);'),
('}}if(!same(before,facts(cursor)))throw Error("Windows relative enumeration changed");internal();return finish(seen);',
 '}}const finalFacts=facts(cursor);if(!same(before,finalFacts))throw refuseFacts("enumeration",framePath(),before,finalFacts);internal();return finish(seen);'),
('return trackArtifactWindowsRelativeChildV1(frame,async()=>withArtifactRelativePathOwnersV1(owned,rows,(parent,step)=>{internal();if(!same(step.proof.parent,facts(parent)))throw Error("Current Windows relative path parent changed");return openChild(parent,step.name,"directory");},(parent,child,step)=>{internal();if(!same(step.proof.parent,facts(parent))||!same(step.proof.facts,facts(child)))throw Error("Current exact Windows relative path descriptor changed");},child=>child.close(),()=>state.uncertain,child=>directory(child,frame,callback,',
 'const pathAt=(count:number)=>artifactRelativeWindowsPathV1.join(framePath(),...rows.slice(0,count).map(row=>row.name));return trackArtifactWindowsRelativeChildV1(frame,async()=>withArtifactRelativePathOwnersV1(owned,rows,(parent,step)=>{internal();const current=facts(parent);if(!same(step.proof.parent,current))throw refuseFacts("pathParent",pathAt(rows.indexOf(step)),step.proof.parent,current);return openChild(parent,step.name,"directory");},(parent,child,step)=>{internal();const up=facts(parent);if(!same(step.proof.parent,up))throw new ArtifactWindowsRelativeFactsChangedErrorV1("pathDescriptor",pathAt(rows.indexOf(step)+1),artifactWindowsRelativeFactsDifferencesV1(step.proof.parent,up,"parent"));const down=facts(child);if(!same(step.proof.facts,down))throw new ArtifactWindowsRelativeFactsChangedErrorV1("pathDescriptor",pathAt(rows.indexOf(step)+1),artifactWindowsRelativeFactsDifferencesV1(step.proof.facts,down,"entry"));},child=>child.close(),()=>state.uncertain,child=>directory(child,frame,()=>pathAt(rows.length),callback,'),
('if(!same(proof.facts,facts(child)))throw Error("Exact Windows child directory required");return directory(child,frame,callback);',
 'if(!same(proof.facts,facts(child)))throw Error("Exact Windows child directory required");return directory(child,frame,()=>artifactRelativeWindowsPathV1.join(framePath(),name),callback);'),
('output=await directory(child,frame,async created=>{',
 'output=await directory(child,frame,()=>artifactRelativeWindowsPathV1.join(framePath(),name),async created=>{'),
('if(index===parts.length)return directory(owned,undefined,run,root.replace(',
 'if(index===parts.length)return directory(owned,undefined,()=>root,run,root.replace('),
]
for old,new in reps:
    n=src.count(old)
    assert n==1,(n,old[:80])
    src=src.replace(old,new)
tmp=path+".w23.tmp"
open(tmp,"w",encoding="utf8").write(src);os.replace(tmp,path)
print("ok")
