import { load, save } from "./lib.ts";
const clone = (x: any) => JSON.parse(JSON.stringify(x));
const NOTE = {
  en: " Conventional symbol: it is not drawn to scale and carries no body dimension in the side, front or top view; the top view repeats the side view because a position mark looks the same from above, the front view shows the heading as a ring around the dot (the arrow points at the viewer).",
  de: " Konventionelles Symbol: Es ist nicht massstäblich gezeichnet und trägt weder in der Seiten-, Vorder- noch Draufsicht ein Körpermaß; die Draufsicht wiederholt die Seitenansicht, da eine Positionsmarke von oben gleich aussieht, die Vorderansicht zeigt die Richtung als Ring um den Punkt (der Pfeil zeigt auf die Betrachtenden).",
};
const NOTE_STATIC = {
  en: " Conventional symbol: it is not drawn to scale and carries no body dimension; the dot looks the same in the side, front and top view because a seated person does not travel.",
  de: " Konventionelles Symbol: Es ist nicht massstäblich gezeichnet und trägt kein Körpermaß; der Punkt sieht in Seiten-, Vorder- und Draufsicht gleich aus, da eine sitzende Person nicht läuft.",
};
for (const [slug, travels] of [["jogger", true], ["running-child", true], ["walking-commuter", true], ["wheelchair-user", true], ["seated-office-worker", false], ["seated-reader", false]] as const) {
  const d = load(slug); const sym = d.asset.symbols[0];
  if (sym.geometry.front && sym.geometry.plan) continue;
  const side = sym.geometry.elevation;
  const dot = side.find((s: any) => s.kind === "ellipse");
  sym.geometry.plan = clone(side).map((s: any) => ({ ...s, id: `plan-${s.id}` }));
  sym.geometry.front = travels
    ? [{ ...clone(dot), id: `front-${dot.id}` }, { ...clone(dot), id: "front-heading-ring", role: "detail", z: dot.z + 0.1, filled: false, rx: +(dot.rx * 1.9).toFixed(3), ry: +(dot.ry * 1.9).toFixed(3) }]
    : [{ ...clone(dot), id: `front-${dot.id}` }];
  const note = travels ? NOTE : NOTE_STATIC;
  sym.dimensionalNote = { en: sym.dimensionalNote.en + note.en, de: sym.dimensionalNote.de + note.de };
  save(slug, d); console.log(slug, "symbol views", Object.keys(sym.geometry).join(","));
}
