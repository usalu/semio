import {createHash,createPrivateKey,createPublicKey} from "node:crypto";
import {writeFileSync} from "node:fs";
import {exchangePublicKeyV1,openRequestV1,sealRequestV1,exchangePrivateKeyV1} from "SEALED_MODULE";
const seed=(label:string)=>createHash("sha256").update(label).digest();
const edPrivate=(s:Buffer)=>Buffer.concat([Buffer.from("302e020100300506032b657004220420","hex"),s]).toString("base64url");
const vectors=[] as unknown[];
const cases=[
 {name:"a cache read of a blob by a peer group",group:"g1",label:"one",request:{head:{method:"GET",target:"/semio/cas/"+"ab".repeat(32),headers:{authorization:"Semio-Fleet o=g2-server r=cache-read a=cache t=1794700000000 n=AAAAAAAAAAAAAAAAAAAAAA s="+"A".repeat(86),"x-semio-hop":"1"}},bodyHex:""},reply:{head:{status:200,headers:{"content-type":"application/octet-stream"}},bodyHex:Buffer.from("blob of project 🦀️alpha").toString("hex")}},
 {name:"a claim post with a body and a miss reply",group:"g2",label:"two",request:{head:{method:"POST",target:"/semio/claim/ac/"+"cd".repeat(32),headers:{"content-type":"application/json"}},bodyHex:Buffer.from('{"owner":"g1-m01","ttlMilliseconds":1000}').toString("hex")},reply:{head:{status:404,headers:{"x-semio-time":"1794700000123"}},bodyHex:""}},
];
cases.forEach((c,index)=>{
 const origin=seed("origin-"+c.label),asker=seed("asker-"+c.label),server=seed("server-"+c.label),nonce=seed("nonce-"+c.label).subarray(0,16);
 const edPub=createPublicKey(createPrivateKey({key:Buffer.from(edPrivate(origin),"base64url"),format:"der",type:"pkcs8"})).export({type:"spki",format:"der"}).toString("base64url");
 const serverPublic=exchangePublicKeyV1(edPub),timestamp=1794700000000+index*1000;
 const request={method:c.request.head.method,target:c.request.head.target,headers:c.request.head.headers,body:Buffer.from(c.request.bodyHex,"hex")};
 const sealed=sealRequestV1({serverPublic,group:c.group,request,now:timestamp,seeds:{ephemeral:asker,nonce}});
 const incoming=openRequestV1({serverPrivate:exchangePrivateKeyV1(edPrivate(origin)),group:c.group,header:sealed.headers["x-semio-sealed"]!,body:sealed.body,now:timestamp});
 const reply=incoming.seal({status:c.reply.head.status,headers:c.reply.head.headers,body:Buffer.from(c.reply.bodyHex,"hex")},server);
 const opened=sealed.open(reply.header,reply.body);
 if(opened.status!==c.reply.head.status)throw Error("round trip failed");
 vectors.push({name:c.name,group:c.group,originSeed:origin.toString("hex"),askerSeed:asker.toString("hex"),serverSeed:server.toString("hex"),timestamp,nonce:nonce.toString("base64url"),request:c.request,reply:c.reply,serverX25519:serverPublic.toString("hex"),requestKey:"",requestIv:"",replyKey:"",replyIv:"",requestHeader:sealed.headers["x-semio-sealed"],replyHeader:reply.header,requestCiphertext:Buffer.from(sealed.body).toString("hex"),replyCiphertext:Buffer.from(reply.body).toString("hex")});
});
writeFileSync(process.argv[2]!,JSON.stringify({version:1,vectors},null,1)+"\n");
