import { readFileSync } from 'fs';
import { resolve } from 'path';

const basePath = resolve(__dirname, '../assets/semio');
const oldKitPath = resolve(basePath, 'kit_metabolism.json.old');
const newKitPath = resolve(basePath, 'kit_metabolism.json');

const oldKit = JSON.parse(readFileSync(oldKitPath, 'utf-8'));
const newKit = JSON.parse(readFileSync(newKitPath, 'utf-8'));

console.log('=== FINAL MIGRATION VERIFICATION ===\n');

// 1. Check model counts
const oldTotalReps = oldKit.types.reduce((sum: number, t: any) => sum + (t.representations?.length || 0), 0);
const newTotalModels = newKit.types.reduce((sum: number, t: any) => sum + (t.models?.length || 0), 0);

console.log('1. Model Counts:');
console.log(`   Old representations: ${oldTotalReps}`);
console.log(`   New models: ${newTotalModels}`);
console.log(`   ✓ Match: ${oldTotalReps === newTotalModels ? 'YES' : 'NO'}\n`);

// 2. Check each type has models
const parentNameMapping: Record<string, string> = {
  'Capsule with Balcony': 'Balcony',
  'Ellipsoid Capsule': 'Ellipsoid',
  'Trapezoid Capsule': 'Trapezoid',
};

const newTypesByGuid = new Map<string, any>();
for (const type of newKit.types) {
  newTypesByGuid.set(type.guid, type);
}

function findNewType(oldName: string, oldVariant: string): any {
  if (!oldVariant) {
    return newKit.types.find((t: any) => t.name === oldName && !t.parent);
  } else {
    const mappedParentName = parentNameMapping[oldName] || oldName;
    return newKit.types.find((t: any) => {
      if (t.name !== oldVariant) return false;
      if (!t.parent) return false;
      const parentType = newTypesByGuid.get(t.parent.guid);
      return parentType && parentType.name === mappedParentName;
    });
  }
}

console.log('2. Type-by-Type Verification:');
let allMatch = true;
for (const oldType of oldKit.types) {
  const oldId = oldType.variant ? `${oldType.name}:${oldType.variant}` : oldType.name;
  const newType = findNewType(oldType.name, oldType.variant);

  if (!newType) {
    console.log(`   ✗ ${oldId} - NOT FOUND`);
    allMatch = false;
    continue;
  }

  const oldCount = oldType.representations?.length || 0;
  const newCount = newType.models?.length || 0;

  if (oldCount !== newCount) {
    console.log(`   ✗ ${oldId} - Count mismatch: ${oldCount} vs ${newCount}`);
    allMatch = false;
  }
}

if (allMatch) {
  console.log('   ✓ All types matched with correct model counts\n');
} else {
  console.log('');
}

// 3. Verify URL and tags integrity (sample check)
console.log('3. Data Integrity Check (sampling):');
let integrityOk = true;
for (let i = 0; i < Math.min(5, oldKit.types.length); i++) {
  const oldType = oldKit.types[i];
  const newType = findNewType(oldType.name, oldType.variant);

  if (!oldType.representations || !newType || !newType.models) continue;

  for (let j = 0; j < oldType.representations.length; j++) {
    const oldRep = oldType.representations[j];
    const newModel = newType.models[j];

    if (oldRep.url !== newModel.url) {
      console.log(`   ✗ URL mismatch in ${oldType.name}:${oldType.variant || '(root)'}`);
      integrityOk = false;
    }

    if (JSON.stringify(oldRep.tags) !== JSON.stringify(newModel.tags)) {
      console.log(`   ✗ Tags mismatch in ${oldType.name}:${oldType.variant || '(root)'}`);
      integrityOk = false;
    }
  }
}

if (integrityOk) {
  console.log('   ✓ URLs and tags match (sample verified)\n');
}

// 4. Check kit metadata
console.log('4. Kit Metadata:');
const metadataFields = ['name', 'description', 'icon', 'image', 'preview', 'version', 'remote', 'homepage', 'license'];
let metadataOk = true;

for (const field of metadataFields) {
  const oldValue = oldKit[field];
  const newValue = newKit[field];

  if (oldValue && !newValue) {
    console.log(`   ✗ Missing: ${field}`);
    metadataOk = false;
  }
}

if (metadataOk) {
  console.log('   ✓ All metadata present\n');
}

// 5. Summary
console.log('=== SUMMARY ===');
if (allMatch && integrityOk && metadataOk && oldTotalReps === newTotalModels) {
  console.log('✅ Migration is COMPLETE - all data successfully migrated!');
} else {
  console.log('❌ Migration has ISSUES - see details above');
}
