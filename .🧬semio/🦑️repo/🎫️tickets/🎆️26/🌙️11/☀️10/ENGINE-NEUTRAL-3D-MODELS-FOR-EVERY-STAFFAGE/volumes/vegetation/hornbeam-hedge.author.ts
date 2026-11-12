import { Author } from "../../veg-lib.ts";
const a = new Author("hornbeam-hedge");
for (const n of [1, 2, 3, 4, 5, 6]) {
  const e = a.box2("elevation", `block-${n}`), f = a.box2("front", `block-${n}`);
  const cx = (e.minX + e.maxX) / 2;
  a.box(`block-${n}`, `block-${n}`, [cx, (e.minY + e.maxY) / 2, 0], [e.maxX - e.minX, e.maxY - e.minY, f.maxX - f.minX], a.meta(`block-${n}`), { cornerRadius: 0.1 });
  const c = a.box2("elevation", `core-${n}`);
  a.box(`core-${n}`, `block-${n}`, [(c.minX + c.maxX) / 2, (c.minY + c.maxY) / 2, 0], [c.maxX - c.minX, c.maxY - c.minY, 0.4], a.meta(`core-${n}`, { material: "hornbeam-leaf", region: "leaf-core" }), { cornerRadius: 0.08 });
  for (const s of ["a", "b"]) {
    const sb = a.box2("elevation", `stem-${n}-${s}`);
    a.cap(`stem-${n}-${s}`, `stem-${n}`, [(sb.minX + sb.maxX) / 2, 0, s === "a" ? -0.06 : 0.06], [(sb.minX + sb.maxX) / 2, 0.42, s === "a" ? -0.06 : 0.06], 0.045, 0.04, a.meta(`stem-${n}-${s}`));
  }
}
a.note("A trimmed hornbeam hedge of six rounded blocks, each a box with 0.1 m corner radius bound to its own block bone and as wide as its elevation outline (0.64 m, overlapping its neighbours by 0.14 m) and as deep as the front outline (0.7 m); a smaller denser core box per block carries the leaf-core thermal region.", "Eine geschnittene Hainbuchenhecke aus sechs gerundeten Blöcken, je eine Box mit 0,1 m Eckenradius am eigenen Blockknochen, so breit wie ihr Umriss in der Ansicht (0,64 m, 0,14 m Überlappung mit den Nachbarn) und so tief wie der Front-Umriss (0,7 m); eine kleinere dichtere Kernbox je Block trägt die thermische Region Blattkern.");
a.note("Two stems per block stand as thin capsules at the lateral positions +-0.06 m (the front view draws none; the pairs are offset in depth so they read as two stems).", "Zwei Stämme je Block stehen als dünne Kapseln bei +-0,06 m Seitenlage (die Front zeichnet keine; die Paare sind in der Tiefe versetzt, damit sie als zwei Stämme lesbar sind).");
a.save();
