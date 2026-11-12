import { Author } from "../authoring-animals-lib.ts";

const a = new Author("pigeon");

a.fit("body", "body", { zc: 0, zh: 0.063, p: 0.3, ex: 2.2 });
for (const k of ["1", "2", "3"]) a.fit(`neck-${k}`, `neck-${k}`, { p: 0.4, ex: 2 });
a.fit("head", "head", { p: 0.4, ex: 2 });
a.fit("beak", "beak", { p: 0.4, ex: 2.2 });
a.fit("cere", "cere", { p: 0.4, ex: 2 });
a.fit("jaw", "jaw", { zc: 0, zh: 0.007, p: 0.4, ex: 2 });
a.fit("tail", "tail", { p: 0.3, ex: 2.2 });
a.fit("far-tail", "far-tail", { p: 0.3, ex: 2.2 });
a.fit("near-coverts", "near-coverts", { p: 0.2, ex: 2.2 });
a.fit("far-coverts", "near-coverts", { bone: "forearm-f", from: "far-coverts", p: 0.2, ex: 2.2 }, { tone: 0.5 });
a.fit("near-primaries", "near-primaries", { p: 0.2, ex: 2.2 });
a.fit("far-primaries", "far-primaries", { p: 0.2, ex: 2.2 });
for (const [n, z] of [["n", -1], ["f", 1]] as const) {
  a.ell(`eye-${n}`, "head", [0.1185, 0.1245, z * 0.022], [0.0055, 0.0065, 0.004], 0, ["eye"]);
  a.tube(`humerus-${n}`, `humerus-${n}`, [
    { p: [0.043, 0.039, z * 0.05], rz: 0.012, ry: 0.024 },
    { p: [0.0135, 0.033, z * 0.05], rz: 0.012, ry: 0.024 },
    { p: [-0.016, 0.027, z * 0.05], rz: 0.011, ry: 0.022 },
  ], ["near-coverts"], { region: "plumage-wings", tone: 0.5 });
  const side = n === "n" ? "near" : "far";
  for (const p of ["thigh", "shank", "tarsus", "toes"]) a.fit(`${side}-${p}`, `${side}-${p}`, { p: p === "toes" ? 0.3 : 0.5, ex: p === "toes" ? 2.4 : 2 });
}
a.decide("The body is a 0.126 m wide loft, neck in three lofts, head, beak, cere and lower beak lofts; eyes are ellipsoids. The wing is split like the rig: a short humerus plate, the covert plate on the forearm and the primary plate on the hand, all 0.03 to 0.05 m thick lofts lying in the u-v plane of their bones 0.05 m off the midline so abduction about v spreads them sideways; the far coverts reuse the near outline on the far forearm bone.", "Der Körper ist ein 0,126 m breiter Loft, der Hals besteht aus drei Lofts, Kopf, Schnabel, Wachshaut und Unterschnabel sind Lofts; die Augen sind Ellipsoide. Der Flügel ist wie die Rig geteilt: eine kurze Oberarmplatte, die Deckfederplatte am Unterarm und die Handschwingenplatte an der Hand, alles 0,03 bis 0,05 m dicke Lofts in der u-v-Ebene ihrer Bones 0,05 m neben der Mittellinie, sodass die Abduktion um v sie seitlich ausbreitet; die fernen Deckfedern übernehmen die nahe Kontur auf der fernen Unterarmbone.");
a.decide("The two tail fans are lofts on the tail and tail-f bones; legs are four lofts per side; eye ring, neck sheen and wing bar are drawn detail and not modelled.", "Die beiden Schwanzfächer sind Lofts auf den Bones tail und tail-f; die Beine bestehen aus vier Lofts je Seite; Augenring, Halsglanz und Flügelbinde sind Zeichnungsdetail und nicht modelliert.");
a.write();
