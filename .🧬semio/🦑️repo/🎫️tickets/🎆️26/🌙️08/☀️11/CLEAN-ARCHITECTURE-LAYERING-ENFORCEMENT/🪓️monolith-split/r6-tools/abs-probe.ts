import { absolutizeSuperPaths, hostPrivate } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
const st = "use super::super::{GisTerrainWindowConfig, GisTerrainWindowConfigMutation};";
console.log(absolutizeSuperPaths(st, ["editor","gis3d","config","mutations","set_camera"]));
const host = "use super::super::{GisTerrainWindowConfig, GisTerrainWindowConfigDelta};\nfn x(){}\n";
console.log(JSON.stringify(hostPrivate(host, new Set(["GisTerrainWindowConfig"]), new Set(), "GisTerrainWindowConfig")));
