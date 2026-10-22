// run-closure.ts <entry relative to repo root> [args...]: captures the closure like the CLI (installed mode) and runs the real stream loader on it without the CLI queue.
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {buildSourceClosureStreamLoaderV1,buildSourceClosureIdentityV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts";
import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root="/Users/ueli/Documents/semio",[entry,...args]=process.argv.slice(2);
const t0=Date.now(),source=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});
const bytes=nxAuthorizerCapturedBytesV1(source),imports=nxAuthorizerCapturedImportsV1(root,source);
const files=[...bytes].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))).map(([path,b])=>({path,sha256:createHash("sha256").update(b).digest("hex"),base64:Buffer.from(b).toString("base64"),imports:imports.get(path)!}));
const snapshot={version:1 as const,root,entry,files},text=JSON.stringify(snapshot),sha=buildSourceClosureIdentityV1(snapshot);
console.error("captured",files.length,"files in",Date.now()-t0,"ms; parent loader sha",createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"));
const child=spawn(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",sha,...args],{cwd:root,stdio:["inherit","inherit","inherit","pipe"],env:process.env});
(child.stdio[3] as NodeJS.WritableStream).end(text);
process.exit(await new Promise<number>(r=>child.on("close",c=>r(c??1))));
