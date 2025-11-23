import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
import { guid } from "../js/js/semio";

const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
const oldKitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json.old");
const kit = JSON.parse(readFileSync(kitPath, "utf-8"));
const oldKit = JSON.parse(readFileSync(oldKitPath, "utf-8"));

// Define interfaces (formerly port families) with their compatibility relationships
const interfaceDefinitions = [
  { name: "core circular bottom", compatible: ["core circular top"] },
  { name: "core circular top", compatible: ["core circular bottom"] },
  { name: "core rectangular bottom", compatible: ["core rectangular top"] },
  { name: "core rectangular top", compatible: ["core rectangular bottom"] },
  { name: "door capsule east", compatible: ["door tambour east"] },
  { name: "door capsule west", compatible: ["door tambour west"] },
  { name: "door tambour east", compatible: ["door capsule west", "platform east"] },
  { name: "door tambour west", compatible: ["door capsule east", "platform west"] },
  { name: "platform east", compatible: ["door tambour west"] },
  { name: "platform west", compatible: ["door tambour east"] },
  { name: "roof circular bottom", compatible: ["roof circular top"] },
  { name: "roof circular top", compatible: ["roof circular bottom"] },
  { name: "roof rectangular bottom", compatible: ["roof rectangular top"] },
  { name: "roof rectangular top", compatible: ["roof rectangular bottom"] },
  { name: "tambour circular bottom", compatible: ["tambour circular top"] },
  { name: "tambour circular top", compatible: ["tambour circular bottom"] },
  { name: "tambour rectangular bottom", compatible: ["tambour rectangular top"] },
  { name: "tambour rectangular top", compatible: ["tambour rectangular bottom"] },
];

// Create interface objects with GUIDs
const interfaces = interfaceDefinitions.map((def) => ({
  guid: guid(),
  name: def.name,
  compatibleInterfaces: [], // Will be filled in second pass
}));

// Create a mapping from name to interface for lookup
const interfaceMap = new Map(interfaces.map((iface) => [iface.name, iface]));

// Fill in compatible interfaces using GUIDs
interfaceDefinitions.forEach((def, index) => {
  const compatibleGuids = def.compatible.map((name) => {
    const compatibleInterface = interfaceMap.get(name);
    if (!compatibleInterface) {
      console.warn(`Warning: Compatible interface "${name}" not found for "${def.name}"`);
      return null;
    }
    return { guid: compatibleInterface.guid };
  }).filter((x) => x !== null);
  
  interfaces[index].compatibleInterfaces = compatibleGuids.length > 0 ? compatibleGuids : undefined;
});

// Add interfaces to kit
kit.interfaces = interfaces;

// Build a mapping from old type name+variant to port families
// In the old structure, variants were separate fields, in the new structure they are separate types
const oldTypePortFamilies = new Map<string, Map<string, string>>();
oldKit.types.forEach((oldType: any) => {
  // Use the variant as the type name if it exists, otherwise use the base name
  const typeName = oldType.variant || oldType.name;
  const portMap = new Map<string, string>();
  if (oldType.ports) {
    oldType.ports.forEach((port: any, index: number) => {
      if (port.family) {
        // Use the port id_ if it exists, otherwise use empty string (some ports have no id)
        const portId = port.id_ || "";
        portMap.set(portId, port.family);
      }
    });
  }
  oldTypePortFamilies.set(typeName, portMap);
});

// Update all ports in types with interface references
let portsUpdated = 0;
let portsNotFound = 0;
if (kit.types) {
  kit.types.forEach((type: any) => {
    if (type.ports) {
      // Look up by the new type name
      const portFamilyMap = oldTypePortFamilies.get(type.name);
      
      if (!portFamilyMap) {
        console.warn(`Warning: No port family mapping found for type "${type.name}"`);
        portsNotFound += type.ports.length;
        return;
      }
      
      type.ports.forEach((port: any) => {
        // Use port name, or empty string if no name
        const portName = port.name || "";
        const family = portFamilyMap.get(portName);
        if (family) {
          const iface = interfaceMap.get(family);
          if (iface) {
            port.interface = { guid: iface.guid };
            portsUpdated++;
          } else {
            console.warn(`Warning: Interface not found for family "${family}" in port ${portName || '(empty)'} of type ${type.name}`);
          }
        } else {
          console.warn(`Warning: No family found for port "${portName || '(empty)'}" in type "${type.name}"`);
        }
      });
    }
  });
}

// Write updated kit
writeFileSync(kitPath, JSON.stringify(kit, null, 2));

console.log(`Added ${interfaces.length} interfaces to metabolism kit`);
console.log(`Updated ${portsUpdated} ports with interface references`);
console.log(`Ports not found in old mapping: ${portsNotFound}`);
console.log("\nInterfaces:");
interfaces.forEach((iface) => {
  console.log(`  ${iface.name} (${iface.guid})`);
  if (iface.compatibleInterfaces && iface.compatibleInterfaces.length > 0) {
    const compatibleNames = iface.compatibleInterfaces
      .map((ref) => {
        const found = interfaces.find((i) => i.guid === ref.guid);
        return found ? found.name : ref.guid;
      })
      .join(", ");
    console.log(`    Compatible with: ${compatibleNames}`);
  }
});
