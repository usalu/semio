const fs=require("fs");
const [,, file, out]=process.argv;
const j=JSON.parse(fs.readFileSync(file,"utf8"));
const rows=[];
for(const f of j.testResults){ const rel=f.name.replace("/Users/ueli/Documents/semio/",""); if(f.assertionResults.length===0) rows.push(`${f.status}\t${rel}\t<file>\t${(f.message||"").split("\n")[0].slice(0,160)}`); for(const a of f.assertionResults) rows.push(`${a.status}\t${rel}\t${a.fullName}`); }
rows.sort();
if(out) fs.writeFileSync(out, rows.join("\n")+"\n");
const c={}; for(const r of rows){const s=r.split("\t")[0]; c[s]=(c[s]||0)+1;}
console.log(file.split("/").pop(), "files", j.testResults.length, "tests", j.numTotalTests, "pass", j.numPassedTests, "fail", j.numFailedTests, JSON.stringify(c));
