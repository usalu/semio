/** 🧰️ The repairs of lane CL-T2 per asset id, each idempotent on the asset document, and the log of what they changed.
 *
 * - `shower-head`: the zone ring of the spray face at 1:50 is the closed loop it was drawn as (first profile point repeated); as an open profile both ends dropped onto the axis along the same line and the outline touched itself.
 * - `gas-boiler`: the mass marker of 1:500 is a block (see below), and the heat exchanger of the thermal list at 1:200 is the closed loop it was drawn as (the drum with the 73 mm burner bore, first profile point repeated, as `th-hx-shell` of 1:100 is); as an open profile its last edge folded back onto the drop to the axis.
 * - `used-forced-draught-oil-burner`: the four 96 mm floor boards of 1:50 alternate as two repeated pairs, and the decision names what is modelled (it said 93 mm boards with 4 mm gaps; with those gaps the plan silhouette leaves its gate, 114 mm against 15 mm).
 * - `solid-brick-wall-1960`, `etics-brick-wall-12`, `etics-brick-wall-30`: the six bond bands of 1:200 alternate as two repeated bands (as the courses of 1:100 do), so no copy of one solid shares a face with the next.
 * - `used-wall-unit`: the three blankets laid end to end are the outer pair and the middle one at every scale.
 * - `used-bicycle-shelter`: the spokes of 1:100, 1:200 and 1:500 are rods through the axle (four, two, two) instead of twice as many capsules that all began in the axle point, where the end caps of opposite spokes shared their rim.
 * - the assets whose structure list gains triangles towards a coarser scale ({@link BLOCKS}, `ceiling-diffuser`, `kitchen-extractor-hood`, `used-park-armchair`, `used-backrest-park-bench`, `room-thermostat`, `gas-boiler`): the mass markers of that scale (and of every coarser one) are blocks with the centres and extents of their ellipsoids, as the catalog already draws them elsewhere at 1:500 (`used-sewing-machine`); a marker of one feature keeps its paper size at every scale, so as an ellipsoid it has more triangles the coarser the scale is. `used-park-armchair` turns its joint markers too, `kitchen-extractor-hood` only the fan unit at 1:200 (a block at 1:500 already), `used-backrest-park-bench` also turns its capsule joints into cylinders of the same axis and extents, `room-thermostat` turns the knob mass of 1:100.
 * @see ./fix.ts — the command */
type Solid = Record<string, any>;
type Text = { en: string; de: string };
type Volume = { body: Solid[]; thermal: Solid[]; structure: Solid[]; decisions: Text[] };

/** 📜️ One line per change of the running repair. */
export const log: string[] = [];

const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const set = (target: Solid, key: string, value: unknown, what: string): void => {
  if (same(target[key], value)) return;
  log.push(`${what}: ${key} ${JSON.stringify(target[key])?.slice(0, 120)} -> ${JSON.stringify(value)?.slice(0, 120)}`);
  if (value === undefined) delete target[key];
  else target[key] = value;
};
const find = (list: Solid[], id: string, what: string): Solid => {
  const solid = list.find((s) => s.id === id);
  if (solid === undefined) throw new Error(`${what}: no "${id}"`);
  return solid;
};
const after = (list: Solid[], id: string, entry: Solid, what: string): void => {
  if (list.some((s) => s.id === entry.id)) return;
  const at = list.findIndex((s) => s.id === id);
  if (at < 0) throw new Error(`${what}: no "${id}" to place "${entry.id}" after`);
  list.splice(at + 1, 0, entry);
  log.push(`${what}: ${entry.id} added after ${id}`);
};
const drop = (list: Solid[], id: string, what: string): void => {
  const at = list.findIndex((s) => s.id === id);
  if (at < 0) return;
  list.splice(at, 1);
  log.push(`${what}: ${id} removed`);
};
const rename = (list: Solid[], from: string, to: string, what: string): Solid => {
  const solid = list.find((s) => s.id === from);
  if (solid === undefined) return find(list, to, what);
  solid.id = to;
  log.push(`${what}: ${from} renamed ${to}`);
  return solid;
};
const sentence = (decision: Text, from: Text, to: Text, what: string): void => {
  for (const language of ["en", "de"] as const) {
    if (decision[language].includes(to[language])) continue;
    if (!decision[language].includes(from[language])) throw new Error(`${what}: the ${language} text to replace is not there: ${from[language]}`);
    decision[language] = decision[language].replace(from[language], to[language]);
    log.push(`${what}: ${language} text`);
  }
};
const append = (decision: Text, text: Text, what: string): void => {
  for (const language of ["en", "de"] as const) {
    if (decision[language].includes(text[language])) continue;
    decision[language] = `${decision[language]} ${text[language]}`;
    log.push(`${what}: ${language} text added`);
  }
};
const volume = (asset: any, scale: number): Volume => asset.volumes[String(scale)];

