/** 🔎️ [isolate] runs the test-store-isolation scanner of mirror <mirror-root> over repository <root>; prints findings per file and totals. usage: bun scan.ts <mirror-root> <root> [--files] */
const [mirror,root,...flags]=process.argv.slice(2);
const law=await import(mirror+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧫️test-store-isolation/🟦️.ts");
const started=Date.now(),report=law.testStoreIsolationReportV1(root),files=new Map<string,number>();
for(const finding of report.findings)files.set(finding.path,(files.get(finding.path)??0)+1);
if(flags.includes("--files"))for(const [path,count] of [...files].sort())console.log(count+"\t"+path);
else for(const finding of report.findings)console.log(finding.path.split("/").slice(-4,-1).join("/")+":"+finding.line+" "+finding.rule+" "+finding.text.slice(0,110));
console.error("scanned",report.scanned,"findings",report.findings.length,"files",files.size,"stale",report.stale.length,"ms",Date.now()-started);
