// Switches the German chrome of @semio-tech/quiz-react from "Sie" to "du" (the register of the catalog content).
// Usage: node quiz_react_german_du.mjs <repo root>. Every phrase must occur exactly once, else nothing is written.
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const file = join(process.argv[2], "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts");
const replacements = [
  ["Wie möchten Sie erscheinen?", "Wie möchtest du erscheinen?"],
  ["Wählen Sie, wie Ihre Ergebnisse in der Rangliste erscheinen.", "Wähle, wie deine Ergebnisse in der Rangliste erscheinen."],
  ["Sie erscheinen als „Anonym“ mit einem kurzen Code. Ihr Fortschritt bleibt an dieses Gerät gebunden.", "Du erscheinst als „Anonym“ mit einem kurzen Code. Dein Fortschritt bleibt an dieses Gerät gebunden."],
  ["Sie erscheinen unter einem selbst gewählten Namen.", "Du erscheinst unter einem selbst gewählten Namen."],
  ["Sie erscheinen unter Ihrem Namen.", "Du erscheinst unter deinem Namen."],
  ['phrase("Ihr Pseudonym")', 'phrase("Dein Pseudonym")'],
  ['phrase("Ihr Name")', 'phrase("Dein Name")'],
  ["setzt diesen Fortschritt fort; wählen Sie also etwas, das andere nicht erraten.", "setzt diesen Fortschritt fort; wähle also etwas, das andere nicht erraten."],
  ["Ihr Quiz wird vorbereitet…", "Dein Quiz wird vorbereitet…"],
  ["Erledigen Sie alle Aufgaben, um abzugeben.", "Erledige alle Aufgaben, um abzugeben."],
  ["Nach der Abgabe können Sie Ihre Antworten nicht mehr ändern.", "Nach der Abgabe kannst du deine Antworten nicht mehr ändern."],
  ["Abgebrochen. Ihre Antworten bleiben erhalten.", "Abgebrochen. Deine Antworten bleiben erhalten."],
  ["daher wurde Ihr Durchgang verworfen. Bitte starten Sie neu.", "daher wurde dein Durchgang verworfen. Bitte starte neu."],
  ["Ordnen Sie die Elemente nach {{quantity}}", "Ordne die Elemente nach {{quantity}}"],
  ["sobald Sie ein Element verschieben oder die aktuelle Reihenfolge übernehmen.", "sobald du ein Element verschiebst oder die aktuelle Reihenfolge übernimmst."],
  ["Ordnen Sie jedem Element pro Größe eine Wertkarte zu.", "Ordne jedem Element pro Größe eine Wertkarte zu."],
  ["Ihre Wertung: {{score}}", "Deine Wertung: {{score}}"],
  ['phrase("Ihre Antwort")', 'phrase("Deine Antwort")'],
  ['phrase("Ihre Position")', 'phrase("Deine Position")'],
  ["Wählen Sie eine Spaltenüberschrift zum Sortieren.", "Wähle eine Spaltenüberschrift zum Sortieren."],
  ['you: phrase("Sie")', 'you: phrase("du")'],
  ["Bitte wählen Sie Ihre Identität erneut.", "Bitte wähle deine Identität erneut."],
  ["Bitte erledigen Sie vor der Abgabe alle Aufgaben.", "Bitte erledige vor der Abgabe alle Aufgaben."],
  ["bitte starten Sie neu.", "bitte starte neu."],
];

let text = readFileSync(file, "utf8");
for (const [formal, informal] of replacements) {
  const count = text.split(formal).length - 1;
  if (count !== 1) throw new Error(`expected exactly one ${JSON.stringify(formal)}, found ${count}`);
  text = text.replace(formal, informal);
}
const handles = text.split("Bitte geben Sie 1 bis 64 Zeichen ein.").length - 1;
if (handles !== 2) throw new Error(`expected two handle messages, found ${handles}`);
text = text.replaceAll("Bitte geben Sie 1 bis 64 Zeichen ein.", "Bitte gib 1 bis 64 Zeichen ein.");
writeFileSync(file, text);
console.log(`replaced ${replacements.length + 2} formal phrases`);
