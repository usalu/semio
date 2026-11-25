/** 🔭️ W15: which revisions of the Ready workspace scope's selected head the fleet still holds (read-only lookups of the action results), with document sizes. */
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",X=L+"🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(X+"🟦️.ts"),at=await import(X+"🔌️attempt/🟦️.ts"),pb=await import(L+"🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const scope="2f245bafbb6a1b8305c18d9386d61ec1853ca12938201ee6677109a1a44acc64",client=at.selectedFleetClientV1(root)!;
const now=ex.SELECTED_EXCHANGE_POLICY_V1.revision;console.log("policy revision",now);
for(let revision=now;revision>=Math.max(1,now-6);revision--){const answer=await client.getResult(ex.selectedHeadKeyV1({scope,platform:"darwin-arm64",revision}));if(typeof answer==="string"){console.log(revision,answer);continue;}const result=pb.decodeProtobufV1(pb.REAPI_V1.actionResult,answer),head=JSON.parse(Buffer.from(result.stdout_raw).toString());console.log(revision,new Date(head.createdMs).toISOString(),Object.keys(head.owners).length,JSON.stringify(head.documents.map((d:any)=>[d.name,Math.round(d.size/1048576)])));}
