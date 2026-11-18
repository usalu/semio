import{issueStoreReceiptV1,settleStoreReceiptV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs";
const[mode,projectRoot,target,issuedJson,ok]=process.argv.slice(2);
const declaration={workspaceRoot:"/Users/ueli/Documents/semio",projectRoot,target,outputs:["{projectRoot}/.🧾️store/"+target+".json"]};
try{if(mode==="issue")process.stdout.write(JSON.stringify(issueStoreReceiptV1(declaration)));
else await settleStoreReceiptV1(declaration,JSON.parse(issuedJson),{succeeded:ok==="1",startedAtMs:Number(process.argv[7])});}catch{if(mode==="issue")process.stdout.write("{}");}
