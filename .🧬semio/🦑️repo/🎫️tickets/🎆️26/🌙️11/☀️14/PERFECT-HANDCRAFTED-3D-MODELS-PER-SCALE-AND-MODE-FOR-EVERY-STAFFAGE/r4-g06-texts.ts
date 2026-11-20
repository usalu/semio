/** 📝 The bilingual structure decisions of the round 4 review group G06 (`humans/walking-commuter`, `humans/jogger`, `humans/running-child`): one sentence per scale for the shared axial skeleton, with the figure's own limbs and loads appended. Run as a script it writes the four sentences into `r3-walking-commuter-texts.json`.
 * @see ./r4-g06-skeleton.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export type Text = { en: string; de: string };
export type Scale = 50 | 100 | 200 | 500;

const AXIAL: Record<Scale, Text> = {
  50: {
    en: "skull with facial bones and a lower jaw of two rami and a body; cervical spine; the vertebral column from the sacrum to the neck with lumbar lordosis and thoracic kyphosis and a spinous process on every vertebra from L5 to T1; twelve ribs per side, each a bar from its head at the vertebra round the flank and down to the front, the cartilages of the upper seven rising to the breastbone, those of ribs eight to ten joining the costal arch, eleven and twelve ending free; breastbone with manubrium, body and xiphoid; S-curved collar bones; shoulder blades as thin blades on the back of the ribs with spine, acromion, socket and coracoid; sacrum with coccyx, hip bones with wings up to the crests, pubic rami meeting at the symphysis, ischia with tuberosities and the lower rami that close the obturator openings",
    de: "Schädel mit Gesichtsschädel und einem Unterkiefer aus zwei Ästen und Körper; Halswirbelsäule; die Wirbelsäule vom Kreuzbein bis zum Hals mit Lendenlordose und Brustkyphose und einem Dornfortsatz an jedem Wirbel von L5 bis T1; zwölf Rippen je Seite, jede ein Stab vom Köpfchen am Wirbel um die Flanke nach vorn unten, die Knorpel der oberen sieben steigen zum Brustbein, die der achten bis zehnten Rippe münden in den Rippenbogen, die elfte und zwölfte enden frei; Brustbein mit Handgriff, Körper und Schwertfortsatz; S-förmig geschwungene Schlüsselbeine; Schulterblätter als dünne Blätter hinten auf den Rippen mit Gräte, Schulterhöhe, Gelenkpfanne und Rabenschnabelfortsatz; Kreuzbein mit Steißbein, Hüftbeine mit Schaufeln bis zu den Darmbeinkämmen, Schambeinäste, die sich in der Schambeinfuge treffen, Sitzbeine mit Sitzbeinhöckern und die unteren Äste, die die Hüftlöcher schließen",
  },
  100: {
    en: "skull with face and lower jaw, cervical spine, the column with both curves, six rib pairs (first, third, fifth, seventh, ninth, eleventh) as sloping bars with their cartilages, breastbone, collar bones, shoulder blades with spine and acromion, sacrum, hip bones with wings, pubic and ischial rami with tuberosities",
    de: "Schädel mit Gesicht und Unterkiefer, Halswirbelsäule, die Wirbelsäule mit beiden Krümmungen, sechs Rippenpaare (erste, dritte, fünfte, siebte, neunte, elfte) als geneigte Stäbe mit ihren Knorpeln, Brustbein, Schlüsselbeine, Schulterblätter mit Gräte und Schulterhöhe, Kreuzbein, Hüftbeine mit Schaufeln, Scham- und Sitzbeinäste mit Sitzbeinhöckern",
  },
  200: {
    en: "skull, cervical spine, the column with both curves, the rib pairs the 30 mm bar leaves room for (second, fourth and seventh on an adult, second and sixth on a child) as sloping bars, breastbone, collar bones, shoulder blades as blades, sacrum, hip bones with wings, pubic and ischial bars",
    de: "Schädel, Halswirbelsäule, die Wirbelsäule mit beiden Krümmungen, so viele Rippenpaare, wie der 30-mm-Stab zulässt (zweite, vierte und siebte beim Erwachsenen, zweite und sechste beim Kind), als geneigte Stäbe, Brustbein, Schlüsselbeine, Schulterblätter als Blätter, Kreuzbein, Hüftbeine mit Schaufeln, Scham- und Sitzbeinstäbe",
  },
  500: {
    en: "the skull, the column as a slender curved rod at the back, the rib cage as one egg that is narrow at the first rib and widest at the eighth, the pelvis as one basin from the crests to the ischia, and the long bones as slender rods of their real thickness (C2), so that the skeleton never swells into the body",
    de: "der Schädel, die Wirbelsäule als schlanker, gekrümmter Stab am Rücken, der Brustkorb als ein Ei, schmal an der ersten und am breitesten an der achten Rippe, das Becken als eine Schale von den Darmbeinkämmen bis zu den Sitzbeinen, und die Röhrenknochen als schlanke Stäbe in ihrer wirklichen Dicke (C2), damit das Skelett nie zum Körper anschwillt",
  },
};
const LIMBS: Record<Scale, Text> = {
  50: {
    en: "femora with head, neck, greater trochanter, bowed shaft and condyles, patellae, tibiae and fibulae, heel, midfoot and toe bones, humeri, radius and ulna, carpus, metacarpus and the bones of every finger. Joint solids sit at the rig joints, masses at the centres of mass",
    de: "Oberschenkelknochen mit Kopf, Hals, großem Rollhügel, gebogenem Schaft und Gelenkrollen, Kniescheiben, Schien- und Wadenbeine, Fersen-, Mittelfuß- und Zehenknochen, Oberarmknochen, Speiche und Elle, Handwurzel, Mittelhand und die Knochen jedes Fingers. Gelenkkörper sitzen an den Gelenken des Rigs, Massen in den Schwerpunkten",
  },
  100: {
    en: "each long bone one form with its joint ends, fibulae, one foot and one toe form, radius and ulna, hand forms; joints at the rig joints and masses at the centres of mass",
    de: "jeder Röhrenknochen eine Form mit seinen Gelenkenden, Wadenbeine, je eine Fuß- und eine Zehenform, Speiche und Elle, Handformen; Gelenke an den Gelenken des Rigs und Massen in den Schwerpunkten",
  },
  200: {
    en: "one bone per limb segment, foot and toe forms, hands as single forms; joints at the large joints and masses at the centres of mass",
    de: "ein Knochen je Gliedmaßenabschnitt, Fuß- und Zehenformen, Hände als einzelne Formen; Gelenke an den großen Gelenken und Massen in den Schwerpunkten",
  },
  500: { en: "masses at the centres of the heaviest segments", de: "Massen in den Schwerpunkten der schwersten Segmente" },
};

/** 🧾 The structure decision of one scale: `who` names the figure, `load` adds what it carries (empty for none). */
export function structureText(scale: Scale, who: Text, load: Text = { en: "", de: "" }): Text {
  return {
    en: `Structure 1:${scale}: the skeleton of ${who.en} as the load path, never the body: ${AXIAL[scale].en}; ${LIMBS[scale].en}.${load.en === "" ? "" : ` ${load.en}`}`,
    de: `Tragwerk 1:${scale}: das Skelett ${who.de} als Lastpfad, nie der Körper: ${AXIAL[scale].de}; ${LIMBS[scale].de}.${load.de === "" ? "" : ` ${load.de}`}`,
  };
}

