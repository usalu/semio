import { Ctx, auto, buildStructure, buildThermal, decisions, emitters, fillMissing, finish, type Note, type ScaleOut } from "./author-06-lib.ts";

const notes: Record<number, Note[]> = {
  50: [
    {
      en: "Body at 1:50: every drawn shape is a loft along its bone whose sections take height and centre from the elevation outline and width and offset from the plan outline, so the barrel is wider than high behind the shoulder, the ham and the shoulder are separate masses, the snout is a flat disc at the end of a wedge-shaped head and the ears are flat flaps 0.12 m wide. Near and far legs are separate lofts on their own bones with their own lateral offsets; hooves are short wedge lofts. Eye spheres, mouth and jaw are modelled; the curly tail is two lofts.",
      de: "Körper bei 1:50: Jede gezeichnete Form ist ein Loft entlang ihres Knochens, dessen Querschnitte Höhe und Mitte aus der Ansicht und Breite und Versatz aus dem Grundriss nehmen; so ist die Tonne hinter der Schulter breiter als hoch, Schinken und Schulter sind getrennte Massen, der Rüssel ist eine flache Scheibe am Ende eines keilförmigen Kopfes und die Ohren sind flache Lappen von 0,12 m Breite. Nahe und ferne Beine sind eigene Lofts auf ihren Knochen mit eigenem seitlichem Versatz; Klauen sind kurze Keillofts. Augen, Maul und Kiefer sind modelliert; der Ringelschwanz besteht aus zwei Lofts.",
    },
  ],
  100: [
    {
      en: "Body at 1:100: modelled anew against the 1:100 drawings, which merge the eye and mouth lines and the tail root into the head and the hindquarter; the 1:100 list has no eye, mouth line or jaw solid, the legs keep their four segments per side but with 13 instead of 18 sections per loft and cruder hoof wedges. Cross-scale tolerances are met by the same bones, proportions and materials, not by copying the 1:50 solids.",
      de: "Körper bei 1:100: neu gegen die 1:100-Zeichnungen modelliert, die Augen- und Maullinien und die Schwanzwurzel in Kopf und Hinterhand verschmelzen; die 1:100-Liste hat keinen Augen-, Maullinien- oder Kieferkörper, die Beine behalten ihre vier Segmente je Seite, aber mit 13 statt 18 Querschnitten je Loft und gröberen Klauenkeilen. Die Massstabsübergänge werden durch dieselben Knochen, Proportionen und Materialien erreicht, nicht durch Kopieren der 1:50-Körper.",
    },
  ],
  200: [
    {
      en: "Body at 1:200: the drawings give one merged ear and a far ear as a separate ellipse, no snout, no tail shape: the head is one wedge loft, the barrel is two overlapping lofts (hindbody, chest), each leg keeps its gaskin, cannon and hoof as three tapered lofts of 9 sections at most. The far ear exists only in the front and plan drawings and is a single ellipsoid.",
      de: "Körper bei 1:200: Die Zeichnungen geben ein verschmolzenes Ohr und ein fernes Ohr als eigene Ellipse, keinen Rüssel, keine Schwanzform: der Kopf ist ein Keilloft, die Tonne besteht aus zwei sich überlappenden Lofts (Hinterhand, Brust), jedes Bein behält Unterschenkel, Mittelfuss und Klaue als drei verjüngte Lofts mit höchstens 9 Querschnitten. Das ferne Ohr kommt nur in Vorder- und Draufsicht vor und ist ein einzelnes Ellipsoid.",
    },
  ],
  500: [
    {
      en: "Body at 1:500: two barrel masses, a neck wedge and one head wedge plus eight leg tubes of the minimal printable width (0.1 m, the stroke width drawn at 1:500): the near legs follow the strokes in the elevation, the far legs exist in the front and plan drawings and are tubes along their own bones, so in the elevation they stand beside the near legs. Everything else (ears, snout, tail, hooves) is merged into the masses.",
      de: "Körper bei 1:500: zwei Tonnenmassen, ein Halskeil und ein Kopfkeil sowie acht Beinröhren der kleinsten druckbaren Breite (0,1 m, die in 1:500 gezeichnete Strichbreite): die nahen Beine folgen den Strichen der Ansicht, die fernen Beine kommen in Vorder- und Draufsicht vor und sind Röhren entlang ihrer eigenen Knochen und stehen daher in der Ansicht neben den nahen Beinen. Alles übrige (Ohren, Rüssel, Schwanz, Klauen) ist in die Massen verschmolzen.",
    },
  ],
};

const out: Record<number, ScaleOut> = {};
const ear = { p: 0.1, ex: 2.5, fw: 0.5 };
for (const scale of [50, 100, 200, 500]) {
  const c = new Ctx("pig", scale);
  auto(c, { over: { "ear-n": ear, "ear-f": ear, ear: ear, "ear-far": ear } });
  const extra = emitters(c, { "breath-nose": "snout", "breath-mouth": "skin-head" });
  const thermal = buildThermal(c, extra);
  fillMissing(c, thermal);
  const structure = buildStructure(c, { spine: ["pelvis", "lumbar", "thorax"], skipJoints: ["ear-n", "ear-f"] });
  out[scale] = { body: c.body, thermal, structure, decisions: decisions(c, notes[scale]!, thermal, structure) };
}
finish("pig", out);
