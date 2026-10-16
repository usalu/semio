// 🗺️ mp3: the new io-metadata crate joins the Rust package router through the generator's own patchRouterPackages.
import {DRY, log, put, read} from "./dd.ts";
import {join} from "node:path";
const {patchRouterPackages} = await import(join("/Users/ueli/Documents/semio", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts"));
const script = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust/📜️script.ts";
const patched = patchRouterPackages(read(script), "semio-s-artifact-stdio-mp3", ["semio-s-artifact-stdio-mp3-io-metadata"]);
put(script, patched);
log(DRY ? "dry run" : "written");
