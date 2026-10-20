#!/bin/zsh
# usage: revert2.sh <spec.json> <snapshot> : removes group/context/file dirs of a ladder or phase spec and restores the root
node -e '
const fs=require("fs"),path=require("path");const s=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));
const file=s.file;const base=path.dirname(file);const dirs=[...(s.files||[]).map(f=>f.dir),...(s.groups||[]).map(f=>f.dir),...(s.context?[s.context.dir]:[]),...(s.shared?[s.shared.dir]:[]),...(s.helpers?[s.helpers.dir]:[])];
for(const d of dirs){fs.rmSync(path.join(base,d),{recursive:true,force:true});}
fs.copyFileSync(process.argv[2],file);console.log("reverted",dirs.length);' "$1" "$2"
