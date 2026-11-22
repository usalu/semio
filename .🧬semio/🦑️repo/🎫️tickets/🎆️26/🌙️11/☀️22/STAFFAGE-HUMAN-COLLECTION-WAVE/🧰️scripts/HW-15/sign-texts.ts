/** 📝️ Identity, provenance, pose and clip texts, activity and the per-scale decisions of `person-signing` that change from `standing-adult`.
 * @see ./gen-signing.ts
 */
import { t } from "./lib.ts";

const REFS = [
  { id: "klima-bellugi-1979", citation: "Klima, E. S. and Bellugi, U. (1979). The Signs of Language. Harvard University Press: hand shape, location, movement and orientation; the signing space in front of the body from the waist to above the head." },
  { id: "boyes-braem-1995", citation: "Boyes Braem, P. (1995). Einführung in die Gebärdensprache und ihre Erforschung, 3rd ed. Signum, Hamburg: German-language sign phonology and the signing space (DGS HAUS, DANKE)." },
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (standing, talking in person, code 09071: 1.8 MET; standing quietly, code 07040: 1.3 MET)." },
];

const POSES: Record<string, [{ en: string; de: string }, { en: string; de: string }]> = {
  "relaxed": [t("Standing relaxed", "Entspannt stehen"), t("Feet at hip width, weight on both legs, arms hanging loosely at the sides, head level: waiting for the turn in a conversation.", "Füße hüftbreit, Gewicht auf beiden Beinen, Arme locker seitlich hängend, Kopf gerade: wartet auf den eigenen Gesprächsbeitrag.")],
  "signing-ready": [t("Ready to sign", "Gebärdenbereit"), t("Both forearms raised in front of the waist, elbows bent at about 95 degrees, hands relaxed with the palms turned towards each other in the lower signing space, head up towards the conversation partner.", "Beide Unterarme vor der Taille angehoben, Ellbogen etwa 95 Grad gebeugt, die Hände locker mit einander zugewandten Handflächen im unteren Gebärdenraum, der Kopf zum Gesprächspartner erhoben.")],
  "sign-house": [t("Sign HOUSE (roof)", "Gebärde HAUS (Dach)"), t("Two flat hands meet with the fingertips in front of the chest and form a roof, palms down and inwards (DGS HAUS, ASL HOUSE, first part).", "Zwei flache Hände treffen sich mit den Fingerspitzen vor der Brust und bilden ein Dach, Handflächen nach unten und innen (DGS HAUS, ASL HOUSE, erster Teil).")],
  "sign-thank-you": [t("Sign THANK YOU (at the chin)", "Gebärde DANKE (am Kinn)"), t("The flat right hand touches the chin with the fingertips, palm towards the face; the left hand waits in the signing space (DGS DANKE, ASL THANK-YOU, start).", "Die flache rechte Hand berührt mit den Fingerspitzen das Kinn, Handfläche zum Gesicht; die linke Hand wartet im Gebärdenraum (DGS DANKE, ASL THANK-YOU, Beginn).")],
  "sign-you": [t("Sign YOU (pointing)", "Gebärde DU (Zeigen)"), t("The right index finger points ahead at the conversation partner, the other fingers closed, palm down; the left hand waits in the signing space.", "Der rechte Zeigefinger zeigt nach vorn auf den Gesprächspartner, die übrigen Finger geschlossen, Handfläche nach unten; die linke Hand wartet im Gebärdenraum.")],
  "sign-house-walls": [t("Sign HOUSE (walls)", "Gebärde HAUS (Wände)"), t("Key pose: the flat hands part and move down, fingers up and palms facing, drawing the walls under the roof.", "Schlüsselpose: Die flachen Hände lösen sich und bewegen sich nach unten, Finger nach oben und Handflächen zueinander, und zeichnen die Wände unter dem Dach.")],
  "sign-thank-you-out": [t("Sign THANK YOU (forward)", "Gebärde DANKE (nach vorn)"), t("Key pose: the flat right hand has moved from the chin forward and down towards the partner, palm up.", "Schlüsselpose: Die flache rechte Hand hat sich vom Kinn nach vorn und unten zum Gegenüber bewegt, Handfläche nach oben.")],
};

