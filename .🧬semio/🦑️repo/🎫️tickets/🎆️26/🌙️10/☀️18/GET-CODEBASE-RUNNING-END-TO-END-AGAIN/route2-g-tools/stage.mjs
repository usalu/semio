/** 🧪 route2-g: starts staging a build-relative file: copies the current original into staged/ and records its base digest. usage: bun stage.mjs <build-relative-file> */
import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname} from "node:path";
const root="/Users/ueli/Documents/semio",C=root+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN",T=C+"/route2-g-tools",build=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",library=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const manifestPath=T+"/staged-base.json",manifest=existsSync(manifestPath)?JSON.parse(readFileSync(manifestPath,"utf8")):{};
for(const arg of process.argv.slice(2)){const lib=arg.startsWith("lib:"),rel=lib?arg.slice(4):arg,bytes=readFileSync((lib?library:build)+rel),dest=T+(lib?"/staged-lib/":"/staged/")+rel;mkdirSync(dirname(dest),{recursive:true});if(!existsSync(dest)){writeFileSync(dest,bytes);manifest[(lib?"lib:":"")+rel]=createHash("sha256").update(bytes).digest("hex");}console.log(dest);}
writeFileSync(manifestPath,JSON.stringify(manifest,null,1));
