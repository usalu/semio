// 🛠️ Shape V2 retrofit — fix #[path] strings in norm's relocated lib.rs.
// Leaf paths (real on-disk segments) get a "../../" prefix (file moved 2 levels deeper: owner root ->
// 📦️packages/🦀️rust/). Grouping-module "." resets stay exactly "." (see TEMPLATE.md §14 step 3 /
// master.md's SHAPE V2 RETROFIT CORRECTION note).
import { readFileSync, writeFileSync } from "fs";

const path = "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️lib.rs";
const src = readFileSync(path, "utf8");

// Rename the two folded sibling-variant targets to their new folder/component.rs locations first.
const renamed = src
  .replace('#[path = "🫀️core/🦀️config.rs"]', '#[path = "🫀️core/🎚️config/🦀️component.rs"]')
  .replace('#[path = "🫀️core/🦀️app.rs"]', '#[path = "🫀️core/🖥️app-surface/🦀️component.rs"]');

let leafCount = 0;
let resetCount = 0;
const fixed = renamed.replace(/#\[path = "([^"]+)"\]/g, (whole, target: string) => {
  if (target === ".") {
    resetCount++;
    return whole;
  }
  leafCount++;
  return `#[path = "../../${target}"]`;
});

writeFileSync(path, fixed);
console.log(`leaf paths prefixed: ${leafCount}, "." resets left unprefixed: ${resetCount}`);
