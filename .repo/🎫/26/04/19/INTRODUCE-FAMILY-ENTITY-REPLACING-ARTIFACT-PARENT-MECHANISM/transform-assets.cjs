const fs = require("fs");
const path = require("path");
const dir = "semio/assets/semio";

function buildFamilyMap(entities) {
  if (!entities || !Array.isArray(entities)) return new Map();
  const map = new Map();
  entities.forEach((e) => map.set(e.guid, e));
  function findRoot(guid) {
    const e = map.get(guid);
    if (!e || !e.parent || !e.parent.guid) return guid;
    return findRoot(e.parent.guid);
  }
  const rootHasChildren = new Set();
  entities.forEach((e) => {
    if (e.parent && e.parent.guid) rootHasChildren.add(findRoot(e.guid));
  });
  const families = new Map();
  entities.forEach((e) => {
    const root = findRoot(e.guid);
    const rootEntity = map.get(root);
    if (rootHasChildren.has(root) && rootEntity) families.set(e.guid, [rootEntity.name]);
  });
  return families;
}

function processEntities(entities) {
  if (!entities || !Array.isArray(entities)) return;
  const families = buildFamilyMap(entities);
  entities.forEach((e) => {
    delete e.parent;
    const fam = families.get(e.guid);
    if (fam) e.families = fam;
  });
}

function processDiffArray(arr) {
  if (!Array.isArray(arr)) return false;
  let changed = false;
  arr.forEach((item) => {
    if (item && item.parent !== undefined) {
      delete item.parent;
      changed = true;
    }
  });
  return changed;
}

const files = fs.readdirSync(dir).filter((f) => f.endsWith(".json"));
files.forEach((f) => {
  const filePath = path.join(dir, f);
  const content = fs.readFileSync(filePath, "utf8");
  if (content.indexOf('"parent"') === -1) return;
  const obj = JSON.parse(content);
  let changed = false;
  if (Array.isArray(obj.types) && obj.types.some((t) => t.parent)) {
    processEntities(obj.types);
    changed = true;
  }
  if (Array.isArray(obj.designs) && obj.designs.some((d) => d.parent)) {
    processEntities(obj.designs);
    changed = true;
  }
  if (obj.forward) {
    if (processDiffArray(obj.forward.types)) changed = true;
    if (processDiffArray(obj.forward.designs)) changed = true;
  }
  if (obj.backward) {
    if (processDiffArray(obj.backward.types)) changed = true;
    if (processDiffArray(obj.backward.designs)) changed = true;
  }
  if (changed) {
    fs.writeFileSync(filePath, JSON.stringify(obj, null, 2) + "\n");
    console.log("Updated:", f);
  }
});
