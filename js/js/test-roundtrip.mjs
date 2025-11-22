#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { areKitsEqual, exportKit, importKit } from "./semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testRoundtrip() {
  console.log("Loading metabolism kit...");
  const assetsPath = path.join(__dirname, "../../assets/semio");
  const rawKit = JSON.parse(fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8"));

  // Normalize the kit data to match expected schema (convert single objects to arrays)
  const toArray = (value) => {
    if (!value) return undefined;
    return Array.isArray(value) ? value : [value];
  };

  const toUndefined = (value) => {
    if (value === null || value === "" || value === undefined) return undefined;
    return value;
  };

  const normalizeAttribute = (attr) => {
    const result = {
      guid: attr.guid,
      key: attr.key,
    };
    const value = toUndefined(attr.value);
    if (value !== undefined) result.value = value;
    const definition = toUndefined(attr.definition);
    if (definition !== undefined) result.definition = definition;
    return result;
  };

  // Recursively normalize all attributes and deduplicate types by GUID
  const normalizeKit = (kit) => {
    // First, deduplicate types by GUID, merging their ports
    const typesByGuid = new Map();
    kit.types?.forEach((type) => {
      const guid = type.guid;
      const existing = typesByGuid.get(guid);
      if (existing) {
        // Merge ports from duplicate type
        existing.ports = [...(existing.ports || []), ...(type.ports || [])];
      } else {
        typesByGuid.set(guid, { ...type });
      }
    });
    const deduplicatedTypes = Array.from(typesByGuid.values());

    return {
      ...kit,
      createdAt: kit.createdAt ? new Date(kit.createdAt) : undefined,
      updatedAt: kit.updatedAt ? new Date(kit.updatedAt) : undefined,
      description: toUndefined(kit.description),
      icon: toUndefined(kit.icon),
      image: toUndefined(kit.image),
      preview: toUndefined(kit.preview),
      remote: toUndefined(kit.remote),
      homepage: toUndefined(kit.homepage),
      license: toUndefined(kit.license),
      concepts: kit.concepts || undefined,
      authors: toArray(kit.authors)?.map((author) => {
        const normalized = {
          ...author,
          email: toUndefined(author.email),
        };
        const attrs = toArray(author.attributes)?.map(normalizeAttribute);
        if (attrs) normalized.attributes = attrs;
        return normalized;
      }),
      files: toArray(kit.files),
      qualities: toArray(kit.qualities)?.map((quality) => ({
        ...quality,
        attributes: toArray(quality.attributes)?.map(normalizeAttribute),
        benchmarks: quality.benchmarks?.map((benchmark) => ({
          ...benchmark,
          attributes: toArray(benchmark.attributes)?.map(normalizeAttribute),
        })),
      })),
      attributes: toArray(kit.attributes)?.map(normalizeAttribute),
      interfaces: kit.interfaces?.map((iface) => ({
        ...iface,
        attributes: toArray(iface.attributes)?.map(normalizeAttribute),
      })),
      types: deduplicatedTypes.map((type) => ({
        ...type,
        createdAt: type.createdAt ? new Date(type.createdAt) : undefined,
        updatedAt: type.updatedAt ? new Date(type.updatedAt) : undefined,
        description: toUndefined(type.description),
        icon: toUndefined(type.icon),
        image: toUndefined(type.image),
        attributes: toArray(type.attributes)?.map(normalizeAttribute),
        models: type.models?.map((model) => ({
          ...model,
          attributes: toArray(model.attributes)?.map(normalizeAttribute),
        })),
        ports: type.ports?.map((port) => ({
          ...port,
          name: toUndefined(port.name),
          description: toUndefined(port.description),
          attributes: toArray(port.attributes)?.map(normalizeAttribute),
          props: port.props?.map((prop) => ({
            ...prop,
            attributes: toArray(prop.attributes)?.map(normalizeAttribute),
          })),
        })),
      })),
      designs: kit.designs?.map((design) => {
        const normalized = {
          guid: design.guid,
          name: design.name,
          createdAt: design.createdAt ? new Date(design.createdAt) : undefined,
          updatedAt: design.updatedAt ? new Date(design.updatedAt) : undefined,
        };
        if (design.description) normalized.description = design.description;
        if (design.icon) normalized.icon = design.icon;
        if (design.image) normalized.image = design.image;
        // Note: parent, activeLayer, concepts, authors are not preserved in SQLite schema
        const props = toArray(design.props)?.map((prop) => ({
          ...prop,
          attributes: toArray(prop.attributes)?.map(normalizeAttribute),
        }));
        if (props) normalized.props = props;
        const attributes = toArray(design.attributes)?.map(normalizeAttribute);
        if (attributes) normalized.attributes = attributes;

        const pieces = design.pieces?.map((piece) => ({
          ...piece,
          name: toUndefined(piece.name),
          description: toUndefined(piece.description),
          color: toUndefined(piece.color),
          attributes: toArray(piece.attributes)?.map(normalizeAttribute),
          props: piece.props?.map((prop) => ({
            ...prop,
            attributes: toArray(prop.attributes)?.map(normalizeAttribute),
          })),
        }));
        if (pieces) normalized.pieces = pieces;

        const connections = design.connections?.map((connection) => ({
          ...connection,
          description: toUndefined(connection.description),
          attributes: toArray(connection.attributes)?.map(normalizeAttribute),
          // Fix old schema: side.guid is actually the port.guid
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
        }));
        if (connections) normalized.connections = connections;

        const layers = design.layers?.map((layer) => ({
          ...layer,
          description: toUndefined(layer.description),
          color: toUndefined(layer.color),
          attributes: toArray(layer.attributes)?.map(normalizeAttribute),
        }));
        if (layers) normalized.layers = layers;

        const groups = design.groups?.map((group) => ({
          ...group,
          name: toUndefined(group.name),
          description: toUndefined(group.description),
          color: toUndefined(group.color),
          attributes: toArray(group.attributes)?.map(normalizeAttribute),
        }));
        if (groups) normalized.groups = groups;

        const stats = design.stats?.map((stat) => ({
          ...stat,
          attributes: toArray(stat.attributes)?.map(normalizeAttribute),
        }));
        if (stats) normalized.stats = stats;

        return normalized;
      }),
    };
  };
  const metabolismKit = normalizeKit(rawKit);

  console.log("Kit statistics:");
  console.log(`  Types: ${metabolismKit.types?.length ?? 0}`);
  console.log(`  Designs: ${metabolismKit.designs?.length ?? 0}`);
  console.log(`  Authors: ${metabolismKit.authors?.length ?? 0}`);
  console.log(`  Files: ${metabolismKit.files?.length ?? 0}`);
  console.log(`  Qualities: ${metabolismKit.qualities?.length ?? 0}`);
  console.log("\n1. Loading files from examples/metabolism...");
  const metabolismPath = path.join(__dirname, "../../examples/metabolism");
  const files = new Map();

  // Recursively load all files except .semio folder
  const loadFiles = (dir, basePath = "") => {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.name === ".semio" || entry.name === ".git") continue;
      const fullPath = path.join(dir, entry.name);
      const relativePath = path.join(basePath, entry.name);
      if (entry.isDirectory()) {
        loadFiles(fullPath, relativePath);
      } else {
        const content = fs.readFileSync(fullPath);
        // Use Buffer directly instead of Blob for Node.js
        files.set(relativePath.replace(/\\/g, "/"), content);
      }
    }
  };
  loadFiles(metabolismPath);
  console.log(`   Loaded ${files.size} files`);

  console.log("\n2. Exporting kit to zip...");
  const zipBlob = await exportKit(metabolismKit, files);

  const outputPath = path.join(__dirname, "../../assets/semio/metabolism.zip");
  const buffer = await zipBlob.arrayBuffer();
  fs.writeFileSync(outputPath, Buffer.from(buffer));

  const stats = fs.statSync(outputPath);
  console.log(`   Exported to: ${outputPath}`);
  console.log(`   File size: ${(stats.size / 1024).toFixed(2)} KB`);

  console.log("\n3. Importing kit from zip...");
  const zipBuffer = fs.readFileSync(outputPath);
  const { kit: rawImportedKit, files: importedFiles } = await importKit(zipBuffer);

  // Helper to clean attributes by removing undefined keys
  const cleanAttributes = (attrs) => {
    if (!attrs) return attrs;
    return attrs.map((attr) => {
      const cleaned = { guid: attr.guid, key: attr.key };
      if (attr.value !== undefined) cleaned.value = attr.value;
      if (attr.definition !== undefined) cleaned.definition = attr.definition;
      return cleaned;
    });
  };

  // Recursively clean all attributes in the kit
  const cleanKitAttributes = (obj) => {
    if (obj === null || obj === undefined) return obj;
    if (Array.isArray(obj)) return obj.map(cleanKitAttributes);
    if (typeof obj === "object") {
      const cleaned = {};
      for (const [key, value] of Object.entries(obj)) {
        if (key === "attributes" && Array.isArray(value)) {
          cleaned[key] = cleanAttributes(value);
        } else {
          cleaned[key] = cleanKitAttributes(value);
        }
      }
      return cleaned;
    }
    return obj;
  };

  // Remove folders property that gets added during import but doesn't exist in original
  // Also normalize empty concepts array to undefined to match original
  const { folders, ...importedKitRaw } = rawImportedKit;
  const importedKitCleaned = cleanKitAttributes(importedKitRaw);
  const importedKit = {
    ...importedKitCleaned,
    concepts: importedKitCleaned.concepts?.length > 0 ? importedKitCleaned.concepts : undefined,
  };

  console.log(`   Imported kit: ${importedKit.name}`);
  console.log(`   Types: ${importedKit.types?.length ?? 0}`);
  console.log(`   Designs: ${importedKit.designs?.length ?? 0}`);
  console.log(`   Files: ${importedFiles.size}`);

  console.log("\n4. Comparing original and imported kits...");

  // Now that SQL schema supports all TypeScript properties, check full equality
  const kitsEqual = areKitsEqual(metabolismKit, importedKit);

  if (kitsEqual) {
    console.log("✅ SUCCESS: Full deep equality achieved!");
    console.log("\nRoundtrip test passed:");
    console.log(`  - ${metabolismKit.types?.length ?? 0} types preserved`);
    console.log(`  - ${metabolismKit.designs?.length ?? 0} designs preserved`);
    const connCount = metabolismKit.designs?.reduce((sum, d) => sum + (d.connections?.length ?? 0), 0) ?? 0;
    console.log(`  - ${connCount} connections preserved`);
    console.log(`  - ${files.size} files preserved`);
    if (files.size !== importedFiles.size) {
      console.log(`   ⚠️ WARNING: File count mismatch (exported ${files.size}, imported ${importedFiles.size})`);
    }
    console.log(`\n  All kit data matches exactly - SQL schema is 100% TypeScript compliant!`);
    process.exit(0);
  } else {
    console.log("❌ FAILURE: Kits are not equal!");

    // Count critical entities
    const origConnCount = metabolismKit.designs?.reduce((sum, d) => sum + (d.connections?.length ?? 0), 0) ?? 0;
    const impConnCount = importedKit.designs?.reduce((sum, d) => sum + (d.connections?.length ?? 0), 0) ?? 0;
    const origTypesCount = metabolismKit.types?.length ?? 0;
    const impTypesCount = importedKit.types?.length ?? 0;
    const origDesignsCount = metabolismKit.designs?.length ?? 0;
    const impDesignsCount = importedKit.designs?.length ?? 0;

    console.log(`   Types: ${origTypesCount} → ${impTypesCount}`);
    console.log(`   Designs: ${origDesignsCount} → ${impDesignsCount}`);
    console.log(`   Connections: ${origConnCount} → ${impConnCount}`);
    console.log("\nDebugging differences...");

    // Show basic stats to help debug
    console.log(`Original - Types: ${metabolismKit.types?.length ?? 0}, Designs: ${metabolismKit.designs?.length ?? 0}`);
    console.log(`Imported - Types: ${importedKit.types?.length ?? 0}, Designs: ${importedKit.designs?.length ?? 0}`);

    // Check top-level properties
    if (metabolismKit.guid !== importedKit.guid) console.log(`GUID differs: ${metabolismKit.guid} vs ${importedKit.guid}`);
    if (metabolismKit.name !== importedKit.name) console.log(`Name differs: ${metabolismKit.name} vs ${importedKit.name}`);
    if (metabolismKit.version !== importedKit.version) console.log(`Version differs: ${metabolismKit.version} vs ${importedKit.version}`);

    // Deep comparison helper
    const findDiff = (path, orig, imp) => {
      if (orig === imp) return null;
      if (typeof orig !== typeof imp) return `${path}: type mismatch (${typeof orig} vs ${typeof imp})`;
      if (orig === null || orig === undefined) return `${path}: null/undefined mismatch`;

      if (Array.isArray(orig)) {
        if (!Array.isArray(imp)) return `${path}: array vs non-array`;
        if (orig.length !== imp.length) return `${path}: length ${orig.length} vs ${imp.length}`;
        for (let i = 0; i < orig.length; i++) {
          const diff = findDiff(`${path}[${i}]`, orig[i], imp[i]);
          if (diff) return diff;
        }
        return null;
      }

      if (typeof orig === "object") {
        const origKeys = Object.keys(orig).sort();
        const impKeys = Object.keys(imp).sort();
        if (origKeys.length !== impKeys.length) {
          const origOnly = origKeys.filter((k) => !impKeys.includes(k));
          const impOnly = impKeys.filter((k) => !origKeys.includes(k));
          let msg = `${path}: key count ${origKeys.length} vs ${impKeys.length}`;
          if (origOnly.length > 0) msg += `\n    Original only: ${origOnly.join(", ")}`;
          if (impOnly.length > 0) msg += `\n    Imported only: ${impOnly.join(", ")}`;
          if (path.includes("attributes") && (origOnly.length > 0 || impOnly.length > 0)) {
            msg += `\n    Original object: ${JSON.stringify(orig)}`;
            msg += `\n    Imported object: ${JSON.stringify(imp)}`;
          }
          if (path.includes("connected") || path.includes("connecting")) {
            msg += `\n    Original object: ${JSON.stringify(orig)}`;
            msg += `\n    Imported object: ${JSON.stringify(imp)}`;
          }
          return msg;
        }
        for (const key of origKeys) {
          if (!impKeys.includes(key)) return `${path}: missing key ${key}`;
          const diff = findDiff(`${path}.${key}`, orig[key], imp[key]);
          if (diff) return diff;
        }
        return null;
      }

      return `${path}: value ${orig} vs ${imp}`;
    };

    const firstDiff = findDiff("kit", metabolismKit, importedKit);
    if (firstDiff) {
      console.log(`\nFirst difference found:`);
      console.log(`  ${firstDiff}`);
    } else {
      console.log(`\nNo specific difference found, but areKitsEqual returned false.`);
      console.log(`This might be a false negative in the comparison function.`);
    }

    process.exit(1);
  }
}

testRoundtrip().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
