import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";
const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compFile(dir) {
  return join(dir, readdirSync(dir).find((n) => n.endsWith("component.tsx")));
}

let b = readFileSync(barrel, "utf8");
if (!/^export const dropZoneReadyClass\b/m.test(b)) {
  b = b.replace(
    /export const dropZoneReadyFillClass = "bg-\[var\(--accent-secondary\)\]";\n/,
    'export const dropZoneReadyFillClass = "bg-[var(--accent-secondary)]";\n\n/** @emoji Combined passive drop-zone treatment (fill + emphasized text/icons). */\nexport const dropZoneReadyClass = cn(dropZoneReadyFillClass, dropZoneReadyTextClass);\n',
  );
  writeFileSync(barrel, b);
  console.log("restored dropZoneReadyClass");
} else {
  console.log("dropZoneReadyClass ok");
}

const icons = compFile(resolveUnder(el, "Icons"));
let it = readFileSync(icons, "utf8");
it = it.replace(
  /import \{ domSizePx, uiSpacingLen, activeUiTheme, subscribeActiveUiTheme, type UiTheme \} from "@semio-tech\/ui-styling";/,
  'import { domSizePx, uiSpacingLen, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, type UiTheme } from "@semio-tech/ui-styling";',
);
writeFileSync(icons, it);
console.log("icons styling import updated", it.includes("STYLING_COMPACT_ROOT_PX"));

const ptb = compFile(resolveUnder(el, "PanelTabBar"));
let p = readFileSync(ptb, "utf8");
if (!p.includes("@semio-tech/framework-core")) {
  p = p.replace(
    'import { type IconName } from "@semio-tech/assets";',
    'import { type IconName } from "@semio-tech/assets";\nimport { type DockSkeleton, type DockTabSkeleton } from "@semio-tech/framework-core";',
  );
  writeFileSync(ptb, p);
  console.log("added framework-core DockSkeleton");
} else {
  console.log("framework-core already imported");
}
