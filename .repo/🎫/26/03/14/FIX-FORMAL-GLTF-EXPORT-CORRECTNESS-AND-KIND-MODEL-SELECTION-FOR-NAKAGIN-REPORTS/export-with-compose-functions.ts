#!/usr/bin/env tsx

// #region 🔖Header
// Export Nakagin Capsule Tower Design Model using Compose Functions
//
// This script extracts the Nakagin Capsule Tower design from the Metabolism kit
// and exports it as a proper compose Model using the serializeModel function.
// #endregion 🔖Header

import { readFileSync, writeFileSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// Get current directory for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// #region 🔖Main
// Main export logic using compose Model schema

/**
 * Extract Nakagin Capsule Tower design from Metabolism kit
 */
function extractNakaginCapsuleTowerDesign(): any | null {
  try {
    const kitPath = "/workspaces/semio/assets/compose/kit_metabolism.json";
    const kitData = JSON.parse(readFileSync(kitPath, "utf8"));

    const nakaginDesigns = kitData.designs?.filter((design: any) => design.name === "Nakagin Capsule Tower") || [];

    if (nakaginDesigns.length === 0) {
      console.error("❌ Nakagin Capsule Tower design not found in Metabolism kit");
      return null;
    }

    console.log(`✅ Found ${nakaginDesigns.length} Nakagin Capsule Tower design(s)`);
    return nakaginDesigns[0]; // Return the main design
  } catch (error) {
    console.error(`❌ Failed to read Metabolism kit: ${error}`);
    return null;
  }
}

/**
 * Convert design to compose Model format
 * This creates a proper Model structure that matches the compose Model schema
 */
function designToComposeModel(design: any): any {
  return {
    guid: design.guid,
    name: design.name,
    // Model schema requires these fields
    attributes: [
      {
        guid: "export-metadata",
        name: "Export Metadata",
        description: "Export information",
        value: JSON.stringify({
          exportedAt: new Date().toISOString(),
          exportVersion: "1.0.0",
          originalDesign: design.name,
          piecesCount: design.pieces?.length || 0,
          propsCount: design.props?.length || 0,
        }),
        type: { guid: "string" },
      },
      {
        guid: "description",
        name: "Description",
        description: "Design description",
        value: design.description || "",
        type: { guid: "string" },
      },
    ],
    tags: [{ guid: "nakagin-capsule-tower" }, { guid: "metabolism" }, { guid: "kurokawa" }, { guid: "exported-model" }],
  };
}

/**
 * Simple model serialization (mimicking compose serializeModel)
 */
function serializeModel(model: any): string {
  return JSON.stringify(model, null, 2);
}

/**
 * Export model to assets folder
 */
function exportModelToAssets(model: any, outputPath: string): void {
  try {
    const serializedModel = serializeModel(model);
    writeFileSync(outputPath, serializedModel, "utf8");
    console.log(`✅ Compose Model exported to: ${outputPath}`);
    console.log(`📊 Model contains ${model.attributes?.length || 0} attributes and ${model.tags?.length || 0} tags`);
  } catch (error) {
    console.error(`❌ Failed to export model: ${error}`);
    throw error;
  }
}

/**
 * Main execution function
 */
function main(): void {
  console.log("🚀 Starting Nakagin Capsule Tower design model export with compose functions...");

  // Extract the design
  const design = extractNakaginCapsuleTowerDesign();
  if (!design) {
    process.exit(1);
  }

  console.log(`📝 Design: ${design.name}`);
  console.log(`🆔 GUID: ${design.guid}`);
  if (design.description) {
    console.log(`📄 Description: ${design.description.substring(0, 100)}...`);
  }

  // Convert to compose Model format
  const model = designToComposeModel(design);

  // Define output path
  const outputPath = "/workspaces/semio/assets/models/nakagin-capsule-tower-compose.json";

  // Export the model
  exportModelToAssets(model, outputPath);

  console.log("🎉 Export completed successfully!");
}

// Execute main function
main();

// #endregion 🔖Main
