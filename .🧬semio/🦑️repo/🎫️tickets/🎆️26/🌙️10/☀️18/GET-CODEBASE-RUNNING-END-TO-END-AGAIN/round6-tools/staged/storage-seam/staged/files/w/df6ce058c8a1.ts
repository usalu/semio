import {spawnSync} from "node:child_process";

/** ✍️ Re-signs a native executable ad-hoc on macOS and does nothing anywhere else; the only place that names the macOS signing tool. See https://ss64.com/mac/codesign.html */
export function sealMacExecutableV1(binary:string,platform:string=process.platform):void{
 if(platform!=="darwin")return;
 const signed=spawnSync("codesign",["--force","--sign","-",binary],{stdio:["ignore","inherit","inherit"]});
 if(signed.status!==0)throw new Error(`codesign refused ${binary} (status ${signed.status??"unknown"})`);
}
