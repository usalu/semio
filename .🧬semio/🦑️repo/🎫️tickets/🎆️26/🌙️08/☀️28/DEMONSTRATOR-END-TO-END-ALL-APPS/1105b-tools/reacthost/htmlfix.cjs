const fs=require("fs");const D=process.argv[2];
function edit(f,fn){const p=D+"/"+f;fs.writeFileSync(p,fn(fs.readFileSync(p,"utf8")))}
function once(s,a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}return s.replace(a,()=>b)}
const T="Designing with Existing Stock · Demonstrator · Entwerfen mit Bestand · Demonstrator";
edit("🌐️.html",s=>{s=once(s,'<html lang="mul">',"<html>");return once(s,"<title>Designing with Existing Stock / Entwerfen mit Bestand</title>","<title>"+T+"</title>")});
edit("🏗️builder/🌐️vite/🟦️.ts",s=>once(s,'title: "Entwerfen mit Bestand · Demonstrator",','title: [{ lang: "en", text: "Designing with Existing Stock · Demonstrator" }, { lang: "de", text: "Entwerfen mit Bestand · Demonstrator" }],'));
edit("🧪️tests/🧪️demonstratorlocale/🧫️cases.json",s=>once(s,'"required": "<title>Entwerfen mit Bestand</title>"','"required": "<title>'+T+'</title>"'));
