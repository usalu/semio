
import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join, relative } from "path";
const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compFile(dir) {
  return join(dir, readdirSync(dir).find((n) => n.endsWith("component.tsx")));
}
const rel = (from, to) => {
  let r = relative(from, to).replaceAll("\\", "/");
  return r.startsWith(".") ? r : "./" + r;
};

let b = readFileSync(barrel, "utf8");
if (b.includes("\nconst ContextMenuChrome =")) {
  b = b.replace("\nconst ContextMenuChrome =", "\nexport const ContextMenuChrome =");
  writeFileSync(barrel, b);
  console.log("exported ContextMenuChrome");
} else if (b.includes("export const ContextMenuChrome =")) {
  console.log("ContextMenuChrome already exported");
} else {
  console.log("ContextMenuChrome pattern not found");
}

const cmDir = resolveUnder(el, "ContextMenu");
const cm = compFile(cmDir);
const core = resolveUnder(el, "core");
const uiLabel = compFile(resolveUnder(core, "UiLabel"));
const icons = compFile(resolveUnder(el, "Icons"));
const ports = compFile(resolveUnder(core, "Ports"));
const cnFile = compFile(resolveUnder(core, "ClassNames"));
let c = readFileSync(cm, "utf8");
const adaptersOpen = c.match(/\/\/ #region [^\n]*Adapters/)[0];
const adaptersClose = c.match(/\/\/ #endregion [^\n]*Adapters/)[0];
const header = c.split("\n").slice(0, 6).join("\n");
const body = c.match(/\/\/ #region [^\n]*ContextMenu[\s\S]*\/\/ #endregion [^\n]*ContextMenu/)[0];
const interimLine = "// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.";
const file = header + "\n\n" + adaptersOpen + "\n" +
  "import * as React from \"react\";\n" +
  "import { createPortal } from \"react-dom\";\n" +
  "import { reactHostPort } from \"" + rel(cmDir, ports) + "\";\n" +
  "import { cn } from \"" + rel(cmDir, cnFile) + "\";\n" +
  "import { type UiLabel, uiDataLabel } from \"" + rel(cmDir, uiLabel) + "\";\n" +
  "import { Icon, type IconSource } from \"" + rel(cmDir, icons) + "\";\n" +
  interimLine + "\n" +
  "import {\n" +
  "  floatingMenuItemClass,\n" +
  "  ContextMenuChrome,\n" +
  "  useLabel,\n" +
  "  useShellScopeOptional,\n" +
  "  useFlow,\n" +
  "  formatKeybindingShortcut,\n" +
  "} from \"" + rel(cmDir, barrel) + "\";\n" +
  adaptersClose + "\n\n" + body + "\n";
writeFileSync(cm, file);
console.log("ContextMenu fixed");

const iconsDir = resolveUnder(el, "Icons");
const iconsFile = compFile(iconsDir);
const tree = compFile(resolveUnder(el, "Tree"));
let it = readFileSync(iconsFile, "utf8");
it = it.replace(
  /import \{ domSizePx, uiSpacingLen, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, type UiTheme \} from "@semio-tech\/ui-styling";/,
  "import { domSizePx, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, type UiTheme } from \"@semio-tech/ui-styling\";"
);
it = it.replace(
  /import \{ domSizePx, uiSpacingLen, activeUiTheme, subscribeActiveUiTheme, type UiTheme \} from "@semio-tech\/ui-styling";/,
  "import { domSizePx, activeUiTheme, subscribeActiveUiTheme, STYLING_COMPACT_ROOT_PX, type UiTheme } from \"@semio-tech/ui-styling\";"
);
if (!it.includes("uiSpacingLen")) {
  it = it.replace(
    /import \{ cn \} from "([^"]+)";/,
    "import { cn } from \"$1\";\nimport { uiSpacingLen } from \"" + rel(iconsDir, tree) + "\";"
  );
}
writeFileSync(iconsFile, it);
console.log("Icons fixed", /uiSpacingLen/.test(it), /STYLING_COMPACT/.test(it));
