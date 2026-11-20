/** 🧵️ The 1:100 body, the thermal model, the skeleton and the decisions of the round 3 kneeling gardener, stated by hand.
 * @see ./r3-kneeling-gardener-body.ts */
import { Kit, aboutW, inset, parts, quat, aboutV, thermalOf, type Json, type P2, type Row, type V3 } from "./r3-kneeling-gardener-kit.ts";
import { CANVAS, CLOG, FOAM, GLOVE, HAIR, HL, LINEN, PE, SIDES, SKIN, STEEL, STRAW, T, hf, m } from "./r3-kneeling-gardener-body.ts";

/* ------------------------------------------------------------------ body 1:100 */

/** 🥄️ The 1:100 blade section: the same 80 mm scoop drawn 15 mm deep overall so it still prints. */
const BLADE100: P2[] = [[-0.0235, -0.04], [-0.031, -0.02], [-0.0335, 0], [-0.031, 0.02], [-0.0235, 0.04], [-0.0185, 0.04], [-0.0185, -0.04]];

export function body100(k: Kit): void {
  k.box("pad", "pad", FOAM, T.pad, [-0.18, 0.02, 0], [0.76, 0.04, 0.4], 0.018, "pad");
  k.rev("trug", "trug", PE, T.tub, [[0, 0], [0, 0.17], [0.012, 0.187], [0.203, 0.2465], [0.211, 0.249], [0.211, 0.2415], [0.2, 0.2405], [0.014, 0.184], [0.004, 0.168], [0.004, 0]], [0, 1, 0], [0, 0, 0], 1, "trug");
  for (const [sfx, g] of SIDES) k.bar(`trug-handle-${sfx}`, "trug", PE, T.tub, [[-0.062, 0.196, 0.24 * g, 0.0078], [-0.05, 0.234, 0.242 * g, 0.0078], [0, 0.25, 0.242 * g, 0.0078], [0.05, 0.234, 0.242 * g, 0.0078], [0.062, 0.196, 0.24 * g, 0.0078]], "trug", "detail");
  k.ell("weeds-bed", "trug", PE, T.weeds, [0.005, 0.11, 0], [0.206, 0.105, 0.201], "trug");
  k.ell("weeds-heap-1", "trug", PE, T.weeds, [-0.06, 0.195, 0.04], [0.13, 0.066, 0.12], "trug", "body", aboutW(10));
  k.ell("weeds-heap-2", "trug", PE, T.weeds, [0.07, 0.19, -0.04], [0.12, 0.062, 0.115], "trug", "body", aboutW(-12));
  ([[8, 0.196, 0.234, -24, 0.074], [128, 0.186, 0.23, 0, 0.058], [262, 0.186, 0.23, 0, 0.058], [200, 0.064, 0.268, 0, 0.045]] as const).forEach(([deg, r, y, pitch, len], i) => k.ell(`weed-leaf-${i}`, "trug", PE, T.leaf, [r * Math.cos((deg * Math.PI) / 180), y, r * Math.sin((deg * Math.PI) / 180)], [len, 0.0078, len * 0.34], "trug", "detail", quat(aboutV(-deg), aboutW(pitch))));
  k.cap("weed-stalk-0", "trug", PE, T.leaf, [-0.02, 0.24, -0.1], [-0.035, 0.335, -0.1], 0.009, 0.0078, "trug", "detail");
  k.cap("weed-stalk-1", "trug", PE, T.leaf, [0.1, 0.22, 0.1], [0.14, 0.305, 0.1], 0.009, 0.0078, "trug", "detail");

  k.rev("trowel-handle", "trowel", STEEL, T.handle, [[-0.056, 0], [-0.054, 0.0125], [-0.03, 0.0162], [0.02, 0.014], [0.07, 0.0152], [0.076, 0]], [1, 0, 0], [0, 0, 0], 1, "trowel");
  k.bar("trowel-neck", "trowel", STEEL, T.steel, [[0.07, 0, 0, 0.0076], [0.09, -0.004, 0, 0.0076], [0.116, -0.022, 0, 0.0076]], "trowel", "detail");
  k.ext("trowel-blade", "trowel", STEEL, T.steel, BLADE100, 0.108, 0.2, [0.55, 1], "trowel");
  k.ext("trowel-blade-point", "trowel", STEEL, T.steel, BLADE100, 0.2, 0.26, [1, 0.2], "trowel");

  k.loft("trousers-seat", "pelvis", CANVAS, T.trousers, [[-0.1, 0.014, 0, 0.034, 0.03, 2.2], [-0.082, 0.02, 0, 0.1, 0.082, 2.2], [-0.05, 0.026, 0, 0.152, 0.116, 2.2], [-0.01, 0.028, 0, 0.172, 0.126, 2.2], [0.05, 0.016, 0, 0.167, 0.112, 2.3], [0.128, 0, 0, 0.147, 0.099, 2.3]], "trousers");
  k.loft("shirt", "torso", LINEN, T.shirt, [[-0.072, -0.02, 0, 0.17, 0.113, 2.5], [-0.02, -0.019, 0, 0.168, 0.12, 2.5], [0.06, -0.019, 0, 0.15, 0.123, 2.4], [0.16, -0.013, 0, 0.152, 0.125, 2.4], [0.26, -0.012, 0, 0.162, 0.14, 2.3], [0.33, 0, 0, 0.166, 0.128, 2.3], [0.4, 0.013, 0, 0.17, 0.097, 2.2], [0.432, 0.014, 0, 0.14, 0.078, 2.1], [0.455, 0.012, 0, 0.08, 0.065, 2], [0.47, 0.01, 0, 0.06, 0.058, 2]], "shirt-torso");
  k.loft("neck", "head", SKIN, T.skin, [[-0.035, 0.016, 0, 0.054, 0.052], [0.02, 0.009, 0, 0.048, 0.05], [0.08, 0, 0, 0.048, 0.055]], "face");
  k.loft("head", "head", SKIN, T.skin, [[0.0175, -0.066, 0, 0.02, 0.016, 2.2], [0.045, -0.044, 0, 0.045, 0.047, 2.2], [0.085, -0.014, 0, 0.06, 0.079], [0.125, -0.006, 0, 0.068, 0.089], [0.16, -0.003, 0, 0.0725, 0.092], [0.2, 0.001, 0, 0.064, 0.081], [0.2325, 0.002, 0, 0.02, 0.026]], "face");
  k.cap("nose", "head", SKIN, T.skin, [0.134, -0.09, 0], [0.099, -0.104, 0], 0.008, 0.0115, "face");
  k.loft("hair", "head", HAIR, T.hair, [[0.066, 0.07, 0, 0.03, 0.03], [0.092, 0.062, 0, 0.056, 0.05], [0.13, 0.03, 0, 0.076, 0.07], [0.176, 0.006, 0, 0.075, 0.087]], "hat");
  k.rev("hat-crown", "head", STRAW, T.hat, [[0.166, 0], [0.166, 0.1005], [0.21, 0.098], [0.245, 0.089], [0.262, 0.06], [0.267, 0]], [1, 0, 0], [0, 0.002, 0], 0.82, "hat");
  k.rev("hat-brim", "head", STRAW, T.hat, [[0.171, 0.074], [0.16, 0.191], [0.145, 0.1905], [0.156, 0.074], [0.171, 0.074]], [1, 0, 0], [0, 0.002, 0], 1, "hat");

  for (const [sfx, g] of SIDES) {
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, CANVAS, T.trousers, m([[-0.05, 0, -0.004, 0.06, 0.07, 2.2], [0.02, 0.006, -0.001, 0.087, 0.098, 2.2], [0.12, 0.01, 0, 0.085, 0.093], [0.26, 0.009, 0, 0.07, 0.077], [0.37, 0.007, 0, 0.06, 0.067], [0.42, 0.007, 0, 0.057, 0.064], [0.452, 0.004, 0, 0.045, 0.049], [0.4625, 0.002, 0, 0.03, 0.032]], g), "trousers");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, CANVAS, T.trousers, [[-0.052, 0.004, 0, 0.032, 0.032], [-0.02, 0.004, 0, 0.053, 0.057], [0.02, 0.003, 0, 0.057, 0.062], [0.12, -0.008, 0, 0.056, 0.064], [0.24, -0.006, 0, 0.049, 0.055], [0.352, -0.002, 0, 0.0455, 0.0505]], "trousers");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, CLOG, T.sock, [[0.325, -0.002, 0, 0.034, 0.039], [0.3961, -0.001, 0, 0.034, 0.039], [0.42, -0.001, 0, 0.03, 0.035]], "clogs");
    k.loft(`sock-foot-${sfx}`, `foot-${sfx}`, CLOG, T.sock, [[-0.056, -0.03, 0, 0.022, 0.026], [-0.02, -0.02, 0, 0.032, 0.039], [0.035, -0.02, 0, 0.036, 0.031], [0.075, -0.026, 0, 0.038, 0.022]], "clogs");
    k.loft(`clog-${sfx}`, `foot-${sfx}`, CLOG, T.clog, m([[-0.072, -0.054, 0, 0.024, 0.02], [-0.045, -0.05, 0, 0.037, 0.0254, 3], [-0.01, -0.0485, 0.001, 0.04, 0.0269, 3], [0.03, -0.039, 0.002, 0.043, 0.0364, 3], [0.08, -0.0375, 0.002, 0.047, 0.0379, 3], [0.13, -0.0422, 0.001, 0.051, 0.0332, 3]], g), "clogs", "body", 3);
    k.loft(`clog-toe-${sfx}`, `toe-${sfx}`, CLOG, T.clog, m([[-0.004, 0.0332, 0.001, 0.051, 0.0332], [0.035, 0.032, -0.002, 0.048, 0.031], [0.068, 0.028, -0.005, 0.036, 0.024], [0.0868, 0.023, -0.007, 0.016, 0.012]], g), "clogs", "body", 3);
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m([[-0.054, 0, -0.012, 0.014, 0.015], [-0.04, 0, -0.008, 0.037, 0.04], [-0.012, 0, -0.002, 0.054, 0.057], [0.03, 0.001, 0.003, 0.058, 0.061], [0.12, 0.003, 0.003, 0.053, 0.057], [0.19, 0.003, 0.002, 0.052, 0.056], [0.2, 0.003, 0.002, 0.055, 0.059], [0.232, 0.002, 0.002, 0.054, 0.058]], g), "shirt-sleeves");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.222, 0.002, 0.002, 0.043, 0.047], [0.2925, -0.003, 0, 0.04, 0.039], [0.318, -0.003, 0, 0.034, 0.033], [0.33, -0.002, 0, 0.022, 0.022]], g), "forearms");
  }
  k.loft("forearm-l", "forearm-l", SKIN, T.skin, [[-0.032, 0, 0, 0.022, 0.022], [-0.008, 0, 0, 0.038, 0.036], [0.045, 0.003, -0.006, 0.042, 0.041], [0.13, 0.002, -0.018, 0.036, 0.035], [0.22, 0, -0.03, 0.0275, 0.021]], "forearms");
  k.loft("glove-cuff-l", "forearm-l", GLOVE, T.glove, [[0.205, 0, -0.028, 0.0315, 0.0255, 2.3], [0.266, 0, -0.0365, 0.029, 0.021, 2.3]], "gloves");
  k.loft("hand-l", "hand-l", GLOVE, T.glove, [[-0.012, 0, HL, 0.029, 0.021, 2.4], [0.03, 0.001, HL - 0.002, 0.041, 0.019, 2.6], [0.085, 0, HL, 0.046, 0.016, 2.6], [0.135, -0.004, HL, 0.045, 0.0125, 2.6], [0.17, -0.014, HL, 0.04, 0.0105, 2.6], [0.184, -0.02, HL, 0.03, 0.0085, 2.6]], "gloves");
  k.cap("thumb-l", "hand-l", GLOVE, T.glove, [0.028, -0.002, HL - 0.034], [0.094, -0.014, HL - 0.068], 0.0125, 0.0095, "gloves");
  k.loft("forearm-r", "forearm-r", SKIN, T.skin, [[-0.032, 0, 0, 0.022, 0.022], [-0.008, 0, 0, 0.038, 0.036], [0.045, 0.003, 0, 0.041, 0.042], [0.13, 0.002, 0, 0.034, 0.037], [0.22, 0, 0, 0.021, 0.028]], "forearms");
  k.loft("glove-cuff-r", "forearm-r", GLOVE, T.glove, [[0.205, 0, 0, 0.0255, 0.0315, 2.3], [0.266, 0, 0, 0.021, 0.029, 2.3]], "gloves");
  k.ell("hand-r", "hand-r", GLOVE, T.glove, [0.046, -0.002, -0.004], [0.058, 0.047, 0.036], "gloves", "body", aboutW(18));
  k.cap("thumb-r", "hand-r", GLOVE, T.glove, hf(-0.006, 0.034, -0.012), hf(0.046, 0.004, -0.034), 0.0125, 0.01, "gloves");
}

