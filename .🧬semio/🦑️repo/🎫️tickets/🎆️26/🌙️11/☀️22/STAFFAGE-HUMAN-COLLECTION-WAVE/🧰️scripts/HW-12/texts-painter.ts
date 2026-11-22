/** 📝️ Identity, poses, clips, thermal and structural model and the per-scale decisions of `painter-at-easel`.
 * Sources: ISO 7730:2005 Annex A (standing, light activity 1.6 met; standing relaxed 1.2 met), ISO 9920 clothing, the body of the archive's elderly-with-cane (de Leva 1996 male fractions scaled to a 74-year-old of 1.65 m, 70 kg). */
import { at, dot, frameOf, rd, sub, T, type Any, type Scale, type V3 } from "./lib.ts";
import { BARS, CANVAS, lengthOf } from "./easel.ts";
import type { Edits } from "./decisions.ts";

const featureOf = (scale: string): number => (0.15 * Number(scale)) / 1000;

/** 🦴️ Structure solids of the props at one scale: every easel bar, the canvas, the brush and the palette as slender members, joints of brush and palette into the hand skeleton, masses at the centres of mass. */
export function propStructure(a: Any, scale: string, state: Any): Any[] {
  const f = featureOf(scale);
  const r = Math.max(0.012, f / 2 + 0.002);
  const sol = (id: string, bone: string, member: string, part: string, body: Any) => ({ id, bone, material: "bone", role: "body", tone: 0.6, ...body, member, part });
  const local = (bone: string, p: V3): number[] => { const F = frameOf(state, bone); const d = sub(p, F.o); return [rd(dot(d, F.u), 5), rd(dot(d, F.v), 5), rd(dot(d, F.w), 5)]; };
  const wrist = (side: "l" | "r"): V3 => (scale === "500" ? [state.bones[`forearm-${side}`].tip.x, state.bones[`forearm-${side}`].tip.y, state.bones[`forearm-${side}`].tip.z] : at(frameOf(state, `hand-${side}`), 0.05, 0, 0));
  return [
    ...BARS.map((bar) => { const L = lengthOf(bar); const s = Math.max(0.02, Math.min(bar.depth, bar.width)); return sol(`${bar.id}-member`, bar.id, bar.id, "member", { kind: "box", size: [rd(L, 4), s, s], transform: { translation: [rd(L / 2, 4), 0, 0] }, slender: true }); }),
    sol("mass-easel", "easel", "easel", "mass", { kind: "ellipsoid", radii: [r * 1.5, r * 1.5, r * 1.5], transform: { translation: [0.3, 0.85, 0] } }),
    sol("canvas-member", "canvas", "canvas", "member", { kind: "box", size: [CANVAS.height, 0.02, 0.02], transform: { translation: [CANVAS.height / 2, -0.011, 0] }, slender: true }),
    sol("mass-canvas", "canvas", "canvas", "mass", { kind: "ellipsoid", radii: [r, r, r], transform: { translation: [0.3, -0.011, 0] } }),
    sol("brush-member", "brush", "brush", "member", { kind: "box", size: [0.302, 0.008, 0.008], transform: { translation: [0.151, 0, 0] }, slender: true }),
    sol("joint-brush-grip", "brush", "brush", "joint", { kind: "capsule", p0: [0.12, 0, 0], p1: local("brush", wrist("r")), r0: r, r1: r }),
    sol("palette-member", "palette", "palette", "member", { kind: "box", size: [0.4, 0.01, 0.02], transform: { translation: [0.16, 0, 0] }, slender: true }),
    sol("joint-palette-thumb", "palette", "palette", "joint", { kind: "capsule", p0: [0, 0, 0], p1: local("palette", wrist("l")), r0: r, r1: r }),
    sol("mass-palette", "palette", "palette", "mass", { kind: "ellipsoid", radii: [r, r, r], transform: { translation: [0.16, 0, 0] } }),
  ];
}

