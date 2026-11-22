/** 📝️ Identity, poses, clips, thermal and structural model and the per-scale decisions of `standing-violinist`.
 * Sources: Ainsworth et al. (2011) Compendium of Physical Activities, code 10120 (violin, 2.5 MET); Askenfelt (1989) bow force 0.5 to 2 N;
 * ISO 9920 clothing, ISO 7730 / ISO 8996 metabolic rates; the body is the archive's standing-adult (de Leva 1996, ANSUR II, DIN 33402-2). */
import { at, dot, evalOffsets, frameOf, rd, sub, T, type Any, type V3 } from "./lib.ts";
import { JAW } from "./hold.ts";
import type { Edits } from "./decisions.ts";

const featureOf = (scale: string): number => (0.15 * Number(scale)) / 1000;

/** 🦴️ Structure solids of the props at one scale: slender members along violin and bow, joints that reach the skeleton, masses at the centres of mass. */
function propStructure(a: Any, scale: string, state: Any): Any[] {
  const f = featureOf(scale);
  const r = Math.max(0.012, f / 2 + 0.002);
  const sec = Math.max(0.03, f + 0.002);
  const V = frameOf(state, "violin"), Bw = frameOf(state, "bow");
  const local = (F: ReturnType<typeof frameOf>, p: V3): number[] => { const d = sub(p, F.o); return [rd(dot(d, F.u), 5), rd(dot(d, F.v), 5), rd(dot(d, F.w), 5)]; };
  const H = frameOf(state, "head"), C = frameOf(state, "clavicle-l");
  const jaw = at(H, JAW[0], JAW[1] + 0.02, JAW[2] - 0.01);
  const clavicle = at(C, 0.09, 0, 0);
  const hand = scale === "500" ? [state.bones["forearm-r"].tip.x, state.bones["forearm-r"].tip.y, state.bones["forearm-r"].tip.z] as V3 : at(frameOf(state, "hand-r"), 0.05, 0, 0);
  const sol = (id: string, bone: string, member: string, part: string, body: Any) => ({ id, bone, material: "bone", role: "body", tone: 0.6, ...body, member, part });
  return [
    sol("violin-member", "violin", "violin", "member", { kind: "box", size: [0.593, sec, sec], transform: { translation: [0.2965, 0, 0] }, slender: true }),
    sol("joint-chin", "violin", "violin", "joint", { kind: "capsule", p0: [0.04, 0.03, 0.04], p1: local(V, jaw), r0: r, r1: r }),
    sol("joint-collarbone", "violin", "violin", "joint", { kind: "capsule", p0: [0.1, -0.02, 0.0], p1: local(V, clavicle), r0: r, r1: r }),
    sol("mass-violin", "violin", "violin", "mass", { kind: "ellipsoid", radii: [r * 1.4, r * 1.4, r * 1.4], transform: { translation: [0.19, 0, 0] } }),
    sol("bow-member", "bow", "bow", "member", { kind: "box", size: [0.75, Math.max(0.012, f * 0.5), Math.max(0.012, f * 0.5)], transform: { translation: [0.375, 0.006, 0] }, slender: true }),
    sol("joint-grip", "bow", "bow", "joint", { kind: "capsule", p0: [0.05, 0.004, 0], p1: local(Bw, hand), r0: r, r1: r }),
    sol("mass-bow", "bow", "bow", "mass", { kind: "ellipsoid", radii: [r, r, r], transform: { translation: [0.25, 0.006, 0] } }),
  ];
}