if (import.meta.main) {
  const file = join(import.meta.dir, "r3-walking-commuter-texts.json");
  const texts = JSON.parse(readFileSync(file, "utf8"));
  const who: Text = { en: "the commuter", de: "des Pendlers" };
  const load: Text = { en: "The bag hangs on its strap, which is the member of the bag bone by reference, and loads it with a block at its centre of gravity; the asset declares no structural supports, so there is no support solid.", de: "Die Tasche hängt an ihrem Riemen, der per Verweis das Glied des Taschenknochens ist, und belastet ihn mit einem Block in ihrem Schwerpunkt; das Objekt nennt keine Auflager, daher gibt es keinen Auflagerkörper." };
  for (const scale of [50, 100, 200, 500] as const) texts.volumes[String(scale)][2] = structureText(scale, who, load);
  texts.representations["500"].merged = { en: "Forefoot and heel merge into one shoe, the neck into scarf and head, fingers into the hands, the right fist on the strap into its forearm.", de: "Vorfuß und Ferse gehen in einem Schuh auf, der Hals in Schal und Kopf, die Finger in den Händen, die rechte Faust am Riemen in ihrem Unterarm." };
  texts.representations["500"].omitted = { en: "Lapels, collar, buttons, pockets, face, the fringe of the hair and the bag flap are left out.", de: "Revers, Kragen, Knöpfe, Taschen, Gesicht, die Stirnhaare und die Taschenklappe entfallen." };
  writeFileSync(file, `${JSON.stringify(texts, null, 1)}\n`);
  console.log("written", file);
}
