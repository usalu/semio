// 🏷️ Taxonomy validators for one plugin directory (relative to the repo root).
import fastGlob from "/Users/ueli/Documents/semio/node_modules/fast-glob";
import {validateRustTaxonomyMounts, validateTaxonomyTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const pluginRoot = process.argv[2]!;
const name = pluginRoot.split("/").pop()!.replace(/^[^\p{L}\p{N}]+/u, "");
const abs = `${root}/${pluginRoot}`;
const rs = fastGlob.sync("**/*.rs", {cwd: abs, ignore: ["**/node_modules/**", "**/target/**"], absolute: true});
const manifests = fastGlob.sync("**/📦️packages/🦀️rust/Cargo.toml", {cwd: abs, ignore: ["**/node_modules/**"]});
const mounts = validateRustTaxonomyMounts(abs, name, rs, rs, manifests);
const tree = validateTaxonomyTree(abs, name);
console.log(JSON.stringify({plugin: name, rs: rs.length, manifests: manifests.length, mounts: mounts.length, tree: tree.length, mountSample: mounts.slice(0, 3), treeSample: tree.slice(0, 3)}));
