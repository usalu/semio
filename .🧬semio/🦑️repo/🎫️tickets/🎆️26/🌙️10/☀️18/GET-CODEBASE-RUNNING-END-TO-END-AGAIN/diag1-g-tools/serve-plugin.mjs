// [diag1-g] preload (first): serves chosen library files from the git INDEX (pre-repair state) or from a scratch patch directory, with the overlay fs shim applied; nothing is written to the tree.
// DIAG_INDEX=rel1,rel2 (served from `git show :rel`)   DIAG_PATCHDIR=<dir> (file <dir>/<n>.mjs, DIAG_PATCHMAP=rel1=<n>,... served instead)
import{execFileSync}from"node:child_process";import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",here=import.meta.dir,fsShim=here+"/../cutoff-g-tools/overlay-fs-shim.mjs",fspShim=here+"/../cutoff-g-tools/overlay-fsp-shim.mjs";
const E=process.env,idx=new Set((E.DIAG_INDEX??"").split(",").filter(Boolean).map(r=>R+r)),patch=new Map((E.DIAG_PATCHMAP??"").split(",").filter(Boolean).map(s=>{const[i,n]=s.split("=");return[R+i,E.DIAG_PATCHDIR+"/"+n];}));
const shim=text=>/["'](?:node:)?fs(?:\/promises)?["']/.test(text)?text.replace(/(["'])(?:node:)?fs\/promises\1/g,(_m,q)=>q+fspShim+q).replace(/(["'])(?:node:)?fs\1/g,(_m,q)=>q+fsShim+q):text;
Bun.plugin({name:"diag1-serve",setup(b){b.onLoad({filter:/\.(mjs|ts)$/},async args=>{
 let text;
 if(patch.has(args.path))text=readFileSync(patch.get(args.path),"utf8");
 else if(idx.has(args.path))text=execFileSync("git",["show",":"+args.path.slice(R.length)],{cwd:R,maxBuffer:1<<28}).toString("utf8");
 else return undefined;
 console.error("[DEBUG] served "+(patch.has(args.path)?"patch":"index")+": "+args.path.slice(-70));
 return{contents:shim(text),loader:args.path.endsWith(".ts")?"ts":"js"};});}});
await import("../cutoff-g-tools/overlay-plugin.mjs");
await import("./invk-plugin.mjs");
