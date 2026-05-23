// 🧹 Removes specific narrow-typed owner/spine fields from `target.schema.graphql`,
//    leaving only the general interface fields (`ownerEntity`, `ownedEntities`).
//    Run from repo root: `node .repo/🎫/26/05/10/GRAPHQL-TARGET-MUTATION-CLEANUP/strip_specific_owner_fields.js`.

const fs = require('fs');
const path = 'C:/git/semio/semio/graphql/target.schema.graphql';

const src = fs.readFileSync(path, 'utf8');
const lines = src.split(/\r?\n/);

// 🚧 Whitelisted spine-reference field names. Only these are removed; everything else
//    starting with `owner...:` (e.g. data fields like `ownerId: ID!`) stays untouched.
const spineOwnerNames = new Set([
  'ownerModifications',
  'ownerDiffs',
]);

const dropPatterns = [
  // specific union owner field on a type or interface, e.g. `owner: VectorOwner!` (any trailing comment or none).
  // Restricted to type names ending in `Owner` so data fields like `owner: Author` (if any) are not eaten.
  /^  owner: \w+Owner!?(\s+#.*)?$/,
  // specific arm owner field, e.g. `planeOwner: Plane`, `changeOwner: Change # reference // spine`.
  // Field name MUST end in `Owner`, value MUST be a TypeName (capital first), not `ID!`.
  /^  \w+Owner: [A-Z]\w*!?(\s+#.*)?$/,
  // placeholder comments inside the interfaces that document the to-be-added narrow fields
  /^  # owner: [A-Z][A-Z0-9]* # reference$/,
  /^  # [A-Z][A-Z0-9]*Owner: [A-Z][A-Z0-9]*Owner # computed(\s*\/\/.*)?$/,
  /^  # owns: [A-Z][A-Z0-9]*Connection # computed(\s*\/\/.*)?$/,
];

// Custom predicate for spine references like `ownerModifications: Modifications`.
const spinePattern = /^  (\w+): [A-Z]\w*!?(\s+#.*)?$/;
function isSpineReference(line) {
  const m = line.match(spinePattern);
  return !!m && spineOwnerNames.has(m[1]);
}

let removed = 0;
const out = [];
for (const line of lines) {
  if (dropPatterns.some((re) => re.test(line)) || isSpineReference(line)) {
    removed++;
    continue;
  }
  out.push(line);
}

fs.writeFileSync(path, out.join('\n'));
console.log(`Removed ${removed} lines.`);
