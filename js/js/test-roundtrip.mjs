#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { exportKit } from "./semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function exportMetabolismKit() {
  console.log("Loading metabolism kit...");
  const assetsPath = path.join(__dirname, "../../assets/semio");
  const rawKit = JSON.parse(fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8"));

  // Normalize the kit data to match expected schema (convert single objects to arrays)
  const toArray = (value) => {
    if (!value) return undefined;
    return Array.isArray(value) ? value : [value];
  };

  // Recursively normalize all attributes and deduplicate types by GUID
  const normalizeKit = (kit) => {
    // First, deduplicate types by GUID, merging their ports
    const typesByGuid = new Map();
    console.log(`[DEBUG] Processing ${kit.types?.length ?? 0} types for deduplication...`);
    kit.types?.forEach((type, idx) => {
      const guid = type.guid;
      const existing = typesByGuid.get(guid);
      if (existing) {
        console.log(`[DEBUG] Found duplicate type at index ${idx}: GUID=${guid}, Name=${type.name}`);
        // Merge ports from duplicate type
        existing.ports = [...(existing.ports || []), ...(type.ports || [])];
      } else {
        typesByGuid.set(guid, { ...type });
      }
    });
    const deduplicatedTypes = Array.from(typesByGuid.values());
    console.log(`[DEBUG] Deduplicated to ${deduplicatedTypes.length} unique types`);

    return {
      ...kit,
      authors: toArray(kit.authors),
      files: toArray(kit.files),
      qualities: toArray(kit.qualities)?.map(quality => ({
        ...quality,
        attributes: toArray(quality.attributes),
        benchmarks: quality.benchmarks?.map(benchmark => ({
          ...benchmark,
          attributes: toArray(benchmark.attributes),
        })),
      })),
      attributes: toArray(kit.attributes),
      interfaces: kit.interfaces?.map(iface => ({
        ...iface,
        attributes: toArray(iface.attributes),
      })),
      types: deduplicatedTypes.map(type => ({
        ...type,
        attributes: toArray(type.attributes),
        models: type.models?.map(model => ({
          ...model,
          attributes: toArray(model.attributes),
        })),
        ports: type.ports?.map(port => ({
          ...port,
          attributes: toArray(port.attributes),
          props: port.props?.map(prop => ({
            ...prop,
            attributes: toArray(prop.attributes),
          })),
        })),
      })),
      designs: kit.designs?.map(design => ({
        ...design,
        attributes: toArray(design.attributes),
        pieces: design.pieces?.map(piece => ({
          ...piece,
          attributes: toArray(piece.attributes),
          props: piece.props?.map(prop => ({
            ...prop,
            attributes: toArray(prop.attributes),
          })),
        })),
        connections: design.connections?.map(connection => ({
          ...connection,
          attributes: toArray(connection.attributes),
        })),
        layers: design.layers?.map(layer => ({
          ...layer,
          attributes: toArray(layer.attributes),
        })),
        groups: design.groups?.map(group => ({
          ...group,
          attributes: toArray(group.attributes),
        })),
        stats: design.stats?.map(stat => ({
          ...stat,
          attributes: toArray(stat.attributes),
        })),
        props: design.props?.map(prop => ({
          ...prop,
          attributes: toArray(prop.attributes),
        })),
      })),
    };
  };  const metabolismKit = normalizeKit(rawKit);

  console.log("Kit statistics:");
  console.log(`  Types: ${metabolismKit.types?.length ?? 0} (deduplicated from ${rawKit.types?.length ?? 0})`);
  console.log(`  Designs: ${metabolismKit.designs?.length ?? 0}`);
  console.log(`  Authors: ${metabolismKit.authors?.length ?? 0}`);
  console.log(`  Files: ${metabolismKit.files?.length ?? 0}`);
  console.log(`  Qualities: ${metabolismKit.qualities?.length ?? 0}`);

  console.log("Exporting kit to zip...");
  const zipBlob = await exportKit(metabolismKit, new Map());

  const outputPath = path.join(__dirname, "../../assets/metabolism.zip");
  const buffer = await zipBlob.arrayBuffer();
  fs.writeFileSync(outputPath, Buffer.from(buffer));

  const stats = fs.statSync(outputPath);
  console.log(`✅ Exported to: ${outputPath}`);
  console.log(`   File size: ${(stats.size / 1024).toFixed(2)} KB`);
}

exportMetabolismKit().catch(error => {
  console.error("❌ Error:", error);
  process.exit(1);
});
