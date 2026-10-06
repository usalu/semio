import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
const p=join(process.cwd(),'🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts'),original=readFileSync(p,'utf8');
const from=original.indexOf('/** 📚️ Collects physical source key regions'),to=original.indexOf('/** 📖 The whole public API',from);
if(from<0||to<0)throw Error('collector boundary');
const body=String.raw`/** 🧭️ Selects complete rendered rows from one actual native owner scope. */
function vizDocumentKeyReader(reference: string, model: KeysModel): (scope: string) => ReadonlyMap<string, VizApiKey> {
  const text = vizPrintedSource(reference), tables = vizPrintedKeyRows(text), sections = vizPrintedSections(text);
  const declared = new Map([...text.matchAll(/\\ApiScopeSource\{([a-z0-9-]+)\}\{([^}]+)\}/g)].map(match => [match[1]!, match[2]!.replaceAll("\\_", "_")]));
  return scope => {
    const bound = [...declared].filter(([, path]) => path === scope).map(([owner]) => owner);
    const family = /^semio\s*\/\s*viz\s*\/\s*family\s*\/\s*([a-z0-9-]+)$/.exec(scope)?.[1];
    const owners = bound.length ? bound : family ? [family] : [scope];
    const result = new Map<string, VizApiKey>();
    for (const owner of owners) for (const row of vizPrintedOwnerRows(owner, tables, declared, sections, model)) for (const name of row.names) {
      if (!row.type.trim() || !row.meaning.en.trim() || !row.meaning.de.trim()) throw new Error("Incomplete printed key contract: " + scope + ":" + name);
      const key: VizApiKey = { name, type: row.type, default: row.default, description: row.meaning }, prior = result.get(name);
      if (prior && JSON.stringify(prior) !== JSON.stringify(key)) throw new Error("Ambiguous printed key contract: " + scope + ":" + name);
      result.set(name, key);
    }
    return result;
  };
}

/** 📚️ Projects each native declaration scope through its complete rendered row contracts. */
export function vizDocumentKeyFamilies(implementation: string, reference: string, model: KeysModel = vizKeysModel([implementation])): readonly VizApiKeyFamily[] {
  return vizDocumentKeyFamilyProjection(implementation, model, vizDocumentKeyReader(reference, model));
}

/** 🗝️ Shares the parsed reference across all physical packages in one collection. */
function vizDocumentKeyFamilyProjection(implementation: string, model: KeysModel, read: (scope: string) => ReadonlyMap<string, VizApiKey>): readonly VizApiKeyFamily[] {
  const families: VizApiKeyFamily[] = [];
  for (const entry of vizDocumentKeyRegions(implementation, model)) for (const scope of entry.scopes) {
    const accepted = scope.startsWith("helper:") ? helperDeclaredKeyNames(model, scope.slice(7), new Set()) : keysOfPath(model, scope, new Set());
    const names = entry.keys.map(key => key.name).filter(name => accepted.has(name));
    if (!names.length) continue;
    const rows = read(scope);
    const keys = names.map(name => {
      const key = rows.get(name);
      if (!key) throw new Error("Missing printed key contract: " + scope + ":" + name);
      return key;
    });
    families.push({ region: entry.region, scope, keys });
  }
  return families;
}
`;
let x=original.slice(0,from)+body+original.slice(to);
x=x.replace('readonly region: string; readonly keys: readonly VizApiKey[]','readonly region: string; readonly scope: string; readonly keys: readonly VizApiKey[]');
x=x.replace('  const translate = vizDocumentGermanReader(reference);\r\n  const packages:', '  const packages:');
x=x.replace('  const model = vizKeysModel();\r\n  let commandCount', '  const model = vizKeysModel(), readKeys = vizDocumentKeyReader(reference, model);\r\n  let commandCount');
const start=x.indexOf('    const keyFamilies: VizApiKeyFamily[] = vizDocumentKeyRegions(source, model)'),end=x.indexOf('    for (const [index, line]',start);
if(start<0||end<0)throw Error('integration boundary');
x=x.slice(0,start)+'    const keyFamilies = vizDocumentKeyFamilyProjection(source, model, readKeys);\r\n'+x.slice(end);
writeFileSync(join(import.meta.dir,'gallery-before.ts'),original);writeFileSync(join(import.meta.dir,'gallery-candidate.ts'),x);
