// 🦋 Removes every `union` and rewrites union-typed fields into general-interface fields whose
//    `# reference // Member1 | Member2 | …` comment documents the original union members.
//    Specifically:
//    • `ownerEntity: OwnerEntity` → `owner: Entity # reference // <members of OwnerEntity or specific narrow union>`
//    • `ownedEntities: OwnedEntityConnection` → `owned: EntityConnection # reference // <members>`
//    • `blueprint: Blueprint!?` → `blueprint: Entity!? # … // Type | Design`
//    • `scope: Scope!` → `scope: <ConcreteOpName>Scope! # data` per concrete operation type.
//    • `input: Input!` → `input: <ConcreteOpName>Input! # data` per concrete operation type.
//    • Removes all `union` declarations.
//    • Renames `EntityConnectionInterface` → `EntityConnection` (the union of that name is gone).

const fs = require("fs");
const path = "C:/git/compose/compose/graphql/target.schema.graphql";

const src = fs.readFileSync(path, "utf8");
const lines = src.split(/\r?\n/);

// ─── 1) Collect all unions: name → [memberTypes] ──────────────────────────────
const unions = new Map();
for (let i = 0; i < lines.length; i++) {
  const single = lines[i].match(/^union (\w+)\s*=\s*(.+?)\s*$/);
  if (single) {
    const name = single[1];
    const members = single[2]
      .split("|")
      .map((s) => s.trim())
      .filter(Boolean);
    unions.set(name, members);
    continue;
  }
  const multi = lines[i].match(/^union (\w+)\s*=\s*$/);
  if (multi) {
    const name = multi[1];
    const members = [];
    let j = i + 1;
    while (j < lines.length && /^\s*\|\s*\w+\s*$/.test(lines[j])) {
      members.push(lines[j].trim().replace(/^\|\s*/, ""));
      j++;
    }
    unions.set(name, members);
  }
}
console.log(`Parsed ${unions.size} unions.`);

// ─── 2) Find every line owned by a `union ... = …` declaration so we can drop them. ──
const dropLineIndices = new Set();
for (let i = 0; i < lines.length; i++) {
  if (/^union \w+\s*=\s*.+$/.test(lines[i])) {
    dropLineIndices.add(i);
    continue;
  }
  if (/^union \w+\s*=\s*$/.test(lines[i])) {
    dropLineIndices.add(i);
    let j = i + 1;
    while (j < lines.length && /^\s*\|\s*\w+\s*$/.test(lines[j])) {
      dropLineIndices.add(j);
      j++;
    }
  }
}

function membersComment(unionName) {
  const members = unions.get(unionName);
  if (!members || members.length === 0) return "";
  return ` // ${members.join(" | ")}`;
}

// ─── 3) Track the "current concrete operation type name" so we can narrow `scope`/`input`. ─
//    A concrete operation is `type <Name> implements Operation …`. The script remembers the
//    most recently-opened type name and rewrites `scope: Scope!` / `input: Input!` inside it
//    to `scope: <Name>Scope!` / `input: <Name>Input!`. Inside `interface Operation` we drop
//    `scope`/`input` entirely (the narrow form is a per-implementation responsibility).
let currentTypeName = null;
let currentTypeIsConcreteOperation = false;
let currentBlockIsInterfaceOperation = false;

let transformedOwner = 0;
let transformedOwned = 0;
let narrowedScope = 0;
let narrowedInput = 0;
let blueprintRewrites = 0;
let droppedInterfaceScopeInput = 0;

