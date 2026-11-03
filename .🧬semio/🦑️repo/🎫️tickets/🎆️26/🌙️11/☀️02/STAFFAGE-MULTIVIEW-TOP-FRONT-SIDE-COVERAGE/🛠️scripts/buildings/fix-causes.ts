import { load, save, text } from "./lib.ts";

type Row = { id: string; nameEn: string; nameDe: string; w: number; h: number; partEn: string; partDe: string; size: number; art?: string; nart?: string };

const f2 = (v: number) => v.toFixed(2);
const de2 = (v: number) => v.toFixed(2).replace(".", ",");

const rows: Row[] = [
  { id: "ceiling-diffuser", nameEn: "diffuser", nameDe: "Auslass", w: 1.19, h: 0.24, partEn: "flange", partDe: "Flansch", art: "der", size: 0.012 },
  { id: "circulation-pump", nameEn: "pump", nameDe: "Pumpe", nart: "die", w: 0.36, h: 0.3, partEn: "status LED", partDe: "Status-LED", size: 0.009 },
  { id: "decentralised-hrv-fan", nameEn: "fan unit", nameDe: "Lüftergerät", nart: "das", w: 0.52, h: 0.52, partEn: "mounting screws", partDe: "Befestigungsschrauben", size: 0.014 },
  { id: "duct-fan", nameEn: "fan", nameDe: "Ventilator", w: 0.44, h: 0.44, partEn: "mounting plate", partDe: "Montageplatte", size: 0.012 },
  { id: "electricity-meter", nameEn: "meter", nameDe: "Zähler", w: 0.34, h: 0.57, partEn: "register digits", partDe: "Zählwerkziffern", size: 0.011 },
  { id: "gas-meter", nameEn: "meter", nameDe: "Zähler", w: 0.48, h: 0.54, partEn: "dial needle", partDe: "Zeiger", art: "der", size: 0.012 },
  { id: "gas-boiler", nameEn: "boiler", nameDe: "Kessel", w: 0.88, h: 1.44, partEn: "control knobs", partDe: "Reglerknöpfe", size: 0.01 },
  { id: "panel-radiator", nameEn: "radiator", nameDe: "Heizkörper", w: 0.27, h: 1.42, partEn: "wall brackets", partDe: "Wandkonsolen", size: 0.012 },
];

for (const r of rows) {
  const doc = load(r.id);
  const rep = doc.asset.representations["500"];
  const mm = r.size * 2;
  rep.reason = text(
    `At 1:500 the ${r.nameEn} is ${f2(r.w)} mm by ${f2(r.h)} mm on paper, well under the 3 mm a recognisable drawing needs, and its smallest drawn part, the ${r.partEn} of ${r.size * 1000} mm, is ${f2(mm)} mm, below the 0.2 mm office-laser minimum feature; draw it at 1:200 or larger.`,
    `Bei 1:500 ist ${r.nart ?? "der"} ${r.nameDe} ${de2(r.w)} mm mal ${de2(r.h)} mm groß auf Papier, deutlich unter den 3 mm, die eine erkennbare Zeichnung braucht, und sein kleinstes gezeichnetes Teil, ${r.art ?? "die"} ${r.partDe} von ${r.size * 1000} mm, misst ${de2(mm)} mm, unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 oder größer zeichnen.`,
  );
  rep.limitingFeature = text(`${r.partEn[0]!.toUpperCase()}${r.partEn.slice(1)} of ${r.size * 1000} mm`, `${r.partDe} von ${r.size * 1000} mm`);
  rep.limitingSizeM = r.size;
  save(r.id, doc);
}