/** 🗣️ The last decision of a volume that matches `pattern` in English, else its last decision. */
const saying = (v: Volume, pattern: RegExp): Text => [...v.decisions].reverse().find((d) => pattern.test(d.en)) ?? v.decisions[v.decisions.length - 1]!;

/** 🧱️ The six bond bands of a 1:200 brick wall as two alternating repeated solids `<id>-a` and `<id>-b` (bottom band first), as the courses of 1:100 alternate. */
const alternate = (list: Solid[], id: string, what: string): void => {
  const a = rename(list, id, `${id}-a`, what);
  const [x, , z] = a.transform.translation as number[];
  if (a.sectionFaces !== undefined) set(a, "sectionFaces", ["-v", "-w", "+w"], `${what} ${id}-a`);
  set(a, "repeat", { count: 3, translation: [0.0, 0.333333334, 0.0] }, `${what} ${id}-a`);
  after(list, `${id}-a`, { ...structuredClone(a), id: `${id}-b`, transform: { translation: [x, 0.25, z] }, ...(a.sectionFaces === undefined ? {} : { sectionFaces: ["+v", "-w", "+w"] }) }, what);
};
const TOUCH = { en: ", alternating as two repeated bands that touch", de: ", abwechselnd als zwei wiederholte Bänder, die sich berühren" };

/** 🧱️ Turns the ellipsoid markers of the parts `parts` (only the ids `only` when given) in the structure list of a scale into blocks with the same centre and extents, and says so in the decision that names the structure. */
const blocks = (asset: any, scale: number, text?: Text, parts: readonly string[] = ["mass"], only?: readonly string[]): void => {
  const v = volume(asset, scale);
  v.structure.forEach((solid, at) => {
    if (solid.kind !== "ellipsoid" || !parts.includes(solid.part) || (only !== undefined && !only.includes(solid.id))) return;
    v.structure[at] = Object.fromEntries(Object.entries(solid).map(([key, value]) => (key === "kind" ? [key, "box"] : key === "radii" ? ["size", (value as number[]).map((r) => Number((2 * r).toFixed(9)))] : [key, value])));
    log.push(`1:${scale} structure: ${solid.id} ellipsoid ${JSON.stringify(solid.radii)} -> box ${JSON.stringify(v.structure[at]!.size)}`);
  });
  if (text !== undefined) append(saying(v, /structure/iu), text, `1:${scale} decision`);
};
const MASS = (finer: number): Text => ({ en: `The mass markers are blocks, not ellipsoids as at 1:${finer}, so that the structure list has fewer triangles than at 1:${finer}.`, de: `Die Massenmarken sind Blöcke, nicht Ellipsoide wie bei 1:${finer}, damit die Tragwerksliste weniger Dreiecke hat als bei 1:${finer}.` });

/** 🧊️ The assets whose mass markers become blocks at 1:500. */
export const BLOCKS = ["height-adjustable-desk", "used-baby-grand-piano", "used-chess-table", "used-pump-drinking-fountain", "used-recliner-chair", "used-tree-surround-bench", "used-upright-piano", "used-writing-desk", "picnic-table", "tilt-window", "used-bathtub", "used-washing-machine", "used-electric-guitar-and-combo", "used-recycling-bin-cluster", "used-xylophone", "used-tv-sideboard", "used-extendable-dining-table"] as const;

/** 🛢️ Turns the capsule joints of constant radius in the structure list of a scale into cylinders of the same axis and extents (a revolve from one cap end to the other). */
const cylinders = (asset: any, scale: number): void => {
  const v = volume(asset, scale);
  v.structure.forEach((solid, at) => {
    if (solid.kind !== "capsule" || solid.part !== "joint" || solid.r0 !== solid.r1) return;
    const [a, b, r] = [solid.p0 as number[], solid.p1 as number[], solid.r0 as number];
    const length = Math.hypot(b[0]! - a[0]!, b[1]! - a[1]!, b[2]! - a[2]!), round = (x: number): number => Number(x.toFixed(9));
    const swap: Record<string, [string, unknown] | null> = { kind: ["kind", "revolve"], p0: ["profile", [[-r, 0], [-r, r], [round(length + r), r], [round(length + r), 0]]], p1: ["axis", b.map((x, i) => round((x - a[i]!) / length))], r0: ["origin", a], r1: null };
    v.structure[at] = Object.fromEntries(Object.entries(solid).flatMap(([key, value]) => (key in swap ? (swap[key] === null ? [] : [swap[key]!]) : [[key, value]])));
    log.push(`1:${scale} structure: ${solid.id} capsule -> cylinder of ${round(length + 2 * r)} m`);
  });
};

