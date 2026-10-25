import {readJavascriptLockScopesV1,checkJavascriptLockScopesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/🟦️.ts";
const only=process.argv[2];
for(const r of checkJavascriptLockScopesV1(readJavascriptLockScopesV1(process.cwd()).filter(s=>!only||s.scope===only))){
 console.log(r.scope,"findings",r.drift.length,"frozen",r.drift.filter(d=>d.frozen).length);
 for(const d of r.drift.slice(0,60))console.log(" ",d.frozen?"F":"-",d.kind,d.manifest.slice(-70),d.section,d.dependency,d.manifestVersion,d.lockVersion);
}
