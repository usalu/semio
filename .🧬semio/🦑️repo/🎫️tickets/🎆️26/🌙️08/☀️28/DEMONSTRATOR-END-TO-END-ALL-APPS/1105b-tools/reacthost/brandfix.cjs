const fs=require("fs");const D=process.argv[2];
function edit(f,fn){const p=D+"/"+f;let s=fs.readFileSync(p,"utf8");s=fn(s);fs.writeFileSync(p,s)}
function once(s,a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}return s.replace(a,()=>b)}
edit("🪧️brand.ts",s=>{
 s=once(s,'/** 🏷️ The project name, the same proper noun in every locale. */\nexport const ENTWERFEN_MIT_BESTAND_NAME = "Entwerfen mit Bestand";','/** 🏷️ Language-neutral title naming the project in both languages, used where no viewer language is resolved. */\nexport const DEMONSTRATOR_NEUTRAL_TITLE = `${demonstratorText("demonstrator.brand.name", "en")} / ${demonstratorText("demonstrator.brand.name", "de")}`;');
 s=once(s,'role="img" aria-label="Entwerfen mit Bestand"','aria-hidden="true" focusable="false"');
 s=s.split("windowTitle: ENTWERFEN_MIT_BESTAND_NAME,").join("windowTitle: DEMONSTRATOR_NEUTRAL_TITLE,");
 if(s.includes("ENTWERFEN_MIT_BESTAND_NAME")){console.error("left");process.exit(1)}
 return s});
edit("🐚️shell/🟦️.ts",s=>{s=once(s,"ENTWERFEN_MIT_BESTAND_NAME, demonstratorPaneBootVariants","DEMONSTRATOR_NEUTRAL_TITLE, demonstratorPaneBootVariants");return once(s,"|| ENTWERFEN_MIT_BESTAND_NAME;","|| DEMONSTRATOR_NEUTRAL_TITLE;")});
edit("🟦️.tsx",s=>{
 s=once(s,'import { type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";','import { demonstratorText, type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";');
 s=once(s,"ENTWERFEN_MIT_BESTAND_LOGO_SVG, ENTWERFEN_MIT_BESTAND_NAME, demonstratorGeneralIntroduction","ENTWERFEN_MIT_BESTAND_LOGO_SVG, demonstratorGeneralIntroduction");
 s=once(s,"label={ENTWERFEN_MIT_BESTAND_NAME}",'label={demonstratorText("demonstrator.brand.name", locale)}');
 s=once(s,"{ENTWERFEN_MIT_BESTAND_NAME}",'{demonstratorText("demonstrator.brand.name", locale)}');
 return s});
edit("🌐️.html",s=>{s=once(s,"<title>Entwerfen mit Bestand</title>","<title>Designing with Existing Stock / Entwerfen mit Bestand</title>");return once(s,"<html>",'<html lang="mul">')});
