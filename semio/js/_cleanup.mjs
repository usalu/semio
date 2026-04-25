import { readFileSync, writeFileSync } from 'fs';

const file = 'semio/js/index.ts';
let content = readFileSync(file, 'utf8');
const originalLines = content.split('\n').length;

// Helper: remove a block of lines matching a regex pattern for start, up to (and including) a line matching end pattern
function removeBlock(startPattern, endPattern) {
  const lines = content.split('\n');
  const result = [];
  let skipping = false;
  for (let i = 0; i < lines.length; i++) {
    if (!skipping && startPattern.test(lines[i])) {
      skipping = true;
      continue;
    }
    if (skipping && endPattern.test(lines[i])) {
      skipping = false;
      continue;
    }
    if (!skipping) {
      result.push(lines[i]);
    }
  }
  content = result.join('\n');
}

// Helper: remove lines matching a pattern (single line removal)
function removeLines(pattern) {
  const lines = content.split('\n');
  content = lines.filter(l => !pattern.test(l)).join('\n');
}

// Helper: remove a specific string (multiline)
function removeText(text) {
  content = content.replace(text, '');
}

// Helper: replace text
function replaceText(from, to) {
  if (content.includes(from)) {
    content = content.replace(from, to);
    return true;
  }
  return false;
}

// ============================================================
// 1. Remove all free ID factory functions (create*Id, areSame*Id, get*Id)
// ============================================================
const entities = [
  'Attribute', 'Location', 'Author', 'File', 'Folder', 'Benchmark',
  'Quality', 'Port', 'Prop', 'Representation', 'Connector', 'Type',
  'Layer', 'Piece', 'Group', 'Connection', 'Stat', 'Design', 'Kit',
  'Tag', 'Concept', 'Family'
];

for (const e of entities) {
  // Remove create*Id free functions
  const createPattern = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const create${e}Id = \\(id: Id\\): ${e}Id => \\(\\{ id \\}\\);`,
    'g'
  );
  content = content.replace(createPattern, '');

  // Remove areSame*Id free functions
  const samePattern = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const areSame${e}Id = \\(a: ${e}Id, b: ${e}Id\\): boolean => a\\.id === b\\.id;`,
    'g'
  );
  content = content.replace(samePattern, '');

  // Remove get*Id free functions
  const getPattern = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const get${e}Id = \\(id: ${e}Id\\): Id => id\\.id;`,
    'g'
  );
  content = content.replace(getPattern, '');
}

// ============================================================
// 2. Remove free serialize/deserialize functions for ALL entities
// ============================================================
const serEntities = [
  'Coordinate', 'Vec', 'Point', 'Vector', 'Plane', 'Camera',
  'Attribute', 'Location', 'Author', 'File', 'Folder', 'Benchmark',
  'Quality', 'Port', 'Family', 'Prop', 'Tag', 'Concept',
  'Representation', 'Connector', 'Type', 'Piece', 'Connection',
  'Design', 'Layer', 'Group', 'Side', 'Stat'
];

for (const e of serEntities) {
  // serialize* free function
  const serPat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const serialize${e} = [^;]+;`,
    'g'
  );
  content = content.replace(serPat, '');

  // deserialize* free function
  const deserPat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const deserialize${e} = [^;]+;`,
    'g'
  );
  content = content.replace(deserPat, '');
}

// Also remove MetadataDto and Shallow serialize/deserialize free functions
const projections = ['MetadataDto', 'Shallow'];
for (const e of serEntities) {
  for (const proj of projections) {
    const serPat = new RegExp(
      `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const serialize${e}${proj} = [^;]+;`,
      'g'
    );
    content = content.replace(serPat, '');
    const deserPat = new RegExp(
      `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const deserialize${e}${proj} = [^;]+;`,
      'g'
    );
    content = content.replace(deserPat, '');
  }
}

// Also remove AttributeMetadataDto serialize/deserialize
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const serializeAttributeMetadataDto = [^;]+;/g, '');
content = content.replace(/\/\*\*[\s\S]*?\*\*\/\s*\nexport const deserializeAttributeMetadataDto = [^;]+;/g, '');

// ============================================================
// 3. Remove ALL free diff functions (get*Diff, inverse*Diff, merge*Diff, apply*Diff)
// ============================================================
const diffEntities = [
  'Coordinate', 'Vec', 'Point', 'Vector', 'Plane', 'Camera',
  'Attribute', 'Attributes', 'Location', 'Author', 'File', 'Folder',
  'Benchmark', 'Benchmarks', 'Quality', 'Port', 'Ports', 'Family', 'Families',
  'Prop', 'Props', 'Tag', 'Tags', 'Concept', 'Concepts',
  'Representation', 'Connector', 'Connectors',
  'Connection', 'Group', 'Side', 'Stat'
];

for (const e of diffEntities) {
  // get*Diff
  const getPat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const get${e}Diff = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(getPat, '');

  // inverse*Diff
  const invPat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const inverse${e}Diff = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(invPat, '');

  // merge*Diff
  const mergePat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const merge${e}Diff = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(mergePat, '');

  // apply*Diff
  const applyPat = new RegExp(
    `\\/\\*\\*[\\s\\S]*?\\*\\*\\/\\s*\\nexport const apply${e}Diff = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(applyPat, '');
}

// Remove private collection diff functions (getAttributesDiff, etc.)
const collectionDiffEntities = [
  'Attributes', 'Benchmarks', 'Ports', 'Families', 'Props', 'Tags',
  'Concepts', 'Connectors'
];

for (const e of collectionDiffEntities) {
  // Private get*Diff (const, not export)
  const privatePat = new RegExp(
    `// [^\\n]*${e.charAt(0).toLowerCase() + e.slice(1)}[^\\n]*\\nconst get${e}Diff = [\\s\\S]*?};`,
    'g'
  );
  content = content.replace(privatePat, '');
}

// Also remove roundPlane
content = content.replace(/\/\/ [^\n]*roundPlane[^\n]*\nconst roundPlane = [^;]+;/g, '');

console.log(`After ID/serialize/diff cleanup: ${content.split('\n').length} lines (was ${originalLines})`);

writeFileSync(file, content, 'utf8');
console.log('Phase 1 complete');
