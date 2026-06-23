// Regenerate diff and inverted diff files from original and diffed kits
import { readFileSync, writeFileSync } from "fs";
import { resolve } from "path";

async function main() {
  const root = process.cwd();
  const indexPath = new URL(`file:///${resolve(root, "compose/js/index.ts").replace(/\\/g, "/")}`).href;
  const { KitImpl, getKitChange } = await import(indexPath);

  const original = JSON.parse(readFileSync(resolve(root, "compose/assets/compose/metabolism.kit.compose.json"), "utf8"));
  const diffed = JSON.parse(readFileSync(resolve(root, "compose/assets/compose/metabolism.kit.diffed.compose.json"), "utf8"));

  const kitOriginal = new KitImpl(original);
  const kitDiffed = new KitImpl(diffed);

  const change = getKitChange(kitOriginal, kitDiffed);

  const diffPath = resolve(root, "compose/assets/compose/metabolism.kit.diff.compose.json");
  const invertedPath = resolve(root, "compose/assets/compose/metabolism.kit.diff.inverted.compose.json");

  writeFileSync(diffPath, JSON.stringify(change.forward, null, 2) + "\n");
  console.log("Wrote diff to:", diffPath);
  writeFileSync(invertedPath, JSON.stringify(change.backward, null, 2) + "\n");
  console.log("Wrote inverted diff to:", invertedPath);

  console.log("Done.");
}

main().catch(console.error);
