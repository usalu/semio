import { readFileSync } from 'fs';
import { resolve } from 'path';

const basePath = resolve(__dirname, '../assets/semio');
const oldKitPath = resolve(basePath, 'kit_metabolism.json.old');
const newKitPath = resolve(basePath, 'kit_metabolism.json');

const oldKit = JSON.parse(readFileSync(oldKitPath, 'utf-8'));
const newKit = JSON.parse(readFileSync(newKitPath, 'utf-8'));

// Sample old type with representations
const oldBaseType = oldKit.types.find((t: any) => t.name === 'Base' && !t.variant);
console.log('\n=== OLD KIT - Base type representations (sample) ===');
console.log(JSON.stringify(oldBaseType.representations.slice(0, 2), null, 2));

// Sample new type with models
const newBaseType = newKit.types.find((t: any) => t.name === 'Base' && !t.parent);
console.log('\n=== NEW KIT - Base type models (sample) ===');
console.log(JSON.stringify(newBaseType.models.slice(0, 2), null, 2));

// Check for missing data
console.log('\n=== VERIFICATION ===');
const oldRep = oldBaseType.representations[0];
const newModel = newBaseType.models[0];

console.log(`URLs match: ${oldRep.url === newModel.url}`);
console.log(`Tags match: ${JSON.stringify(oldRep.tags) === JSON.stringify(newModel.tags)}`);
console.log(`Old has description: ${!!oldRep.description}`);
console.log(`New has description: ${!!newModel.description}`);
console.log(`Old has attributes: ${!!oldRep.attributes}`);
console.log(`New has attributes: ${!!newModel.attributes}`);

// Count totals
const oldTotalReps = oldKit.types.reduce((sum: number, t: any) => sum + (t.representations?.length || 0), 0);
const newTotalModels = newKit.types.reduce((sum: number, t: any) => sum + (t.models?.length || 0), 0);

console.log(`\nOld total representations: ${oldTotalReps}`);
console.log(`New total models: ${newTotalModels}`);
console.log(`All models migrated: ${oldTotalReps === newTotalModels}`);
