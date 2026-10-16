#!/bin/zsh
# usage: cmp-titles.sh <before.json> <after.json> <file-substring-filter-for-before>   -> compares multiset of "status<TAB>fullName" restricted (before) to files containing the substring
. "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pfs-tools/env.sh"
node -e '
const fs=require("fs");const [b,a,f]=process.argv.slice(1);
const rows=(p,flt)=>{const j=JSON.parse(fs.readFileSync(p,"utf8"));const r=[];for(const x of j.testResults){if(flt&&!x.name.includes(flt))continue;for(const t of x.assertionResults)r.push(t.status+"\t"+t.fullName)}return r.sort()};
const B=rows(b,f),A=rows(a,"");
const cnt=l=>{const m=new Map();for(const x of l)m.set(x,(m.get(x)||0)+1);return m};
const cb=cnt(B),ca=cnt(A);let miss=0,extra=0;
for(const [k,n] of cb){const m=ca.get(k)||0;if(m<n){miss+=n-m;console.log("MISSING",n-m,k.slice(0,160))}}
for(const [k,n] of ca){const m=cb.get(k)||0;if(m<n){extra+=n-m;console.log("EXTRA",n-m,k.slice(0,160))}}
console.log("before",B.length,"after",A.length,"missing",miss,"extra",extra,miss==0&&extra==0?"IDENTICAL":"DIFFERENT");
' "$1" "$2" "$3"