/** 🏷️ Applies identity, texts, activity notes and decisions. */
export function signTexts(a: any): void {
  a.title = t("Person Signing", "Gebärdende Person");
  a.summary = t(
    "An adult of 1.69 m in a fleece hoodie, jeans and canvas sneakers communicating in sign language: hand shapes with articulated fingers and the signs HOUSE (the roof and walls drawn by two flat hands), THANK YOU (from the chin forward) and YOU (pointing at the partner) in the signing space in front of the body, a looping signed phrase and a relaxed waiting stance; about 1.5 met.",
    "Eine erwachsene Person von 1,69 m in Fleece-Kapuzenpulli, Jeans und Stoffsneakern, die in Gebärdensprache kommuniziert: Handformen mit gegliederten Fingern und die Gebärden HAUS (Dach und Wände mit zwei flachen Händen), DANKE (vom Kinn nach vorn) und DU (Zeigen auf das Gegenüber) im Gebärdenraum vor dem Körper, eine gebärdete Phrase als Schleife und ein entspannter Wartestand; etwa 1,5 met.",
  );
  a.tags = ["human", "adult", "standing", "sign-language", "deaf", "communication", "gesture", "hands", "accessibility"];
  a.provenance.author = "Semio staffage human worker HW-15 (derived from standing-adult)";
  a.provenance.created = "2026-11-22";
  a.provenance.references = a.provenance.references.filter((r: any) => !REFS.some((x) => x.id === r.id)).concat(REFS);
  a.provenance.notes = t(
    "Body, clothing, rig with fifteen finger bones per hand, outlines and volumes are derived from standing-adult (1.69 m barefoot, 59 kg, de Leva female fractions, breadths after ANSUR II and DIN 33402-2). New: six sign poses whose wrists, finger directions and palm normals are targets in the signing space after Klima and Bellugi (1979) and Boyes Braem (1995), solved by inverse kinematics of shoulder (flexion, abduction, rotation), elbow, forearm rotation and wrist within the joint limits; hand shapes relaxed, flat (B) and index point as finger offsets; the looping phrase HOUSE, THANK YOU, YOU. Signing while standing is 1.5 met (between standing quietly, 1.3 MET, and standing while talking, 1.8 MET, Ainsworth 2011).",
    "Körper, Kleidung, Skelett mit fünfzehn Fingerknochen je Hand, Umrisse und Volumen stammen von standing-adult (1,69 m barfuß, 59 kg, weibliche Anteile nach de Leva, Breiten nach ANSUR II und DIN 33402-2). Neu: sechs Gebärdenposen, deren Handgelenke, Fingerrichtungen und Handflächennormalen Ziele im Gebärdenraum nach Klima und Bellugi (1979) und Boyes Braem (1995) sind, gelöst durch inverse Kinematik von Schulter (Beugung, Abspreizung, Drehung), Ellbogen, Unterarmdrehung und Handgelenk innerhalb der Gelenkgrenzen; die Handformen locker, flach (B) und Zeigefinger als Fingerversätze; die Phrase HAUS, DANKE, DU als Schleife. Gebärden im Stehen sind 1,5 met (zwischen ruhigem Stehen, 1,3 MET, und Stehen mit Sprechen, 1,8 MET, Ainsworth 2011).",
  );
  for (const p of a.poses) {
    const [title, description] = POSES[p.id]!;
    p.title = title;
    p.description = description;
  }
  const phrase = a.clips.find((c: any) => c.id === "signing-phrase");
  phrase.title = t("Signing a phrase", "Eine Phrase gebärden");
  phrase.description = t("A 5.6-second loop in the signing space at 1.5 met: HOUSE (the flat hands form the roof, hold, part and draw the walls), back to the ready position, THANK YOU (the flat hand from the chin forward and down) and YOU (pointing at the partner); the feet stay planted, the head follows the hands.", "Eine Schleife von 5,6 Sekunden im Gebärdenraum bei 1,5 met: HAUS (die flachen Hände bilden das Dach, halten, lösen sich und zeichnen die Wände), zurück in die Bereitschaft, DANKE (die flache Hand vom Kinn nach vorn und unten) und DU (Zeigen auf das Gegenüber); die Füße bleiben stehen, der Kopf folgt den Händen.");
  phrase.phases = [
    { id: "house", fromS: 0, toS: 2.35, title: t("HOUSE", "HAUS") },
    { id: "thank-you", fromS: 2.35, toS: 3.85, title: t("THANK YOU", "DANKE") },
    { id: "you", fromS: 3.85, toS: 5.6, title: t("YOU", "DU") },
  ];
  phrase.events = [
    { t: 0.7, kind: "motion", title: t("Roof formed", "Dach gebildet") },
    { t: 1.75, kind: "motion", title: t("Walls drawn", "Wände gezeichnet") },
    { t: 2.95, kind: "motion", title: t("Fingertips at the chin", "Fingerspitzen am Kinn") },
    { t: 4.45, kind: "motion", title: t("Index points at the partner", "Zeigefinger zeigt auf das Gegenüber") },
  ];
  const idle = a.clips.find((c: any) => c.id === "idle-breathing");
  idle.description = t("A five-second loop at about 1.2 met while watching the partner sign: the chest rises and falls with quiet breathing, the head and arms settle in a small slow sway; the feet stay planted.", "Eine Schleife von fünf Sekunden bei etwa 1,2 met, während die Person dem Gegenüber beim Gebärden zusieht: Die Brust hebt und senkt sich mit dem ruhigen Atmen, Kopf und Arme wiegen sich klein und langsam; die Füße bleiben stehen.");
  a.thermal.assumptions = a.thermal.assumptions.map((x: any) => /Standing at ease/.test(x.en) ? t("Standing at ease is 1.2 met (ISO 7730 Annex A); signing while standing is 1.5 to 1.6 met, between standing quietly (1.3 MET) and standing while talking (1.8 MET) in the compendium of Ainsworth et al. (2011), authored.", "Entspanntes Stehen sind 1,2 met (ISO 7730 Anhang A); Gebärden im Stehen 1,5 bis 1,6 met, zwischen ruhigem Stehen (1,3 MET) und Stehen mit Sprechen (1,8 MET) im Kompendium von Ainsworth et al. (2011), geschätzt.") : x);
  Object.assign(a.dimensions, { widthM: 0.565, depthM: 0.555 });

  const rep = a.representations;
  const swap = (scale: string, key: string, from: [string, string], to: [string, string]) => {
    const d = rep[scale].decisions[key];
    if (!d.en.includes(from[0]) || !d.de.includes(from[1])) throw new Error(`[DEBUG] ${scale} ${key}: ${from[0]}`);
    d.en = d.en.replace(from[0], to[0]);
    d.de = d.de.replace(from[1], to[1]);
  };
  swap("50", "silhouette", ["in the relaxed pose", "in der entspannten Pose"], ["in the ready-to-sign pose", "in der gebärdenbereiten Pose"]);
  swap("50", "silhouette", ["the hands are seen edge-on", "die Hände sind von der Kante gesehen"], ["the hands are held in front of the waist", "die Hände werden vor der Taille gehalten"]);
  swap("50", "silhouette", ["hands, forearms and sleeves beside the sweatshirt", "Hände, Unterarme und Ärmel neben dem Sweatshirt"], ["hands, forearms and sleeves reaching forward from the sweatshirt", "Hände, Unterarme und Ärmel vor dem Sweatshirt nach vorn reichend"]);
  swap("500", "silhouette", ["with both arms hanging beside the trunk", "mit beiden Armen neben dem Rumpf hängend"], ["with both forearms raised in front of the waist", "mit beiden vor der Taille angehobenen Unterarmen"]);
  swap("500", "retained", ["The relaxed vertical stance, the near arm that gives the figure its width", "Die entspannte senkrechte Haltung, der nahe Arm, der der Figur ihre Breite gibt"], ["The upright stance, the forearms and hands raised forward that give the figure its reach and read as signing", "Die aufrechte Haltung, die nach vorn angehobenen Unterarme und Hände, die der Figur ihre Reichweite geben und als Gebärden lesbar sind"]);
  rep["200"].decisions.retained.en = rep["200"].decisions.retained.en.replace("so the pose changes read", "so the changes between the signs read");
  rep["200"].decisions.retained.de = rep["200"].decisions.retained.de.replace("damit die Posenwechsel lesbar bleiben", "damit die Wechsel zwischen den Gebärden lesbar bleiben");
  for (const scale of ["50", "100", "200", "500"]) {
    const d = rep[scale].decisions.overlays;
    d.en += " The hands keep their own skin region in every sign pose.";
    d.de += " Die Hände behalten in jeder Gebärdenpose ihre eigene Hautregion.";
  }
}
