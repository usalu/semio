const fs=require("fs");const p=process.argv[2]+"/🧪️tests/🏛️shell-brand/🟦️.ts";let s=fs.readFileSync(p,"utf8");
for(const [a,b] of [['energy("en").brand.windowTitle).toBe("Entwerfen mit Bestand · Energy")','energy("en").brand.windowTitle).toBe("Designing with Existing Stock · Energy")'],['energy("en").brand.shortWindowTitle).toBe("EmB · Energy")','energy("en").brand.shortWindowTitle).toBe("DwES · Energy")']]){if(s.split(a).length!==2)process.exit(1);s=s.replace(a,()=>b)}
fs.writeFileSync(p,s);
