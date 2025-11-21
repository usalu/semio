#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { areKitsEqual, exportKit, importKit } from "./semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testKitRoundtrip() {
  console.log("Loading metabolism kit...");
  const assetsPath = path.join(__dirname, "../../assets/semio");
  const rawKit = JSON.parse(fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8"));
  
  // Normalize the kit data to match expected schema (convert single objects to arrays)
  const toArray = (value) => {
    if (!value) return undefined;
    return Array.isArray(value) ? value : [value];
  };
  
  const metabolismKit = {
    ...rawKit,
    authors: toArray(rawKit.authors),
    files: toArray(rawKit.files),
    qualities: toArray(rawKit.qualities),
  };
  
  console.log("After normalization:");
  console.log(`  authors: ${metabolismKit.authors?.length ?? 'undefined'}`);
  console.log(`  files: ${metabolismKit.files?.length ?? 'undefined'}`);
  console.log(`  qualities: ${metabolismKit.qualities?.length ?? 'undefined'}`);
  
  console.log("Exporting kit to zip...");
  const zipBlob = await exportKit(metabolismKit, new Map());
  
  console.log("Creating temporary URL for zip blob...");
  const tempUrl = URL.createObjectURL(zipBlob);
  
  console.log("Importing kit from zip...");
  const importResult = await importKit(tempUrl);
  const importedKit = importResult.kit;
  
  // Clean up the temporary URL
  URL.revokeObjectURL(tempUrl);
  
  console.log("Comparing kits...");
  
  // Manual deep comparison to find differences
  const check = (field, aVal, bVal) => {
    if (aVal !== bVal) {
      console.error(`[DIFF] ${field}: ${aVal} !== ${bVal}`);
      return false;
    }
    return true;
  };
  
  let allMatch = true;
  allMatch = check("guid", metabolismKit.guid, importedKit.guid) && allMatch;
  allMatch = check("name", metabolismKit.name, importedKit.name) && allMatch;
  allMatch = check("version", metabolismKit.version, importedKit.version) && allMatch;
  allMatch = check("types.length", metabolismKit.types?.length, importedKit.types?.length) && allMatch;
  allMatch = check("designs.length", metabolismKit.designs?.length, importedKit.designs?.length) && allMatch;
  allMatch = check("files", metabolismKit.files, importedKit.files) && allMatch;
  allMatch = check("qualities", metabolismKit.qualities, importedKit.qualities) && allMatch;
  allMatch = check("authors.length", metabolismKit.authors?.length, importedKit.authors?.length) && allMatch;
  
  const areEqual = areKitsEqual(metabolismKit, importedKit);
  console.log(`areKitsEqual result: ${areEqual}`);
  console.log(`manual checks result: ${allMatch}`);
  
  if (!areEqual) {
    // Debug: log differences
    console.error("❌ FAIL: Kits are NOT equal");
    console.error("\nOriginal kit:");
    console.error(`  Types: ${metabolismKit.types?.length ?? 'undefined'}`);
    console.error(`  Designs: ${metabolismKit.designs?.length ?? 'undefined'}`);
    console.error(`  Files: ${metabolismKit.files?.length ?? 'undefined'}`);
    console.error(`  Authors: ${metabolismKit.authors?.length ?? 'undefined'}`);
    console.error(`  Qualities: ${metabolismKit.qualities?.length ?? 'undefined'}`);
    console.error("\nImported kit:");
    console.error(`  Types: ${importedKit.types?.length ?? 'undefined'}`);
    console.error(`  Designs: ${importedKit.designs?.length ?? 'undefined'}`);
    console.error(`  Files: ${importedKit.files?.length ?? 'undefined'}`);
    console.error(`  Authors: ${importedKit.authors?.length ?? 'undefined'}`);
    console.error(`  Qualities: ${importedKit.qualities?.length ?? 'undefined'}`);
    process.exit(1);
  }
  
  console.log("✅ SUCCESS: Kits are deeply equal!");
  
  // Export to assets if requested
  if (process.env.EXPORT_TO_ASSETS === "true") {
    const outputPath = path.join(__dirname, "../../assets/metabolism.zip");
    const buffer = await zipBlob.arrayBuffer();
    fs.writeFileSync(outputPath, Buffer.from(buffer));
    console.log(`Exported to: ${outputPath}`);
    const stats = fs.statSync(outputPath);
    console.log(`File size: ${(stats.size / 1024).toFixed(2)} KB`);
  }
  
  process.exit(0);
}

testKitRoundtrip().catch(error => {
  console.error("Error:", error);
  process.exit(1);
});
