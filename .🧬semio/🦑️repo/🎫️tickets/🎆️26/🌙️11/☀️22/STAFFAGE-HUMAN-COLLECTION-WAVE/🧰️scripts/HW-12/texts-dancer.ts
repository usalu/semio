/** 📝️ Identity, poses, clips and the thermal and structural model of `dancer`.
 * Sources: Ainsworth et al. (2011) Compendium codes 03015 (ballet, modern or jazz, general, rehearsal or class, 5.0 MET) and 03010 (performance, 6.8 MET); the body is the archive's jogger (1.82 m, 72 kg, DIN 33402-2, ANSUR II, de Leva 1996); hip range of motion of trained dancers after Steinberg et al. (2006). */
import { rd, T, type Any } from "./lib.ts";
import type { Edits } from "./decisions.ts";

/** ✍️ Writes texts, joints, thermal binding, poses and clips. */
export function dancerTexts(a: Any, P: Record<string, Any>): void {
  a.title = T("Dancer", "Tänzer");
  a.summary = T(
    "A contemporary and ballet dancer of 1.82 m and 72 kg in a fitted top, dance shorts, ankle socks and split-sole jazz shoes: a first arabesque on the flat left foot with the right leg stretched back, a relevé in first position with the arms overhead, a passé on demi-pointe and the fourth-position preparation of a pirouette. Clips: a pirouette en dehors that turns the whole figure once about the ball of the supporting foot, a looping plié and relevé exercise at 5 met and a breathing loop.",
    "Ein Tänzer für zeitgenössischen Tanz und Ballett von 1,82 m und 72 kg in eng anliegendem Oberteil, Tanzshorts, Knöchelsocken und Jazzschuhen mit geteilter Sohle: eine erste Arabeske auf dem flachen linken Fuß mit nach hinten gestrecktem rechtem Bein, ein Relevé in der ersten Position mit den Armen über dem Kopf, ein Passé auf halber Spitze und die Vorbereitung einer Pirouette in der vierten Position. Clips: eine Pirouette en dehors, die die ganze Figur einmal um den Ballen des Standfußes dreht, eine Schleife aus Plié und Relevé bei 5 met und eine Atemschleife.",
  );
  a.tags = ["human", "adult", "standing", "culture", "dance", "ballet", "contemporary-dance", "pirouette", "balance"];
  a.provenance.author = "Semio staffage human worker HW-12";
  a.provenance.created = "2026-11-22";
  a.provenance.references.push(
    { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; codes 03015 (ballet, modern or jazz, general, rehearsal or class, 5.0 MET) and 03010 (performance, vigorous, 6.8 MET)." },
    { id: "steinberg-2006", citation: "Steinberg, N. et al. (2006). Range of joint movement in female dancers and nondancers aged 8 to 16 years: anatomical and clinical implications. American Journal of Sports Medicine 34(5), 814-823; hip extension, external rotation and ankle plantar flexion beyond the non-dancer range." },
  );
  a.provenance.notes = T(
    "Body, rig and every modelled part of the person are those of the archive's jogger (an athletic man of 1.82 m and 72 kg after DIN 33402-2, ANSUR II girths and de Leva segment fractions), re-dressed for the studio; hip extension (to 50 degrees), hip flexion (to 140 degrees) and plantar flexion (to 62 degrees) are widened to a trained dancer's range, and the pelvis may turn a whole turn about its vertical axis. Every drawn shape is a solid (round limbs about their bones, side profiles swept across their own width), so the figure reads right in all three views while it turns. The single-leg poses were solved through the library's evaluate: the ball of the supporting left foot lies under the vertical axis of the pelvis and the centre of mass over the support, so the pirouette turns about the planted ball.",
    "Körper, Skelett und jeder modellierte Teil der Person stammen vom Archivobjekt jogger (ein sportlicher Mann von 1,82 m und 72 kg nach DIN 33402-2, ANSUR-II-Umfängen und den Segmentanteilen nach de Leva), für den Ballettsaal neu eingekleidet; Hüftstreckung (bis 50 Grad), Hüftbeugung (bis 140 Grad) und Plantarflexion (bis 62 Grad) sind auf den Bewegungsumfang eines ausgebildeten Tänzers erweitert, und das Becken darf sich eine ganze Umdrehung um seine senkrechte Achse drehen. Jede gezeichnete Form ist ein Körper (runde Glieder um ihre Knochen, Seitenprofile über ihre eigene Breite gezogen), damit die Figur beim Drehen in allen drei Ansichten richtig liest. Die Posen auf einem Bein wurden mit evaluate der Bibliothek gelöst: Der Ballen des linken Standfußes liegt unter der senkrechten Achse des Beckens und der Schwerpunkt über der Stützfläche, sodass sich die Pirouette um den stehenden Ballen dreht.",
  );
  const title = (id: string, en: string, de: string) => { const x = a.thermal.regions.find((r: Any) => r.id === id); if (x) x.title = T(en, de); };
  title("torso-clothing", "Fitted short-sleeved dance top", "Eng anliegendes kurzärmeliges Tanzoberteil");
  title("arm-clothing", "Short sleeves", "Kurze Ärmel");
  title("leg-clothing", "Dance shorts", "Tanzshorts");
  title("shoes", "Split-sole jazz shoes over socks", "Jazzschuhe mit geteilter Sohle über Socken");
  title("headband", "Cotton headband", "Stirnband aus Baumwolle");
  const mat = (id: string, en: string, de: string) => { const x = a.materials.find((m: Any) => m.id === id); if (x) x.title = T(en, de); };
  mat("nylon-clothing", "Elastane-cotton jersey", "Elastan-Baumwoll-Jersey");
  mat("leather-shoe", "Soft leather jazz shoe with split rubber sole", "Weicher Lederjazzschuh mit geteilter Gummisohle");

  const t = a.thermal;
  t.environment = { airTempC: 22, meanRadiantTempC: 22, skyTempC: 22, groundTempC: 21, relativeHumidity: 0.45, windMS: 0.15, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
  t.baseMet = 5;
  t.respiration.patterns = [
    { id: "dancing", title: T("Dancing, deeper and faster", "Tanzen, tiefer und schneller"), rateBpm: 24, tidalVolumeL: 1.2, inhaleFraction: 0.42, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.06, route: "nose-and-mouth", source: "authored-estimate", reference: "gupta-2010" },
    { id: "holding", title: T("Holding a balance", "Eine Balance halten"), rateBpm: 18, tidalVolumeL: 0.9, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.48, pauseAfterExhaleFraction: 0.08, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
    { id: "rest", title: T("Calm standing breathing", "Ruhige Atmung im Stand"), rateBpm: 15, tidalVolumeL: 0.7, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.11, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
  ];
  t.assumptions = [
    T("DuBois skin area for 1.82 m and 72 kg; a fitted short-sleeved jersey top of about 0.08 clo and dance shorts of 0.05 clo with bare arms and legs, socks and soft jazz shoes (ISO 9920 values, 1 clo = 0.155 m2K/W). The reference room is a 22 degrees C dance studio with still air.", "DuBois-Hautfläche für 1,82 m und 72 kg; ein eng anliegendes kurzärmeliges Jerseyoberteil von etwa 0,08 clo und Tanzshorts von 0,05 clo mit nackten Armen und Beinen, Socken und weichen Jazzschuhen (Werte nach ISO 9920, 1 clo = 0,155 m2K/W). Der Bezugsraum ist ein Ballettsaal bei 22 Grad C mit ruhiger Luft."),
    T("Metabolic rate 5 met for class and rehearsal (Compendium code 03015), held balances 3 to 4 met and quiet standing 1.4 met (authored between the table values); skin wettedness 0.15 while dancing. Skin blood flow limits and set points of the two-node model as for the jogger.", "Stoffwechsel 5 met für Training und Probe (Kompendium-Code 03015), gehaltene Balancen 3 bis 4 met und ruhiges Stehen 1,4 met (zwischen den Tabellenwerten geschätzt); Hautbenetzung 0,15 beim Tanzen. Grenzen der Hautdurchblutung und Sollwerte des Zwei-Knoten-Modells wie beim Jogger."),
    T("Breathing patterns are authored estimates guided by Gupta et al. (2010): through nose and mouth while dancing, through the nose while holding a balance.", "Die Atemmuster sind Schätzungen nach Gupta et al. (2010): beim Tanzen durch Nase und Mund, beim Halten einer Balance durch die Nase."),
  ];
  a.structure.assumptions = [
    T("Inertial model: segment masses as point masses at de Leva centres; in the single-leg poses the centre of mass lies over the ball or the sole of the supporting foot, so the ground reaction is one body weight through that foot.", "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva; in den Posen auf einem Bein liegt der Schwerpunkt über dem Ballen oder der Sohle des Standfußes, sodass die Bodenreaktion ein Körpergewicht über diesen Fuß ist."),
    T("The pirouette turns the whole figure about the vertical axis of the pelvis, which stands over the ball of the supporting foot; that ball stays planted while the foot turns on it.", "Die Pirouette dreht die ganze Figur um die senkrechte Achse des Beckens, die über dem Ballen des Standfußes steht; dieser Ballen bleibt stehen, während sich der Fuß auf ihm dreht."),
  ];

  const ids: string[] = t.regions.map((r: Any) => r.id);
  const regions = (wet: number) => Object.fromEntries(ids.map((id) => [id, { exposedFraction: id === "shoes" ? 0.9 : 1, sunlitFraction: 0, ...(/skin|hair/.test(id) ? { wettedness: wet } : {}), ...(id === "shoes" ? { contact: { fraction: 0.15, tempC: 21 } } : {}) }]));
  const emitters = Object.fromEntries(t.emitters.filter((e: Any) => e.kind !== "breath").map((e: Any) => [e.id, 0.7]));
  const pose = (id: string, title: Any, description: Any, bones: Any, contacts: string[], met: number, breathing: string, wet = 0.15) => ({ id, title, description, bones, ground: "auto", contacts, thermal: { activityMet: met, breathing, regions: regions(wet), emitters, operating: {} }, loads: [] });
  const both = ["contact-l", "heel-l", "contact-r", "heel-r"];
  const first = { ...P.firstFlat, ...P.armsLow };
  a.poses = [
    pose("arabesque", T("First arabesque", "Erste Arabeske"), T("Standing on the flat left foot with the knee straight, the right leg stretched straight back to about the height of the hip with a pointed foot, the trunk inclined forward and lifted, the left arm reaching forward at shoulder height and the right arm open to the side.", "Auf dem flachen linken Fuß mit gestrecktem Knie, das rechte Bein gestreckt nach hinten bis etwa auf Hüfthöhe mit gestrecktem Fuß, der Rumpf nach vorn geneigt und gehoben, der linke Arm auf Schulterhöhe nach vorn und der rechte Arm zur Seite geöffnet."), P.arabesque, ["contact-l", "heel-l"], 3.5, "holding"),
    pose("releve-first", T("Relevé in first position", "Relevé in der ersten Position"), T("Both heels raised onto demi-pointe with the legs turned out in first position, the knees straight and the arms rounded overhead in fifth position.", "Beide Fersen auf die halbe Spitze gehoben, die Beine in der ersten Position ausgedreht, die Knie gestreckt und die Arme in der fünften Position rund über dem Kopf."), { ...P.releve, ...P.armsHigh }, ["contact-l", "contact-r"], 3, "holding"),
    pose("passe-releve", T("Passé on demi-pointe", "Passé auf halber Spitze"), T("The pirouette shape: on the ball of the left foot under the axis of the body, the right foot drawn up to the inside of the left knee with the thigh turned out, the arms rounded in first position in front of the body.", "Die Form der Pirouette: auf dem Ballen des linken Fußes unter der Körperachse, der rechte Fuß an die Innenseite des linken Knies gezogen und der Oberschenkel ausgedreht, die Arme rund in der ersten Position vor dem Körper."), P.passe, ["contact-l"], 4, "holding"),
    pose("preparation-fourth", T("Preparation in fourth position", "Vorbereitung in der vierten Position"), T("Both knees bent in a demi-plié, the right foot a step behind the left, the left arm rounded in front and the right arm open to the side, ready to push into a pirouette.", "Beide Knie im Demi-plié gebeugt, der rechte Fuß einen Schritt hinter dem linken, der linke Arm rund vorn und der rechte Arm zur Seite geöffnet, bereit für den Abstoß in eine Pirouette."), P.prep, both, 3.5, "dancing"),
    pose("first-position", T("Standing in first position", "Stehen in der ersten Position"), T("Heels together and the legs turned out, standing tall with the arms low and rounded in preparatory position.", "Fersen zusammen und die Beine ausgedreht, aufrecht stehend mit den Armen tief und rund in der Vorbereitungsposition."), first, both, 1.4, "rest", 0.05),
  ];
  const twisted = (bones: Any, deg: number) => ({ ...bones, pelvis: { ...(bones.pelvis ?? {}), twistDeg: deg } });
  const spans = (anchor: string, ranges: [number, number][]) => ranges.map(([fromS, toS]) => ({ anchor, fromS, toS }));
  a.clips = [
    {
      id: "pirouette", title: T("Single pirouette en dehors", "Einfache Pirouette en dehors"),
      description: T("From the preparation in fourth the dancer pushes up onto the ball of the left foot into passé and turns once to the right about the ball in 0.8 s, then lands back in fourth; the ball of the supporting foot stays on its spot and the arms close to first for the turn.", "Aus der Vorbereitung in der vierten Position drückt sich der Tänzer auf den Ballen des linken Fußes ins Passé und dreht sich in 0,8 s einmal um den Ballen, dann landet er wieder in der vierten Position; der Ballen des Standfußes bleibt auf seinem Fleck und die Arme schließen sich für die Drehung zur ersten Position."),
      durationS: 2.2, fps: 30, loop: false, rootMotion: "none",
      keys: [
        { t: 0, pose: "preparation-fourth", ease: "smooth" },
        { t: 0.45, pose: "preparation-fourth", ease: "smooth" },
        { t: 0.75, pose: "passe-releve", ease: "linear", bones: twisted(P.passe, 0) },
        { t: 0.95, pose: "passe-releve", ease: "linear", bones: twisted(P.passe, 90) },
        { t: 1.15, pose: "passe-releve", ease: "linear", bones: twisted(P.passe, 180) },
        { t: 1.35, pose: "passe-releve", ease: "linear", bones: twisted(P.passe, 270) },
        { t: 1.55, pose: "passe-releve", ease: "smooth", bones: twisted(P.passe, 360) },
        { t: 1.85, pose: "preparation-fourth", ease: "smooth", bones: twisted(P.prep, 360) },
        { t: 2.2, pose: "preparation-fourth", ease: "smooth", bones: twisted(P.prep, 360) },
      ],
      contacts: [...spans("contact-l", [[0, 2.2]]), ...spans("heel-l", [[0, 0.55], [1.75, 2.2]]), ...spans("contact-r", [[0, 0.55], [1.75, 2.2]]), ...spans("heel-r", [[0, 0.55], [1.75, 2.2]])],
      phases: [{ id: "preparation", fromS: 0, toS: 0.55, title: T("Preparation", "Vorbereitung") }, { id: "turn", fromS: 0.55, toS: 1.75, title: T("Turn in passé", "Drehung im Passé") }, { id: "landing", fromS: 1.75, toS: 2.2, title: T("Landing in fourth", "Landung in der vierten Position") }],
      events: [{ t: 0.55, kind: "contact-off", title: T("Back foot leaves the floor", "Hinterer Fuß verlässt den Boden"), target: "contact-r" }, { t: 1.75, kind: "contact-on", title: T("Back foot lands", "Hinterer Fuß landet"), target: "contact-r" }],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 2.2, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 5 }, { t: 2.2, v: 5 }] }],
    },
    {
      id: "plies-and-rises", title: T("Pliés and rises in first position", "Pliés und Relevés in der ersten Position"),
      description: T("A four-second loop of the class exercise: a demi-plié with the arms rounding to first, the stretch with the arms opening to second, a rise onto demi-pointe with the arms overhead and the lowering of the heels back to first with the arms low.", "Eine Schleife von vier Sekunden aus dem Training: ein Demi-plié mit zur ersten Position gerundeten Armen, das Strecken mit zur zweiten Position geöffneten Armen, ein Relevé auf die halbe Spitze mit den Armen über dem Kopf und das Senken der Fersen zurück in die erste Position mit tiefen Armen."),
      durationS: 4, fps: 30, loop: true, rootMotion: "none",
      keys: [
        { t: 0, pose: "first-position", ease: "smooth" },
        { t: 1, pose: "first-position", ease: "smooth", bones: { ...P.plie, ...P.armsFirst } },
        { t: 2, pose: "first-position", ease: "smooth", bones: { ...P.firstFlat, ...P.armsSecond } },
        { t: 3, pose: "releve-first", ease: "smooth" },
        { t: 4, pose: "first-position", ease: "smooth" },
      ],
      contacts: [...spans("contact-l", [[0, 4]]), ...spans("contact-r", [[0, 4]]), ...spans("heel-l", [[0, 2.2], [3.8, 4]]), ...spans("heel-r", [[0, 2.2], [3.8, 4]])],
      phases: [{ id: "plie", fromS: 0, toS: 2, title: T("Demi-plié and stretch", "Demi-plié und Strecken") }, { id: "releve", fromS: 2, toS: 4, title: T("Rise and lower", "Heben und Senken") }],
      events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 5 }, { t: 4, v: 5 }] }],
    },
    {
      id: "idle-breathing", title: T("Breathing in first position", "Atmen in der ersten Position"),
      description: T("A five-second loop standing in first position between exercises: the chest rises and falls with calm breathing and the low arms sway a little; the feet stay planted.", "Eine Schleife von fünf Sekunden in der ersten Position zwischen zwei Übungen: Die Brust hebt und senkt sich mit ruhigem Atmen und die tiefen Arme schwingen ein wenig; die Füße bleiben stehen."),
      durationS: 5, fps: 30, loop: true, rootMotion: "none",
      keys: [0, 1.25, 2.5, 3.75, 5].map((tt, i) => ({ t: tt, pose: "first-position", ease: "smooth", ...(i % 4 === 0 ? {} : { bones: { "upper-arm-l": { ...P.armsLow["upper-arm-l"], angleDeg: rd(P.armsLow["upper-arm-l"].angleDeg + [0, 2, -1, 1.5][i]!, 2) }, "upper-arm-r": { ...P.armsLow["upper-arm-r"], angleDeg: rd(P.armsLow["upper-arm-r"].angleDeg + [0, 1.5, -1, 2][i]!, 2) }, chest: { angleDeg: [0, 0.6, -0.4, 0.5][i] } } }) })),
      contacts: spans("contact-l", [[0, 5]]).concat(spans("contact-r", [[0, 5]]), spans("heel-l", [[0, 5]]), spans("heel-r", [[0, 5]])),
      phases: [{ id: "rest", fromS: 0, toS: 5, title: T("Between exercises", "Zwischen den Übungen") }], events: [],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 2 }, { t: 5, v: 2 }] }],
    },
  ];
  a.presentation = { defaultPose: "arabesque", defaultClip: "pirouette", defaultScale: 50 };
}

