import { readdirSync, statSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const ignoreDirs = new Set([
  "node_modules", ".git", ".repo", "dist", "target", ".nx", "storybook-static",
  ".cargo", ".venv", ".repo-cache", ".storybook", "public", ".vscode", "temp", "test-results"
]);

// Exact list of German words with transliterated Umlaute (ue/ae/oe/Ue/Ae/Oe)
const patterns = [
  /\bOeffnen\b/g,
  /\bRueckgaengig\b/g,
  /\bLoeschen\b/g,
  /\bloeschen\b/g,
  /\bverfuegbar\b/g,
  /\bZuruecksetzen\b/g,
  /\bzuruecksetzen\b/g,
  /\bStrichstaerken\b/g,
  /\bOberflaeche\b/g,
  /\bOberflaeche\b/g,
  /\bLaedt\b/g,
  /\bZurueck\b/g,
  /\bzurueck\b/g,
  /\bAusfuehren\b/g,
  /\bausfuehren\b/g,
  /\bAuswaehlen\b/g,
  /\bauswaehlen\b/g,
  /\bAbwaehlen\b/g,
  /\babwaehlen\b/g,
  /\bEinfuegen\b/g,
  /\beinfuegen\b/g,
  /\bpruefen\b/g,
  /\bPruefen\b/g,
  /\bPruefe\b/g,
  /\bVervollstaendigungen\b/g,
  /\bhinzufuegen\b/g,
  /\bhinzufuegen\b/g,
  /\bFlaeche\b/g,
  /\bflaeche\b/g,
  /\bWaerme\b/g,
  /\bwaerme\b/g,
  /\bLueftung\b/g,
  /\blueftung\b/g,
  /\bGebaeude\b/g,
  /\bgebaeude\b/g,
  /\bUeber\b/g,
  /\bueber\b/g,
  /\bFuer\b/g,
  /\bfuer\b/g,
  /\bGroesse\b/g,
  /\bgroesse\b/g,
  /\bAender\b/g,
  /\baender\b/g,
  /\bBegruen\b/g,
  /\bbegruen\b/g,
  /\bSued\b/g,
  /\bsued\b/g,
  /\bStueck\b/g,
  /\bstueck\b/g,
  /\bGueltig\b/g,
  /\bgueltig\b/g,
  /\bKuehl\b/g,
  /\bkuehl\b/g,
  /\bPlatzhalter-Mesh zuruecksetzen\b/g,
  /\bErgebnis loeschen\b/g,
  /\bAusfuehrung\b/g,
  /\bausfuehrung\b/g,
];

function scanDir(dir: string) {
  const entries = readdirSync(dir);
  for (const entry of entries) {
    if (ignoreDirs.has(entry)) continue;
    if (entry.startsWith(".")) continue;
    const full = join(dir, entry);
    const stat = statSync(full);
    if (stat.isDirectory()) {
      scanDir(full);
    } else if (stat.isFile() && (full.endsWith(".rs") || full.endsWith(".ts") || full.endsWith(".tsx"))) {
      try {
        const content = readFileSync(full, "utf8");
        const lines = content.split("\n");
        lines.forEach((line, idx) => {
          for (const p of patterns) {
            if (p.test(line)) {
              console.log(`${full}:${idx + 1}: ${line.trim()}`);
              break;
            }
          }
        });
      } catch (e) {}
    }
  }
}

scanDir(root);
