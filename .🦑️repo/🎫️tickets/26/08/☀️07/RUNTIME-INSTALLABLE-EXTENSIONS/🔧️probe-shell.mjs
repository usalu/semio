import fs from "fs";

const shell = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/️elements/ShellHost/🟦️component.tsx".replace(
  "️elements",
  "🧱️elements",
);
const lines = fs.readFileSync(shell, "utf8").split("\n");

const needles = [
  "InstallProgram",
  "UninstallProgram",
  "spaceOp",
  "dispatchSpace",
  "SpaceOperation",
  "programs",
  "invoke",
  "consumes",
  "contributes",
  "role",
  "extends",
  "frameworkPluginsTabs",
  "shellLabel",
  "ui.plugins",
];
for (const n of needles) {
  let count = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(n)) {
      if (count < 15) console.log(String(i + 1).padStart(5), n, lines[i].trim().slice(0, 140));
      count++;
    }
  }
  if (count > 15) console.log("  ... +" + (count - 15) + " more for " + n);
}

console.log("\n=== imports top ===");
console.log(lines.slice(350, 420).map((l, i) => `${351 + i}|${l}`).join("\n"));

console.log("\n=== invokeExtension branch ===");
console.log(lines.slice(1975, 2040).map((l, i) => `${1976 + i}|${l}`).join("\n"));

console.log("\n=== frameworkPluginsTabs merge ===");
for (let i = 0; i < lines.length; i++) {
  if (lines[i].includes("frameworkPluginsTabs") || lines[i].includes("frameworkSettingsTabs") || lines[i].includes("panelTabs")) {
    console.log(i + 1, lines[i].trim().slice(0, 160));
  }
}
