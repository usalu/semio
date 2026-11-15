/** 🔍️ Close-up render of selected body solids (zoom by auto fit): bun figure-closeup.ts <asset> --scale N --ids <regex on solid id or bone> [--az 40] [--el 15] [--width 1200]. Output `🗑️generated/figure/closeup-<slug>.s<N>.png`. */
import { join } from "node:path";
import { entryOf, loadSource, merged, GENERATED } from "./volumes-lib.ts";
import { renderScale } from "./volumes-render.ts";
const args = process.argv.slice(2);
const flag = (n: string, d?: string) => (args.indexOf(n) < 0 ? d : args[args.indexOf(n) + 1]);
const { entry, asset } = entryOf(args[0]!);
const m = merged(asset, loadSource(entry)!);
const scale = Number(flag("--scale", "50"));
const re = new RegExp(flag("--ids", ".")!);
const v = m.volumes[String(scale)];
v.body = v.body.filter((s: any) => re.test(s.id) || re.test(s.bone));
const out = join(GENERATED, "figure", `closeup-${entry.slug}.s${scale}.png`);
const files = renderScale(m, scale, {}, ["body"], { az: flag("--az") === undefined ? undefined : Number(flag("--az")), el: Number(flag("--el", "15")), cut: false, width: Number(flag("--width", "1200")) }, () => out);
console.log(files.join("\n"));
