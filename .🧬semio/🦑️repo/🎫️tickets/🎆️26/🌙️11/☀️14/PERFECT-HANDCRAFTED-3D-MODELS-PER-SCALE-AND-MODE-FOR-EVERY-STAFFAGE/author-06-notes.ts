import type { Note } from "./author-06-lib.ts";

type Facts = { en: string; de: string; f50: Note; f100: Note; f200?: Note; f500?: Note };

const F: Record<string, Facts> = {
  "mallard-duck": {
    en: "mallard", de: "Stockente",
    f50: { en: "the round body with the chestnut breast, the three neck lofts with the white ring, the head with the flat bill, the folded wings as flat plate lofts on their bones, the tail and the webbed feet with thigh, shank, tarsus and foot", de: "der runde Körper mit der kastanienbraunen Brust, die drei Halslofts mit dem weissen Ring, der Kopf mit dem flachen Schnabel, die angelegten Flügel als flache Plattenlofts auf ihren Knochen, der Schwanz und die Schwimmfüsse mit Oberschenkel, Unterschenkel, Lauf und Fuss" },
    f100: { en: "the speculum, the neck ring, the thigh and the toe lines are merged, the neck is two lofts, each leg is one stroke tube along its bone chain", de: "Spiegel, Halsring, Oberschenkel und Zehenlinie sind verschmolzen, der Hals besteht aus zwei Lofts, jedes Bein ist eine Strichröhre entlang seiner Knochenkette" },
    f200: { en: "the neck and head are merged to fewer lofts, the wings are one plate each and the legs are minimal tubes of 30 mm", de: "Hals und Kopf sind zu weniger Lofts verschmolzen, die Flügel sind je eine Platte und die Beine minimale Röhren von 30 mm" },
  },
  pig: {
    en: "pig", de: "Schwein",
    f50: { en: "the barrel, ham, shoulder, the wedge head with the snout disc and flap ears, jaw, eye, curled tail and four-segment legs with wedge hooves", de: "die Tonne, Schinken, Schulter, der Keilkopf mit der Rüsselscheibe und den Lappenohren, Kiefer, Auge, Ringelschwanz und viersegmentige Beine mit Keilklauen" },
    f100: { en: "eye, mouth line and jaw are merged into the head and the tail root into the hindquarter", de: "Auge, Maullinie und Kiefer sind in den Kopf, die Schwanzwurzel in die Hinterhand verschmolzen" },
    f200: { en: "ears are one flap plus a far-ear ellipsoid, snout and tail are merged", de: "die Ohren sind ein Lappen plus ein Ellipsoid für das ferne Ohr, Rüssel und Schwanz sind verschmolzen" },
    f500: { en: "two barrel masses, neck, head wedge and eight 0.1 m leg tubes", de: "zwei Tonnenmassen, Hals, Kopfkeil und acht Beinröhren von 0,1 m" },
  },
  pigeon: {
    en: "pigeon", de: "Taube",
    f50: { en: "the plump body and breast, the neck with its sheen, the head with beak, cere, jaw and eye, the wing split into humerus, forearm coverts and primaries, the fanned tail and the legs with thigh, shank, tarsus and toes", de: "der rundliche Körper und die Brust, der Hals mit dem Glanz, der Kopf mit Schnabel, Wachshaut, Kiefer und Auge, der Flügel geteilt in Oberarm, Unterarmdecken und Handschwingen, der gefächerte Schwanz und die Beine mit Oberschenkel, Unterschenkel, Lauf und Zehen" },
    f100: { en: "feather lines, eye ring and wing bar are dropped and the toes are merged into the tarsus", de: "Federlinien, Augenring und Flügelbinde entfallen und die Zehen sind im Lauf verschmolzen" },
    f200: { en: "wing, neck, head and tail are single masses and the legs are minimal tubes", de: "Flügel, Hals, Kopf und Schwanz sind einzelne Massen und die Beine minimale Röhren" },
  },
  "red-fox": {
    en: "red fox", de: "Rotfuchs",
    f50: { en: "the slim barrel, the long neck, the pointed head with ears, jaw and nose, the thick brush as three tail lofts and four-segment legs with paws", de: "die schlanke Tonne, der lange Hals, der spitze Kopf mit Ohren, Kiefer und Nase, die dichte Lunte als drei Schwanzlofts und viersegmentige Beine mit Pfoten" },
    f100: { en: "the tail is merged to fewer lofts and the facial details are dropped", de: "der Schwanz ist zu weniger Lofts verschmolzen und die Gesichtsdetails entfallen" },
    f200: { en: "legs and ears are reduced to the minimal printable widths and the body to the merged masses of the drawing", de: "Beine und Ohren sind auf die kleinsten druckbaren Breiten reduziert und der Körper auf die verschmolzenen Massen der Zeichnung" },
  },
  "red-squirrel": {
    en: "red squirrel", de: "Eichhörnchen",
    f50: { en: "the sitting body with its two spine segments, the head with ear tufts, the bushy tail, arms with hands and legs with the long hind feet", de: "der sitzende Körper mit seinen zwei Wirbelsäulensegmenten, der Kopf mit Ohrpinseln, der buschige Schwanz, Arme mit Händen und Beine mit den langen Hinterfüssen" },
    f100: { en: "limbs and head details are merged into the body and tail masses", de: "Gliedmassen und Kopfdetails sind in die Körper- und Schwanzmassen verschmolzen" },
  },
  "roe-deer": {
    en: "roe deer", de: "Reh",
    f50: { en: "the slim barrel with the white rump patch, the long neck, the small head with ears, eye and nose, and the four long legs with thigh, shank, cannon and hoof", de: "die schlanke Tonne mit dem weissen Spiegel, der lange Hals, der kleine Kopf mit Ohren, Auge und Nase und die vier langen Beine mit Oberschenkel, Unterschenkel, Lauf und Klaue" },
    f100: { en: "rump patch, eye and nose are merged into the masses", de: "Spiegel, Auge und Nase sind in die Massen verschmolzen" },
    f200: { en: "ears and legs are minimal tubes and the neck and head are single masses", de: "Ohren und Beine sind minimale Röhren und Hals und Kopf einzelne Massen" },
    f500: { en: "body, neck and head masses plus eight leg tubes of 75 mm", de: "Körper-, Hals- und Kopfmassen plus acht Beinröhren von 75 mm" },
  },
  sheep: {
    en: "sheep", de: "Schaf",
    f50: { en: "the woolly barrel, the fleece neck, the short-haired head with ears, the wool-covered upper legs and the bare lower legs with hooves", de: "die wollige Tonne, der Vlieshals, der kurzhaarige Kopf mit Ohren, die wollbedeckten Oberschenkel und die kahlen Unterbeine mit Klauen" },
    f100: { en: "fold lines, eye and mouth are dropped and the hooves are merged into the lower legs", de: "Faltenlinien, Auge und Maul entfallen und die Klauen sind in die Unterbeine verschmolzen" },
    f200: { en: "ears, head and neck are single masses and the tail is one stroke", de: "Ohren, Kopf und Hals sind einzelne Massen und der Schwanz ein Strich" },
    f500: { en: "two fleece masses, neck, head and eight leg tubes", de: "zwei Vliesmassen, Hals, Kopf und acht Beinröhren" },
  },
};

