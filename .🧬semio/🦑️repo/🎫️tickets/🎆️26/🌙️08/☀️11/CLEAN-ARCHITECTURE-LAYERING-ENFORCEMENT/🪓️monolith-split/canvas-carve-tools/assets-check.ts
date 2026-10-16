// 🧾️ Runs the assets freshness check or publish without the repo script router (read the result in the log).
import { checkAssetArtifacts, publishAssetArtifacts, assetOutputManifest } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts";
const mode = process.argv[2] ?? "check";
if (mode === "check") { checkAssetArtifacts(); console.log("[assets] fresh", assetOutputManifest().length); }
else if (mode === "publish") console.log("[assets] wrote", publishAssetArtifacts());
else if (mode === "manifest") console.log(assetOutputManifest().join("\n"));
