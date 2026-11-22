/** 📥️ Writes the manifest row of each HW-12 asset to `📥️manifest-rows/<id>.json`, every value derived from the evaluated asset (power from `totalPowerW`).
 *  bun manifest-rows.ts [slug ...] */
import { totalPowerW, validateManifest } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { HUMANS, TICKET, rd, T } from "./lib.ts";
const DISTINCTION: Record<string, ReturnType<typeof T>> = {
  "standing-violinist": T("A street violinist of 1.69 m in a hooded sweatshirt and jeans playing a full-size violin with a 750 mm bow at 2.5 met: mid-bow, frog on the E string, tip on the G string and holding between phrases, with a looping down-bow and up-bow and a breathing loop.", "Eine Straßengeigerin von 1,69 m in Kapuzensweatshirt und Jeans spielt bei 2,5 met eine ganze Geige mit 750 mm langem Bogen: Bogenmitte, Frosch auf der E-Saite, Spitze auf der G-Saite und Halten zwischen den Phrasen, mit einer Schleife aus Ab- und Aufstrich und einer Atemschleife."),
  "painter-at-easel": T("A 74-year-old plein-air painter in tweed jacket and flat cap at a 1.86 m beech A-frame easel with a 50 × 60 cm canvas, brush in the right hand and palette on the left thumb at 1.6 met: painting a stroke, mixing on the palette and looking, with a looping brush-stroke clip.", "Ein 74-jähriger Freilichtmaler in Tweedjacke und Schiebermütze an einer 1,86 m hohen A-Staffelei aus Buche mit einer Leinwand von 50 × 60 cm, Pinsel in der rechten Hand und Palette am linken Daumen bei 1,6 met: einen Strich malen, auf der Palette mischen und betrachten, mit einer Schleife aus Pinselstrichen."),
  dancer: T("A 1.82 m contemporary and ballet dancer in rehearsal wear: first arabesque, relevé in first, passé on demi-pointe and the fourth-position preparation, with a full pirouette about the ball of the supporting foot, a looping plié-and-relevé exercise at 5 met and a breathing loop.", "Ein Tänzer von 1,82 m für zeitgenössischen Tanz und Ballett in Probenkleidung: erste Arabeske, Relevé in der ersten Position, Passé auf halber Spitze und die Vorbereitung in der vierten Position, mit einer ganzen Pirouette um den Ballen des Standfußes, einer Schleife aus Plié und Relevé bei 5 met und einer Atemschleife."),
};
for (const slug of process.argv.slice(2).length ? process.argv.slice(2) : Object.keys(DISTINCTION)) {
  const a = (await Bun.file(`${HUMANS}/${slug}/🔣️.json`).json()).asset;
  const row = {
    id: a.id, category: a.category, title: a.title, heightM: a.dimensions.heightM, widthM: a.dimensions.widthM, powerW: rd(totalPowerW(a), 1),
    poses: a.poses.filter((p: any) => !p.key).map((p: any) => p.id), clips: a.clips.map((c: any) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, a.representations[k].status])), symbols: a.symbols.map((s: any) => s.id),
    analytics: ["thermal", "transport", "structural"], exports: a.exports, distinction: DISTINCTION[slug], completion: a.completion,
  };
  await Bun.write(`${TICKET}/📥️manifest-rows/${slug}.json`, JSON.stringify(row, null, 2) + "\n");
  const real = await Bun.file(`${HUMANS}/../🔣️.json`).json();
  const issues = validateManifest({ ...real, collections: [], entries: [row] } as never, [a] as never);
  console.log(slug, row.powerW, "W", JSON.stringify(issues).slice(0, 300));
}