/* ------------------------------------------------------------------ thermal */

/** 🧅️ Which layer shells a body solid gives at 1:50 when it is not the whole stack of its region: `outer` only the outermost layer, a layer id only that layer. */
const LAYER: [RegExp, string][] = [[/^hair/, "hair-pack"], [/^hat-(brim|band)$/, "outer"], [/^sock-/, "sock"], [/^clog-strap-/, "outer"], [/^(shirt-collar|sleeve-roll-|knee-patch-)/, "outer"]];

export function thermal(asset: Json, k: Kit, scale: number): Json[] {
  const regions = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  const out: Json[] = [];
  const t = new Kit();
  for (const s of k.solids) {
    if (scale !== 50 && s.id === "hair") continue;
    if (s.id === "head" || s.id === "neck" || /^(weeds-|weed-|sleeve-[lr]$)/.test(s.id)) continue;
    const region = regions.get(k.region.get(s.id)!)!;
    const layers: Json[] = region.layers ?? [];
    const only = LAYER.find(([re]) => re.test(s.id))?.[1];
    if (scale !== 50 || layers.length === 0 || only === undefined) out.push(...thermalOf(s, region, scale === 50));
    else out.push({ id: `th-${s.id}`, of: s.id, region: region.id, layer: only === "outer" ? layers[layers.length - 1]!.id : only });
  }
  const face: Row[] = scale === 50 ? [[0.0175, -0.068, 0, 0.016, 0.013, 2.2], [0.032, -0.055, 0, 0.036, 0.033, 2.2], [0.055, -0.045, 0, 0.05, 0.047, 2.2], [0.085, -0.042, 0, 0.06, 0.05], [0.12, -0.044, 0, 0.066, 0.05], [0.15, -0.056, 0, 0.066, 0.039], [0.168, -0.064, 0, 0.058, 0.03]] : [[0.0175, -0.066, 0, 0.02, 0.016, 2.2], [0.045, -0.05, 0, 0.045, 0.041, 2.2], [0.085, -0.042, 0, 0.06, 0.051], [0.125, -0.045, 0, 0.066, 0.05], [0.168, -0.064, 0, 0.058, 0.03]];
  out.push({ ...t.loft("th-face", "head", SKIN, T.skin, face), region: "face" });
  out.push({ ...t.loft("th-neck", "head", SKIN, T.skin, [[0.014, 0.01, 0, 0.049, 0.05], [0.04, 0.006, 0, 0.047, 0.05], [0.062, 0.003, 0, 0.0475, 0.052]]), region: "face" });
  const sleeves = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  for (const [sfx, g] of SIDES) {
    const cap = t.loft(`shoulder-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m(scale === 50 ? [[-0.054, 0, -0.012, 0.012, 0.013], [-0.047, 0, -0.01, 0.028, 0.03], [-0.034, 0, -0.007, 0.042, 0.045], [-0.014, 0, -0.003, 0.053, 0.056], [0.01, 0.001, 0.001, 0.058, 0.06], [0.04, 0.001, 0.003, 0.058, 0.061]] : [[-0.054, 0, -0.012, 0.014, 0.015], [-0.04, 0, -0.008, 0.037, 0.04], [-0.012, 0, -0.002, 0.054, 0.057], [0.04, 0.001, 0.003, 0.0575, 0.0605]], g));
    const arm = t.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m(scale === 50 ? [[0.04, 0.001, 0.003, 0.058, 0.061], [0.075, 0.002, 0.004, 0.056, 0.06], [0.13, 0.003, 0.003, 0.052, 0.057], [0.19, 0.003, 0.002, 0.05, 0.054], [0.205, 0.003, 0.002, 0.05, 0.054]] : [[0.04, 0.001, 0.003, 0.0575, 0.0605], [0.12, 0.003, 0.003, 0.053, 0.057], [0.19, 0.003, 0.002, 0.052, 0.056], [0.2, 0.003, 0.002, 0.055, 0.059], [0.232, 0.002, 0.002, 0.054, 0.058]], g));
    for (const [solid, region] of [[cap, "shirt-torso"], [arm, "shirt-sleeves"]] as const) {
      const layers: Json[] = sleeves.get(region)!.layers;
      if (scale !== 50) out.push({ ...solid, id: `th-${solid.id}`, region });
      else out.push({ ...solid, id: `th-${solid.id}-${layers[1]!.id}`, region, layer: layers[1]!.id }, { ...inset(solid, layers[1]!.thicknessM, `th-${solid.id}-${layers[0]!.id}`), region, layer: layers[0]!.id });
    }
  }
  const h = (0.0077 * scale) / 100;
  const e = (id: string, material: string, at: V3, radii: V3, region: string, emitter: string): Json => ({ ...t.ell(id, "head", material, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(e("th-nose-breath", SKIN, [0.0885, -0.1065 + h, 0], [h, h, Math.max(h, 0.008)], "face", "nose-breath"));
  out.push(e("th-mouth-breath", SKIN, [0.0655, -0.091 + h, 0], [h, h, Math.max(h, 0.0105)], "face", "mouth-breath"));
  out.push(e("th-plume", STRAW, [0.2665 - h, 0.002, 0], [h, 0.05, 0.041], "shirt-torso", "plume"));
  return out;
}

/* ------------------------------------------------------------------ structure */

const B = SKIN;
function masses(asset: Json): Json[] {
  const k = new Kit();
  const out: Json[] = [];
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < 0.1 * asset.dimensions.massKg) continue;
    const r = 0.011 * Math.cbrt(b.mass.kg);
    out.push(...parts([k.ell(`mass-${b.id}`, b.id, B, 0.4, [b.mass.comX, b.mass.comY ?? 0, 0], [r, r, r])], b.id, "mass"));
  }
  return out;
}
/** 🔩️ Joint solids at the real joints: balls at hip and shoulder, hinge pins through knee, elbow, ankle and the toe joints, discs between the vertebrae at L5/S1 and C7/T1, a short pin where the tang enters the handle; `f` enlarges them for a coarser scale. */
const JOINT: Record<string, [at: V3, radii: V3]> = { torso: [[0, 0.045, 0], [0.007, 0.02, 0.027]], head: [[0, 0.034, 0], [0.006, 0.016, 0.019]], thigh: [[0, 0, 0], [0.025, 0.025, 0.025]], shank: [[0, 0, 0], [0.013, 0.013, 0.042]], foot: [[0, 0, 0], [0.01, 0.01, 0.028]], toe: [[0, 0.028, 0], [0.0075, 0.0075, 0.042]], "upper-arm": [[0, 0, 0], [0.024, 0.024, 0.024]], forearm: [[0, 0, 0], [0.0095, 0.0095, 0.034]], hand: [[0, 0, 0], [0.0085, 0.012, 0.012]] };
function joints(f: number): Json[] {
  const k = new Kit();
  const ids = ["torso", "head", ...["thigh", "shank", "foot", "toe", "upper-arm", "forearm", "hand"].flatMap((b) => SIDES.map(([s]) => `${b}-${s}`))];
  return ids.flatMap((id) => {
    const [at, r] = JOINT[id.replace(/-[lr]$/, "")]!;
    const shift: V3 = id === "hand-l" ? [0, 0, HL] : [0, 0, 0];
    return parts([k.ell(`joint-${id}`, id, B, 0.5, [at[0] + shift[0], at[1] + shift[1], at[2] + shift[2]], [r[0] * f, r[1] * f, Math.max(r[2], r[2] * (1 + (f - 1) / 3))])], id, "joint");
  });
}
const SPINE: Row[] = [[-0.012, 0.045, 0, 0.023, 0.017], [0.07, 0.036, 0, 0.022, 0.017], [0.15, 0.046, 0, 0.02, 0.016], [0.27, 0.068, 0, 0.017, 0.015], [0.38, 0.058, 0, 0.015, 0.013], [0.462, 0.036, 0, 0.014, 0.012]];
const equipment = (fine: boolean): Json[] => {
  const k = new Kit();
  k.cap("sk-trowel-tang", "trowel", STEEL, T.steel, [-0.04, 0, 0], [0.072, 0, 0], fine ? 0.004 : 0.0076, fine ? 0.004 : 0.0076);
  return [{ id: "sk-pad", of: "pad", member: "pad", part: "member" }, { id: "sk-trug", of: "trug", member: "trug", part: "member" }, { id: "sk-trug-handle-l", of: "trug-handle-l", member: "trug", part: "member" }, { id: "sk-trug-handle-r", of: "trug-handle-r", member: "trug", part: "member" }, ...parts(k.solids, "trowel", "member"), { id: "sk-trowel-neck", of: "trowel-neck", member: "trowel", part: "member" }, ...(fine ? ["trowel-blade-shoulder", "trowel-blade", "trowel-blade-point"] : ["trowel-blade", "trowel-blade-point"]).map((id) => ({ id: `sk-${id}`, of: id, member: "trowel", part: "member" }))];
};

function structure50(asset: Json): Json[] {
  const out: Json[] = equipment(true);
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.154, 0.002, 0], [0.074, 0.088, 0.068]);
    k.loft("sk-face", "head", B, T.bone, [[0.07, -0.05, 0, 0.044, 0.036], [0.095, -0.05, 0, 0.052, 0.04], [0.125, -0.05, 0, 0.056, 0.04]]);
    k.loft("sk-mandible", "head", B, T.bone, [[0.02, -0.062, 0, 0.016, 0.011], [0.036, -0.05, 0, 0.034, 0.03], [0.058, -0.03, 0, 0.046, 0.046], [0.068, -0.024, 0, 0.047, 0.048]], undefined, "body", 2.4);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.034, 0, 0.015, 0.013], [0.028, 0.024, 0, 0.014, 0.012], [0.058, 0.018, 0, 0.013, 0.012], [0.09, 0.018, 0, 0.016, 0.013]]);
  });
  put("torso", (k) => {
    k.loft("sk-spine", "torso", B, T.bone, SPINE);
    for (const [i, s, cv, d, w] of [[1, 0.412, 0.016, 0.05, 0.068], [2, 0.376, 0.01, 0.067, 0.094], [3, 0.34, 0.004, 0.08, 0.109], [4, 0.304, 0, 0.088, 0.118], [5, 0.268, -0.002, 0.092, 0.123], [6, 0.232, -0.002, 0.092, 0.125], [7, 0.196, 0, 0.088, 0.123], [8, 0.16, 0.004, 0.08, 0.117]] as const) k.ring(`sk-rib-${i}`, "torso", B, T.bone, s, cv, d, w, 0.006, -14);
    k.cap("sk-sternum", "torso", B, T.bone, [0.238, -0.088, 0], [0.4, -0.058, 0], 0.011, 0.013);
    for (const [sfx, g] of SIDES) {
      k.cap(`sk-clavicle-${sfx}`, "torso", B, T.bone, [0.408, -0.05, 0.02 * g], [0.428, 0.012, 0.158 * g], 0.0072, 0.0068);
      k.ell(`sk-scapula-${sfx}`, "torso", B, T.bone, [0.335, 0.086, 0.09 * g], [0.07, 0.0075, 0.047], undefined, "body", aboutW(5));
      k.cap(`sk-acromion-${sfx}`, "torso", B, T.bone, [0.392, 0.074, 0.066 * g], [0.43, 0.022, 0.162 * g], 0.006, 0.0072);
    }
  });
  put("pelvis", (k) => {
    k.loft("sk-sacrum", "pelvis", B, T.bone, [[0.1, 0.045, 0, 0.03, 0.02], [0.055, 0.062, 0, 0.05, 0.017], [0.01, 0.072, 0, 0.028, 0.013], [-0.028, 0.066, 0, 0.011, 0.009]]);
    for (const [sfx, g] of SIDES) {
      k.loft(`sk-hip-bone-${sfx}`, "pelvis", B, T.bone, m([[0.124, 0.02, 0.122, 0.009, 0.06], [0.092, 0.022, 0.122, 0.011, 0.068], [0.046, 0.014, 0.108, 0.013, 0.046], [0, 0.006, 0.094, 0.023, 0.033], [-0.035, 0.024, 0.07, 0.012, 0.026], [-0.062, 0.034, 0.052, 0.011, 0.018]], g));
      k.cap(`sk-pubis-${sfx}`, "pelvis", B, T.bone, [-0.004, -0.018, 0.084 * g], [-0.026, -0.07, 0.005 * g], 0.011, 0.009);
    }
  });
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => {
      k.ell(`sk-femur-head-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.022, 0.022, 0.022]);
      k.cap(`sk-femur-neck-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.03, 0, 0.045 * g], 0.014, 0.013);
      k.ell(`sk-trochanter-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.028, -0.004, 0.052 * g], [0.028, 0.018, 0.013]);
      k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[0.038, 0, 0.047, 0.015, 0.015], [0.11, 0.006, 0.037, 0.0135, 0.0135], [0.23, 0.01, 0.02, 0.013, 0.013], [0.33, 0.006, 0.007, 0.015, 0.014], [0.375, 0.002, 0.002, 0.026, 0.02], [0.4055, 0, 0, 0.038, 0.03], [0.423, 0, 0, 0.031, 0.022]], g));
      k.ell(`sk-patella-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.432, 0.024, 0], [0.02, 0.0095, 0.019], undefined, "body", aboutW(-40));
    });
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.011, 0, 0, 0.037, 0.028], [0.018, 0.004, -0.002, 0.028, 0.024], [0.064, 0.009, -0.004, 0.016, 0.018], [0.18, 0.009, -0.004, 0.012, 0.014], [0.33, 0.004, -0.004, 0.012, 0.012], [0.378, 0, -0.004, 0.018, 0.016], [0.4, 0, -0.007, 0.019, 0.015]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.032, -0.011, 0.031 * g], [0.402, -0.007, 0.023 * g], 0.007, 0.006);
    });
    put(`foot-${sfx}`, (k) => {
      k.ell(`sk-talus-${sfx}`, `foot-${sfx}`, B, T.bone, [0.005, -0.013, 0], [0.026, 0.015, 0.019]);
      k.ell(`sk-calcaneus-${sfx}`, `foot-${sfx}`, B, T.bone, [-0.031, -0.032, 0.003 * g], [0.031, 0.016, 0.015]);
      k.loft(`sk-metatarsus-${sfx}`, `foot-${sfx}`, B, T.bone, [[0.018, -0.026, 0, 0.024, 0.015], [0.064, -0.035, 0, 0.03, 0.01], [0.105, -0.043, 0, 0.037, 0.0075], [0.126, -0.046, 0, 0.039, 0.0075]], undefined, "body", 2.6);
    });
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.029, 0, 0.037, 0.0075], [0.032, 0.028, -0.004, 0.031, 0.0065], [0.064, 0.027, -0.006, 0.018, 0.0055]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => {
      k.ell(`sk-humerus-head-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [0, 0, -0.003 * g], [0.021, 0.021, 0.021]);
      k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[0.014, 0, 0, 0.0155, 0.0155], [0.073, 0.002, 0, 0.011, 0.011], [0.22, 0, 0, 0.0095, 0.0095], [0.268, 0, 0, 0.018, 0.01], [0.2925, 0, 0, 0.029, 0.013], [0.303, 0, 0, 0.024, 0.011]]);
    });
  }
  put("forearm-l", (k) => {
    k.cap("sk-ulna-l", "forearm-l", B, T.bone, [-0.011, -0.011, 0.004], [0.25, -0.002, -0.016], 0.01, 0.0055);
    k.cap("sk-radius-l", "forearm-l", B, T.bone, [0.011, 0.004, 0.017], [0.25, 0.002, -0.044], 0.0072, 0.0095);
  });
  put("forearm-r", (k) => {
    k.cap("sk-ulna-r", "forearm-r", B, T.bone, [-0.011, -0.012, 0.006], [0.25, -0.015, 0.002], 0.01, 0.0055);
    k.cap("sk-radius-r", "forearm-r", B, T.bone, [0.011, 0.004, -0.016], [0.25, 0.011, -0.002], 0.0072, 0.0095);
  });
  put("hand-l", (k) => {
    k.ell("sk-carpus-l", "hand-l", B, T.bone, [0.013, 0, HL], [0.016, 0.009, 0.023]);
    k.loft("sk-metacarpus-l", "hand-l", B, T.bone, [[0.028, 0.001, HL, 0.026, 0.0065], [0.096, 0, HL, 0.036, 0.0055]], undefined, "body", 3);
    for (const [i, w, len] of [[1, -0.031, 0.072], [2, -0.0105, 0.08], [3, 0.0105, 0.074], [4, 0.0305, 0.058]] as const) {
      const p1: V3 = [0.098 + 0.55 * len, -0.004, HL + w * 1.12];
      k.cap(`sk-phalanx-${i}-l`, "hand-l", B, T.bone, [0.098, 0, HL + w], p1, 0.0045, 0.004);
      k.cap(`sk-phalanx-tip-${i}-l`, "hand-l", B, T.bone, p1, [0.098 + len - 0.006, -0.019, HL + w * 1.2], 0.004, 0.004);
    }
    k.cap("sk-thumb-l", "hand-l", B, T.bone, [0.028, -0.002, HL - 0.034], [0.07, -0.008, HL - 0.064], 0.005, 0.0045);
    k.cap("sk-thumb-tip-l", "hand-l", B, T.bone, [0.07, -0.008, HL - 0.064], [0.098, -0.016, HL - 0.07], 0.0045, 0.004);
  });
  put("hand-r", (k) => {
    k.ell("sk-carpus-r", "hand-r", B, T.bone, [0.012, 0, -0.006], [0.016, 0.022, 0.01]);
    k.ell("sk-metacarpus-r", "hand-r", B, T.bone, [0.05, -0.006, -0.02], [0.036, 0.034, 0.006]);
    for (const [i, a] of [[1, 0.0335], [2, 0.0115], [3, -0.0105], [4, -0.0315]] as const) {
      const ring = (deg: number, r = 0.0255): [number, number] => [-r * Math.sin((deg * Math.PI) / 180), r * Math.cos((deg * Math.PI) / 180)];
      const [k0, k1, k2, k3] = [ring(25), ring(118), ring(180), ring(232)];
      k.cap(`sk-phalanx-${i}-r`, "hand-r", B, T.bone, hf(a, k0[0], k0[1]), hf(a, k1[0], k1[1]), 0.0045, 0.004);
      k.cap(`sk-phalanx-mid-${i}-r`, "hand-r", B, T.bone, hf(a, k1[0], k1[1]), hf(a, k2[0], k2[1]), 0.004, 0.004);
      k.cap(`sk-phalanx-tip-${i}-r`, "hand-r", B, T.bone, hf(a, k2[0], k2[1]), hf(a, k3[0], k3[1]), 0.004, 0.004);
    }
    k.cap("sk-thumb-r", "hand-r", B, T.bone, hf(-0.004, 0.034, -0.01), hf(0.03, 0.02, -0.03), 0.005, 0.0045);
    k.cap("sk-thumb-tip-r", "hand-r", B, T.bone, hf(0.03, 0.02, -0.03), hf(0.05, -0.002, -0.031), 0.0045, 0.004);
  });
  out.push(...joints(1), ...masses(asset));
  return out;
}

