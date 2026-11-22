import {generateKeyPairSync,createPublicKey} from "node:crypto";
import {sealRequestV1,openRequestV1,exchangePrivateKeyV1,exchangePublicKeyV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🔐️sealed/🟦️.ts";
const pair=generateKeyPairSync("ed25519");
const priv=exchangePrivateKeyV1(Buffer.from(pair.privateKey.export({type:"pkcs8",format:"der"})).toString("base64url")),pubB64=Buffer.from(createPublicKey(pair.privateKey).export({type:"spki",format:"der"})).toString("base64url");
for(const mb of [1,16,64,256]){
 const body=new Uint8Array(mb*1048576).fill(7);
 let t=performance.now();const sealed=sealRequestV1({serverPublic:exchangePublicKeyV1(pubB64),group:"g1",now:Date.now(),request:{method:"PUT",target:"/main/cas/"+"a".repeat(64),headers:{},body}});const s=performance.now()-t;
 t=performance.now();const inc=openRequestV1({serverPrivate:priv,group:"g1",header:sealed.headers["x-semio-sealed"]??Object.values(sealed.headers)[0],body:sealed.body,now:Date.now()});const o=performance.now()-t;
 t=performance.now();inc.seal({status:200,headers:{},body:new Uint8Array(0)});
 console.log(mb,"MB seal(client)",s.toFixed(0),"ms; open(server, sync on loop)",o.toFixed(0),"ms");
}
