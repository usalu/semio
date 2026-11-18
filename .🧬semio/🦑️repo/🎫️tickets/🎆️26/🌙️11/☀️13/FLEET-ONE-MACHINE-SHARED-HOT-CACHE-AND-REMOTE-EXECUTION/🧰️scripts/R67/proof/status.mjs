const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");
const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env);console.log(s.endpoint,s.instance,s.upload);
const f=remoteCacheFetchV1(env,{root});
const r=await f(s.endpoint+"/status",{method:"GET"});console.log(r.status);const t=await r.text();console.log(t.slice(0,6000));
