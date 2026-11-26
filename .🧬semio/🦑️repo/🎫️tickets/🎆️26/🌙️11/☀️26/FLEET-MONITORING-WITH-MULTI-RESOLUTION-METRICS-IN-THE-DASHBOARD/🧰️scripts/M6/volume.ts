import{rmSync,mkdirSync}from"node:fs";import{join}from"node:path";
const F="/Users/ueli/Documents/semio/🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",H=F+"/🔨️modules/🏃️process/🏗️build";
const m=await import(H+"/📈️metrics/🟦️.ts"),col=await import(H+"/📈️metrics/🛰️collector/🟦️.ts"),pub=await import(H+"/📈️metrics/🛰️collector/📤️publication/🟦️.ts"),prod=await import(B+"/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const scratch=process.argv[2]!,hours=Number(process.argv[3]??"3");rmSync(scratch,{recursive:true,force:true});mkdirSync(join(scratch,"store"),{recursive:true});
const sources=prod.fleetMetricsSourcesV1({root:"/Users/ueli/Documents/semio",environment:{...process.env,SEMIO_FLEET_STORE_ROOT:join(scratch,"store")},origin:"g1-m01"});
const rings=join(scratch,"rings"),to=join(scratch,"pub");
const collector=col.createFleetMetricsCollectorV1({directory:rings,sources});const publisher=pub.createMetricsPublisherV1({from:rings,to});
let at=Date.now()-hours*3600_000;at-=at%3600_000;
const perHour:number[]=[];let hourBytes=0,total=0;
for(let s=1;s<=hours*3600;s++){collector.step(at);at+=1000;if(s%5===0){const o=publisher.publish(at);hourBytes+=o.bytes;total+=o.bytes;}if(s%3600===0){perHour.push(hourBytes);hourBytes=0;}}
collector.close();
console.log(JSON.stringify({hours,bytesPerHour:perHour,avgBytesPerSecondPerHour:perHour.map(b=>Math.round(b/3600)),totalBytes:total}));