/** ✍️ Writes everything but the prisms into the asset. */
export function violinistTexts(a: Any, p: { mid: Any; frog: Any; tip: Any; frogE: Any; tipG: Any; strokes: Any[]; playBase: Record<string, Any>; rest: Record<string, Any> }): void {
  a.title = T("Standing Violinist", "Stehende Geigerin");
  a.summary = T(
    "A street violinist of 1.69 m in a hooded sweatshirt, jeans and canvas sneakers playing a full-size violin standing: the violin rests on the left collarbone under the chin, the left hand stops the strings in first position and the right arm draws a 750 mm bow across the A string at about 2.5 met. Poses for mid-bow, the frog on the E string, the tip on the G string and holding the violin between phrases; a looping down-bow and up-bow and a breathing loop.",
    "Eine Straßenmusikerin von 1,69 m in Kapuzensweatshirt, Jeans und Stoffsneakern spielt im Stehen eine ganze Geige: Die Geige liegt unter dem Kinn auf dem linken Schlüsselbein, die linke Hand greift in der ersten Lage und der rechte Arm streicht einen 750 mm langen Bogen über die A-Saite, bei etwa 2,5 met. Posen für Bogenmitte, Frosch auf der E-Saite, Spitze auf der G-Saite und das Halten der Geige zwischen zwei Phrasen; eine Schleife aus Ab- und Aufstrich und eine Atemschleife.",
  );
  a.tags = ["human", "adult", "standing", "music", "violin", "street-musician", "culture", "bowing", "public-space"];
  a.provenance.author = "Semio staffage human worker HW-12";
  a.provenance.created = "2026-11-22";
  a.provenance.references.push(
    { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; code 10120, playing the violin, 2.5 MET." },
    { id: "askenfelt-1989", citation: "Askenfelt, A. (1989). Measurement of the bowing parameters in violin playing. II: Bow-bridge distance, dynamic range, and limits of bow force. Journal of the Acoustical Society of America 86(2), 503-516." },
    { id: "violin-dimensions", citation: "Full-size (4/4) violin of the usual pattern: overall length about 593 mm, body length 356 mm, bout widths 168 / 112 / 208 mm, rib height about 31 mm, mass about 400 to 480 g with chin rest; see e.g. Heron-Allen, E. (1884). Violin-Making as It Was and Is, Ward, Lock and Co." },
    { id: "bow-dimensions", citation: "Violin bow of the usual Tourte pattern: length about 750 mm, mass about 60 g, about 150 to 200 hairs in a ribbon of 10 to 12 mm; see e.g. Fletcher, N. H. and Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed., Springer, chapter 10." },
    { id: "iso-8996-2021", citation: "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate." },
  );
  a.provenance.notes = T(
    "Body, rig, clothing and every drawn and modelled part of the person are those of the archive's standing-adult (1.69 m, 59 kg slim adult after de Leva, ANSUR II and DIN 33402-2); the violin and bow are new handcrafted parts at all four scales, drawn as the true projections of their own 3D prisms. The hold was solved through the library's evaluate: the head turns 36 degrees to the left and rests the left jaw on the chin rest, the violin points 40 degrees to the left of ahead, rises 8 degrees and rolls 32 degrees towards the E string, the left hand holds the neck in first position and the bow hair stays on the string at right angles to it along the whole stroke. Playing 2.5 met follows the compendium value for the violin; the bow force of about 1 N lies in the measured range and is internal to the figure.",
    "Körper, Skelett, Kleidung und jeder gezeichnete und modellierte Teil der Person stammen vom Archivobjekt standing-adult (schlanke erwachsene Person von 1,69 m und 59 kg nach de Leva, ANSUR II und DIN 33402-2); Geige und Bogen sind neue, in allen vier Maßstäben von Hand gebaute Teile, gezeichnet als die wahren Projektionen ihrer eigenen 3D-Prismen. Die Haltung wurde mit evaluate der Bibliothek gelöst: Der Kopf dreht sich 36 Grad nach links und legt den linken Unterkiefer auf den Kinnhalter, die Geige zeigt 40 Grad links von vorn, steigt 8 Grad an und ist 32 Grad zur E-Saite gekippt, die linke Hand hält den Hals in der ersten Lage und das Bogenhaar bleibt über den ganzen Strich rechtwinklig auf der Saite. Die 2,5 met beim Spielen folgen dem Kompendiumswert für die Geige; die Bogenkraft von etwa 1 N liegt im gemessenen Bereich und wirkt innerhalb der Figur.",
  );

  a.anchors.push(
    { id: "chin-rest", bone: "violin", x: 0.035, y: 0.0445, z: 0.04, kind: "attachment", title: T("Jaw on the chin rest", "Unterkiefer auf dem Kinnhalter") },
    { id: "bow-frog-grip", bone: "bow", x: 0.05, y: 0.004, kind: "attachment", title: T("Right hand at the frog", "Rechte Hand am Frosch") },
    { id: "bow-tip", bone: "bow", x: 0.75, y: 0.003, kind: "attachment", title: T("Tip of the bow", "Bogenspitze") },
  );

  const t = a.thermal;
  t.environment = { ...t.environment, airTempC: 18, meanRadiantTempC: 18, skyTempC: 18, groundTempC: 18, relativeHumidity: 0.55, windMS: 0.3 };
  t.baseMet = 2.5;
  t.regions.push(
    { id: "violin", title: T("Varnished violin", "Lackierte Geige"), kind: "surface", model: "passive", material: "violin-wood", areaM2: 0.17, characteristicLengthM: 0.2, geometry: "horizontal-plate-up", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 3000, source: "authored-estimate", reference: "violin-dimensions" },
    { id: "bow", title: T("Bow stick and hair", "Bogenstange und Haar"), kind: "surface", model: "passive", material: "pernambuco-horsehair", areaM2: 0.022, characteristicLengthM: 0.008, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 900, source: "authored-estimate", reference: "bow-dimensions" },
  );
  t.respiration.patterns = [
    { id: "playing", title: T("Playing, breathing with the phrase", "Spielen, Atmen mit der Phrase"), rateBpm: 15, tidalVolumeL: 0.65, inhaleFraction: 0.35, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.11, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
    { id: "quiet-standing", title: T("Quiet standing between phrases", "Ruhiges Stehen zwischen den Phrasen"), rateBpm: 13, tidalVolumeL: 0.5, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
  ];
  t.assumptions = [
    t.assumptions[0],
    T("Clothing is a T-shirt, fleece sweatshirt, jeans, socks and canvas sneakers of about 0.7 clo after the ISO 9920 tables (1 clo = 0.155 m2K/W); the reference surroundings are a calm 18 degrees C street corner in the shade.", "Die Kleidung aus T-Shirt, Fleece-Sweatshirt, Jeans, Socken und Stoffsneakern hat etwa 0,7 clo nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W); die Bezugsumgebung ist eine ruhige, schattige Straßenecke bei 18 Grad C."),
    T("Playing the violin standing is 2.5 met after the compendium value of code 10120 (Ainsworth et al. 2011, read as ISO 8996 met); holding the violin between phrases 1.4 met (authored). The violin and bow are passive wooden surfaces at the temperature of the air; the hand, the jaw and the collarbone touch them without a modelled conduction.", "Geigenspiel im Stehen sind 2,5 met nach dem Kompendiumswert von Code 10120 (Ainsworth et al. 2011, als met nach ISO 8996 gelesen); das Halten der Geige zwischen den Phrasen 1,4 met (geschätzt). Geige und Bogen sind passive Holzoberflächen auf Lufttemperatur; Hand, Unterkiefer und Schlüsselbein berühren sie ohne modellierte Wärmeleitung."),
    T("Breathing patterns are authored after quiet adult values (ICRP 66): a little deeper while playing, through the nose.", "Die Atemmuster sind nach ruhigen Erwachsenenwerten (ICRP 66) geschätzt: beim Spielen etwas tiefer, durch die Nase."),
  ];
  a.structure.assumptions = [
    a.structure.assumptions[0],
    T("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose, the weight shared by both legs.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite, das Gewicht auf beide Beine verteilt."),
    T("The violin of 0.46 kg is a point mass rigidly held by the trunk between jaw and collarbone, the bow of 60 g a point mass in the right hand; the bow force on the string (about 1 N, Askenfelt 1989) acts between two parts of the figure and is not applied as an external load.", "Die Geige von 0,46 kg ist eine Punktmasse, die der Rumpf zwischen Unterkiefer und Schlüsselbein starr hält, der Bogen von 60 g eine Punktmasse in der rechten Hand; die Bogenkraft auf die Saite (etwa 1 N, Askenfelt 1989) wirkt zwischen zwei Teilen der Figur und wird nicht als äußere Last angesetzt."),
  ];

  const regions = (play: boolean) => ({
    "face-skin": { exposedFraction: 0.95, sunlitFraction: 0 }, hair: { exposedFraction: 1, sunlitFraction: 0 },
    "hand-skin": { exposedFraction: play ? 0.85 : 0.9, sunlitFraction: 0 }, hoodie: { exposedFraction: 0.93, sunlitFraction: 0 },
    jeans: { exposedFraction: 1, sunlitFraction: 0 }, sneakers: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.3, tempC: 18 } },
    violin: { exposedFraction: 0.9, sunlitFraction: 0 }, bow: { exposedFraction: 0.97, sunlitFraction: 0 },
  });
  const contacts = ["heel-l", "ball-l", "heel-r", "ball-r"];
  const pose = (id: string, title: Any, description: Any, bones: Any, met: number, breathing: string, play: boolean) => ({ id, title, description, bones, ground: "auto", contacts, thermal: { activityMet: met, breathing, regions: regions(play), emitters: { "body-plume": 0.6 }, operating: {} }, loads: [] });
  a.poses = [
    pose("playing", T("Playing at mid-bow", "Spielen in der Bogenmitte"), T("Standing with the feet at hip width, the violin under the chin on the left collarbone, the head turned left onto the chin rest, the left hand in first position and the bow at its middle on the A string, at right angles to it between bridge and fingerboard.", "Stehend mit hüftbreit gestellten Füßen, die Geige unter dem Kinn auf dem linken Schlüsselbein, der Kopf nach links auf den Kinnhalter gedreht, die linke Hand in der ersten Lage und der Bogen mit seiner Mitte rechtwinklig auf der A-Saite zwischen Steg und Griffbrett."), { ...p.playBase, ...p.mid }, 2.5, "playing", true),
    pose("bow-at-frog", T("Bow at the frog on the E string", "Bogen am Frosch auf der E-Saite"), T("At the start of a down-bow: the right hand close to the strings with the elbow low, the frog over the E string, the left hand still in first position.", "Am Beginn eines Abstrichs: Die rechte Hand nah an den Saiten mit tiefem Ellbogen, der Frosch über der E-Saite, die linke Hand weiter in der ersten Lage."), { ...p.playBase, ...p.frogE }, 2.5, "playing", true),
    pose("bow-at-tip", T("Bow at the tip on the G string", "Bogen an der Spitze auf der G-Saite"), T("At the end of a down-bow on the lowest string: the right arm stretched out to the right and forward, the elbow raised to the level of the G string, the tip of the bow on the string.", "Am Ende eines Abstrichs auf der tiefsten Saite: Der rechte Arm nach rechts vorn gestreckt, der Ellbogen auf die Höhe der G-Saite gehoben, die Bogenspitze auf der Saite."), { ...p.playBase, ...p.tipG }, 2.5, "playing", true),
    pose("holding-between-phrases", T("Holding the violin between phrases", "Die Geige zwischen zwei Phrasen halten"), T("The violin stays under the chin and in the left hand while the bow arm rests at the side with the bow pointing down along the leg.", "Die Geige bleibt unter dem Kinn und in der linken Hand, während der Bogenarm seitlich ruht und der Bogen am Bein entlang nach unten zeigt."), p.rest, 1.4, "quiet-standing", false),
  ];
  const span = (d: number) => contacts.map((anchor) => ({ anchor, fromS: 0, toS: d }));
  const S = [0.1, 0.24, 0.38, 0.52, 0.66];
  const order = [0, 1, 2, 3, 4, 3, 2, 1, 0];
  a.clips = [
    {
      id: "bowing", title: T("Bowing long notes", "Lange Noten streichen"),
      description: T("A 2.4-second loop on the A string: a whole down-bow from the frog to the tip in 1.2 s and the up-bow back; the hair stays on the string at right angles to it while the right arm opens and closes from the elbow and the shoulder; head, violin and left hand stay still.", "Eine Schleife von 2,4 Sekunden auf der A-Saite: ein ganzer Abstrich vom Frosch zur Spitze in 1,2 s und der Aufstrich zurück; das Haar bleibt rechtwinklig auf der Saite, während sich der rechte Arm aus Ellbogen und Schulter öffnet und schließt; Kopf, Geige und linke Hand bleiben ruhig."),
      durationS: 2.4, fps: 30, loop: true, rootMotion: "none",
      keys: order.map((i, k) => ({ t: rd(k * 0.3, 3), pose: "playing", ease: "smooth", bones: p.strokes[i] })),
      contacts: span(2.4),
      phases: [{ id: "down-bow", fromS: 0, toS: 1.2, title: T("Down-bow", "Abstrich") }, { id: "up-bow", fromS: 1.2, toS: 2.4, title: T("Up-bow", "Aufstrich") }],
      events: [{ t: 0, kind: "motion", title: T("Down-bow at the frog", "Abstrich am Frosch") }, { t: 1.2, kind: "motion", title: T("Up-bow at the tip", "Aufstrich an der Spitze") }],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 2.4, v: 0 }] }],
    },
    {
      id: "idle-breathing", title: T("Breathing between phrases", "Atmen zwischen den Phrasen"),
      description: T("A five-second loop at 1.4 met holding the violin under the chin: the chest rises and falls with quiet breathing and the lowered bow arm sways a little; the feet stay planted.", "Eine Schleife von fünf Sekunden bei 1,4 met mit der Geige unter dem Kinn: Die Brust hebt und senkt sich mit ruhigem Atmen und der gesenkte Bogenarm schwingt ein wenig; die Füße bleiben stehen."),
      durationS: 5, fps: 30, loop: true, rootMotion: "none",
      keys: [
        { t: 0, pose: "holding-between-phrases", ease: "smooth" },
        { t: 1.25, pose: "holding-between-phrases", ease: "smooth", bones: { "arm-r": { ...p.rest["arm-r"], angleDeg: p.rest["arm-r"].angleDeg + 2 }, torso: { angleDeg: -0.5 } } },
        { t: 2.5, pose: "holding-between-phrases", ease: "smooth", bones: { "arm-r": { ...p.rest["arm-r"], angleDeg: p.rest["arm-r"].angleDeg - 1 }, torso: { angleDeg: 0.4 } } },
        { t: 3.75, pose: "holding-between-phrases", ease: "smooth", bones: { "arm-r": { ...p.rest["arm-r"], angleDeg: p.rest["arm-r"].angleDeg + 1.5 }, torso: { angleDeg: -0.4 } } },
        { t: 5, pose: "holding-between-phrases", ease: "smooth" },
      ],
      contacts: span(5), phases: [{ id: "rest", fromS: 0, toS: 5, title: T("Between phrases", "Zwischen den Phrasen") }], events: [],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: 5, v: 1 }] }],
    },
  ];
  a.presentation = { defaultPose: "playing", defaultClip: "bowing", defaultScale: 50 };
  const state = evalOffsets(a, a.poses[0].bones, "playing");
  for (const scale of ["50", "100", "200", "500"]) a.volumes[scale].structure.push(...propStructure(a, scale, state));
  void S;
}