/** ✍️ Writes texts, anchors, thermal regions, poses and clips. */
export function painterTexts(a: Any, p: { base: Record<string, Any>; pal: Record<string, Any>; strokes: Record<string, Any>[]; mix: Record<string, Any> }): void {
  a.title = T("Painter at an Easel", "Malender an der Staffelei");
  a.summary = T(
    "A 74-year-old plein-air painter of 1.65 m in a tweed jacket, flat cap and spectacles standing slightly stooped at a beech A-frame easel 1.86 m high: a 50 × 60 cm canvas on the tray, a flat bristle brush in the right hand and a paint-laden palette on the left thumb, at about 1.6 met in the open air. Poses for a brush stroke on the canvas, mixing paint on the palette and stepping back to look; a looping brush-stroke clip and a breathing loop.",
    "Ein 74-jähriger Freilichtmaler von 1,65 m in Tweedjacke, Schiebermütze und Brille steht leicht gebeugt an einer 1,86 m hohen A-Staffelei aus Buche: eine Leinwand von 50 × 60 cm auf der Ablage, ein flacher Borstenpinsel in der rechten Hand und eine Palette mit Farbe am linken Daumen, bei etwa 1,6 met im Freien. Posen für einen Pinselstrich auf der Leinwand, das Mischen auf der Palette und den prüfenden Blick; eine Schleife aus Pinselstrichen und eine Atemschleife.",
  );
  a.tags = ["human", "adult", "elderly", "standing", "culture", "painting", "easel", "plein-air", "public-space"];
  a.provenance.author = "Semio staffage human worker HW-12";
  a.provenance.created = "2026-11-22";
  a.provenance.references.push(
    { id: "iso-7730-2005", citation: "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort; Annex A metabolic rates (standing, light activity 1.6 met; standing relaxed 1.2 met)." },
    { id: "easel-dimensions", citation: "Studio A-frame (lyre) easel of beech: overall height about 1.8 to 1.9 m, front legs about 35 × 22 mm, canvas tray at about 0.9 m, holding canvases up to about 1.2 m; mass about 4 to 5 kg (typical catalogue values)." },
  );
  a.provenance.notes = T(
    "Body, rig, clothing and every drawn and modelled part of the person are those of the archive's elderly-with-cane (a 74-year-old man of 1.65 m and 70 kg, de Leva male fractions scaled to his stature, a stooped stance with thoracic kyphosis); the cane is removed and the easel, canvas, brush and palette are new handcrafted parts at all four scales, each bar of the easel a bone of its own and every part drawn as the true projection of its own 3D prism. The easel stands on its three feet on the ground beside the person and is placed from the pelvis through a vertical slide, so it stays on the floor while the painter keeps his feet. Arms were solved through the library's evaluate: the brush tip lies on the canvas face along the stroke and on the palette when mixing, the palette stays level on the left thumb. Activity follows ISO 7730 Annex A; easel and paint values are catalogue-typical estimates.",
    "Körper, Skelett, Kleidung und jeder gezeichnete und modellierte Teil der Person stammen vom Archivobjekt elderly-with-cane (ein 74-jähriger Mann von 1,65 m und 70 kg, männliche Anteile nach de Leva auf seine Größe skaliert, gebeugte Haltung mit Brustkyphose); der Stock ist entfernt, Staffelei, Leinwand, Pinsel und Palette sind neue, in allen vier Maßstäben von Hand gebaute Teile, jede Latte der Staffelei ein eigener Knochen und jeder Teil als wahre Projektion seines eigenen 3D-Prismas gezeichnet. Die Staffelei steht mit ihren drei Füßen neben der Person auf dem Boden und ist vom Becken aus über eine senkrechte Führung gesetzt, damit sie am Boden bleibt, solange der Maler seine Füße stehen lässt. Die Arme wurden mit evaluate der Bibliothek gelöst: Die Pinselspitze liegt entlang des Strichs auf der Leinwand und beim Mischen auf der Palette, die Palette bleibt waagerecht am linken Daumen. Die Aktivität folgt ISO 7730 Anhang A; Werte für Staffelei und Farbe sind katalogübliche Schätzungen.",
  );
  const foot = (id: string, bone: string, title: Any) => ({ id, bone, x: 0, y: 0, kind: "contact", title, support: "contact", normalDeg: 90, footprintLengthM: 0.035, footprintWidthM: 0.025, frictionCoefficient: 0.5 });
  a.anchors.push(
    foot("easel-foot-l", "easel-leg-l", T("Left front foot of the easel", "Linker Vorderfuß der Staffelei")),
    foot("easel-foot-r", "easel-leg-r", T("Right front foot of the easel", "Rechter Vorderfuß der Staffelei")),
    foot("easel-foot-back", "easel-leg-back", T("Back foot of the easel", "Hinterfuß der Staffelei")),
    { id: "brush-tip", bone: "brush", x: 0.302, y: 0, kind: "attachment", title: T("Bristle tip of the brush", "Borstenspitze des Pinsels") },
    { id: "palette-thumb-hole", bone: "palette", x: 0, y: 0, kind: "attachment", title: T("Thumb hole of the palette", "Daumenloch der Palette") },
  );

  const t = a.thermal;
  t.environment = { airTempC: 20, meanRadiantTempC: 24, skyTempC: 8, groundTempC: 22, relativeHumidity: 0.55, windMS: 0.8, windDirectionDeg: 90, solarWM2: 350, diffuseFraction: 0.4, pressurePa: 101325 };
  t.baseMet = 1.6;
  const passive = (id: string, title: Any, material: string, areaM2: number, L: number, geometry: string, cap: number) => ({ id, title, kind: "surface", model: "passive", material, areaM2, characteristicLengthM: L, geometry, skyViewFactor: 0.5, layers: [], heatCapacityJM2K: cap, source: "authored-estimate", reference: "easel-dimensions" });
  t.regions.push(
    passive("easel", T("Oiled beech easel", "Staffelei aus geölter Buche"), "oiled-beech", 0.62, 0.035, "cylinder", 12000),
    passive("canvas", T("Painted canvas on its stretcher", "Bemalte Leinwand auf dem Keilrahmen"), "primed-canvas", 0.67, 0.5, "vertical-plate", 2500),
    passive("brush", T("Brush", "Pinsel"), "lacquered-brush", 0.008, 0.008, "cylinder", 1500),
    passive("palette", T("Palette with paint", "Palette mit Farbe"), "palette-plywood", 0.19, 0.3, "horizontal-plate-up", 4500),
  );
  t.respiration.patterns = [
    { id: "painting", title: T("Concentrated light work", "Konzentrierte leichte Arbeit"), rateBpm: 16, tidalVolumeL: 0.6, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.46, pauseAfterExhaleFraction: 0.12, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
    { id: "quiet", title: T("Quiet standing", "Ruhiges Stehen"), rateBpm: 16, tidalVolumeL: 0.55, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate", reference: "gupta-2010" },
  ];
  t.assumptions = [
    ...t.assumptions.filter((x: Any) => !/cane|Stock/i.test(x.en + x.de)),
    T("Painting at an easel standing is light standing work, 1.6 met after ISO 7730 Annex A; mixing on the palette 1.5 met and stepping back to look 1.3 met (authored between the table values). The reference case is a mild sunny afternoon outdoors (20 degrees C air, 350 W/m2 of sun, a light breeze).", "Malen an der Staffelei im Stehen ist leichte Stehtätigkeit, 1,6 met nach ISO 7730 Anhang A; Mischen auf der Palette 1,5 met und der prüfende Blick 1,3 met (zwischen den Tabellenwerten geschätzt). Der Bezugsfall ist ein milder sonniger Nachmittag im Freien (Luft 20 Grad C, 350 W/m2 Sonne, leichter Wind)."),
    T("Easel, canvas, brush and palette are passive surfaces; they take the sun on their exposed faces and exchange heat with air, sky and ground only.", "Staffelei, Leinwand, Pinsel und Palette sind passive Oberflächen; sie nehmen die Sonne auf ihren freien Flächen auf und tauschen Wärme nur mit Luft, Himmel und Boden."),
  ];
  a.structure.assumptions = [
    ...a.structure.assumptions.filter((x: Any) => !/cane|Stock/i.test(x.en + x.de)),
    T("The easel of 4.6 kg and the canvas of 0.65 kg stand on the three easel feet through their own contacts and do not load the person; the palette of 0.38 kg hangs on the left thumb and the 18 g brush lies in the right hand, both as point masses on their bones. Brush pressure on the canvas is a few newtons at most and is not applied as an external load.", "Die Staffelei von 4,6 kg und die Leinwand von 0,65 kg stehen über eigene Kontakte auf den drei Staffeleifüßen und belasten die Person nicht; die Palette von 0,38 kg hängt am linken Daumen und der 18 g schwere Pinsel liegt in der rechten Hand, beide als Punktmassen auf ihren Knochen. Der Pinseldruck auf die Leinwand beträgt höchstens einige Newton und wird nicht als äußere Last angesetzt."),
  ];

  const regionIds: string[] = t.regions.map((r: Any) => r.id);
  const sun: Record<string, number> = { "face-skin": 0.35, hair: 0.2, cap: 0.6, "hand-skin": 0.4, jacket: 0.45, trousers: 0.4, shoes: 0.4, "spectacle-frame": 0.4, easel: 0.4, canvas: 0.1, brush: 0.4, palette: 0.6 };
  const exposed: Record<string, number> = { "hand-skin": 0.85, palette: 0.92, brush: 0.9, canvas: 1, easel: 1 };
  const regions = Object.fromEntries(regionIds.map((id) => [id, { exposedFraction: exposed[id] ?? (id === "shoes" ? 0.95 : 1), sunlitFraction: sun[id] ?? 0.3, ...(id === "shoes" ? { contact: { fraction: 0.3, tempC: 22 } } : {}) }]));
  const emitters = Object.fromEntries(t.emitters.filter((e: Any) => e.kind !== "breath").map((e: Any) => [e.id, 0.6]));
  const contacts = ["heel-l", "ball-l", "heel-r", "ball-r", "easel-foot-l", "easel-foot-r", "easel-foot-back"];
  const lift = { "easel-lift": { dx: 0 } };
  const pose = (id: string, title: Any, description: Any, bones: Any, met: number, breathing: string) => ({ id, title, description, bones: { ...p.base, ...lift, ...bones }, ground: "auto", contacts, thermal: { activityMet: met, breathing, regions, emitters, operating: {} }, loads: [] });
  const lowered = { "arm-r": { angleDeg: 6, abductionDeg: -4, twistDeg: -10 }, "forearm-r": { angleDeg: 38 }, "hand-r": { angleDeg: 0 }, brush: { angleDeg: 0 } };
  a.poses = [
    pose("painting", T("Painting a stroke", "Einen Strich malen"), T("Standing slightly stooped half a step behind the easel, the palette level on the left thumb in front of the hip, the right arm reaching forward so that the flat brush lays paint on the canvas at shoulder height.", "Leicht gebeugt einen halben Schritt hinter der Staffelei stehend, die Palette waagerecht am linken Daumen vor der Hüfte, der rechte Arm nach vorn gestreckt, sodass der Flachpinsel auf Schulterhöhe Farbe auf die Leinwand legt."), { ...p.pal, ...p.strokes[1] }, 1.6, "painting"),
    pose("mixing-paint", T("Mixing paint on the palette", "Farbe auf der Palette mischen"), T("The brush comes down to the palette and works a colour between the dabs of paint while the eyes stay on the canvas.", "Der Pinsel kommt zur Palette herunter und mischt eine Farbe zwischen den Farbklecksen, während der Blick auf der Leinwand bleibt."), { ...p.pal, ...p.mix }, 1.5, "painting"),
    pose("stepping-back-to-look", T("Looking at the painting", "Das Bild prüfend betrachten"), T("A pause: the trunk straightens a little, the brush hand rests beside the thigh and the painter looks at the canvas, the palette still on the thumb.", "Eine Pause: Der Rumpf richtet sich ein wenig auf, die Pinselhand ruht neben dem Oberschenkel und der Maler betrachtet die Leinwand, die Palette weiter am Daumen."), { ...p.pal, ...lowered, torso: { angleDeg: 2 } }, 1.3, "quiet"),
  ];
  const span = (d: number) => contacts.map((anchor) => ({ anchor, fromS: 0, toS: d }));
  const keys = [0, 1, 2, 3, 4, 0].map((i, k) => ({ t: rd(k * 0.4, 3), pose: "painting", ease: "smooth", bones: p.strokes[i] }));
  a.clips = [
    {
      id: "painting-strokes", title: T("Painting brush strokes", "Pinselstriche malen"),
      description: T("A two-second loop: the brush lays a stroke of about 20 cm up and across the canvas in 0.8 s, lifts 3 cm off the face and comes back to start the next stroke; the palette hand, the trunk and the feet stay still.", "Eine Schleife von zwei Sekunden: Der Pinsel legt in 0,8 s einen etwa 20 cm langen Strich schräg nach oben über die Leinwand, hebt 3 cm ab und kehrt zum nächsten Strich zurück; Palettenhand, Rumpf und Füße bleiben ruhig."),
      durationS: 2, fps: 30, loop: true, rootMotion: "none", keys, contacts: span(2),
      phases: [{ id: "stroke", fromS: 0, toS: 0.8, title: T("Stroke on the canvas", "Strich auf der Leinwand") }, { id: "return", fromS: 0.8, toS: 2, title: T("Lift and return", "Abheben und zurück") }],
      events: [{ t: 0, kind: "contact-on", title: T("Brush touches the canvas", "Pinsel berührt die Leinwand"), target: "brush-tip" }, { t: 0.8, kind: "contact-off", title: T("Brush lifts off", "Pinsel hebt ab"), target: "brush-tip" }],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 2, v: 0 }] }],
    },
    {
      id: "idle-breathing", title: T("Breathing while looking", "Atmen beim Betrachten"),
      description: T("A five-second loop at 1.3 met looking at the painting: the chest rises and falls with quiet breathing and the lowered brush hand sways a little; the feet and the easel stay put.", "Eine Schleife von fünf Sekunden bei 1,3 met beim Betrachten des Bildes: Die Brust hebt und senkt sich mit ruhigem Atmen und die gesenkte Pinselhand schwingt ein wenig; Füße und Staffelei bleiben stehen."),
      durationS: 5, fps: 30, loop: true, rootMotion: "none",
      keys: [
        { t: 0, pose: "stepping-back-to-look", ease: "smooth" },
        { t: 1.25, pose: "stepping-back-to-look", ease: "smooth", bones: { "arm-r": { ...lowered["arm-r"], angleDeg: 8 }, torso: { angleDeg: 2.4 } } },
        { t: 2.5, pose: "stepping-back-to-look", ease: "smooth", bones: { "arm-r": { ...lowered["arm-r"], angleDeg: 5 }, torso: { angleDeg: 1.6 } } },
        { t: 3.75, pose: "stepping-back-to-look", ease: "smooth", bones: { "arm-r": { ...lowered["arm-r"], angleDeg: 7.5 }, torso: { angleDeg: 2.3 } } },
        { t: 5, pose: "stepping-back-to-look", ease: "smooth" },
      ],
      contacts: span(5), phases: [{ id: "look", fromS: 0, toS: 5, title: T("Looking", "Betrachten") }], events: [],
      tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: 5, v: 1 }] }],
    },
  ];
  a.presentation = { defaultPose: "painting", defaultClip: "painting-strokes", defaultScale: 50 };
}

