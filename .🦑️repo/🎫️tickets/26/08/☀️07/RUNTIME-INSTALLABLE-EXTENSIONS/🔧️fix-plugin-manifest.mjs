import fs from "fs";

const contribPath = process.argv[2];
let src = fs.readFileSync(contribPath, "utf8");

const marker = "pub struct ProgramContributionEntry";
if (!src.includes(marker)) {
  console.error("ProgramContributionEntry missing");
  process.exit(1);
}

// Remove orphaned derives sitting between Contribution enum close and ProgramContributionEntry docs.
const orphanRe =
  /\n\n#\[derive\(Clone, Debug, PartialEq, Serialize, Deserialize\)\]\n#\[cfg_attr\(feature = "typegen", derive\(ts_rs::TS\)\)\]\n#\[serde\(rename_all = "camelCase"\)\]\n\n(\/\/\/ 🧩️ One host-aggregated plugin contribution entry)/;
if (!orphanRe.test(src)) {
  console.error("orphan derives pattern not found");
  process.exit(1);
}
src = src.replace(orphanRe, "\n\n$1");

// Ensure trailing comma on ImperativeModule variant
src = src.replace(
  /manifest_json: String,\n    \}\n\n\}/,
  "manifest_json: String,\n    },\n}",
);

// Restore PluginManifest derives if missing
if (!src.includes("#[serde(rename_all = \"camelCase\")]\npub struct PluginManifest")) {
  src = src.replace(
    "pub struct PluginManifest {",
    `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {`,
  );
}

if (!src.includes("ProgramContributionEntry::export()")) {
  src = src.replace(
    "crate::ui::Contribution::export().unwrap();",
    "crate::ui::Contribution::export().unwrap();\n        crate::ui::ProgramContributionEntry::export().unwrap();",
  );
}

fs.writeFileSync(contribPath, src);
console.log("fixed", contribPath);
