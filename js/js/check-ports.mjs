#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { deepEqual, exportKit, importKit } from "./semio.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function checkPorts() {
  const assetsPath = path.join(__dirname, "../../assets/semio");
  const rawKit = JSON.parse(fs.readFileSync(path.join(assetsPath, "kit_metabolism.json"), "utf-8"));

  const toArray = (value) => {
    if (!value) return undefined;
    return Array.isArray(value) ? value : [value];
  };

  const normalizeKit = (kit) => {
    const typesByGuid = new Map();
    kit.types?.forEach((type) => {
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
    };
  };

  const metabolismKit = normalizeKit(rawKit);
  const zipBlob = await exportKit(metabolismKit, new Map());
  const outputPath = path.join(__dirname, "../../assets/metabolism.zip");
  const buffer = await zipBlob.arrayBuffer();
  fs.writeFileSync(outputPath, Buffer.from(buffer));

  const zipBuffer = fs.readFileSync(outputPath);
  const { kit: importedKit } = await importKit(zipBuffer);

  const origType = metabolismKit.types.find((t) => t.name === "Base");
  const impType = importedKit.types.find((t) => t.guid === origType.guid);

  console.log(`Original type "${origType.name}" has ${origType.ports?.length ?? 0} ports`);
  console.log(`Imported type "${impType.name}" has ${impType.ports?.length ?? 0} ports`);

  if (origType.ports && impType.ports) {
    console.log("\nFirst original port:");
    console.log(JSON.stringify(origType.ports[0], null, 2));
    console.log("\nFirst imported port:");
    console.log(JSON.stringify(impType.ports[0], null, 2));

    console.log("\nDeep equal check:");
    console.log(`  guid: ${origType.ports[0].guid === impType.ports[0].guid ? "✓" : `✗ ${origType.ports[0].guid} vs ${impType.ports[0].guid}`}`);
    console.log(`  name: ${origType.ports[0].name === impType.ports[0].name ? "✓" : `✗ ${origType.ports[0].name} vs ${impType.ports[0].name}`}`);
    console.log(`  point: ${deepEqual(origType.ports[0].point, impType.ports[0].point) ? "✓" : "✗"}`);
    console.log(`  direction: ${deepEqual(origType.ports[0].direction, impType.ports[0].direction) ? "✓" : "✗"}`);
    console.log(`  mandatory: ${origType.ports[0].mandatory === impType.ports[0].mandatory ? "✓" : `✗ ${origType.ports[0].mandatory} vs ${impType.ports[0].mandatory}`}`);
    console.log(`  interface: ${deepEqual(origType.ports[0].interface, impType.ports[0].interface) ? "✓" : `✗`}`);
    console.log(`  description: ${origType.ports[0].description === impType.ports[0].description ? "✓" : `✗`}`);
    console.log(`  attributes: ${deepEqual(origType.ports[0].attributes, impType.ports[0].attributes) ? "✓" : "✗"}`);
    console.log(`  props: ${deepEqual(origType.ports[0].props, impType.ports[0].props) ? "✓" : "✗"}`);
  }
}

checkPorts().catch(console.error);
