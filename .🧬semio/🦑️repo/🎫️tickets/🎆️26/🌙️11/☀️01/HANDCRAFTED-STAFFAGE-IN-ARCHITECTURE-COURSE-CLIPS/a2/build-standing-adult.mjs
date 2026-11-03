import fs from "node:fs";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const doc = JSON.parse(fs.readFileSync(`${S}/standing-phone-user/🔣️.json`, "utf8"));
const a = doc.asset;

const T = (en, de) => ({ en, de });
const DROP_SHAPE = /^(phone|phone-screen|headphone-.*|headphones|ponytail|pony-tie|sil-pony)$/;
const DROP_REGION = new Set(["headphones", "phone"]);

a.id = "standing-adult";
a.dimensions.widthM = 0.328;
a.dimensions.heightM = 1.69;
a.title = T("Standing Adult", "Stehende erwachsene Person");
a.summary = T(
  "A neutral adult of 1.67 m standing relaxed in a fleece sweatshirt, jeans and canvas sneakers, the archive's generic scale figure. Four poses (relaxed, arms crossed, pointing, hands warming) and three loops (idle breathing, shivering, fanning) show comfort, cold and heat at about 1.2 met in normal indoor clothing.",
  "Eine neutrale erwachsene Person von 1,67 m, entspannt stehend in Fleece-Sweatshirt, Jeans und Stoffsneakern, die generische Maßstabsfigur des Archivs. Vier Posen (entspannt, Arme verschränkt, zeigend, Hände wärmend) und drei Schleifen (ruhiges Atmen, Zittern, Fächeln) zeigen Behaglichkeit, Kälte und Hitze bei etwa 1,2 met in normaler Innenraumkleidung."
);
a.tags = ["human", "adult", "standing", "scale-figure", "neutral", "thermal-comfort", "shivering", "fanning", "pointing"];
a.provenance.author = "Semio staffage content worker A2";
a.provenance.created = "2026-11-01";
a.provenance.notes = T(
  "Rig and outlines derived by hand from the archive's standing-phone-user asset at 1.67 m (de Leva segment fractions); phone, headphones and ponytail removed so the figure is a neutral scale figure; poses and clips handcrafted. Breathing patterns for shivering and fanning and the activity of fanning are authored estimates; shivering heat itself is not authored but follows from the two-node model (Gagge et al. 1971) when the surroundings are cold.",
  "Skelett und Umrisse von Hand aus dem Archivobjekt standing-phone-user bei 1,67 m abgeleitet (de-Leva-Segmentanteile); Telefon, Kopfhörer und Pferdeschwanz entfernt, damit die Figur eine neutrale Maßstabsfigur ist; Posen und Clips von Hand gezeichnet. Atemmuster für Zittern und Fächeln sowie die Aktivität beim Fächeln sind Schätzwerte; die Zitterwärme selbst wird nicht vorgegeben, sondern folgt aus dem Zwei-Knoten-Modell (Gagge et al. 1971), wenn die Umgebung kalt ist."
);

a.rig.bones = a.rig.bones.filter((b) => b.id !== "phone");
const mass = a.rig.bones.reduce((s, b) => s + (b.mass?.kg ?? 0), 0);
a.dimensions.massKg = Math.round(mass * 100) / 100;
a.anchors = a.anchors.filter((x) => !["phone-cog", "grip-l"].includes(x.id));
a.materials = a.materials.filter((m) => !["headphone-polymer", "phone-glass-metal"].includes(m.id));

for (const rep of Object.values(a.representations)) {
  const g = rep.geometry;
  for (const k of Object.keys(g)) {
    if (!Array.isArray(g[k])) continue;
    g[k] = g[k].filter((s) => !DROP_SHAPE.test(s.id));
    for (const s of g[k]) {
      if (s.regions) {
        s.regions = s.regions.filter((r) => !DROP_REGION.has(r));
        if (s.regions.length === 1) { s.region = s.regions[0]; delete s.regions; }
      }
    }
  }
}