const out = [];
for (let i = 0; i < lines.length; i++) {
  if (dropLineIndices.has(i)) continue;
  let line = lines[i];

  // Track entry/exit of `type X implements …` and `interface Operation` blocks.
  const typeOpen = line.match(/^type (\w+)\s+implements\s+([^{]+)\s*\{/);
  if (typeOpen) {
    currentTypeName = typeOpen[1];
    currentTypeIsConcreteOperation = /\bOperation\b/.test(typeOpen[2]);
    currentBlockIsInterfaceOperation = false;
  } else if (/^interface Operation\b[^{]*\{/.test(line)) {
    currentTypeName = "Operation";
    currentTypeIsConcreteOperation = false;
    currentBlockIsInterfaceOperation = true;
  } else if (/^type \w+\s*\{/.test(line) || /^interface \w+\b/.test(line)) {
    currentTypeName = (line.match(/^(?:type|interface) (\w+)/) || [])[1] || null;
    currentTypeIsConcreteOperation = false;
    currentBlockIsInterfaceOperation = false;
  } else if (/^\}\s*$/.test(line)) {
    currentTypeName = null;
    currentTypeIsConcreteOperation = false;
    currentBlockIsInterfaceOperation = false;
  }

  // 3a) ownerEntity → owner. Append `// Member1 | Member2 | …` ONLY when the line had a specific
  //     narrow union annotation (e.g. `// VectorOwner`). For lines using just the global
  //     `OwnerEntity` (no narrow annotation) the new field type `Entity` already carries the
  //     full meaning, so we leave the trailing comment empty to avoid noise.
  let m = line.match(/^(\s*)ownerEntity:\s*OwnerEntity!?\s*(?:#\s*computed)?(?:\s*\/\/\s*([^\n]*))?\s*$/);
  if (m) {
    const indent = m[1];
    const trailing = m[2] ? m[2].trim() : "";
    const firstToken = trailing.split(/\s+/)[0] || "";
    const comment = unions.has(firstToken) ? membersComment(firstToken) : "";
    out.push(`${indent}owner: Entity # reference${comment}`);
    transformedOwner++;
    continue;
  }

  // 3b) ownedEntities → owned (same rule as ownerEntity)
  m = line.match(/^(\s*)ownedEntities:\s*OwnedEntityConnection!?\s*(?:#\s*computed)?(?:\s*\/\/\s*([^\n]*))?\s*$/);
  if (m) {
    const indent = m[1];
    const trailing = m[2] ? m[2].trim() : "";
    const firstToken = trailing.split(/\s+/)[0] || "";
    const comment = unions.has(firstToken) ? membersComment(firstToken) : "";
    out.push(`${indent}owned: EntityConnection # reference${comment}`);
    transformedOwned++;
    continue;
  }

  // 3c) `interface Operation { … scope: Scope! …  input: Input! … }` — drop these two fields
  //     since Operation no longer has a uniform scope/input type after removing the unions.
  if (currentBlockIsInterfaceOperation && /^\s*(scope|input):\s*(Scope|Input)!?\s*(#.*)?$/.test(line)) {
    droppedInterfaceScopeInput++;
    continue;
  }

  // 3d) Concrete operation type: narrow `scope: Scope!` → `scope: <TypeName>Scope!` etc.
  if (currentTypeIsConcreteOperation && currentTypeName) {
    const sm = line.match(/^(\s*)scope:\s*Scope(!?)\s*(#.*)?$/);
    if (sm) {
      out.push(`${sm[1]}scope: ${currentTypeName}Scope${sm[2]} ${sm[3] || "# data"}`.replace(/\s+$/, ""));
      narrowedScope++;
      continue;
    }
    const im = line.match(/^(\s*)input:\s*Input(!?)\s*(#.*)?$/);
    if (im) {
      out.push(`${im[1]}input: ${currentTypeName}Input${im[2]} ${im[3] || "# data"}`.replace(/\s+$/, ""));
      narrowedInput++;
      continue;
    }
  }

  // 3e) `blueprint: Blueprint(!)?` → `blueprint: Entity(!)? # … // Type | Design`
  m = line.match(/^(\s*)blueprint:\s*Blueprint(!?)\s*(#.*)?$/);
  if (m) {
    const indent = m[1];
    const bang = m[2];
    const trailing = m[3] || "# reference";
    out.push(`${indent}blueprint: Entity${bang} ${trailing}${membersComment("Blueprint")}`.replace(/\s+$/, ""));
    blueprintRewrites++;
    continue;
  }

  // 3f) Rename interface `EntityConnectionInterface` → `EntityConnection` everywhere.
  line = line.replace(/\bEntityConnectionInterface\b/g, "EntityConnection");

  out.push(line);
}

fs.writeFileSync(path, out.join("\n"));
console.log(`Removed ${dropLineIndices.size} union lines.`);
console.log(`Rewrote ${transformedOwner} owner / ${transformedOwned} owned fields.`);
console.log(`Narrowed ${narrowedScope} scope / ${narrowedInput} input fields on concrete operations.`);
console.log(`Rewrote ${blueprintRewrites} blueprint fields.`);
console.log(`Dropped ${droppedInterfaceScopeInput} scope/input fields from interface Operation.`);
