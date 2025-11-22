#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { deepEqual, exportKit, importKit } from "./semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function findDifference() {
  const assetsPath = path.join(__dirname, "../../assets/semio");
  const rawKit = JSON.parse(fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8"));

  const toArray = (value) => {
    if (!value) return undefined;
    return Array.isArray(value) ? value : [value];
  };

  const normalizeKit = (kit) => {
    const typesByGuid = new Map();
    kit.types?.forEach((type, idx) => {
      const guid = type.guid;
      const existing = typesByGuid.get(guid);
      if (existing) {
        existing.ports = [...(existing.ports || []), ...(type.ports || [])];
      } else {
        typesByGuid.set(guid, { ...type });
      }
    });
    const deduplicatedTypes = Array.from(typesByGuid.values());

    return {
      ...kit,
      authors: toArray(kit.authors),
      files: toArray(kit.files),
      qualities: toArray(kit.qualities)?.map((quality) => ({
        ...quality,
        attributes: toArray(quality.attributes),
        benchmarks: quality.benchmarks?.map((benchmark) => ({
          ...benchmark,
          attributes: toArray(benchmark.attributes),
        })),
      })),
      attributes: toArray(kit.attributes),
      interfaces: kit.interfaces?.map((iface) => ({
        ...iface,
        attributes: toArray(iface.attributes),
      })),
      types: deduplicatedTypes.map((type) => ({
        ...type,
        createdAt: type.createdAt ? new Date(type.createdAt) : undefined,
        updatedAt: type.updatedAt ? new Date(type.updatedAt) : undefined,
        attributes: toArray(type.attributes),
        models: type.models?.map((model) => ({
          ...model,
          attributes: toArray(model.attributes),
        })),
        ports: type.ports?.map((port) => ({
          ...port,
          attributes: toArray(port.attributes),
          props: port.props?.map((prop) => ({
            ...prop,
            attributes: toArray(prop.attributes),
          })),
        })),
      })),
      designs: kit.designs?.map((design) => ({
        ...design,
        createdAt: design.createdAt ? new Date(design.createdAt) : undefined,
        updatedAt: design.updatedAt ? new Date(design.updatedAt) : undefined,
        attributes: toArray(design.attributes),
        pieces: design.pieces?.map((piece) => ({
          ...piece,
          attributes: toArray(piece.attributes),
          props: piece.props?.map((prop) => ({
            ...prop,
            attributes: toArray(prop.attributes),
          })),
        })),
        connections: design.connections?.map((connection) => ({
          ...connection,
          attributes: toArray(connection.attributes),
          connected: connection.connected
            ? {
                piece: connection.connected.piece,
                port: connection.connected.port || { guid: connection.connected.guid },
                designPiece: connection.connected.designPiece,
              }
            : connection.connected,
          connecting: connection.connecting
            ? {
                piece: connection.connecting.piece,
                port: connection.connecting.port || { guid: connection.connecting.guid },
                designPiece: connection.connecting.designPiece,
              }
            : connection.connecting,
        })),
        layers: design.layers?.map((layer) => ({
          ...layer,
          attributes: toArray(layer.attributes),
        })),
        groups: design.groups?.map((group) => ({
          ...group,
          attributes: toArray(group.attributes),
        })),
        stats: design.stats?.map((stat) => ({
          ...stat,
          attributes: toArray(stat.attributes),
        })),
        props: design.props?.map((prop) => ({
          ...prop,
          attributes: toArray(prop.attributes),
        })),
      })),
    };
  };

  const metabolismKit = normalizeKit(rawKit);
  const zipBlob = await exportKit(metabolismKit, new Map());
  const outputPath = path.join(__dirname, "../../assets/metabolism.zip");
  const buffer = await zipBlob.arrayBuffer();
  fs.writeFileSync(outputPath, Buffer.from(buffer));

  const zipBuffer = fs.readFileSync(outputPath);
  const { kit: importedKit } = await importKit(zipBuffer);

  // Deep compare each property
  console.log("Comparing top-level properties:");
  console.log(`  guid: ${metabolismKit.guid === importedKit.guid ? "✓" : `✗ ${metabolismKit.guid} vs ${importedKit.guid}`}`);
  console.log(`  name: ${metabolismKit.name === importedKit.name ? "✓" : `✗ ${metabolismKit.name} vs ${importedKit.name}`}`);
  console.log(`  version: ${metabolismKit.version === importedKit.version ? "✓" : `✗ ${metabolismKit.version} vs ${importedKit.version}`}`);
  console.log(`  description: ${metabolismKit.description === importedKit.description ? "✓" : `✗`}`);

  console.log(`\n  types: ${deepEqual(metabolismKit.types, importedKit.types) ? "✓" : "✗"}`);
  console.log(`  designs: ${deepEqual(metabolismKit.designs, importedKit.designs) ? "✓" : "✗"}`);
  console.log(`  authors: ${deepEqual(metabolismKit.authors, importedKit.authors) ? "✓" : "✗"}`);
  console.log(`  qualities: ${deepEqual(metabolismKit.qualities, importedKit.qualities) ? "✓" : "✗"}`);
  console.log(`  files: ${deepEqual(metabolismKit.files, importedKit.files) ? "✓" : "✗"}`);
  console.log(`  interfaces: ${deepEqual(metabolismKit.interfaces, importedKit.interfaces) ? "✓" : "✗"}`);
  console.log(`  attributes: ${deepEqual(metabolismKit.attributes, importedKit.attributes) ? "✓" : "✗"}`);
  console.log(`  concepts: ${deepEqual(metabolismKit.concepts, importedKit.concepts) ? "✓" : "✗"}`);

  if (!deepEqual(metabolismKit.types, importedKit.types)) {
    console.log("\nTypes are different. Checking first type:");
    const origType = metabolismKit.types[0];
    const impType = importedKit.types.find((t) => t.guid === origType.guid);
    console.log(`  name: ${deepEqual(origType.name, impType.name) ? "✓" : `✗ ${origType.name} vs ${impType.name}`}`);
    console.log(`  ports: ${deepEqual(origType.ports, impType.ports) ? "✓" : "✗"}`);
    console.log(`  models: ${deepEqual(origType.models, impType.models) ? "✓" : "✗"}`);
    console.log(`  attributes: ${deepEqual(origType.attributes, impType.attributes) ? "✓" : "✗"}`);
    console.log(`  createdAt: ${deepEqual(origType.createdAt, impType.createdAt) ? "✓" : `✗ ${origType.createdAt} vs ${impType.createdAt}`}`);
    console.log(`  updatedAt: ${deepEqual(origType.updatedAt, impType.updatedAt) ? "✓" : `✗ ${origType.updatedAt} vs ${impType.updatedAt}`}`);
  }

  if (!deepEqual(metabolismKit.designs, importedKit.designs)) {
    console.log("\nDesigns are different. Checking first design:");
    const origDesign = metabolismKit.designs[0];
    const impDesign = importedKit.designs.find((d) => d.guid === origDesign.guid);
    console.log(`  name: ${deepEqual(origDesign.name, impDesign.name) ? "✓" : `✗`}`);
    console.log(`  pieces: ${deepEqual(origDesign.pieces, impDesign.pieces) ? "✓" : "✗"}`);
    console.log(`  connections: ${deepEqual(origDesign.connections, impDesign.connections) ? "✓" : "✗"}`);
    console.log(`  layers: ${deepEqual(origDesign.layers, impDesign.layers) ? "✓" : "✗"}`);
    console.log(`  groups: ${deepEqual(origDesign.groups, impDesign.groups) ? "✓" : "✗"}`);
    console.log(`  createdAt: ${deepEqual(origDesign.createdAt, impDesign.createdAt) ? "✓" : `✗ ${origDesign.createdAt} vs ${impDesign.createdAt}`}`);
    console.log(`  updatedAt: ${deepEqual(origDesign.updatedAt, impDesign.updatedAt) ? "✓" : `✗ ${origDesign.updatedAt} vs ${impDesign.updatedAt}`}`);
  }
}

findDifference().catch(console.error);