/** 🗒️ How each scale draws and models the painter. */
export const PAINTER_DECISIONS: Edits = {
  replace: [
    [", shoes with sole and heel, the cane with its derby handle under the right hand.", " and shoes with sole and heel."],
    [", Schuhe mit Sohle und Absatz, der Stock mit seinem Derbygriff unter der rechten Hand.", " und Schuhe mit Sohle und Absatz."],
    ["both arms, the hand on the handle, shoes and cane.", "both arms, the hands with brush and palette, shoes."],
    ["beide Arme, die Hand auf dem Griff, Schuhe und Stock.", "beide Arme, die Hände mit Pinsel und Palette, Schuhe."],
    ["; shaft, brass collar and handle are one cane outline.", "."],
    ["; Schaft, Messingring und Griff sind ein Stockumriss.", "."],
    ["The cane is drawn at its true 23 to 18 mm (0.4 mm on paper), the glasses as detail strokes.", "The glasses are drawn as detail strokes."],
    ["Der Stock ist in seinen wahren 23 bis 18 mm gezeichnet (0,4 mm auf dem Papier), die Brille als Detailstriche.", "Die Brille ist als Detailstriche gezeichnet."],
    [", trousers per leg bone, shoes, the cane with handle and ferrule.", ", trousers per leg bone, shoes."],
    [", Hose je Beinknochen, Schuhe, der Stock mit Griff und Zwinge.", ", Hose je Beinknochen, Schuhe."],
    ["the fingers round the handle,", "the fingers round brush and palette,"],
    ["die Finger um den Griff,", "die Finger um Pinsel und Palette,"],
    [", shoe and sole are one form, handle and shaft one cane outline.", ", shoe and sole are one form."],
    [", Schuh und Sohle sind eine Form, Griff und Schaft ein Stockumriss.", ", Schuh und Sohle sind eine Form."],
    ["The cane is drawn at its true 23 to 18 mm, 0.2 mm on paper.", "The spectacles are left out; nothing of the figure is thickened."],
    ["Der Stock ist in seinen wahren 23 bis 18 mm gezeichnet, 0,2 mm auf dem Papier.", "Die Brille entfällt; nichts an der Figur wird verdickt."],
    [", trousers per leg bone, shoes, cane.", ", trousers per leg bone, shoes."],
    [", Hose je Beinknochen, Schuhe, Stock.", ", Hose je Beinknochen, Schuhe."],
    ["the hand on the handle and the cane as third support.", "the raised brush arm and the palette at the hip."],
    ["die Hand auf dem Griff und der Stock als dritte Stütze.", "der gehobene Pinselarm und die Palette an der Hüfte."],
    ["The cane shaft (23 mm real) is drawn as a 36 mm strip, 0.18 mm on paper, and its handle as a 0.18 mm stroke in the side view, so the third support stays printable.", "Nothing of the figure is thickened."],
    ["Der Stockschaft (23 mm real) ist als 36 mm breiter Streifen gezeichnet, 0,18 mm auf dem Papier, sein Griff als 0,18 mm starker Strich in der Seitenansicht, damit die dritte Stütze druckbar bleibt.", "Nichts an der Figur wird verdickt."],
    [", thigh and shank per leg, shoes, cane;", ", thigh and shank per leg, shoes;"],
    [", Ober- und Unterschenkel je Bein, Schuhe, Stock;", ", Ober- und Unterschenkel je Bein, Schuhe;"],
    ["both arms, the hand on the cane, both legs and shoes, the cane.", "both arms, the brush hand at the canvas, both legs and shoes."],
    ["beide Arme, die Hand am Stock, beide Beine und Schuhe, der Stock.", "beide Arme, die Pinselhand an der Leinwand, beide Beine und Schuhe."],
    ["Face, ears, fingers, the handle of the cane and all detail lines.", "Face, ears, fingers and all detail lines."],
    ["Gesicht, Ohren, Finger, der Griff des Stocks und alle Detaillinien.", "Gesicht, Ohren, Finger und alle Detaillinien."],
    ["The cane (23 mm real) is drawn as a 71 mm strip, 0.14 mm on paper, so the third support stays visible.", ""],
    ["Der Stock (23 mm real) ist als 71 mm breiter Streifen gezeichnet, 0,14 mm auf dem Papier, damit die dritte Stütze sichtbar bleibt.", ""],
    ["The left hand hangs relaxed; the right wrist is extended 50 degrees over the derby handle and the four fingers close round the 28 mm handle from the outer side while the thumb lies along its inner top.", "The right hand closes round the brush handle, the left carries the palette on the thumb with the fingers under the board."],
    ["Die linke Hand hängt entspannt; das rechte Handgelenk ist 50 Grad über den Derbygriff gestreckt, die vier Finger schließen sich von außen um den 28 mm starken Griff, der Daumen liegt innen oben an.", "Die rechte Hand schließt sich um den Pinselstiel, die linke trägt die Palette auf dem Daumen, die Finger unter dem Brett."],
    ["Cane: an ash cane of 78.5 cm, fitted to the wrist crease of the stooped stance, with a derby handle of 13 cm, a brass collar, a shaft turned from 23 to 18 mm and a 30 mm rubber ferrule. The glasses are declared slender.", "Slender parts: the glasses are declared slender."],
    ["Stock: ein Eschenstock von 78,5 cm, auf die Handgelenkfalte der gebeugten Haltung angepasst, mit 13 cm langem Derbygriff, Messingring, von 23 auf 18 mm gedrehtem Schaft und 30 mm Gummizwinge. Die Brille ist als schlank erklärt.", "Schlanke Teile: die Brille ist als schlank erklärt."],
    ["hands, hair, glasses and cane refer to their body solids;", "hands, hair and glasses refer to their body solids;"],
    ["Hände, Haar, Brille und Stock verweisen auf ihre Körper;", "Hände, Haar und Brille verweisen auf ihre Körper;"],
    ["carry the foot bones to the ground and the ferrule of the cane stands on it.", "carry the foot bones to the ground."],
    ["tragen die Fußknochen zum Boden, die Zwinge des Stocks steht selbst darauf.", "tragen die Fußknochen zum Boden."],
    ["masses spheres at the segment centres, the cane is a member by reference.", "masses spheres at the segment centres."],
    ["Massen Kugeln in den Segmentschwerpunkten, der Stock ist Stab per Verweis.", "Massen Kugeln in den Segmentschwerpunkten."],
    ["because a hand on a cane handle is read by its fingers.", "because a hand on a brush is read by its fingers."],
    ["weil eine Hand am Stockgriff an ihren Fingern gelesen wird.", "weil eine Hand am Pinsel an ihren Fingern gelesen wird."],
    ["cuffs, glasses and the brass collar of the cane are left out.", "cuffs and glasses are left out."],
    ["Manschetten, Brille und der Messingring des Stocks entfallen.", "Manschetten und Brille entfallen."],
    ["so that the grip on the handle still closes. The cane is one slender bent rod of its true 23 mm.", "so that the grip on the brush still closes."],
    ["damit der Griff um den Stockgriff noch schließt. Der Stock ist ein schlanker gebogener Stab in seinen wahren 23 mm.", "damit der Griff um den Pinsel noch schließt."],
    [", each hand one blunt form of 55 mm, the cane a slender rod.", ", each hand one blunt form of 55 mm."],
    [", jede Hand eine stumpfe Form von 55 mm, der Stock ein schlanker Stab.", ", jede Hand eine stumpfe Form von 55 mm."],
  ],
  append: {
    "50": {
      silhouette: T("The easel is drawn as the true projections of its own bars (two splayed front legs, mast, back leg, lower brace and canvas tray), the canvas as a stretched panel with the painted field on its face, the brush as handle, ferrule and bristles and the palette as a kidney-shaped board with three dabs of paint; sleeves and forearms are round bodies about their bones and the hands solids, so the raised arms read right in all three views.", "Die Staffelei ist als wahre Projektion ihrer eigenen Latten gezeichnet (zwei gespreizte Vorderbeine, Mast, Hinterbein, untere Strebe und Bildablage), die Leinwand als Keilrahmen mit dem bemalten Feld auf der Vorderseite, der Pinsel als Stiel, Zwinge und Borsten und die Palette als nierenförmiges Brett mit drei Farbklecksen; Ärmel und Unterarme sind Rundkörper um ihre Knochen und die Hände Körper, damit die gehobenen Arme in allen drei Ansichten richtig lesen."),
      omitted: T("The wing nuts of the easel, the thumb hole of the palette (covered by the thumb), single brush hairs.", "Die Flügelmuttern der Staffelei, das Daumenloch der Palette (vom Daumen verdeckt), einzelne Pinselhaare."),
      thinElements: T("The brush handle keeps its true 8 mm, 0.16 mm on paper; the palette and the paint layer of the canvas are thin sheets at their true thickness.", "Der Pinselstiel behält seine wahren 8 mm, 0,16 mm auf dem Papier; Palette und Farbschicht der Leinwand sind dünne Platten in ihrer wahren Stärke."),
      overlays: T("Easel, canvas, brush and palette carry their own passive regions.", "Staffelei, Leinwand, Pinsel und Palette tragen ihre eigenen passiven Regionen."),
    },
    "100": {
      silhouette: T("Easel bars, canvas with its painted field, the brush as one tapered rod and the palette board without dabs.", "Staffeleilatten, Leinwand mit bemaltem Feld, der Pinsel als ein verjüngter Stab und die Palette ohne Farbkleckse."),
      merged: T("Ferrule and bristles into the brush rod; the paint dabs into the palette.", "Zwinge und Borsten im Pinselstab; die Farbkleckse in der Palette."),
      overlays: T("Easel, canvas, brush and palette carry their own passive regions.", "Staffelei, Leinwand, Pinsel und Palette tragen ihre eigenen passiven Regionen."),
    },
    "200": {
      silhouette: T("Easel bars at their true section, the canvas as one sheet, the brush as a hairline rod and the palette as one board; the hands are gripping fists.", "Staffeleilatten in ihrem wahren Querschnitt, die Leinwand als eine Platte, der Pinsel als Haarlinienstab und die Palette als ein Brett; die Hände sind greifende Fäuste."),
      merged: T("The painted field into the canvas.", "Das bemalte Feld in der Leinwand."),
      overlays: T("Easel, canvas, brush and palette carry their own passive regions.", "Staffelei, Leinwand, Pinsel und Palette tragen ihre eigenen passiven Regionen."),
    },
    "500": {
      silhouette: T("The easel as its three legs and mast, the canvas as one sheet on them, the palette as a rectangle at the hip and the brush as a hairline; the hands are gripping fists.", "Die Staffelei als ihre drei Beine und der Mast, die Leinwand als eine Platte darauf, die Palette als Rechteck an der Hüfte und der Pinsel als Haarlinie; die Hände sind greifende Fäuste."),
      omitted: T("Canvas tray and lower brace of the easel (30 mm, below the 75 mm feature, the canvas sits on the legs).", "Bildablage und untere Strebe der Staffelei (30 mm, unter dem 75-mm-Merkmal, die Leinwand sitzt auf den Beinen)."),
      thinElements: T("The palette seen on edge and the brush are thinner than the printable 0.2 mm (5 mm and 8 mm of real size, 0.01 and 0.02 mm on paper); both keep their true size and are drawn with a 0.20 mm pen so they print as a line, the left fist behind the palette with a 0.16 mm pen.", "Die Palette in Kantenansicht und der Pinsel sind dünner als die druckbaren 0,2 mm (5 mm und 8 mm in Wirklichkeit, 0,01 und 0,02 mm auf dem Papier); beide behalten ihre wahre Größe und sind mit einem 0,20-mm-Stift gezeichnet, sodass sie als Linie drucken, die linke Faust hinter der Palette mit einem 0,16-mm-Stift."),
      overlays: T("Easel, canvas, brush and palette carry their own passive regions.", "Staffelei, Leinwand, Pinsel und Palette tragen ihre eigenen passiven Regionen."),
    },
  },
  volume: {
    "50": [
      T("Easel of beech bars, each an extruded prism along its own bone (front legs 35 × 22 mm, mast 45 × 25 mm, back leg, brace and a 70 mm tray), a stretched canvas 50 × 60 × 2.2 cm with its painted field as a 1.5 mm sheet, a flat brush of handle, nickel ferrule and bristles, and a 6 mm plywood palette (sheet) with three dabs of paint.", "Staffelei aus Buchenlatten, jede ein extrudiertes Prisma entlang ihres eigenen Knochens (Vorderbeine 35 × 22 mm, Mast 45 × 25 mm, Hinterbein, Strebe und eine 70 mm breite Ablage), eine Leinwand von 50 × 60 × 2,2 cm mit dem bemalten Feld als 1,5 mm dünner Platte, ein Flachpinsel aus Stiel, Nickelzwinge und Borsten und eine 6 mm starke Sperrholzpalette (Platte) mit drei Farbklecksen."),
      T("Easel, canvas, brush and palette are passive regions on their own solids.", "Staffelei, Leinwand, Pinsel und Palette sind passive Regionen auf ihren eigenen Körpern."),
      T("Every easel bar is a slender member standing on the ground, the canvas a member on the tray, brush and palette slender members with a joint into the hand; masses at the centres of mass of easel, canvas and palette.", "Jede Staffeleilatte ist ein schlankes Glied auf dem Boden, die Leinwand ein Glied auf der Ablage, Pinsel und Palette schlanke Glieder mit einem Gelenk in die Hand; Massen in den Schwerpunkten von Staffelei, Leinwand und Palette."),
    ],
    "100": [
      T("Easel bars and canvas as at 1:50, the brush as one slender rod, the palette without dabs.", "Staffeleilatten und Leinwand wie bei 1:50, der Pinsel als ein schlanker Stab, die Palette ohne Farbkleckse."),
      T("The props as their own passive regions.", "Die Requisiten als eigene passive Regionen."),
      T("Easel bars, canvas, brush and palette as slender members with their joints.", "Staffeleilatten, Leinwand, Pinsel und Palette als schlanke Glieder mit ihren Gelenken."),
    ],
    "200": [
      T("Easel bars declared slender at their true section, canvas and palette as declared sheets, the brush as one slender rod; the hands closed to gripping fists 110 mm long.", "Staffeleilatten als schlank erklärt in ihrem wahren Querschnitt, Leinwand und Palette als erklärte Platten, der Pinsel als ein schlanker Stab; die Hände zu greifenden Fäusten von 110 mm Länge geschlossen."),
      T("The props as their own passive regions.", "Die Requisiten als eigene passive Regionen."),
      T("The props as slender members with their joints.", "Die Requisiten als schlanke Glieder mit ihren Gelenken."),
    ],
    "500": [
      T("The easel as legs and mast 30 × 35 mm declared slender, tray and brace left out, canvas and palette as declared sheets, the brush as one slender rod; the hands closed to gripping fists 110 mm long.", "Die Staffelei als Beine und Mast von 30 × 35 mm, als schlank erklärt, Ablage und Strebe weggelassen, Leinwand und Palette als erklärte Platten, der Pinsel als ein schlanker Stab; die Hände zu greifenden Fäusten von 110 mm Länge geschlossen."),
      T("The props as their own passive regions.", "Die Requisiten als eigene passive Regionen."),
      T("The props as slender members with their joints.", "Die Requisiten als schlanke Glieder mit ihren Gelenken."),
    ],
  },
};
void (null as unknown as Scale);
