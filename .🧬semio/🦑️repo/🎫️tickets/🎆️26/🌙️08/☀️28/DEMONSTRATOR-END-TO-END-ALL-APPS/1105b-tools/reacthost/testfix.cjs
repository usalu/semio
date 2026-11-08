const fs=require("fs");const D=process.argv[2];
function edit(f,fn){const p=D+"/"+f;fs.writeFileSync(p,fn(fs.readFileSync(p,"utf8")))}
function once(s,a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}return s.replace(a,()=>b)}
edit("🔨️modules/🌐️locale/🔣️.json",s=>once(s,'    "demonstrator.brand.abbreviation": { "en": "DwES", "de": "EmB" },\n',""));
edit("🧪️tests/🏛️shell-brand/🟦️.ts",s=>once(s,'ENERGIE_BRAND.windowTitle).toBe("Entwerfen mit Bestand")','ENERGIE_BRAND.windowTitle).toBe("Designing with Existing Stock / Entwerfen mit Bestand")'));
edit("🧪️tests/🧪️demonstratorpanedescription/🟦️.ts",s=>once(s,'pane.windowTitle.startsWith("Entwerfen mit Bestand · ")','pane.windowTitle.startsWith(`${demonstratorText("demonstrator.brand.name", locale)} · `)'));