/** 🗂️ The repair of each asset of the lane. */
export const repairs: Record<string, (asset: any) => void> = {
  "shower-head": (asset) => {
    const ring = find(volume(asset, 50).body, "face-zone-ring", "1:50 body");
    set(ring, "profile", [[0.011, 0.027], [0.0128, 0.0275], [0.0128, 0.0305], [0.011, 0.031], [0.011, 0.027]], "1:50 face-zone-ring");
  },
  "gas-boiler": (asset) => {
    const hx = find(volume(asset, 200).thermal, "th-hx", "1:200 thermal");
    set(hx, "profile", [[-0.14, 0.0365], [-0.14, 0.115], [0.043, 0.115], [0.043, 0.0], [0.01, 0.0], [0.01, 0.0365], [-0.14, 0.0365]], "1:200 th-hx");
    blocks(asset, 500);
    sentence(saying(volume(asset, 500), /the centre of mass as a/u), { en: "the centre of mass as a ball.", de: "der Schwerpunkt als Kugel." }, { en: "the centre of mass as a block, not a ball as at 1:200, so that the structure list has fewer triangles than at 1:200.", de: "der Schwerpunkt als Block, nicht als Kugel wie bei 1:200, damit die Tragwerksliste weniger Dreiecke hat als bei 1:200." }, "1:500 decision");
  },
  "used-forced-draught-oil-burner": (asset) => {
    const v = volume(asset, 50);
    const boards = find(v.body, "floor-boards", "1:50 body");
    set(boards, "repeat", { count: 2, translation: [0, 0, 0.192] }, "1:50 floor-boards");
    after(v.body, "floor-boards", { ...structuredClone(boards), id: "floor-boards-b", transform: { translation: [0.31, 0.054, -0.048] } }, "1:50 body");
    after(v.thermal, "th-floor-boards", { id: "th-floor-boards-b", of: "floor-boards-b", region: "crate" }, "1:50 thermal");
    after(v.structure, "st-floor-boards", { id: "st-floor-boards-b", of: "floor-boards-b", member: "crate", part: "member" }, "1:50 structure");
    sentence(saying(v, /floor boards/u), { en: "four 93 mm floor boards with 4 mm gaps", de: "vier Bodenbretter 93 mm mit 4 mm Fugen" }, { en: "four 96 mm floor boards laid tight as two alternating pairs", de: "vier dicht verlegte Bodenbretter 96 mm als zwei abwechselnde Paare" }, "1:50 decision");
  },
  ...Object.fromEntries(BLOCKS.map((id) => [id, (asset: any): void => blocks(asset, 500, MASS(200))])),
  "used-park-armchair": (asset) => blocks(asset, 500, { en: "The joint and mass markers are blocks, not ellipsoids as at 1:200, so that the structure list has fewer triangles than at 1:200.", de: "Die Gelenk- und Massenmarken sind Blöcke, nicht Ellipsoide wie bei 1:200, damit die Tragwerksliste weniger Dreiecke hat als bei 1:200." }, ["mass", "joint"]),
  "used-backrest-park-bench": (asset) => {
    cylinders(asset, 500);
    blocks(asset, 500, { en: "The bolt and dowel joints are cylinders and the mass markers blocks, not capsules and ellipsoids as at 1:200, so that the structure list has fewer triangles than at 1:200.", de: "Die Schrauben- und Dübelgelenke sind Zylinder und die Massenmarken Blöcke, nicht Kapseln und Ellipsoide wie bei 1:200, damit die Tragwerksliste weniger Dreiecke hat als bei 1:200." });
  },
  "room-thermostat": (asset) => {
    blocks(asset, 100);
    sentence(saying(volume(asset, 100), /the knob's mass as a/u), { en: "the knob's mass as a 15 mm ball at its centre inside the knob.", de: "die Masse des Knopfs als 15-mm-Kugel in seiner Mitte." }, { en: "the knob's mass as a 15 mm block at its centre inside the knob, because a ball there has more triangles than the whole structure list of 1:50.", de: "die Masse des Knopfs als 15-mm-Block in seiner Mitte, weil eine Kugel dort mehr Dreiecke hat als die ganze Tragwerksliste von 1:50." }, "1:100 decision");
  },
  "ceiling-diffuser": (asset) => {
    blocks(asset, 200, { en: "The mass marker is a block, not an ellipsoid as at 1:100, so that the structure list has fewer triangles than at 1:100.", de: "Die Massenmarke ist ein Block, nicht ein Ellipsoid wie bei 1:100, damit die Tragwerksliste weniger Dreiecke hat als bei 1:100." });
    blocks(asset, 500, { en: "The mass marker is a block as at 1:200.", de: "Die Massenmarke ist ein Block wie bei 1:200." });
  },
  "kitchen-extractor-hood": (asset) => blocks(asset, 200, { en: "The fan unit mass is a block as at 1:500, not an ellipsoid, so that the structure list has fewer triangles than at 1:100.", de: "Die Masse der Lüftereinheit ist ein Block wie bei 1:500, kein Ellipsoid, damit die Tragwerksliste weniger Dreiecke hat als bei 1:100." }, ["mass"], ["mass-fan-unit"]),
  "solid-brick-wall-1960": (asset) => {
    const v = volume(asset, 200);
    alternate(v.body, "bond-band", "1:200 body");
    alternate(v.structure, "member-bond-band", "1:200 structure");
    sentence(v.decisions[0]!, { en: "over the full 400 mm.", de: "über die vollen 400 mm." }, { en: `over the full 400 mm${TOUCH.en}.`, de: `über die vollen 400 mm${TOUCH.de}.` }, "1:200 decision");
  },
  ...Object.fromEntries(["etics-brick-wall-12", "etics-brick-wall-30"].map((id) => [id, (asset: any): void => {
    const v = volume(asset, 200);
    alternate(v.body, "bond-band", "1:200 body");
    sentence(v.decisions[0]!, { en: "with the plaster in them;", de: "samt Innenputz;" }, { en: `with the plaster in them${TOUCH.en};`, de: `samt Innenputz${TOUCH.de};` }, "1:200 decision");
  }])),
  "used-wall-unit": (asset) => {
    for (const [scale, id] of [[50, "blankets"], [100, "blankets-100"], [200, "blankets-200"], [500, "blankets-500"]] as const) {
      const v = volume(asset, scale), outer = find(v.body, id, `1:${scale} body`), mid = id.replace("blankets", "blankets-mid");
      set(outer, "repeat", { count: 2, translation: [0, 0, 1.8] }, `1:${scale} ${id}`);
      const { repeat: _, ...single } = structuredClone(outer);
      after(v.body, id, { ...single, id: mid, from: -0.45, to: 0.45 }, `1:${scale} body`);
      after(v.thermal, "th-blankets", { id: "th-blankets-mid", of: mid, region: "blankets" }, `1:${scale} thermal`);
      after(v.structure, "m-blankets", { id: "m-blankets-mid", of: mid, member: "blankets", part: "member" }, `1:${scale} structure`);
      append(saying(v, /blanket/u), { en: "The three blankets are two solids, the outer pair and the middle one, so that no solid has copies that share a face.", de: "Die drei Decken sind zwei Körper, das äußere Paar und die mittlere, damit kein Körper Kopien mit gemeinsamer Fläche hat." }, `1:${scale} decision`);
    }
  },
  "used-bicycle-shelter": (asset) => {
    for (const [scale, count] of [[100, 4], [200, 2], [500, 2]] as const) {
      const v = volume(asset, scale);
      for (const id of ["spokes-rear", "spokes-front"]) {
        const rod = find(v.body, id, `1:${scale} body`), [ox, oy, oz] = rod.repeat.origin as number[], [x, y, z] = rod.p1 as number[];
        set(rod, "p0", [Number((2 * ox! - x!).toFixed(6)), Number((2 * oy! - y!).toFixed(6)), Number((2 * oz! - z!).toFixed(6))], `1:${scale} ${id}`);
        set(rod, "repeat", { ...rod.repeat, count }, `1:${scale} ${id}`);
      }
    }
    append(volume(asset, 100).decisions[0]!, { en: "The eight spokes of a wheel are four rods through the axle.", de: "Die acht Speichen eines Laufrads sind vier Stäbe durch die Achse." }, "1:100 decision");
    append(volume(asset, 200).decisions[1]!, { en: "The four spokes of a wheel are two rods through the axle.", de: "Die vier Speichen eines Laufrads sind zwei Stäbe durch die Achse." }, "1:200 decision");
    sentence(volume(asset, 500).decisions[0]!, { en: "with four texture spokes each", de: "mit je vier Textur-Speichen" }, { en: "with four texture spokes each as two rods through the axle", de: "mit je vier Textur-Speichen als zwei Stäbe durch die Achse" }, "1:500 decision");
  },
};
