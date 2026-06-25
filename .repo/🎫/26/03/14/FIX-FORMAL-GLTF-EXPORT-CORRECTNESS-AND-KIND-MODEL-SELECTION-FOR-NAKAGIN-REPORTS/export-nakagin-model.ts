#!/usr/bin/env tsx

// #region 🔖Header
// Export Nakagin Capsule Tower Design Model to Compose Assets
// 
// This script extracts the Nakagin Capsule Tower design from the Metabolism kit
// and exports it as a model asset to the compose assets folder.
// #endregion 🔖Header

import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

// Get current directory for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// #region 🔖Main
// Main export logic for Nakagin Capsule Tower design model

interface DesignModel {
  guid: string;
  name: string;
  description?: string;
  pieces?: any[];
  props?: any[];
  image?: string;
  icon?: string;
  unit?: string;
  authors?: any[];
  // Add other relevant fields from the design structure
}

/**
 * Extract Nakagin Capsule Tower design from Metabolism kit
 */
function extractNakaginCapsuleTowerDesign(): DesignModel | null {
  try {
    const kitPath = '/workspaces/semio/assets/compose/kit_metabolism.json';
    const kitData = JSON.parse(readFileSync(kitPath, 'utf8'));

    const nakaginDesigns = kitData.designs?.filter((design: any) => design.name === "Nakagin Capsule Tower") || [];

    if (nakaginDesigns.length === 0) {
      console.error('❌ Nakagin Capsule Tower design not found in Metabolism kit');
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
 * Convert design to model format
 */
function designToModel(design: DesignModel): any {
  return {
    guid: design.guid,
    name: design.name,
    description: design.description,
    pieces: design.pieces || [],
    props: design.props || [],
    image: design.image,
    icon: design.icon,
    unit: design.unit,
    authors: design.authors || [],
    exportedAt: new Date().toISOString(),
    exportVersion: "1.0.0"
  };
}

/**
 * Export model to assets folder
 */
function exportModelToAssets(model: any, outputPath: string): void {
  try {
    const serializedModel = JSON.stringify(model, null, 2);
    writeFileSync(outputPath, serializedModel, 'utf8');
    console.log(`✅ Model exported to: ${outputPath}`);
    console.log(`📊 Model contains ${model.pieces?.length || 0} pieces and ${model.props?.length || 0} props`);
  } catch (error) {
    console.error(`❌ Failed to export model: ${error}`);
    throw error;
  }
}

/**
 * Main execution function
 */
function main(): void {
  console.log('🚀 Starting Nakagin Capsule Tower design model export...');

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

  // Convert to model format
  const model = designToModel(design);

  // Define output path
  const outputPath = '/workspaces/semio/assets/models/nakagin-capsule-tower.json';

  // Export the model
  exportModelToAssets(model, outputPath);

  console.log('🎉 Export completed successfully!');
}

// Execute main function
main();

// #endregion 🔖Main
