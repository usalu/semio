import { readFileSync } from 'fs';
import { resolve } from 'path';

const basePath = resolve(__dirname, '../assets/semio');
const oldKitPath = resolve(basePath, 'kit_metabolism.json.old');

const oldKit = JSON.parse(readFileSync(oldKitPath, 'utf-8'));

console.log('=== Checking for non-empty descriptions and attributes ===\n');

let hasDescriptions = false;
let hasAttributes = false;

for (const type of oldKit.types) {
  if (!type.representations) continue;

  for (const rep of type.representations) {
    if (rep.description && rep.description.trim() !== '') {
      if (!hasDescriptions) {
        console.log('Found non-empty descriptions:');
        hasDescriptions = true;
      }
      console.log(`  ${type.name}${type.variant ? ':' + type.variant : ''} - "${rep.description.substring(0, 50)}..."`);
    }

    if (rep.attributes && rep.attributes.length > 0) {
      if (!hasAttributes) {
        console.log('\nFound non-empty attributes:');
        hasAttributes = true;
      }
      console.log(`  ${type.name}${type.variant ? ':' + type.variant : ''} - ${rep.attributes.length} attributes`);
      console.log(`    ${JSON.stringify(rep.attributes)}`);
    }
  }
}

if (!hasDescriptions) {
  console.log('No non-empty descriptions found.');
}
if (!hasAttributes) {
  console.log('No non-empty attributes found.');
}

console.log('\n=== Checking kit-level metadata ===\n');
console.log(`name: ${oldKit.name}`);
console.log(`description: ${oldKit.description?.substring(0, 50)}...`);
console.log(`icon: ${oldKit.icon}`);
console.log(`image: ${oldKit.image}`);
console.log(`preview: ${oldKit.preview}`);
console.log(`version: ${oldKit.version}`);
console.log(`remote: ${oldKit.remote}`);
console.log(`homepage: ${oldKit.homepage}`);
console.log(`license: ${oldKit.license}`);
