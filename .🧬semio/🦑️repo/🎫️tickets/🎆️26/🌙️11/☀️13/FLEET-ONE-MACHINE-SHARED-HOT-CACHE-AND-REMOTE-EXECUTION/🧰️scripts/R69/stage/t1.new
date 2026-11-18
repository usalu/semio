import{test,expect}from"bun:test";
import{javascriptDependencyUnreadyRetentionIntentV1,readJavascriptDependencySelectedRetirementCommandV1}from"../../🟦️.ts";
import{describeRepositoryDependencyCandidateV1}from"../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts";

const sha="a".repeat(64),intent={version:1,purpose:"retain-current-unready-dependency-generation",scope:".",generation:sha,directory:"/store/j/"+sha.slice(0,20),rootStamp:"1:2",parentStamp:"3:4",rosterSha256:sha,sourceInputsSha256:sha,sourceEndpointsSha256:sha,recipeSha256:sha,materializationSha256:sha,fundingSha256:sha,readyAbsent:true,directSourceReferences:0,inactivityProved:false};
const refusal=(call:()=>unknown):string=>{try{call();}catch(error){return String((error as Error).message);}return "";};

test("a closed selected DATA row that misses or adds a field names every missing and unexpected field",()=>{
 const{fundingSha256,...short}=intent;
 expect(fundingSha256).toBe(sha);
 const message=refusal(()=>javascriptDependencyUnreadyRetentionIntentV1({...short,platform:"linux"}));
 expect(message).toContain("Exact selected dependency fields required");
 expect(message).toContain('missing ["fundingSha256"]');
 expect(message).toContain('unexpected ["platform"]');
 expect(refusal(()=>javascriptDependencyUnreadyRetentionIntentV1(intent))).not.toContain("Exact selected dependency fields");
});

test("an observe command without its expected snapshot names the missing field",()=>{
 const message=refusal(()=>readJavascriptDependencySelectedRetirementCommandV1({version:1,operation:"observe",selection:{kind:"empty-generation",prefix:sha.slice(0,20),rootStamp:"1:2",currentBindingsSha256:sha},parent:{},artifact:{}}));
 expect(message).toContain('missing ["expected"]');
 expect(message).toContain("unexpected []");
});

test("a row with more than sixteen fields is refused with its count",()=>{
 const wide=Object.fromEntries(Array.from({length:17},(_,index)=>["field"+index,index]));
 expect(refusal(()=>javascriptDependencyUnreadyRetentionIntentV1(wide))).toContain("found 17 of 16 allowed");
});

test("a plan read names the retained generation prefix and the receipt file that was refused",()=>{
 const message=refusal(()=>describeRepositoryDependencyCandidateV1(".p1234abcd","i",()=>javascriptDependencyUnreadyRetentionIntentV1({...intent,version:2,extra:1})));
 expect(message.startsWith("Retained dependency generation .p1234abcd file i: Exact selected dependency fields required")).toBe(true);
 expect(message).toContain('unexpected ["extra"]');
});
