/** Author 25: bilingual decisions per scale for the quadruped assets. */
export type T = { en: string; de: string };
export type Names = { en: string; de: string; trunk: T; limb: T; head: T; extra?: T[] };

const feat: Record<number, string> = { 50: "7.5", 100: "15", 200: "30", 500: "75" };

export function decisions(scale: 50 | 100 | 200 | 500, n: Names, own: { body: T; thermal?: T; structure?: T }): T[] {
  const f = feat[scale]!;
  const body: T[] = {
    50: [
      { en: `Body at 1:50: every bone drawn at this scale is its own loft whose sagittal profile follows the 1:50 elevation outline of the ${n.en} and whose lateral half width and centre are read per station from the 1:50 plan and front drawings (never a flat plate: sections are superellipses with exponent 2.2 to 3.0, ${n.trunk.en}, ${n.limb.en}, ${n.head.en}); smallest section ${f} mm.`, de: `Körper bei 1:50: jede in diesem Maßstab gezeichnete Bone ist ein eigener Loft, dessen senkrechtes Profil dem 1:50-Aufriss des Tieres (${n.de}) folgt und dessen seitliche Halbbreite und Mitte je Station aus Grundriss und Vorderansicht von 1:50 gelesen werden (nie eine flache Platte: Schnitte sind Superellipsen mit Exponent 2,2 bis 3,0, ${n.trunk.de}, ${n.limb.de}, ${n.head.de}); kleinster Schnitt ${f} mm.` },
    ],
    100: [
      { en: `Body at 1:100: modelled anew against the 1:100 drawings: the same bones and proportions as 1:50 but with the finer details removed (${n.extra?.[0]?.en ?? "eyes and toes"}), sections cut from this scale's own elevation, plan and front, smallest section ${f} mm.`, de: `Körper bei 1:100: neu gegen die Zeichnungen von 1:100 modelliert: gleiche Bones und Proportionen wie 1:50, aber ohne die feinen Details (${n.extra?.[0]?.de ?? "Augen und Zehen"}), Schnitte aus Aufriss, Grundriss und Vorderansicht dieses Maßstabs, kleinster Schnitt ${f} mm.` },
    ],
    200: [
      { en: `Body at 1:200: merged masses: the hock and paw (and the wrist and forepaw) are one lower-limb loft, the tail is three short lofts cut from the single drawn tail, jaw and ear flaps are reduced to the masses the 1:200 drawings show; smallest section ${f} mm.`, de: `Körper bei 1:200: zusammengefasste Massen: Sprunggelenk und Pfote (und Handwurzel und Vorderpfote) bilden einen Unterlauf-Loft, die Rute besteht aus drei kurzen Lofts aus der einen gezeichneten Rute, Kiefer und Ohrlappen sind auf die Massen reduziert, die die Zeichnungen von 1:200 zeigen; kleinster Schnitt ${f} mm.` },
    ],
    500: [
      { en: `Body at 1:500: the generalised animal: trunk in two overlapping masses, neck wedge, head mass, one tail mass, each leg as thigh, leg and foot lofts at least ${f} mm thick; cut from the 1:500 drawings (whose elevation was redrawn, see the report).`, de: `Körper bei 1:500: das verallgemeinerte Tier: Rumpf in zwei sich überlappenden Massen, Halskeil, Kopfmasse, eine Rutenmasse, jedes Bein als Oberschenkel-, Unterschenkel- und Fußloft von mindestens ${f} mm Dicke; geschnitten aus den Zeichnungen von 1:500 (deren Aufriss neu gezeichnet wurde, siehe Bericht).` },
    ],
  }[scale];
  const thermal: T = own.thermal ?? {
    en: `Thermal at 1:${scale}: the coat regions reference the body solids of the bones they cover (one fur layer per region, so one surface solid per region and no nested shells), the emitters are tagged solids at their anchors, the small skin regions (nose, tongue, pads${scale >= 200 ? ", ears" : ""}) are ${scale >= 100 ? "own thermal solids at this coarser scale" : "their own small body solids"} so that every region has a solid.`,
    de: `Thermik bei 1:${scale}: die Fellregionen verweisen auf die Körper der Bones, die sie bedecken (eine Fellschicht je Region, also ein Oberflächenkörper je Region und keine geschachtelten Schalen), die Emitter sind markierte Körper an ihren Ankern, die kleinen Hautregionen (Nase, Zunge, Ballen${scale >= 200 ? ", Ohren" : ""}) sind ${scale >= 100 ? "in diesem gröberen Maßstab eigene Thermikkörper" : "eigene kleine Körper"}, damit jede Region einen Körper hat.`,
  };
  const structure: T = own.structure ?? {
    en: `Structure at 1:${scale}: a capsule member along every load-bearing bone (spine segments, neck, head axis, jaw, ear, tail, the four limbs with scapula, humerus or femur, shank, metapodium and paw) inside the body, ${scale <= 100 ? "a joint ellipsoid at every rig joint, " : "joints merged into the members at this scale, "}mass ellipsoids for the bones with at least 10 % of the mass, no support solids because the asset declares no supports; radius never below ${(0.51 * Number(f)).toFixed(1)} mm.`,
    de: `Tragwerk bei 1:${scale}: ein Kapselstab entlang jeder tragenden Bone (Wirbelsäulenabschnitte, Hals, Kopfachse, Kiefer, Ohr, Rute, die vier Gliedmaßen mit Schulterblatt, Oberarm bzw. Oberschenkel, Unterschenkel, Mittelfuß und Pfote) im Körper, ${scale <= 100 ? "ein Gelenkellipsoid an jedem Gelenk des Skeletts, " : "Gelenke in diesem Maßstab in die Stäbe zusammengefasst, "}Massenellipsoide für die Bones mit mindestens 10 % der Masse, keine Auflagerkörper, weil das Asset keine Auflager deklariert; Radius nie unter ${(0.51 * Number(f)).toFixed(1)} mm.`,
  };
  return [...body, own.body, thermal, structure];
}
