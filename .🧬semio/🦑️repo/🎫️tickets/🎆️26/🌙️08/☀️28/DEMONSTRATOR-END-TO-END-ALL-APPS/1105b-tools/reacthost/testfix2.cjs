const fs=require("fs");const p=process.argv[2]+"/🧪️tests/🧪️demonstratorpanedescription/🟦️.ts";let s=fs.readFileSync(p,"utf8");
const a='pane.windowTitle.startsWith(`${demonstratorText("demonstrator.brand.name", locale)} · `)';
if(s.split(a).length!==2)process.exit(1);
s=s.replace(a,()=>'pane.windowTitle.startsWith(`${{ en: "Designing with Existing Stock", de: "Entwerfen mit Bestand" }[locale]} · `)');fs.writeFileSync(p,s);
