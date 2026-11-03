import { readFileSync, writeFileSync } from "node:fs";

const file = new URL("./rack.mjs", import.meta.url);
const lines = readFileSync(file, "utf8").split("\n");
const q = (en, de) => `["${en}", "${de}"]`;
const block = (o) => Object.entries(o).map(([k, v]) => `    ${k}: ${q(...v)},`);

const d50 = {
  silhouette: ["A front view 12 mm wide and 40 mm tall on paper: two corner posts, a roof, a base rail and two casters frame a perforated door field, in front of which the 19-inch equipment stack stands.", "Eine Vorderansicht von 12 mm Breite und 40 mm Höhe auf Papier: zwei Eckpfosten, ein Dach, eine Fußschiene und zwei Rollen rahmen ein gelochtes Türfeld, vor dem der 19-Zoll-Geräteturm steht."],
  retained: ["Height 2.0 m, width 0.6 m, the 42 height units of 44.45 mm, the 482.6 mm equipment width, the storage array and six 2U servers at the bottom, patch panel, cable brush and switch at the top, the open units between them, the door handle.", "Höhe 2,0 m, Breite 0,6 m, die 42 Höheneinheiten zu 44,45 mm, die Gerätebreite von 482,6 mm, der Speicher und sechs 2-HE-Server unten, Patchfeld, Kabelbürste und Switch oben, die offenen Einheiten dazwischen, der Türgriff."],
  merged: ["Hinges, lock and the door frame merge into the door field; rails and cable entries merge into the posts and the roof.", "Scharniere, Schloss und der Türrahmen gehen im Türfeld auf; Schienen und Kabeleinführungen gehen in den Pfosten und das Dach auf."],
  omitted: ["The mounting rails, the rear door, the levelling feet, the PDU strips and all cabling; the door is drawn behind the equipment fronts so that the units are seen through its 70 percent open perforation.", "Die Montageschienen, die Rücktür, die Nivellierfüße, die Steckdosenleisten und alle Kabel; die Tür ist hinter den Gerätefronten gezeichnet, damit die Geräte durch ihre zu 70 Prozent offene Lochung zu sehen sind."],
  thinElements: ["The perforation is a dot hatch of 0.2 mm dots on a 0.6 mm grid in the door field; drive bays, vent slits, ports and slots are 0.13 mm lines, LEDs are 13 mm discs.", "Die Lochung ist eine Punktschraffur aus 0,2-mm-Punkten im Raster von 0,6 mm im Türfeld; Laufwerksschächte, Lüftungsschlitze, Anschlüsse und Schlitze sind 0,13-mm-Linien, LEDs sind Scheiben von 13 mm."],
  hatching: ["Dot hatch for the perforated door, 0.6 mm spacing, 0.2 mm dots, graphic views only.", "Punktschraffur für die gelochte Tür, 0,6 mm Abstand, 0,2-mm-Punkte, nur in grafischen Ansichten."],
  styles: ["Posts and roof 0.5 and 0.45, servers 0.45, storage 0.4, switch 0.3, door field 0.18, casters 0.2; body pen 0.18 mm, detail pen 0.13 mm.", "Pfosten und Dach 0,5 und 0,45, Server 0,45, Speicher 0,4, Switch 0,3, Türfeld 0,18, Rollen 0,2; Körperstift 0,18 mm, Detailstift 0,13 mm."],
  overlays: ["Cabinet, servers, switch, patch block, door and casters are separate regions: in the thermal view the warm server fronts stand out from the cool door field and the warm cabinet sheets.", "Schrank, Server, Switch, Patchblock, Tür und Rollen sind getrennte Regionen: In der Thermalansicht heben sich die warmen Serverfronten vom kühlen Türfeld und den warmen Schrankblechen ab."],
};
const d100 = {
  silhouette: ["A front view 6 mm wide and 20 mm tall on paper: the corner posts, roof and base frame a dotted door field with the lower server stack and the upper switch group in front.", "Eine Vorderansicht von 6 mm Breite und 20 mm Höhe auf Papier: Eckpfosten, Dach und Fußrahmen fassen ein gepunktetes Türfeld ein, davor der untere Serverturm und die obere Switchgruppe."],
  retained: ["Height 2.0 m, width 0.6 m, the storage array, each of the six servers, the patch and brush block, the switch, the casters and the door handle.", "Höhe 2,0 m, Breite 0,6 m, der Speicher, jeder der sechs Server, der Patch- und Bürstenblock, der Switch, die Rollen und der Türgriff."],
  merged: ["Patch panel and cable brush are one 2U block.", "Patchfeld und Kabelbürste sind ein 2-HE-Block."],
  omitted: ["Drive bays, vent slits, ports and LEDs of the units and the rails.", "Laufwerksschächte, Lüftungsschlitze, Anschlüsse und LEDs der Geräte sowie die Schienen."],
  thinElements: ["Posts are widened from 30 to 34 mm and the roof to 40 mm so that fill and pen reach 0.34 mm; the 2U servers keep a 3 mm joint to the neighbours.", "Pfosten sind von 30 auf 34 mm und das Dach auf 40 mm verbreitert, damit Füllung und Stift 0,34 mm erreichen; die 2-HE-Server behalten eine Fuge von 3 mm zu den Nachbarn."],
  hatching: ["Dot hatch for the perforated door, 0.45 mm spacing, 0.2 mm dots, graphic views only.", "Punktschraffur für die gelochte Tür, 0,45 mm Abstand, 0,2-mm-Punkte, nur in grafischen Ansichten."],
  styles: ["Posts 0.5, servers 0.45, storage 0.4, switch 0.3, door field 0.18; body pen 0.18 mm.", "Pfosten 0,5, Server 0,45, Speicher 0,4, Switch 0,3, Türfeld 0,18; Körperstift 0,18 mm."],
  overlays: ["The same six regions as at 1:50; the patch block is drawn with the blanks region, the switch with its own.", "Dieselben sechs Regionen wie bei 1:50; der Patchblock gehört zur Region Patchfeld, der Switch zu seiner eigenen."],
};
const d200 = {
  silhouette: ["A front view 3 mm wide and 10 mm tall on paper: a framed tower with one dark server block at the bottom, a top group and a dotted door field between.", "Eine Vorderansicht von 3 mm Breite und 10 mm Höhe auf Papier: ein gerahmter Turm mit einem dunklen Serverblock unten, einer oberen Gruppe und einem gepunkteten Türfeld dazwischen."],
  retained: ["Height 2.0 m, width 0.6 m, the frame, the 16U block of storage and six servers, the top group of switch, patch panel and brush, the door field.", "Höhe 2,0 m, Breite 0,6 m, der Rahmen, der 16-HE-Block aus Speicher und sechs Servern, die obere Gruppe aus Switch, Patchfeld und Bürste, das Türfeld."],
  merged: ["Storage and six servers are one block; switch, patch panel and brush are one top group (regions switch and blanks).", "Speicher und sechs Server sind ein Block; Switch, Patchfeld und Bürste sind eine obere Gruppe (Regionen Switch und Patchfeld)."],
  omitted: ["All unit details, the casters' wheels, the handle and the door edge.", "Alle Gerätedetails, die Räder der Rollen, der Griff und die Türkante."],
  thinElements: ["Posts are widened from 30 to 50 mm (0.25 mm) and the roof to 60 mm; the pen is 0.13 mm.", "Pfosten sind von 30 auf 50 mm (0,25 mm) und das Dach auf 60 mm verbreitert; der Stift ist 0,13 mm."],
  hatching: ["Dot hatch for the perforated door, 0.45 mm spacing, 0.2 mm dots, graphic views only.", "Punktschraffur für die gelochte Tür, 0,45 mm Abstand, 0,2-mm-Punkte, nur in grafischen Ansichten."],
  styles: ["Posts 0.5, server block 0.42, top group 0.4, door field 0.18; one body pen of 0.13 mm.", "Pfosten 0,5, Serverblock 0,42, obere Gruppe 0,4, Türfeld 0,18; ein Körperstift von 0,13 mm."],
  overlays: ["Merged shapes carry area-weighted colour; the door field is the cool region behind the equipment.", "Zusammengeführte Formen tragen flächengewichtete Farbe; das Türfeld ist die kühle Region hinter den Geräten."],
};

const out = [];
const ranges = [[100, 107, d50], [135, 142, d100], [162, 169, d200]];
lines.forEach((l, i) => {
  const n = i + 1;
  const r = ranges.find(([a, b]) => n >= a && n <= b);
  if (!r) out.push(l);
  else if (n === r[0]) out.push(...block(r[2]));
});
writeFileSync(file, out.join("\n"));
