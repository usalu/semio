import { bodyFromShapes, run, type Cfg } from "./author-05-gen.ts";

const cfg: Cfg = {
  slug: "horse",
  rules: [
    { match: /^hindbody$/, skip: true },
    { match: /^(chest)$/, fit: { p: 0, ex: 2.1 } },
    { match: /scapula|thigh|humerus/, fit: { endZ: 0.7 } },
    { match: /^mane/, fit: { auto: false, zc: 0, zh: 0.026, p: 0, ex: 2 } },
    { match: /^forelock$/, fit: { auto: false, zc: 0, zh: 0.03, p: 0.3, ex: 2 } },
  ],
  body: (c) => {
    const a = c.a;
    bodyFromShapes(c, cfg);
    a.fit("rump", "hindbody", { bone: "pelvis", from: "hindbody", auto: true, p: 0, ex: 2.1, uHi: 0.2, flatEnd: true, fracs: c.scale === 50 ? [0, 0.04, 0.2, 0.45, 0.7, 0.9, 1] : [0, 0.3, 0.7, 1] });
    a.fit("hindbody", "hindbody", { bone: "lumbar", from: "hindbody", auto: true, p: 0, ex: 2.1, uLo: 0, flatStart: true, fracs: c.scale === 50 ? [0, 0.1, 0.3, 0.55, 0.8, 0.95, 0.99, 1] : [0, 0.4, 0.8, 1] });
    const eye = a.centreOf("eye")!;
    if (c.scale <= 200) for (const n of ["near", "far"]) a.ell(`${n}-eye`, "head", [eye[0], eye[1], a.zOf(`${n}-eye`) ?? (n === "near" ? -0.085 : 0.085)], [0.016, 0.016, 0.016], 0, [`eye`]);
    const nos = a.centreOf("nostril");
    if (c.scale <= 100 && nos !== null) for (const n of ["n", "f"]) a.ell(`nostril-${n}`, "head", [nos[0], nos[1], a.zOf(`nostril-${n}`) ?? (n === "n" ? -0.026 : 0.026)], [0.012, 0.01, 0.01], 0, [`nostril`], { mat: "bare-skin", tone: 0.1, region: "muzzle-skin" });
  },
  emitterRegion: (em) => (em.kind === "breath" ? "coat-head" : "coat-trunk"),
  decisions: (n) => {
    const d: Record<number, { en: string[]; de: string[] }> = {
      50: {
        en: ["Body: every bone of the drawn 1:50 elevation, front and plan is a loft whose sagittal sections follow the elevation outline and whose lateral width, centre and superellipse exponent are cut from the visual hull of the three drawings, so barrel, neck, scapula, thigh and cannon have real cross-sections rather than slabs; the trunk is split into rump (pelvis) and hindbody (lumbar), mane ribbons, ears, eyes and nostrils are separate solids.", "Thermal: one skin solid per drawn body solid, referenced from the body so geometry is shared; breath-nose and plume-back are thin emitter solids at their anchors; areaM2 values that were estimates were corrected to the exposed model surface.", "Structure: every load-bearing bone has a five-section member along its centreline with bulged epiphyses, a joint ball at every joint, mass ellipsoids at the point masses (pelvis, thorax, head) and no supports because the asset declares none."],
        de: ["Körper: jede Bone der gezeichneten 1:50-Ansicht, -Vorderansicht und -Draufsicht ist ein Loft, dessen senkrechte Schnitte der Kontur folgen und dessen seitliche Breite, Mitte und Superellipsen-Exponent aus der Sichthülle der drei Zeichnungen geschnitten sind, sodass Rumpf, Hals, Schulterblatt, Oberschenkel und Röhrbein echte Querschnitte statt Platten haben; der Rumpf ist in Kruppe (Becken) und Hinterrumpf (Lende) geteilt, Mähnenbänder, Ohren, Augen und Nüstern sind eigene Körper.", "Thermik: ein Hautkörper je gezeichnetem Körper, vom Body referenziert, sodass die Geometrie geteilt ist; breath-nose und plume-back sind dünne Emitterkörper an ihren Ankern; geschätzte areaM2-Werte wurden auf die exponierte Modelloberfläche korrigiert.", "Tragwerk: jede tragende Bone hat ein Fünf-Schnitt-Stabelement entlang ihrer Mittellinie mit verdickten Gelenkenden, an jedem Gelenk eine Gelenkkugel, Massenellipsoide an den Punktmassen (Becken, Brustkorb, Kopf) und keine Auflager, weil das Asset keine deklariert."],
      },
      100: {
        en: ["Body: modelled anew against the 1:100 drawings with seven sections per loft instead of ten, 15 mm minimum cross-section, nostrils and eyes kept, mane ribbons and fine engraved details merged into the neck and head lofts.", "Thermal: same one-skin-per-body-solid derivation at 1:100; emitters become single solids.", "Structure: three-section members, joint balls only at bones longer than 6 % of the height, point masses kept."],
        de: ["Körper: neu gegen die 1:100-Zeichnungen modelliert, sieben statt zehn Schnitte je Loft, 15 mm Mindestquerschnitt, Nüstern und Augen bleiben, Mähnenbänder und feine Gravuren gehen in den Hals- und Kopfloft auf.", "Thermik: gleiche Ableitung eines Hautkörpers je Bodykörper bei 1:100; Emitter werden Einzelkörper.", "Tragwerk: Stäbe mit drei Schnitten, Gelenkkugeln nur an Bones über 6 % der Höhe, Punktmassen bleiben."],
      },
      200: {
        en: ["Body: five-section lofts against the 1:200 drawings, 30 mm minimum cross-section so the pasterns and hooves are widened to the feature, nostrils dropped, eyes kept as 32 mm balls.", "Thermal: skins derived from the body; regions that no longer have a body solid use thin texture sheets copied from the fine model.", "Structure: two-section cylindrical members, joint balls only at the large joints, point masses kept."],
        de: ["Körper: Lofts mit fünf Schnitten gegen die 1:200-Zeichnungen, 30 mm Mindestquerschnitt, sodass Fesseln und Hufe auf die Mindestgröße verbreitert sind, Nüstern entfallen, Augen bleiben als 32-mm-Kugeln.", "Thermik: Häute aus dem Body abgeleitet; Regionen ohne Bodykörper nutzen dünne Texturbleche aus dem feinen Modell.", "Tragwerk: zweischnittige zylindrische Stäbe, Gelenkkugeln nur an den großen Gelenken, Punktmassen bleiben."],
      },
      500: {
        en: ["Body: merged masses at 1:500 following the drawn generalised shapes (upper and lower limb bars, two neck parts, two tail parts, one ear solid), four-section lofts, 75 mm minimum cross-section.", "Thermal: skins derived from the body; eyes, muzzle, mane and hooves are thin texture sheets because the 1:500 body does not draw them.", "Structure: simple cylindrical members on every load-bearing bone, joint balls only at hip, shoulder and neck base, point masses kept."],
        de: ["Körper: zusammengefasste Massen bei 1:500 nach den gezeichneten verallgemeinerten Formen (Ober- und Unterschenkelstäbe, zwei Halsteile, zwei Schweifteile, ein Ohrkörper), Lofts mit vier Schnitten, 75 mm Mindestquerschnitt.", "Thermik: Häute aus dem Body; Augen, Maul, Mähne und Hufe sind dünne Texturbleche, weil der 1:500-Body sie nicht zeichnet.", "Tragwerk: einfache zylindrische Stäbe an jeder tragenden Bone, Gelenkkugeln nur an Hüfte, Schulter und Halsansatz, Punktmassen bleiben."],
      },
    };
    return d[n]!.en.map((en, i) => ({ en, de: d[n]!.de[i]! }));
  },
};
run(cfg);
