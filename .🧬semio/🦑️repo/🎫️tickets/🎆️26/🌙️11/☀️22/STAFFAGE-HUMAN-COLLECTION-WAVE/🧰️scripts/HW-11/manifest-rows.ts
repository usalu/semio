/** 📥️ Writes the manifest rows of the HW-11 humans to T/📥️manifest-rows/<id>.json, every value derived from the evaluated asset (power by `totalPowerW`).
 * @see ../../📋️brief.md — "Manifest" */
import { join, resolve } from "node:path";
import { totalPowerW, validateManifest, type Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { readAsset, t } from "./lib.ts";

const OUT = resolve(import.meta.dir, "../../📥️manifest-rows");
const distinction: Record<string, { en: string; de: string }> = {
  "waving-person": t("A slim 1.69 m adult in a fleece hooded sweatshirt, jeans, canvas sneakers and a nylon belt bag in front of the hips, greeting with the right forearm upright and the open palm forward at 1.7 met; also hailing with the arm stretched up, waving with both arms and standing relaxed, with a wave loop, a hail-and-wave gesture and idle breathing.", "Eine schlanke erwachsene Person von 1,69 m in Fleece-Kapuzenpullover, Jeans, Stoffsneakern und Nylon-Gürteltasche vor der Hüfte, die mit aufrechtem rechtem Unterarm und nach vorn gedrehter offener Handfläche bei 1,7 met grüßt; außerdem Herbeirufen mit hochgestrecktem Arm, Winken mit beiden Armen und entspanntes Stehen, mit einer Winkschleife, einer Geste aus Herbeirufen und Winken und ruhigem Atmen."),
  "photographer-with-camera": t("A 1.75 m press photographer in a softshell jacket, cotton trousers, a cap turned backwards and a press pass, with a 1.25 kg digital SLR at the right eye: standing, in a deep crouch on flat feet, checking the screen and standing ready, both hands placed on the camera by inverse kinematics; a burst loop, a crouch-and-shoot sequence and idle breathing; 4.5 W of camera input.", "Eine 1,75 m große Pressefotografin bzw. ein Pressefotograf in Softshelljacke, Baumwollhose, verkehrt herum getragener Mütze und mit Presseausweis, eine 1,25 kg schwere digitale Spiegelreflexkamera am rechten Auge: stehend, in tiefer Hocke auf flachen Füßen, beim Prüfen des Displays und bereit stehend, beide Hände durch inverse Kinematik an die Kamera gesetzt; eine Serienaufnahme als Schleife, eine Abfolge aus Hocke und Aufnahme und ruhiges Atmen; 4,5 W Leistungsaufnahme der Kamera."),
  "person-taking-selfie": t("A 24-year-old woman of 1.69 m in a hooded sweatshirt, slim jeans, canvas sneakers and a 15-litre daypack, holding her smartphone at arm's length with the screen turned to her face and the other hand on the hip: at arm's length, from above, with the sight behind her and checking the photo, with a posing loop, a take-and-check sequence and idle breathing.", "Eine 24-jährige Frau von 1,69 m in Kapuzenpullover, enger Jeans, Stoffsneakern und mit 15-Liter-Tagesrucksack, die ihr Smartphone mit ausgestrecktem Arm hält, den Bildschirm zum Gesicht gedreht, die andere Hand an der Hüfte: mit ausgestrecktem Arm, von oben, mit der Sehenswürdigkeit dahinter und beim Ansehen des Fotos, mit einer Posierschleife, einer Abfolge aus Aufnehmen und Ansehen und ruhigem Atmen."),
};
for (const id of Object.keys(distinction)) {
  const a: Asset = await readAsset(id);
  const row = {
    id: a.id, category: a.category, title: a.title, heightM: a.dimensions.heightM, widthM: a.dimensions.widthM, powerW: Math.round(totalPowerW(a) * 10) / 10,
    poses: a.poses.filter((p) => !p.key).map((p) => p.id), clips: a.clips.map((c) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((d) => [d, (a.representations as any)[d].status])),
    symbols: a.symbols.map((s) => s.id), analytics: [...(a.thermal.regions.length > 0 ? ["thermal"] : []), ...(a.thermal.emitters.length > 0 ? ["transport"] : []), "structural"],
    exports: a.exports, distinction: distinction[id], completion: a.completion,
  };
  const issues = validateManifest({ categories: [], collections: [], entries: [row] } as never, [a]).filter((i) => !/category-missing|manifest-count/.test(i.code));
  console.log(id, row.powerW, "W", issues.length === 0 ? "manifest ok" : issues.map((i) => i.code).join(","));
  await Bun.write(join(OUT, `${id}.json`), JSON.stringify(row, null, 2) + "\n");
}
