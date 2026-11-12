import { writeFileSync } from "node:fs";
import { defaultDepictionOptions, depict } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { figureState } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
const o = defaultDepictionOptions("light-line", "en", 100);
const opts = { ...o, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false } };
writeFileSync(process.argv[2]!, writeSvg(depict(figureState(100), opts)));