const dec = a.representations["500"].decisions;
dec.silhouette = T(
  "An upright profile of 1.67 m (3.3 mm): head with hair as one mass, torso with hood as one mass, the near arm as three tapered links hanging at the side, two leg columns with shoes.",
  "Ein aufrechtes Profil von 1,67 m (3,3 mm): Kopf mit Haar als eine Masse, Rumpf mit Kapuze als eine Masse, der nahe Arm als drei verjüngte Glieder seitlich hängend, zwei Beinsäulen mit Schuhen."
);
dec.retained = T(
  "The relaxed vertical stance, the near arm that gives the figure its width, the step of the feet and the exact stature and extent.",
  "Die entspannte senkrechte Haltung, der nahe Arm, der der Figur ihre Breite gibt, die Fußstellung sowie genaue Körpergröße und Ausdehnung."
);
dec.merged = T("Face and hair; hoodie in one torso shape; hand merged into the forearm end.", "Gesicht und Haar; Sweatshirt in einer Rumpfform; Hand im Unterarmende verschmolzen.");
dec.thinElements = T(
  "The 4 mm drawstrings and collar rib are not drawn, never thickened.",
  "Die 4 mm breiten Kordeln und der Kragenbund werden nicht gezeichnet, nie verdickt."
);
dec.overlays = T(
  "Thermal and structural overlays paint the torso shape with the hoodie value; head, hand, legs and shoes keep their own value.",
  "Thermische und statische Überlagerungen färben die Rumpfform mit dem Wert des Sweatshirts; Kopf, Hand, Beine und Schuhe behalten ihren eigenen Wert."
);
for (const k of ["50", "100", "200"]) {
  const d = a.representations[k].decisions;
  for (const f of Object.keys(d)) {
    d[f] = {
      en: d[f].en.replace(/,? ?(and )?(the )?(headphones?( band| cups?| cushions?)?|ponytail|phone( screen)?)/gi, "").replace(/\s+,/g, ","),
      de: d[f].de.replace(/,? ?(und )?(die |den |der |das )?(Kopfhörer(bügel|muscheln|polster)?|Pferdeschwanz|Telefon|Handy)/gi, "").replace(/\s+,/g, ","),
    };
  }
}

