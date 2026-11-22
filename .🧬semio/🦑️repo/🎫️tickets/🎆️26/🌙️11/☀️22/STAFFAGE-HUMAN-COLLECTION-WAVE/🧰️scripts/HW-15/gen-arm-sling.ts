/** 🩹️ Generator of `person-with-arm-sling`: derived from the rig, outlines and volumes of `walking-commuter`; the shoulder bag is gone, the near (right) forearm rests in a short-arm fibreglass cast inside a pouch arm sling whose strap runs round the neck, and the far arm swings free.
 *  bun gen-arm-sling.ts
 * @see ./lib.ts
 * @see ./sling-design.ts — the cast, pouch and strap geometry and their drawings
 */
import { load, save, removeBones, removeRegion, removeShapes, find, clone, t } from "./lib.ts";
import { slingParts, SLING } from "./sling-design.ts";
import { slingTexts } from "./sling-texts.ts";

const doc = await load("👤️humans", "walking-commuter");
const a = doc.asset;
a.id = "person-with-arm-sling";

removeBones(a, ["bag"]);
a.anchors = a.anchors.filter((x: any) => x.id !== "strap-grip");
removeShapes(a, (s) => s.region === "bag-leather" || s.material === "leather-bag");
removeRegion(a, "bag-leather");
a.materials = a.materials.filter((m: any) => m.id !== "leather-bag");
for (const p of a.poses) p.loads = p.loads.filter((l: any) => l.id !== "shoulder-bag");

const ARM = SLING.offsets;
for (const p of a.poses) Object.assign(p.bones, clone(ARM));
for (const c of a.clips) for (const k of c.keys) if (k.bones) for (const id of Object.keys(ARM)) if (k.bones[id]) k.bones[id] = clone(ARM[id]);

slingParts(a);
slingTexts(a);

const left = find(a, "bag").concat(find(a, "bag-cog"), find(a, "strap-grip"), find(a, "leather-bag"), find(a, "bag-leather"));
if (left.length) console.log("[DEBUG] leftovers", left.slice(0, 20));
console.log(save(doc));
