import {pluginFamilyText} from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
const s=pluginFamilyText();
const needles=JSON.parse(process.argv[2]!) as string[];
for(const n of needles)console.log(s.includes(n)?"HAS ":"MISS",n.slice(0,150));
