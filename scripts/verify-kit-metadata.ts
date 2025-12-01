import { readFileSync } from 'fs';
import { resolve } from 'path';

const basePath = resolve(__dirname, '../assets/semio');
const newKitPath = resolve(basePath, 'kit_metabolism.json');

const newKit = JSON.parse(readFileSync(newKitPath, 'utf-8'));

console.log('=== NEW KIT Metadata ===\n');
console.log(`guid: ${newKit.guid}`);
console.log(`name: ${newKit.name}`);
console.log(`description: ${newKit.description?.substring(0, 50)}...`);
console.log(`icon: ${newKit.icon}`);
console.log(`image: ${newKit.image}`);
console.log(`preview: ${newKit.preview}`);
console.log(`version: ${newKit.version}`);
console.log(`remote: ${newKit.remote}`);
console.log(`homepage: ${newKit.homepage}`);
console.log(`license: ${newKit.license}`);
