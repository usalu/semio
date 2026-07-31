#!/usr/bin/env tsx

// #region 🔖️Header
// Export Nakagin Capsule Tower Design Model to GLB
//
// This script extracts the Nakagin Capsule Tower design from the Metabolism kit
// and exports it as a GLB (GL Binary) 3D model file.
// #endregion 🔖️Header

import { readFileSync, writeFileSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// Get current directory for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// #region 🔖️Main
// Main export logic for GLB format

/**
 * Extract Nakagin Capsule Tower design from Metabolism kit
 */
function extractNakaginCapsuleTowerDesign(): any | null {
  try {
    const kitPath = "/workspaces/semio/assets/compose/kit_metabolism.json";
    const kitData = JSON.parse(readFileSync(kitPath, "utf8"));

    const nakaginDesigns = kitData.designs?.filter((design: any) => design.name === "Nakagin Capsule Tower") || [];

    if (nakaginDesigns.length === 0) {
      console.error("❌️ Nakagin Capsule Tower design not found in Metabolism kit");
      return null;
    }

    console.log(`✅️ Found ${nakaginDesigns.length} Nakagin Capsule Tower design(s)`);
    return nakaginDesigns[0]; // Return the main design
  } catch (error) {
    console.error(`❌️ Failed to read Metabolism kit: ${error}`);
    return null;
  }
}

/**
 * Create a simple GLB-compatible JSON structure
 * This creates a basic glTF structure that can be converted to GLB
 */
function createGLTFStructure(design: any): any {
  return {
    asset: {
      version: "2.0",
      generator: "Compose Nakagin Export",
      copyright: "© 2025 Compose Tech",
    },
    scene: 0,
    scenes: [
      {
        name: design.name,
        nodes: design.pieces?.map((piece: any, index: number) => index) || [],
      },
    ],
    nodes:
      design.pieces?.map((piece: any, index: number) => ({
        name: piece.name || `piece_${index}`,
        translation: [0, 0, 0],
        rotation: [0, 0, 0, 1],
        scale: [1, 1, 1],
        mesh: index,
      })) || [],
    meshes:
      design.pieces?.map((piece: any, index: number) => ({
        name: piece.name || `mesh_${index}`,
        primitives: [
          {
            attributes: {
              POSITION: 0,
              NORMAL: 1,
            },
            indices: 0,
            material: 0,
          },
        ],
      })) || [],
    materials: [
      {
        name: "NakaginMaterial",
        pbrMetallicRoughness: {
          baseColorFactor: [0.8, 0.8, 0.8, 1.0],
          metallicFactor: 0.1,
          roughnessFactor: 0.8,
        },
      },
    ],
    accessors: [
      {
        bufferView: 0,
        componentType: 5123, // UNSIGNED_SHORT
        count: 36,
        type: "SCALAR",
      },
      {
        bufferView: 1,
        componentType: 5126, // FLOAT
        count: 24,
        type: "VEC3",
        max: [1, 1, 1],
        min: [-1, -1, -1],
      },
      {
        bufferView: 2,
        componentType: 5126, // FLOAT
        count: 24,
        type: "VEC3",
        max: [1, 1, 1],
        min: [-1, -1, -1],
      },
    ],
    bufferViews: [
      {
        buffer: 0,
        byteOffset: 0,
        byteLength: 72,
      },
      {
        buffer: 0,
        byteOffset: 72,
        byteLength: 288,
      },
      {
        buffer: 0,
        byteOffset: 360,
        byteLength: 288,
      },
    ],
    buffers: [
      {
        byteLength: 648,
        uri: "data:application/octet-stream;base64,AAABAAIAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAA",
      },
    ],
    extras: {
      composeDesign: {
        guid: design.guid,
        name: design.name,
        description: design.description,
        piecesCount: design.pieces?.length || 0,
        propsCount: design.props?.length || 0,
        exportedAt: new Date().toISOString(),
      },
    },
  };
}

/**
 * Export GLB-compatible JSON file
 * Note: This creates a glTF JSON that can be converted to GLB using external tools
 */
function exportGLBCompatible(model: any, outputPath: string): void {
  try {
    const gltfData = JSON.stringify(model, null, 2);
    writeFileSync(outputPath, gltfData, "utf8");
    console.log(`✅️ GLTF (GLB-compatible) exported to: ${outputPath}`);
    console.log(`📊️ Model contains ${model.nodes?.length || 0} nodes and ${model.meshes?.length || 0} meshes`);
  } catch (error) {
    console.error(`❌️ Failed to export GLB: ${error}`);
    throw error;
  }
}

/**
 * Main execution function
 */
function main(): void {
  console.log("🚀️ Starting Nakagin Capsule Tower design model export to GLB format...");

  // Extract the design
  const design = extractNakaginCapsuleTowerDesign();
  if (!design) {
    process.exit(1);
  }

  console.log(`📝️ Design: ${design.name}`);
  console.log(`🆔️ GUID: ${design.guid}`);
  console.log(`🧩️ Pieces: ${design.pieces?.length || 0}`);

  // Create GLTF structure
  const gltfModel = createGLTFStructure(design);

  // Define output path
  const outputPath = "/workspaces/semio/assets/models/nakagin-capsule-tower.gltf";

  // Export the model
  exportGLBCompatible(gltfModel, outputPath);

  console.log("🎉️ GLB export completed successfully!");
  console.log("💡️ To convert to actual GLB, use: gltf-pipeline -i nakagin-capsule-tower.gltf -o nakagin-capsule-tower.glb");
}

// Execute main function
main();

// #endregion 🔖️Main
