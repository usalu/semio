// Regenerate diff and inverted diff files from original and diffed kits
import { readFileSync, writeFileSync } from "fs";
import { resolve } from "path";

async function main() {
  const root = process.cwd();
  const indexPath = new URL(`file:///${resolve(root, "semio/js/index.ts").replace(/\\/g, "/")}`).href;
  const { KitImpl, getKitChange } = await import(indexPath);

  const original = JSON.parse(readFileSync(resolve(root, "semio/assets/semio/metabolism.kit.semio.json"), "utf8"));
  const diffed = JSON.parse(readFileSync(resolve(root, "semio/assets/semio/metabolism.kit.diffed.semio.json"), "utf8"));

  const kitOriginal = new KitImpl(original);
  const kitDiffed = new KitImpl(diffed);

  const change = getKitChange(kitOriginal, kitDiffed);

  const diffPath = resolve(root, "semio/assets/semio/metabolism.kit.diff.semio.json");
  const invertedPath = resolve(root, "semio/assets/semio/metabolism.kit.diff.inverted.semio.json");

  writeFileSync(diffPath, JSON.stringify(change.forward, null, 2) + "\n");
  console.log("Wrote diff to:", diffPath);
  writeFileSync(invertedPath, JSON.stringify(change.backward, null, 2) + "\n");
  console.log("Wrote inverted diff to:", invertedPath);

  console.log("Done.");
}

main().catch(console.error);
