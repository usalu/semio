/** 🏗 Provisions the native watcher from its single pinned source file: checks the pin, finds an installed binary, otherwise compiles into a content addressed directory and proves the build with the protocol's own `version` frame. */
import {createHash} from "node:crypto";
import {chmodSync,copyFileSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";

/** 🔏 sha256 of the watcher source. */
export function watcherSourceDigestV1(sourcePath){return createHash("sha256").update(readFileSync(sourcePath)).digest("hex");}

/** 📌 Reads the pin manifest and refuses a manifest that does not describe the source on disk. */
export function readWatcherPinV1(pinPath,sourcePath){
 const pin=JSON.parse(readFileSync(pinPath,"utf8")),row=pin.sources?.[0],digest=watcherSourceDigestV1(sourcePath);
 if(typeof pin.version!=="string"||row?.platform!=="any"||row.architecture!=="any")throw new Error("The watcher pin must name one source for every platform");
 if(row.sha256!==digest||row.bytes!==readFileSync(sourcePath).length)throw new Error("The watcher pin is stale: it names "+String(row.sha256).slice(0,12)+" but the source is "+digest.slice(0,12));
 return{version:pin.version,sha256:digest};
}

/** 📄 The standalone package the compile step builds: one binary, no dependencies, its own workspace so no root lease is taken. */
export function watcherManifestV1(){
 return'[package]\nname = "semio-repo-watch"\nversion = "1.0.0"\nedition = "2021"\n\n[[bin]]\nname = "semio-repo-watch"\npath = "main.rs"\n\n[workspace]\n';
}

const executableName=platform=>platform==="win32"?"semio-repo-watch.exe":"semio-repo-watch";

/** 🛠 Finds or builds the watcher. `locate()` answers an installed path, `compile({directory,signal})` builds `semio-repo-watch` in `directory/target/release` and resolves its path, `outputRoot` is the tools partition directory of the tool, `writable()` answers whether that directory may be written now (an unmounted image is refused before anything is compiled). */
export async function ensureWatcherV1({sourcePath,pinPath,outputRoot,locate,compile,writable=()=>true,signal,progress=()=>undefined,platform=process.platform,probe=defaultProbe}){
 const pin=readWatcherPinV1(pinPath,sourcePath),installed=await locate();
 if(installed)return{path:installed,built:false,version:pin.version,sha256:pin.sha256};
 if(!writable())throw new Error("The watcher output root is an image that is not mounted: "+outputRoot+"; it is installed once a build has mounted the tools partition");
 const work=mkdtempSync(join(tmpdir(),"semio-watch-"));
 try{
  progress("compile");
  writeFileSync(join(work,"Cargo.toml"),watcherManifestV1());copyFileSync(sourcePath,join(work,"main.rs"));
  const built=await compile({directory:work,signal});
  const target=join(outputRoot,pin.sha256.slice(0,16));
  mkdirSync(target,{recursive:true});
  const staged=join(target,executableName(platform)+"."+process.pid+".tmp");
  copyFileSync(built,staged);chmodSync(staged,0o755);
  if(!probe(staged)){rmSync(staged,{force:true});throw new Error("The built watcher does not answer the version frame");}
  const final=join(target,executableName(platform));
  renameSync(staged,final);
  progress("ready");
  return{path:final,built:true,version:pin.version,sha256:pin.sha256};
 }finally{rmSync(work,{recursive:true,force:true});}
}

function defaultProbe(path){
 const result=spawnSync(path,["version"],{encoding:"utf8",timeout:10000,windowsHide:true});
 try{const frame=JSON.parse(String(result.stdout).trim());return result.status===0&&frame.t==="version"&&frame.protocol===1&&frame.name==="semio-repo-watch";}catch{return false;}
}
