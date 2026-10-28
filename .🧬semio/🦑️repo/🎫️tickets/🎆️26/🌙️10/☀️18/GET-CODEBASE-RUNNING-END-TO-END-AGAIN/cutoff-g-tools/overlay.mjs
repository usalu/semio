import{execFileSync}from"node:child_process";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/";
if(!globalThis.__overlay)throw Error("run with: bun --preload overlay-plugin.mjs");
export function headContent(rel){try{return execFileSync("git",["show","HEAD:"+rel],{cwd:R,maxBuffer:1<<28,stdio:["ignore","pipe","ignore"]});}catch{return undefined;}}
export function changedSinceHead(){return execFileSync("git",["diff","HEAD","--name-only","-z"],{cwd:R,maxBuffer:1<<28}).toString().split("\0").filter(Boolean);}
export const setOverlay=map=>globalThis.__overlay.set(map);
export const sha=b=>createHash("sha256").update(b).digest("hex");