function structure100(asset: Json): Json[] {
  const out: Json[] = equipment(false);
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.152, 0.002, 0], [0.076, 0.088, 0.068]);
    k.loft("sk-face", "head", B, T.bone, [[0.02, -0.06, 0, 0.018, 0.014], [0.05, -0.04, 0, 0.042, 0.04], [0.125, -0.05, 0, 0.056, 0.04]]);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.034, 0, 0.015, 0.014], [0.04, 0.02, 0, 0.014, 0.014], [0.09, 0.018, 0, 0.016, 0.014]]);
  });
  put("torso", (k) => {
    k.loft("sk-spine", "torso", B, T.bone, [SPINE[0]!, SPINE[1]!, SPINE[3]!, SPINE[5]!]);
    for (const [i, s, cv, d, w] of [[1, 0.4, 0.014, 0.056, 0.08], [2, 0.33, 0.003, 0.083, 0.113], [3, 0.255, -0.002, 0.092, 0.124], [4, 0.18, 0.002, 0.084, 0.12]] as const) k.ring(`sk-rib-${i}`, "torso", B, T.bone, s, cv, d, w, 0.0085, -14);
    k.cap("sk-sternum", "torso", B, T.bone, [0.238, -0.086, 0], [0.4, -0.056, 0], 0.013, 0.014);
    for (const [sfx, g] of SIDES) {
      k.cap(`sk-clavicle-${sfx}`, "torso", B, T.bone, [0.408, -0.048, 0.02 * g], [0.428, 0.014, 0.16 * g], 0.0085, 0.0085);
      k.ell(`sk-scapula-${sfx}`, "torso", B, T.bone, [0.338, 0.084, 0.09 * g], [0.072, 0.0085, 0.049]);
    }
  });
  put("pelvis", (k) => {
    k.loft("sk-sacrum", "pelvis", B, T.bone, [[0.1, 0.045, 0, 0.03, 0.02], [0.05, 0.064, 0, 0.048, 0.017], [-0.028, 0.066, 0, 0.013, 0.011]]);
    for (const [sfx, g] of SIDES) k.loft(`sk-hip-bone-${sfx}`, "pelvis", B, T.bone, m([[0.124, 0.02, 0.122, 0.011, 0.062], [0.084, 0.022, 0.12, 0.012, 0.066], [0, 0.006, 0.094, 0.023, 0.035], [-0.06, 0.034, 0.052, 0.013, 0.02]], g));
    k.cap("sk-pubis", "pelvis", B, T.bone, [-0.018, -0.06, -0.052], [-0.018, -0.06, 0.052], 0.011, 0.011);
  });
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.02, 0, 0, 0.013, 0.013], [0, 0, 0.004, 0.022, 0.022], [0.028, 0, 0.036, 0.022, 0.019], [0.07, 0.004, 0.043, 0.015, 0.015], [0.23, 0.01, 0.02, 0.0135, 0.0135], [0.365, 0.003, 0.003, 0.022, 0.019], [0.4055, 0, 0, 0.038, 0.03], [0.423, 0, 0, 0.031, 0.022]], g)));
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.011, 0, 0, 0.037, 0.028], [0.046, 0.007, -0.003, 0.019, 0.02], [0.27, 0.006, -0.004, 0.0125, 0.013], [0.4, 0, -0.006, 0.02, 0.016]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.032, -0.011, 0.03 * g], [0.402, -0.007, 0.023 * g], 0.0078, 0.0076);
    });
    put(`foot-${sfx}`, (k) => k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [[-0.06, -0.033, 0, 0.011, 0.011], [-0.032, -0.031, 0, 0.02, 0.017], [0.005, -0.018, 0, 0.024, 0.02], [0.064, -0.035, 0, 0.03, 0.0105], [0.126, -0.046, 0, 0.039, 0.0078]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.029, 0, 0.037, 0.0078], [0.064, 0.027, -0.006, 0.02, 0.0076]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[-0.018, 0, 0, 0.012, 0.012], [0, 0, 0, 0.021, 0.021], [0.032, 0, 0, 0.015, 0.015], [0.22, 0, 0, 0.01, 0.01], [0.2925, 0, 0, 0.029, 0.013], [0.303, 0, 0, 0.024, 0.011]]));
  }
  put("forearm-l", (k) => {
    k.cap("sk-ulna-l", "forearm-l", B, T.bone, [-0.011, -0.011, 0.004], [0.25, -0.002, -0.016], 0.01, 0.0076);
    k.cap("sk-radius-l", "forearm-l", B, T.bone, [0.011, 0.004, 0.017], [0.25, 0.002, -0.044], 0.0078, 0.0095);
  });
  put("forearm-r", (k) => {
    k.cap("sk-ulna-r", "forearm-r", B, T.bone, [-0.011, -0.012, 0.006], [0.25, -0.015, 0.002], 0.01, 0.0076);
    k.cap("sk-radius-r", "forearm-r", B, T.bone, [0.011, 0.004, -0.016], [0.25, 0.011, -0.002], 0.0078, 0.0095);
  });
  put("hand-l", (k) => {
    k.loft("sk-hand-l", "hand-l", B, T.bone, [[0, 0, HL, 0.022, 0.0095], [0.03, 0.001, HL, 0.027, 0.0078], [0.096, 0, HL, 0.036, 0.0076]], undefined, "body", 3);
    k.loft("sk-fingers-l", "hand-l", B, T.bone, [[0.098, 0, HL, 0.036, 0.0076], [0.14, -0.004, HL, 0.038, 0.0076], [0.172, -0.017, HL, 0.034, 0.0076]], undefined, "body", 3);
  });
  put("hand-r", (k) => {
    k.ell("sk-hand-r", "hand-r", B, T.bone, [0.04, -0.004, -0.018], [0.05, 0.036, 0.0078]);
    k.ell("sk-fingers-r", "hand-r", B, T.bone, [0.072, -0.003, 0.004], [0.024, 0.038, 0.02], undefined, "body", aboutW(50));
  });
  out.push(...joints(1.25), ...masses(asset));
  return out;
}

