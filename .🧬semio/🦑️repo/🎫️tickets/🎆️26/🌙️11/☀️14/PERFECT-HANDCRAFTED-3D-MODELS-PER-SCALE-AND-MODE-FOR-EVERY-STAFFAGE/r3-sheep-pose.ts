/** 🤸️ Renders the body of animals/sheep in a pose other than the default one (the asset is changed in memory only). Usage: bun r3-sheep-pose.ts <pose> [scale] [az] [el] [list]
 * @see ./volumes-render.ts */
import { entryOf, loadSource, merged, type ListName } from "./volumes-lib.ts";
import { renderScale } from "./volumes-render.ts";
import { OUT, SPEC } from "./r3-sheep-kit.ts";
const [pose, scale = "50", az = "0", el = "0", list = "body"] = process.argv.slice(2);
const { entry, asset } = entryOf(SPEC);
const m = merged(asset, loadSource(entry)!);
m.presentation = { ...m.presentation, defaultPose: pose };
console.log(renderScale(m, Number(scale), {}, [list as ListName], { az: Number(az), el: Number(el), cut: false, width: 1100 }, () => `${OUT}/pose-${pose}-${list}${scale}-${az}-${el}.png`).join("\n"));
