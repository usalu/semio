const { execSync } = require("child_process");
const fs = require("fs");
const pdf = "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const outPath =
  ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY/zwischenbericht-layout.txt";
execSync(`pdftotext -layout "${pdf}" "${outPath}"`, { stdio: "inherit" });
const out = fs.readFileSync(outPath, "utf8");
const pages = out.split("\f");
console.log("[DEBUG] pages", pages.length);
for (let p = 0; p < pages.length; p++) {
  const t = pages[p];
  if (/Test-Case|Abkürzungs|Abkuerzungs|Glossar/.test(t)) {
    const hasAbkTitle = /Abkürzungsverzeichnis|Abkuerzungsverzeichnis/.test(t);
    const hasGlossTitle = /Glossar/.test(t);
    const hasTest = /Test-Case/.test(t);
    const hasAP = /\bAP\b/.test(t);
    const hasAbkHeader = /Abkürzung/.test(t);
    console.log(
      `[DEBUG] P${p + 1} glossTitle=${hasGlossTitle} abkTitle=${hasAbkTitle} abkHeader=${hasAbkHeader} test=${hasTest} AP=${hasAP}`,
    );
    if (hasTest || hasAbkTitle || hasAP) {
      console.log(
        t
          .split(/\r?\n/)
          .filter((l) => /Test-Case|Abk|Gloss|^\s*AP\b|API|Begriff/.test(l))
          .slice(0, 25)
          .join("\n"),
      );
      console.log("---");
    }
  }
}