export const structure = (asset: Json, scale: number): Json[] => (scale === 50 ? structure50(asset) : structure100(asset));

/* ------------------------------------------------------------------ decisions */

export const DECISIONS: Record<number, Json[]> = {
  50: [
    { en: "Body 1:50: a 1.66 m woman of 58 kneeling on a 0.76 x 0.40 m foam mat, modelled from anthropometric sections. The loose linen shirt is one garment loft from the hem over the hips, waist, bust and the rounded upper back to the sloping shoulders and the collar; the sleeves start as a deltoid dome on each upper arm, so the shoulder stays closed in every arm position, and end in the rolled cuff above the elbow. Head with jaw, chin, nose, lips, ears and eyes, grey hair gathered in a bun under a straw hat with domed crown, drooping brim and band. Canvas trousers with the gluteal masses, thighs that taper to the knee and close over it as the knee point on the mat, knee patches, calves that bulge to the back of the shank; ankle socks in garden clogs with separate sole, heel strap and rounded toe box whose top lies on the mat. The left arm turns in from the elbow so the gloved hand lies flat on the left thigh with four fingers and the thumb; the right hand is a fist around the trowel handle with the fingers wrapped round it and the thumb over them. Trowel with turned handle, cranked neck and a hollow 2.5 mm blade; a 4 mm flexible tub with rolled rim and two moulded handles, filled with a heap of weeds whose leaves hang over the rim. The straw brim is modelled 7.5 mm thick, the smallest part the scale prints.", de: "Körper 1:50: eine 58-jährige, 1,66 m grosse Frau kniet auf einer Schaummatte von 0,76 x 0,40 m, aus anthropometrischen Querschnitten modelliert. Das lockere Leinenhemd ist eine Kleidungsform vom Saum über Hüfte, Taille, Brust und den runden oberen Rücken bis zu den abfallenden Schultern und dem Kragen; die Ärmel beginnen als Deltakuppel auf jedem Oberarm, damit die Schulter in jeder Armstellung geschlossen bleibt, und enden im aufgerollten Bund über dem Ellbogen. Kopf mit Kiefer, Kinn, Nase, Lippen, Ohren und Augen, graues Haar zum Knoten gebunden unter einem Strohhut mit gewölbtem Kopf, hängender Krempe und Band. Canvas-Hose mit den Gesässmassen, Oberschenkel, die sich zum Knie verjüngen und es als Kniespitze auf der Matte schliessen, Knieflicken, Waden, die nach hinten ausbauchen; Söckchen in Gartenclogs mit eigener Sohle, Fersenriemen und runder Zehenkappe, deren Oberseite auf der Matte liegt. Der linke Arm dreht ab dem Ellbogen nach innen, sodass die Hand im Handschuh flach auf dem linken Oberschenkel liegt, mit vier Fingern und Daumen; die rechte Hand ist eine Faust um den Schaufelgriff, die Finger umgreifen ihn, der Daumen liegt darüber. Handschaufel mit gedrehtem Griff, gekröpftem Hals und hohlem 2,5-mm-Blatt; ein 4 mm starker flexibler Trog mit Rollrand und zwei angeformten Griffen, gefüllt mit einem Haufen Unkraut, dessen Blätter über den Rand hängen. Die Strohkrempe ist 7,5 mm stark modelliert, das kleinste Teil, das der Massstab druckt." },
    { en: "Thermal 1:50: every region is the body surface it names. Shirt, sleeves, trousers and gloves are two nested shells (air gap under the fabric), the clogs three (sock, air, rubber), each inset by the real layer thickness; the hat is the straw shell of crown and brim over the hair pack, which is the hair itself and the inside of the crown. The face is its own solid in front of the ears and below the hat and the neck only its bare part above the collar, so scalp and covered skin do not count as face; the shoulder dome of each sleeve belongs to the trunk of the shirt, the sleeve region starts below it. The tub is the polyethylene shell with its handles; the weeds in it are no thermal surface of the asset. The breath outlets are small patches under the nose tip and on the lips, the plume leaves from a patch on the hat crown.", de: "Thermik 1:50: Jede Region ist die Körperoberfläche, die sie benennt. Hemd, Ärmel, Hose und Handschuhe sind zwei ineinanderliegende Schalen (Luftspalt unter dem Stoff), die Clogs drei (Socke, Luft, Gummi), jeweils um die echte Schichtdicke eingerückt; der Hut ist die Strohschale aus Kopf und Krempe über dem Haarpolster, das aus dem Haar selbst und der Innenseite des Hutkopfs besteht. Das Gesicht ist ein eigener Körper vor den Ohren und unter dem Hut, der Hals nur sein freier Teil über dem Kragen, damit Kopfhaut und bedeckte Haut nicht als Gesicht zählen; die Schulterkuppel jedes Ärmels gehört zum Rumpf des Hemds, die Ärmelregion beginnt darunter. Der Trog ist die Polyethylenschale mit ihren Griffen; das Unkraut darin ist keine thermische Oberfläche des Objekts. Die Atemauslässe sind kleine Flächen unter der Nasenspitze und auf den Lippen, die Warmluftfahne verlässt den Körper an einer Fläche auf dem Hutkopf." },
    { en: "Structure 1:50: the skeleton as the load path. Skull with facial block and mandible, cervical, thoracic and lumbar spine with their curvature, eight rib hoops with sternum, clavicles, shoulder blades with acromion, sacrum, two wide hip bones with pubic rami, femora with head, neck, greater trochanter, bowed shaft and condyles, patellae drawn down over the flexed knees, tibiae and fibulae, tarsus and metatarsus, toe phalanges, humeri, the left radius crossed over the ulna (hand turned palm down), the right ones side by side, carpus, metacarpus and finger bones; joint solids at the rig joints, masses at the de Leva centres of pelvis, trunk and thighs. The mat and the tub are their own members, the trowel is tang, neck and blade. The structure declares no supports (inertial model).", de: "Tragwerk 1:50: das Skelett als Lastpfad. Schädel mit Gesichtsblock und Unterkiefer, Hals-, Brust- und Lendenwirbelsäule mit ihrer Krümmung, acht Rippenbögen mit Brustbein, Schlüsselbeine, Schulterblätter mit Schulterhöhe, Kreuzbein, zwei breite Hüftbeine mit Schambeinästen, Oberschenkelknochen mit Kopf, Hals, grossem Rollhügel, gebogenem Schaft und Gelenkrollen, Kniescheiben über den gebeugten Knien nach unten gezogen, Schien- und Wadenbeine, Fusswurzel und Mittelfuss, Zehenglieder, Oberarmknochen, links die Speiche über die Elle gekreuzt (Handfläche nach unten), rechts nebeneinander, Handwurzel, Mittelhand und Fingerknochen; Gelenkkörper an den Rig-Gelenken, Massen in den Schwerpunkten nach de Leva von Becken, Rumpf und Oberschenkeln. Matte und Trog sind eigene Stäbe, die Schaufel besteht aus Angel, Hals und Blatt. Das Tragwerk erklärt keine Auflager (Trägheitsmodell)." },
  ],
  100: [
    { en: "Body 1:100: the same gardener drawn anew with fewer, longer sections for parts of at least 15 mm. Eyes, lips, ears, hat band, collar, knee patches and the heel strap are left out; hair and bun are one mass, the rolled cuff is part of the sleeve, the gluteal masses are part of the trouser seat, sole and upper are one clog form, the left hand is a flat mitten with a separate thumb and the right hand one fist with the thumb. The trowel blade is drawn 15 mm deep, the weeds are two heaps with four leaves.", de: "Körper 1:100: dieselbe Gärtnerin, neu gezeichnet mit weniger und längeren Querschnitten für Teile ab 15 mm. Augen, Lippen, Ohren, Hutband, Kragen, Knieflicken und Fersenriemen entfallen; Haar und Knoten sind eine Masse, der Rollbund gehört zum Ärmel, die Gesässmassen gehören zum Hosenboden, Sohle und Schaft sind eine Clogform, die linke Hand ist ein flacher Fäustling mit eigenem Daumen und die rechte eine Faust mit Daumen. Das Schaufelblatt ist 15 mm tief gezeichnet, das Unkraut besteht aus zwei Haufen mit vier Blättern." },
    { en: "Thermal 1:100: one surface per region and body part, the layers merged into it; face, bare neck, shoulder domes and sleeves are split as at 1:50, the hair pack is merged into the hat surface, the weeds are left out, the outlets are enlarged to the scale.", de: "Thermik 1:100: eine Oberfläche je Region und Körperteil, die Schichten sind darin zusammengefasst; Gesicht, freier Hals, Schulterkuppeln und Ärmel sind wie bei 1:50 getrennt, das Haarpolster geht in der Hutoberfläche auf, das Unkraut entfällt, die Auslässe sind auf den Massstab vergrössert." },
    { en: "Structure 1:100: skeleton with merged parts: skull with face, three spine sections, four rib hoops, sternum, clavicles and shoulder blades, sacrum, hip bones and one pubic bar, each femur one form from head over neck and shaft to the condyles, tibia and fibula, one foot form, humerus, radius and ulna, a hand plate with a finger row; mat, tub and trowel as at 1:50 with a thicker tang.", de: "Tragwerk 1:100: Skelett mit zusammengefassten Teilen: Schädel mit Gesicht, drei Wirbelsäulenabschnitte, vier Rippenbögen, Brustbein, Schlüsselbeine und Schulterblätter, Kreuzbein, Hüftbeine und ein Schambeinstab, jeder Oberschenkelknochen eine Form vom Kopf über Hals und Schaft zu den Gelenkrollen, Schien- und Wadenbein, eine Fussform, Oberarmknochen, Speiche und Elle, eine Handplatte mit Fingerreihe; Matte, Trog und Schaufel wie bei 1:50 mit dickerer Angel." },
  ],
};
