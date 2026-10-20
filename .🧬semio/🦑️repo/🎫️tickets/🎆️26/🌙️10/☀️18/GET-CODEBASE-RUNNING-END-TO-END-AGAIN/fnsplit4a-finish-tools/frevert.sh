#!/bin/zsh
# usage: frevert.sh <spec.json> <snapshot> : removes the child dirs of a phase spec from the overlay and restores the root (overlay only)
node -e '
const fs=require("fs"),path=require("path");const s=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));
if(!s.file.startsWith("/private/tmp/fnsplit4a-finish/"))throw new Error("not an overlay path");
const base=path.dirname(s.file);const dirs=[...(s.files||[]).map(f=>f.dir),...(s.groups||[]).map(f=>f.dir),...(s.context?[s.context.dir]:[]),...(s.shared?[s.shared.dir]:[]),...(s.helpers?[s.helpers.dir]:[])];
for(const d of dirs){fs.rmSync(path.join(base,d),{recursive:true,force:true});}
fs.copyFileSync(process.argv[2],s.file);console.log("reverted",dirs.length);' "$1" "$2"
