import{readFileSync}from"node:fs";
import protocol from"/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/wb/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🔣️.json" with {type:"json"};
import{compileLawV1,scanIsTestPathV1,scanLanguageV1,scanTestsV1}from"/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/wb/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
const law=compileLawV1(protocol as any),files=process.argv.slice(2);
const listing=files.filter(path=>scanLanguageV1(law,path)!==undefined&&scanIsTestPathV1(law,path));
const result=scanTestsV1(law,listing,path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}});
console.log("scanned",result.scanned,"of",files.length);
for(const f of result.findings)console.log(f.severity,f.rule,f.file.slice(-70),f.line??"");
