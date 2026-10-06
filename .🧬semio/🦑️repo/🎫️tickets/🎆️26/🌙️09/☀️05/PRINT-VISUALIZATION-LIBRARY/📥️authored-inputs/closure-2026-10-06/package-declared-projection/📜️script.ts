import{readFileSync,writeFileSync}from'node:fs';import{join}from'node:path';const p=join(process.cwd(),'🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts'),x=readFileSync(p,'utf8');const start=x.indexOf('  const families: VizApiKeyFamily[] = [];',x.indexOf('function vizDocumentKeyFamilyProjection')),end=x.indexOf('\n  return families;',start);if(start<0||end<0)throw Error('projection boundary');
const body=String.raw`  const families: VizApiKeyFamily[] = [], entries = new Map<string, { region: string; names: Set<string> }>();
  const add = (scope: string, region: string, names: readonly string[]): void => {
    const entry = entries.get(scope) ?? { region, names: new Set<string>() };
    for (const name of names) if (!name.startsWith("@forward:")) entry.names.add(name);
    entries.set(scope, entry);
  };
  for (const entry of vizDocumentKeyRegions(implementation, model)) for (const scope of entry.scopes) {
    const accepted = scope.startsWith("helper:") ? helperDeclaredKeyNames(model, scope.slice(7), new Set()) : keysOfPath(model, scope, new Set());
    add(scope, entry.region, entry.keys.map(key => key.name).filter(name => accepted.has(name)));
  }
  for (const unit of scanKeys(implementation).literals) add(unit.path, "Keys", unit.keys);
  for (const match of implementation.matchAll(/\\cs_(?:new|set)[a-z_]*:Npn\s+\\([a-z_]+):/g)) {
    const macro = match[1]!;
    if (model.helpers.get(macro)?.templates.length) add("helper:" + macro, "Keys", [...helperDeclaredKeyNames(model, macro, new Set())]);
  }
  for (const [scope, entry] of entries) {
    if (!entry.names.size) continue;
    const rows = read(scope);
    const keys = [...entry.names].map(name => {
      const key = rows.get(name);
      if (!key) throw new Error("Missing printed key contract: " + scope + ":" + name);
      return key;
    });
    families.push({ region: entry.region, scope, keys });
  }`;
writeFileSync(join(import.meta.dir,'gallery-before.ts'),x);writeFileSync(join(import.meta.dir,'gallery-candidate.ts'),x.slice(0,start)+body+x.slice(end));