const t = a.thermal;
t.environment = { airTempC: 21, meanRadiantTempC: 21, skyTempC: 21, groundTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
t.baseMet = 1.2;
t.body.reference = "mifflin-1990";
t.operating = [];
t.regions = t.regions.filter((r) => !DROP_REGION.has(r.id));
const hair = t.regions.find((r) => r.id === "hair");
hair.title = T("Short hair", "Kurzes Haar");
hair.layers[0].title = T("Short hair", "Kurzes Haar");
hair.layers[0].thicknessM = 0.02;
hair.layers[0].resistanceM2KW = 0.06;
t.regions.find((r) => r.id === "hoodie").layers[1].title = T("Fleece sweatshirt, hood down", "Fleece-Sweatshirt, Kapuze unten");
t.respiration.patterns = [
  { id: "quiet-standing", title: T("Quiet standing", "Ruhiges Stehen"), rateBpm: 14, tidalVolumeL: 0.4, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "talking", title: T("Talking while explaining", "Sprechen beim Erklären"), rateBpm: 17, tidalVolumeL: 0.55, inhaleFraction: 0.25, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.68, pauseAfterExhaleFraction: 0.05, route: "nose-and-mouth", source: "authored-estimate", reference: "gupta-2010" },
  { id: "shivering", title: T("Shivering, quick and shallow", "Zittern, schnell und flach"), rateBpm: 20, tidalVolumeL: 0.45, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.08, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  { id: "warm", title: T("Warm, deeper and faster", "Warm, tiefer und schneller"), rateBpm: 18, tidalVolumeL: 0.55, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.54, pauseAfterExhaleFraction: 0.04, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
];
t.assumptions = [
  T("Skin area 1.675 m2 after DuBois and DuBois (1916) for 58 kg and 1.69 m; regions carry skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).",
    "Hautfläche 1.675 m2 nach DuBois und DuBois (1916) für 58 kg und 1,69 m; die Regionen tragen die Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
  T("Clothing is normal indoor wear of about 0.7 clo (T-shirt, fleece sweatshirt, jeans, socks and sneakers) after the ISO 9920 tables (1 clo = 0.155 m2K/W); vapour resistance R/(0.0165 K/Pa x 0.38). The default surroundings are 21 degrees C still air, the neutral case.",
    "Die Kleidung ist normale Innenraumkleidung von etwa 0,7 clo (T-Shirt, Fleece-Sweatshirt, Jeans, Socken und Sneaker) nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W); Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Standardumgebung ist ruhige Luft von 21 Grad C, der neutrale Fall."),
  T("Standing at ease is 1.2 met (ISO 7730 Annex A), talking and explaining 1.3 met, fanning with a hand 1.4 met (authored). Shivering is not a bound activity: the two-node model adds shivering heat when skin and core are cold, so the shiver clip only becomes warm in a cold environment preset.",
    "Entspanntes Stehen sind 1,2 met (ISO 7730 Anhang A), Sprechen und Erklären 1,3 met, Fächeln mit der Hand 1,4 met (geschätzt). Zittern ist keine gebundene Aktivität: Das Zwei-Knoten-Modell fügt Zitterwärme hinzu, wenn Haut und Kern kalt sind, die Zitter-Schleife erwärmt sich also nur in einer kalten Umgebungsvorgabe."),
  T("Breathing patterns are authored and only guided by quiet adult values (ICRP 66) and Gupta et al. (2010) for talking.",
    "Die Atemmuster sind geschätzt und nur an ruhigen Erwachsenenwerten (ICRP 66) und Gupta et al. (2010) für das Sprechen orientiert."),
];
a.structure.assumptions = [
  T("Segments are point masses at the de Leva centres (female table).", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle)."),
  T("Contacts are heel and ball of each foot; both feet stand flat and close together in every named pose.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach und eng beieinander."),
];

const legs = (s = 0) => ({
  "thigh-l": { angleDeg: 6.0 + s }, "shank-l": { angleDeg: -14.0 }, "foot-l": { angleDeg: 8.0 }, "toe-l": { angleDeg: 0 },
  "thigh-r": { angleDeg: 6.0 + s }, "shank-r": { angleDeg: -14.0 }, "foot-r": { angleDeg: 8.0 }, "toe-r": { angleDeg: 0 },
});
const regionsFor = (opts = {}) => ({
  "face-skin": { exposedFraction: 1, sunlitFraction: 0 },
  hair: { exposedFraction: 1, sunlitFraction: 0 },
  "hand-skin": { exposedFraction: opts.hands ?? 1, sunlitFraction: 0 },
  hoodie: { exposedFraction: opts.hoodie ?? 0.95, sunlitFraction: 0 },
  jeans: { exposedFraction: 1, sunlitFraction: 0 },
  sneakers: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.3, tempC: 21 } },
});
const pose = (id, title, description, bones, met, breathing, opts) => ({
  id, title, description,
  bones: { ...legs(), ...bones },
  ground: "auto",
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r"],
  thermal: { activityMet: met, breathing, regions: regionsFor(opts), emitters: { "body-plume": 0.6 }, operating: {} },
  loads: [],
});
a.poses = [
  pose("relaxed", T("Standing relaxed", "Entspannt stehen"),
    T("Feet together, weight on both legs, arms hanging loosely at the sides, head level.", "Füße beieinander, Gewicht auf beiden Beinen, Arme locker seitlich hängend, Kopf gerade."),
    { torso: { angleDeg: 0 }, neck: { angleDeg: -1 }, head: { angleDeg: 1 }, "arm-l": { angleDeg: 3 }, "forearm-l": { angleDeg: 8 }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: 5 }, "forearm-r": { angleDeg: 10 }, "hand-r": { angleDeg: 6 } },
    1.2, "quiet-standing", {}),
  pose("arms-crossed", T("Arms crossed", "Arme verschränkt"),
    T("Feet together, both forearms folded across the chest, hands tucked under the arms, shoulders slightly raised.", "Füße beieinander, beide Unterarme vor der Brust verschränkt, Hände unter den Armen, Schultern leicht hochgezogen."),
    { torso: { angleDeg: -2 }, neck: { angleDeg: -2 }, head: { angleDeg: 2 }, "arm-l": { angleDeg: 2 }, "forearm-l": { angleDeg: 98 }, "hand-l": { angleDeg: -60 }, "arm-r": { angleDeg: 6 }, "forearm-r": { angleDeg: 104 }, "hand-r": { angleDeg: -60 } },
    1.2, "quiet-standing", { hands: 0.5, hoodie: 0.85 }),
  pose("pointing", T("Pointing sideways", "Seitlich zeigen"),
    T("Weight on both legs, near arm raised and stretched out horizontally with an extended index finger, far arm resting, head turned toward the pointing direction.", "Gewicht auf beiden Beinen, der nahe Arm waagerecht ausgestreckt mit gestrecktem Zeigefinger, der ferne Arm ruht, Kopf zur Zeigerichtung gedreht."),
    { torso: { angleDeg: 1 }, neck: { angleDeg: 0 }, head: { angleDeg: 2 }, "arm-l": { angleDeg: 3 }, "forearm-l": { angleDeg: 8 }, "hand-l": { angleDeg: 4 }, "arm-r": { angleDeg: 80 }, "forearm-r": { angleDeg: 3 }, "hand-r": { angleDeg: -6 } },
    1.3, "talking", {}),
  pose("hands-warming", T("Hands warming", "Hände wärmend"),
    T("Shoulders slightly raised, both elbows bent, palms held out in front of the chest toward a heat source.", "Schultern leicht hochgezogen, beide Ellbogen gebeugt, die Handflächen vor der Brust einer Wärmequelle entgegengehalten."),
    { torso: { angleDeg: 2 }, neck: { angleDeg: 0 }, head: { angleDeg: 0 }, "arm-l": { angleDeg: 32 }, "forearm-l": { angleDeg: 78 }, "hand-l": { angleDeg: -30 }, "arm-r": { angleDeg: 36 }, "forearm-r": { angleDeg: 78 }, "hand-r": { angleDeg: -30 } },
    1.2, "quiet-standing", {}),
];

const contacts = (dur) => ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: dur }));
const k = (t0, pose_, bones, ease = "smooth") => ({ t: t0, pose: pose_, ease, ...(bones ? { bones } : {}) });