/** 🗒️ How each scale draws and models the violinist: the base figure's decisions in the playing pose, plus violin, bow and the round limbs. */
export const VIOLINIST_DECISIONS: Edits = {
  replace: [
    ["in the relaxed pose", "in the playing pose"], ["in der entspannten Pose", "in der Spielpose"],
    ["An upright figure of 1.72 m (3.4 mm)", "An upright figure of 1.72 m (3.4 mm) with the violin at the chin"], ["Eine aufrechte Figur von 1,72 m (3,4 mm)", "Eine aufrechte Figur von 1,72 m (3,4 mm) mit der Geige am Kinn"],
    ["the same masses with both arms hanging beside the trunk", "the same masses with both arms raised to violin and bow"], ["dieselben Massen mit beiden seitlich am Rumpf hängenden Armen", "dieselben Massen mit beiden zu Geige und Bogen gehobenen Armen"],
    ["The relaxed vertical stance, the near arm that gives the figure its width,", "The upright stance, the raised arms with the violin and the bow that give the figure its width,"], ["Die entspannte senkrechte Haltung, der nahe Arm, der der Figur ihre Breite gibt,", "Die aufrechte Haltung, die gehobenen Arme mit Geige und Bogen, die der Figur ihre Breite geben,"],
  ],
  append: {
    "50": {
      silhouette: T("The violin is drawn as the true projection of its own prisms: ribs between arched top and back with the figure-eight outline and its four corners, neck, ebony fingerboard, pegbox with two pairs of pegs, scroll, tailpiece, chin rest and bridge; the bow as stick, hair ribbon, frog and head. Sleeves and forearms are drawn as round bodies about their bones, so the raised arms read right in all three views.", "Die Geige ist als wahre Projektion ihrer eigenen Prismen gezeichnet: Zargen zwischen gewölbter Decke und Boden mit dem Achterumriss und seinen vier Ecken, Hals, Ebenholzgriffbrett, Wirbelkasten mit zwei Wirbelpaaren, Schnecke, Saitenhalter, Kinnhalter und Steg; der Bogen als Stange, Haarband, Frosch und Kopf. Ärmel und Unterarme sind als Rundkörper um ihre Knochen gezeichnet, damit die gehobenen Arme in allen drei Ansichten richtig lesen."),
      omitted: T("Strings, f-holes and purfling of the violin (below 1 mm wide), the screw of the frog.", "Saiten, F-Löcher und Adern der Geige (unter 1 mm breit), die Schraube des Froschs."),
      thinElements: T("The bow stick (8.6 to 5.4 mm) and the hair ribbon (11 mm) keep their true width, 0.17 and 0.22 mm on paper.", "Bogenstange (8,6 bis 5,4 mm) und Haarband (11 mm) behalten ihre wahre Breite, 0,17 und 0,22 mm auf dem Papier."),
      overlays: T("Violin and bow carry their own passive regions.", "Geige und Bogen tragen ihre eigenen passiven Regionen."),
    },
    "100": {
      silhouette: T("The violin as body, neck with fingerboard, scroll and chin rest; the bow as stick, hair and frog; sleeves and forearms as round bodies about their bones.", "Die Geige als Korpus, Hals mit Griffbrett, Schnecke und Kinnhalter; der Bogen als Stange, Haar und Frosch; Ärmel und Unterarme als Rundkörper um ihre Knochen."),
      merged: T("Ribs, top and back of the violin are one body 46 mm thick; pegs, tailpiece and bridge merge into it; the bow head into the stick.", "Zargen, Decke und Boden der Geige sind ein 46 mm starker Korpus; Wirbel, Saitenhalter und Steg gehen in ihm auf; der Bogenkopf in der Stange."),
      thinElements: T("Bow stick and hair stay at their true width (0.09 and 0.11 mm) as the reading of the bow.", "Bogenstange und Haar bleiben in ihrer wahren Breite (0,09 und 0,11 mm) als Zeichen des Bogens."),
      overlays: T("Violin and bow carry their own passive regions.", "Geige und Bogen tragen ihre eigenen passiven Regionen."),
    },
    "200": {
      silhouette: T("The violin as a body with its bouts and waist and one neck-and-scroll bar; the bow as one bar with stick and hair together; the hands as gripping fists.", "Die Geige als Korpus mit Bügeln und Taille und einem Stab aus Hals und Schnecke; der Bogen als ein Stab aus Stange und Haar; die Hände als greifende Fäuste."),
      merged: T("Chin rest, fingerboard and fittings into the violin; hair, frog and head into one bow bar.", "Kinnhalter, Griffbrett und Garnitur in der Geige; Haar, Frosch und Kopf in einem Bogenstab."),
      thinElements: T("The bow is one 24 mm bar at its true section, 0.12 mm on paper.", "Der Bogen ist ein 24 mm starker Stab in seinem wahren Querschnitt, 0,12 mm auf dem Papier."),
      overlays: T("Violin and bow carry their own passive regions.", "Geige und Bogen tragen ihre eigenen passiven Regionen."),
    },
    "500": {
      silhouette: T("The violin is one tapering wedge from the body to the scroll at the left shoulder, the bow one bar across it; the hands are gripping fists.", "Die Geige ist ein vom Korpus zur Schnecke verjüngter Keil an der linken Schulter, der Bogen ein Stab quer darüber; die Hände sind greifende Fäuste."),
      merged: T("The whole violin into one wedge 75 mm thick; the bow into one bar.", "Die ganze Geige in einem 75 mm starken Keil; der Bogen in einem Stab."),
      thinElements: T("The bow keeps its true 24 mm section as a hairline; the violin is thickened to the 75 mm feature of this scale.", "Der Bogen behält seinen wahren Querschnitt von 24 mm als Haarlinie; die Geige ist auf das 75-mm-Merkmal dieses Maßstabs verdickt."),
      overlays: T("Violin and bow carry their own passive regions.", "Geige und Bogen tragen ihre eigenen passiven Regionen."),
    },
  },
  volume: {
    "50": [
      T("Violin of the usual 4/4 pattern as extruded prisms in its own frame: ribs 31 mm high under the figure-eight outline (bouts 208, 112 and 168 mm), arched top and back as inset plates, neck, ebony fingerboard, pegbox with four pegs, scroll, tailpiece, chin rest on the bass side and bridge; bow as tapering stick, hair ribbon, frog and head, the stick and the hair declared slender.", "Geige nach dem üblichen 4/4-Modell als extrudierte Prismen im eigenen Rahmen: 31 mm hohe Zargen unter dem Achterumriss (Bügel 208, 112 und 168 mm), gewölbte Decke und Boden als eingerückte Platten, Hals, Ebenholzgriffbrett, Wirbelkasten mit vier Wirbeln, Schnecke, Saitenhalter, Kinnhalter auf der Bassseite und Steg; Bogen als verjüngte Stange, Haarband, Frosch und Kopf, Stange und Haar als schlank erklärt."),
      T("Violin and bow are passive regions on their own solids.", "Geige und Bogen sind passive Regionen auf ihren eigenen Körpern."),
      T("The violin is one slender member held by two joints, the jaw on the chin rest and the collarbone under the back; the bow one slender member gripped at the frog; masses at both centres of mass.", "Die Geige ist ein schlankes Glied, gehalten von zwei Gelenken, dem Unterkiefer auf dem Kinnhalter und dem Schlüsselbein unter dem Boden; der Bogen ein schlankes Glied, am Frosch gegriffen; Massen in beiden Schwerpunkten."),
    ],
    "100": [
      T("Violin as body, neck with fingerboard, scroll and chin rest; bow as slender stick and hair with the frog.", "Geige als Korpus, Hals mit Griffbrett, Schnecke und Kinnhalter; Bogen als schlanke Stange und Haar mit dem Frosch."),
      T("Violin and bow as their own passive regions.", "Geige und Bogen als eigene passive Regionen."),
      T("Violin and bow as slender members with their joints at jaw, collarbone and grip.", "Geige und Bogen als schlanke Glieder mit ihren Gelenken an Unterkiefer, Schlüsselbein und Griff."),
    ],
    "200": [
      T("Violin as body and neck bar, the bow as one slender bar; the hands closed to gripping fists 125 mm long.", "Geige als Korpus und Halsstab, der Bogen als ein schlanker Stab; die Hände zu greifenden Fäusten von 125 mm Länge geschlossen."),
      T("Violin and bow as their own passive regions.", "Geige und Bogen als eigene passive Regionen."),
      T("Violin and bow as slender members with their joints.", "Geige und Bogen als schlanke Glieder mit ihren Gelenken."),
    ],
    "500": [
      T("Violin as one tapering wedge 75 mm thick, the bow as one slender bar; the hands closed to gripping fists 125 mm long.", "Geige als ein verjüngter Keil von 75 mm Stärke, der Bogen als ein schlanker Stab; die Hände zu greifenden Fäusten von 125 mm Länge geschlossen."),
      T("Violin and bow as their own passive regions.", "Geige und Bogen als eigene passive Regionen."),
      T("Violin and bow as slender members with their joints.", "Geige und Bogen als schlanke Glieder mit ihren Gelenken."),
    ],
  },
};
