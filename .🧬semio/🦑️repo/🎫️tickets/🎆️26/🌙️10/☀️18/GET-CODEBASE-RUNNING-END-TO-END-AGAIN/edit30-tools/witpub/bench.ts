/** ⏱️ Read-only: the publisher's witness cost on a live Bazel workspace (read the capture manifest, digest every captured row as one witness): bun bench.ts <mirror root> <workspace> */
const [mirror,workspace]=process.argv.slice(2) as [string,string];
const base=mirror+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel";
const {readBazelSourceSnapshotV1}=await import(base+"/📸️snapshot/🟦️.ts"),{bazelWitnessFilesOfPlanV1,bazelWitnessPlanTextV1}=await import(base+"/🚦️test-classes/🧾️witness/🟦️.ts");
for(let round=0;round<3;round++){const started=performance.now();const rows=await readBazelSourceSnapshotV1(workspace);const plan=JSON.parse(bazelWitnessPlanTextV1(new Map([["w_all",[...rows.keys()]]])));const files=bazelWitnessFilesOfPlanV1(plan,rows);console.log(JSON.stringify({rows:rows.size,ms:Math.round(performance.now()-started),witness:files[0]![1].trim()}));}