/** 🗒️ How each scale draws and models the dancer: the runner's drawings turned into solids. */
const solids = T("Every shape is a solid: arms and legs are round bodies about their bones from this scale's own limb lofts, trunk, head, hands and feet their side profiles swept across their own width, so all three views stay true while the figure turns about its vertical axis.", "Jede Form ist ein Körper: Arme und Beine sind Rundkörper um ihre Knochen aus den Gliedmaßen-Lofts dieses Maßstabs, Rumpf, Kopf, Hände und Füße ihre Seitenprofile über ihre eigene Breite gezogen, damit alle drei Ansichten richtig bleiben, während sich die Figur um ihre senkrechte Achse dreht.");
export const DANCER_DECISIONS: Edits = {
  replace: [["runner", "dancer"], ["Läufer", "Tänzer"], ["running shirt", "dance top"], ["Laufshirt", "Tanzoberteil"], ["run cycle", "turn and the exercises"], ["Laufzyklus", "Drehung und die Übungen"]],
  set: {
    "50": { silhouette: T(`The three views are the true projections of the 1:50 body in the first arabesque, 35 mm tall: head with hair under the headband, neck, fitted top with short sleeves, bare arms with hands and thumbs, dance shorts, bare legs, socks and split-sole jazz shoes. ${solids.en}`, `Die drei Ansichten sind die wahren Projektionen des 1:50-Körpers in der ersten Arabeske, 35 mm hoch: Kopf mit Haar unter dem Stirnband, Hals, eng anliegendes Oberteil mit kurzen Ärmeln, nackte Arme mit Händen und Daumen, Tanzshorts, nackte Beine, Socken und Jazzschuhe mit geteilter Sohle. ${solids.de}`) },
    "100": { silhouette: T(`The true projections of the 1:100 body, 17.5 mm tall: head and hair, top, sleeves, arms, hands, shorts, legs, socks and shoes. ${solids.en}`, `Die wahren Projektionen des 1:100-Körpers, 17,5 mm hoch: Kopf und Haar, Oberteil, Ärmel, Arme, Hände, Shorts, Beine, Socken und Schuhe. ${solids.de}`) },
    "200": { silhouette: T(`The true projections of the 1:200 body, one mass per part. ${solids.en}`, `Die wahren Projektionen des 1:200-Körpers, eine Masse je Teil. ${solids.de}`) },
    "500": { silhouette: T(`The true projections of the 1:500 body in its fewest masses: head, trunk, and per side upper arm, forearm with hand, thigh, shank and shoe; the arabesque reads by the leg stretched back and the arm reaching forward. ${solids.en}`, `Die wahren Projektionen des 1:500-Körpers in seinen wenigsten Massen: Kopf, Rumpf und je Seite Oberarm, Unterarm mit Hand, Oberschenkel, Unterschenkel und Schuh; die Arabeske liest sich am nach hinten gestreckten Bein und am nach vorn greifenden Arm. ${solids.de}`), retained: T("The first arabesque with the leg stretched back and the forward arm; the 0.5 m reach of the arabesque and the stature are reproduced. Front view and plan: the same masses as solids.", "Die erste Arabeske mit dem nach hinten gestreckten Bein und dem vorderen Arm; die Ausladung von 0,5 m und die Körpergröße sind wiedergegeben. Vorderansicht und Draufsicht: dieselben Massen als Körper.") },
  },
  append: {
    "50": { omitted: T("Inner seams, laces and finger lines that cannot be solids are left out of the turning figure.", "Innere Nähte, Schnürsenkel und Fingerlinien, die keine Körper sein können, entfallen bei der drehenden Figur.") },
    "100": { omitted: T("Inner lines are left out of the turning figure.", "Innenlinien entfallen bei der drehenden Figur.") },
    "200": { omitted: T("Inner lines are left out of the turning figure.", "Innenlinien entfallen bei der drehenden Figur.") },
    "500": {},
  },
  volume: {
    "50": [T("The athletic body of the jogger at the dancer's own poses; the jogger's running shoes stand for soft split-sole jazz shoes of the same size.", "Der sportliche Körper des Joggers in den eigenen Posen des Tänzers; die Laufschuhe des Joggers stehen für weiche Jazzschuhe mit geteilter Sohle derselben Größe.")],
  },
};
