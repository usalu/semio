// 🔍️ Runs the composition ownership law directly (the package script needs the launcher envelope). usage: bun ownership-run.ts [target]
import {testCompositionOwnership, compositionLawGroups} from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts";
const target = process.argv[2];
testCompositionOwnership(target);
console.log("ownership ok", target ?? "(all)", compositionLawGroups().map(g => g.target.name + ":" + g.laws.length).join(" "));
