const fs = require("fs");
const root = process.argv[2];
const edits = [
  ["🔠️english/🎛️viewer.ts", [
    [`"viewer.viewpoint.plan": "Plan",`, `"viewer.viewpoint.plan": "Plan (top view)",`],
    [`"viewer.background.auto": "By view",`, `"viewer.background.auto": "By representation",`],
    [`This ground and this view have little contrast; choose \\"By view\\" or another view.`, `This ground and this representation have little contrast; choose \\"By representation\\" or another representation.`],
    [`thermal, force and moment views.`, `thermal, force and moment representations.`],
    [`Changes the thermal view live`, `Changes the thermal representation live`],
    [`Only the thermal view reads`, `Only the thermal representation reads`],
    [`This view carries no analytic values`, `This representation carries no analytic values`],
    [`This view needs no legend`, `This representation needs no legend`],
    [`"detail.compare": "Compare views"`, `"detail.compare": "Compare representations and viewpoints"`],
    [`The link to this exact view is`, `The link to this exact representation and viewpoint is`],
  ]],
  ["🔡️german/🎛️viewer.ts", [
    [`"viewer.background.auto": "Je Ansicht",`, `"viewer.background.auto": "Je Darstellung",`],
    [`Thema der Analyseansichten`, `Thema der Analysedarstellungen`],
    [`Dieser Untergrund und diese Ansicht haben wenig Kontrast; wählen Sie „Je Ansicht“ oder eine andere Ansicht.`, `Dieser Untergrund und diese Darstellung haben wenig Kontrast; wählen Sie „Je Darstellung“ oder eine andere Darstellung.`],
    [`Diese Optionen gelten für die Thermik-, Kraft- und Momentenansicht.`, `Diese Optionen gelten für die Darstellungen Thermik, Kräfte und Momente.`],
    [`Ändert die Thermikansicht live`, `Ändert die thermische Darstellung live`],
    [`Nur die Thermikansicht liest`, `Nur die thermische Darstellung liest`],
    [`Diese Ansicht trägt keine`, `Diese Darstellung trägt keine`],
    [`Diese Ansicht braucht keine`, `Diese Darstellung braucht keine`],
    [`"detail.compare": "Ansichten vergleichen"`, `"detail.compare": "Darstellungen und Blickrichtungen vergleichen"`],
    [`Der Link auf genau diese Ansicht liegt`, `Der Link auf genau diese Darstellung und Blickrichtung liegt`],
  ]],
  ["🔠️english/🧪️playground.ts", [
    [`Paper (follows view)`, `Paper (follows representation)`],
    [`This view draws light ink`, `This representation draws light ink`],
    [`Choose Paper to follow the view.`, `Choose Paper to follow the representation.`],
  ]],
  ["🔡️german/🧪️playground.ts", [
    [`Papier (folgt der Ansicht)`, `Papier (folgt der Darstellung)`],
    [`Diese Ansicht zeichnet helle`, `Diese Darstellung zeichnet helle`],
    [`damit er der Ansicht folgt.`, `damit er der Darstellung folgt.`],
  ]],
  ["🔠️english/⚖️compare.ts", [
    [`one asset in other views, styles and analytic states.`, `one asset in other viewpoints, representations and analytic states.`],
    [`"Same asset, other views"`, `"Same asset, other viewpoints or representations"`],
    [`"Other assets, same view"`, `"Other assets, same representation and viewpoint"`],
    [`of an analytic view to read`, `of an analytic representation to read`],
  ]],
  ["🔡️german/⚖️compare.ts", [
    [`ein Asset in anderen Ansichten, Stilen und Analysezuständen.`, `ein Asset in anderen Blickrichtungen, Darstellungen und Analysezuständen.`],
    [`"Gleiches Asset, andere Ansichten"`, `"Gleiches Asset, andere Blickrichtungen oder Darstellungen"`],
    [`"Andere Assets, gleiche Ansicht"`, `"Andere Assets, gleiche Darstellung und Blickrichtung"`],
    [`einer Analyseansicht`, `einer Analysedarstellung`],
  ]],
  ["🔠️english/📖️detail.ts", [
    [`Open the thermal view to see`, `Open the thermal representation to see`],
    [`Open the force or moment view to see`, `Open the force or moment representation to see`],
    [`the {view} view and scale 1:{scale}`, `the {representation} representation, the {viewpoint} viewpoint and scale 1:{scale}`],
    [`"This view, one page"`, `"This representation, one page"`],
    [`"All seven views, one page each"`, `"All seven representations, one page each"`],
    [`one page or all seven views.`, `one page or all seven representations.`],
  ]],
  ["🔡️german/📖️detail.ts", [
    [`Öffnen Sie die thermische Ansicht,`, `Öffnen Sie die thermische Darstellung,`],
    [`Öffnen Sie die Kraft- oder Momentenansicht,`, `Öffnen Sie die Kraft- oder Momentendarstellung,`],
    [`die Ansicht {view} und den Maßstab`, `die Darstellung {representation}, die Blickrichtung {viewpoint} und den Maßstab`],
    [`"Diese Ansicht, eine Seite"`, `"Diese Darstellung, eine Seite"`],
    [`"Alle sieben Ansichten, je eine Seite"`, `"Alle sieben Darstellungen, je eine Seite"`],
    [`eine Seite oder alle sieben Ansichten.`, `eine Seite oder alle sieben Darstellungen.`],
  ]],
  ["🔠️english/🧱️core.ts", [[`draws in seven synchronized views.`, `draws in seven synchronized representations.`]]],
  ["🔡️german/🧱️core.ts", [[`in sieben synchronen Ansichten.`, `in sieben synchronen Darstellungen.`]]],
];
for (const [file, list] of edits) {
  const path = `${root}/${file}`;
  let text = fs.readFileSync(path, "utf8");
  for (const [from, to] of list) {
    if (!text.includes(from)) {
      console.log("MISSING", file, from.slice(0, 50));
      continue;
    }
    text = text.replace(from, to);
  }
  fs.writeFileSync(path, text);
}
