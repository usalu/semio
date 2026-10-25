import {createBuildBootstrapCapabilitiesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
const target=process.argv[2]??"@semio-tech/repo-dashboard-rs:build";
const grant=await createBuildBootstrapCapabilitiesV1(process.cwd(),["repo","run",target],{SEMIO_BUILD_TOOL:"nx",SEMIO_BUILD_MODE:"prototype"}) as any;
console.log(JSON.stringify(grant.launches,null,1));
