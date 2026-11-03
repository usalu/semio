import { load, save } from "../humans/lib.ts";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const jobs: [string, number][] = [["courier-with-hand-truck", 500], ["running-child", 500], ["seated-reader", 500], ["worker-carrying-box", 500], ["child-on-kick-scooter", 200], ["cyclist", 200]];
const write = process.argv[2] === "write";
const f2 = (v: number) => v.toFixed(2);
for (const [slug, den] of jobs) {
  const doc = load(slug); const a = doc.asset; const rep = a.representations[String(den)];
  const start = rep.pens.body;
  const suit = () => measureSuitability(a, rep, den, DEFAULT_PRINT_PROFILE);
  let s = suit(); const overrides: string[] = [];
  for (let i = 0; i < 12 && !s.suitable; i++) {
    rep.pens.body = +(rep.pens.body + 0.01).toFixed(3);
    s = suit();
    if (rep.pens.body > start + 0.05) break;
  }
  if (!s.suitable) {
    rep.pens.body = start + 0.03;
    s = suit();
    for (let round = 0; round < 8 && !s.suitable; round++) {
      for (const v of s.violations) for (const id of v.shapes) for (const view of ["elevation", "front", "plan"]) for (const sh of rep.geometry[view]) if (sh.id === id) { sh.penMm = +((sh.penMm ?? rep.pens.body) + 0.03).toFixed(3); if (!overrides.includes(id)) overrides.push(id); }
      s = suit();
    }
  }
  console.log(slug, den, "pen", start, "->", rep.pens.body, "overrides", overrides.join(",") || "-", "suitable", s.suitable, s.violations.map((v: any) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" "));
  if (s.suitable && write) {
    const add = { en: ` Print check: the body pen is ${f2(rep.pens.body)} mm${overrides.length ? ` and the parts ${overrides.join(", ")} are drawn with a heavier pen` : ""}, which lifts the thinnest forms to the 0.2 mm minimum feature width without changing any dimension.`, de: ` Druckprüfung: der Körperstift beträgt ${f2(rep.pens.body).replace(".", ",")} mm${overrides.length ? ` und die Teile ${overrides.join(", ")} sind mit einem stärkeren Stift gezeichnet` : ""}, wodurch die dünnsten Formen die Mindestmerkmalbreite von 0,2 mm erreichen, ohne ein Maß zu ändern.` };
    rep.decisions.thinElements.en += add.en; rep.decisions.thinElements.de += add.de;
    save(slug, doc);
  }
}