const shake = [];
{
  const n = 8, dur = 1.2;
  for (let i = 0; i <= n; i++) {
    const s = i === n ? 0 : (i % 2 === 0 ? 1 : -1);
    shake.push(k(Math.round((dur * i) / n * 1000) / 1000, "arms-crossed", i === 0 || i === n ? undefined : {
      torso: { angleDeg: -2 - 3 + 0.9 * s },
      neck: { angleDeg: -6 },
      head: { angleDeg: 3 + 1.6 * s },
      "thigh-l": { angleDeg: 7 + 1.2 * s }, "thigh-r": { angleDeg: 7 - 1.2 * s },
      "arm-l": { angleDeg: 2 + 2 * s }, "arm-r": { angleDeg: 6 - 2 * s },
      "forearm-l": { angleDeg: 98 + 3 * s }, "forearm-r": { angleDeg: 104 - 3 * s },
    }, "smooth"));
  }
}

a.clips = [
  {
    id: "idle-breathing", title: T("Idle breathing", "Ruhiges Atmen"),
    description: T("A five-second loop at about 1.2 met: the chest rises and falls with quiet breathing, the head and arms settle in a small slow sway; the feet stay planted.", "Eine Schleife von fünf Sekunden bei etwa 1,2 met: Die Brust hebt und senkt sich mit dem ruhigen Atmen, Kopf und Arme wiegen sich klein und langsam; die Füße bleiben stehen."),
    durationS: 5, fps: 30, loop: true, rootMotion: "none",
    keys: [
      k(0, "relaxed"),
      k(1.25, "relaxed", { torso: { angleDeg: -0.6 }, head: { angleDeg: 1.8 }, "arm-r": { angleDeg: 6.5 }, "arm-l": { angleDeg: 4 } }),
      k(2.5, "relaxed", { torso: { angleDeg: 0.4 }, head: { angleDeg: 0.4 }, "arm-r": { angleDeg: 4 }, "forearm-r": { angleDeg: 11.5 } }),
      k(3.75, "relaxed", { torso: { angleDeg: -0.5 }, head: { angleDeg: 1.5 }, "arm-r": { angleDeg: 6 }, "arm-l": { angleDeg: 2 } }),
      k(5, "relaxed"),
    ],
    contacts: contacts(5),
    phases: [{ id: "idle", fromS: 0, toS: 5, title: T("Quiet standing", "Ruhiges Stehen") }],
    events: [],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 0 }] }],
  },
  {
    id: "shiver", title: T("Shivering with cold", "Vor Kälte zittern"),
    description: T("A 1.2-second loop: arms pressed to the chest, shoulders hunched, the whole body trembling in quick small alternating shakes of trunk, head and knees. Use it in a cold environment so the model adds shivering heat.", "Eine Schleife von 1,2 Sekunden: Arme an die Brust gedrückt, Schultern hochgezogen, der ganze Körper zittert in schnellen kleinen wechselnden Stößen von Rumpf, Kopf und Knien. In einer kalten Umgebung verwenden, damit das Modell Zitterwärme ergänzt."),
    durationS: 1.2, fps: 30, loop: true, rootMotion: "none",
    keys: shake, contacts: contacts(1.2),
    phases: [{ id: "shiver", fromS: 0, toS: 1.2, title: T("Shivering", "Zittern") }],
    events: [],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 2 }, { t: 1.2, v: 2 }] }],
  },
  {
    id: "fanning", title: T("Fanning oneself in the heat", "Sich in der Hitze Luft zufächeln"),
    description: T("A 2-second loop: the near hand swings to and fro in front of the face, the head tilts back a little and the breathing is deeper and faster. Use it in a hot environment.", "Eine Schleife von 2 Sekunden: Die nahe Hand schwingt vor dem Gesicht hin und her, der Kopf neigt sich etwas zurück und das Atmen ist tiefer und schneller. In einer heißen Umgebung verwenden."),
    durationS: 2, fps: 30, loop: true, rootMotion: "none",
    keys: [
      k(0, "relaxed", { torso: { angleDeg: 0 }, head: { angleDeg: -3 }, neck: { angleDeg: 2 }, "arm-r": { angleDeg: 38 }, "forearm-r": { angleDeg: 120 }, "hand-r": { angleDeg: 30 } }),
      k(0.5, "relaxed", { torso: { angleDeg: 0.5 }, head: { angleDeg: -3 }, neck: { angleDeg: 2 }, "arm-r": { angleDeg: 40 }, "forearm-r": { angleDeg: 100 }, "hand-r": { angleDeg: -25 } }),
      k(1, "relaxed", { torso: { angleDeg: 0 }, head: { angleDeg: -3 }, neck: { angleDeg: 2 }, "arm-r": { angleDeg: 38 }, "forearm-r": { angleDeg: 120 }, "hand-r": { angleDeg: 30 } }),
      k(1.5, "relaxed", { torso: { angleDeg: 0.5 }, head: { angleDeg: -3 }, neck: { angleDeg: 2 }, "arm-r": { angleDeg: 40 }, "forearm-r": { angleDeg: 100 }, "hand-r": { angleDeg: -25 } }),
      k(2, "relaxed", { torso: { angleDeg: 0 }, head: { angleDeg: -3 }, neck: { angleDeg: 2 }, "arm-r": { angleDeg: 38 }, "forearm-r": { angleDeg: 120 }, "hand-r": { angleDeg: 30 } }),
    ],
    contacts: contacts(2),
    phases: [{ id: "fan", fromS: 0, toS: 2, title: T("Fanning", "Fächeln") }],
    events: [],
    tracks: [
      { target: "breathing", ease: "step", keys: [{ t: 0, v: 3 }, { t: 2, v: 3 }] },
      { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.4 }, { t: 2, v: 1.4 }] },
    ],
  },
];
a.presentation.defaultPose = "relaxed";
a.presentation.defaultClip = "idle-breathing";

fs.mkdirSync(`${S}/standing-adult`, { recursive: true });
fs.writeFileSync(`${S}/standing-adult/🔣️.json`, JSON.stringify(doc, null, 2) + "\n");
console.log("ok mass", a.dimensions.massKg);