export function notesFor(slug: string): Record<number, Note[]> {
  const f = F[slug]!;
  const mk = (n: number, scaleEn: string, scaleDe: string): Note[] => [{ en: `Body of the ${f.en} at 1:${n}: ${scaleEn}`, de: `Körper der ${f.de} bei 1:${n}: ${scaleDe}` }];
  const lofts = (n: number) => ({ en: `Every drawn shape is a loft along its bone with sections from the elevation outline (height, centre) and the plan and front outlines (width, offset); ${n} sections per loft.`, de: `Jede gezeichnete Form ist ein Loft entlang ihres Knochens mit Querschnitten aus dem Umriss der Ansicht (Höhe, Mitte) und den Umrissen von Grundriss und Vorderansicht (Breite, Versatz); ${n} Querschnitte je Loft.` });
  const o: Record<number, Note[]> = {
    50: mk(50, `${lofts(18).en} It carries ${f.f50.en}. Open strokes become tubes, bones drawn without a shape get a core tube. Near and far limbs are separate solids on their own bones.`, `${lofts(18).de} Er trägt ${f.f50.de}. Offene Striche werden zu Röhren, Knochen ohne gezeichnete Form erhalten eine Kernröhre. Nahe und ferne Gliedmassen sind getrennte Körper auf ihren eigenen Knochen.`),
    100: mk(100, `modelled anew against the 1:100 drawings with 13 sections per loft: ${f.f100.en}.`, `neu gegen die 1:100-Zeichnungen modelliert mit 13 Querschnitten je Loft: ${f.f100.de}.`),
  };
  if (f.f200) o[200] = mk(200, `modelled anew against the 1:200 drawings with 9 sections per loft: ${f.f200.en}.`, `neu gegen die 1:200-Zeichnungen modelliert mit 9 Querschnitten je Loft: ${f.f200.de}.`);
  if (f.f500) o[500] = mk(500, `modelled anew against the 1:500 drawings with 7 sections per loft: ${f.f500.en}; nothing is thinner than 75 mm.`, `neu gegen die 1:500-Zeichnungen modelliert mit 7 Querschnitten je Loft: ${f.f500.de}; nichts ist dünner als 75 mm.`);
  return o;
}
