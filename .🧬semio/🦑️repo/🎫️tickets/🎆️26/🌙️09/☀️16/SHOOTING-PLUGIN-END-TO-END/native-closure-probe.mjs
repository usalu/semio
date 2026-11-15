import {nativeDependencyRoots} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs";
const workspaceRoot="/Users/ueli/Documents/semio",root=process.argv[2],mb=()=>Math.round(process.memoryUsage().rss/1048576);
const cache=new Map(),closures=new Map();
for(const tests of [false,true]){const t=performance.now(),before=mb();const r=nativeDependencyRoots(root,workspaceRoot,tests,cache,closures);console.log("[DEBUG] tests",tests,"roots",r.length,"ms",Math.round(performance.now()-t),"rss",before,"->",mb());}
