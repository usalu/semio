import json
d='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🧪️tests/🛬️decoder-absence/'
fx=d+'🧫️fixtures/🔣️.json'
j=json.load(open(fx,encoding='utf8'))
j['packages']={'@semio-tech/framework-async':'🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript'}
open(fx,'w',encoding='utf8').write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
f=d+'🟦️.ts'
t=open(f,encoding='utf8').read()
old1='''  const graph = inspectRuntimeGraphV1([entry], { rootDirectory: root, read: (path: string) => {'''
new1='''  const owners = Object.entries(contract.packages), exported = (specifier: string): string | undefined => {
    const owner = owners.find(([name]) => specifier === name || specifier.startsWith(name + "/"));
    if (!owner) return undefined;
    const map = JSON.parse(readFileSync(resolve(root, owner[1], "package.json"), "utf8")).exports as Record<string, string>, subpath = "." + specifier.slice(owner[0].length), target = map[subpath];
    return target === undefined ? undefined : relative(root, resolve(root, owner[1], target));
  };
  const aliases = Object.fromEntries([...new Set(["@semio-tech/framework-async/continuation"])].map(specifier => [specifier, exported(specifier)!]));
  const graph = inspectRuntimeGraphV1([entry], { rootDirectory: root, aliases, read: (path: string) => {'''
assert t.count(old1)==1
t=t.replace(old1,new1)
old2='''      expect(imported.fileName.startsWith("."), imported.fileName).toBe(true);
      const target = resolve(root, dirname(path), imported.fileName);'''
new2='''      const declared = imported.fileName.startsWith(".") ? undefined : exported(imported.fileName);
      expect(imported.fileName.startsWith(".") || declared !== undefined, imported.fileName).toBe(true);
      const target = declared === undefined ? resolve(root, dirname(path), imported.fileName) : resolve(root, declared);'''
assert t.count(old2)==1
t=t.replace(old2,new2)
open(f,'w',encoding='utf8').write(t)
