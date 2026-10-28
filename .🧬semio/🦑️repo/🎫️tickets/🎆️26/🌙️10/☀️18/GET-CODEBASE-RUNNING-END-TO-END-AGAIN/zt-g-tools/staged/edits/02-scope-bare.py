SCOPES = L + "/🏗️build/🔧️tools/🏗️bazel/🎬️scopes"
ORACLE = SCOPES + "/🧪️tests/🐍️oracle/🐍️.py"
TESTS = SCOPES + "/🧪️tests/🟦️.ts"
EDITS = [
    (ORACLE, '''if __name__ == "__main__":
    request = json.load(sys.stdin)
''', '''DYNAMIC = re.compile(r"""\\b(?:import|require)\\(\\s*(["'`])([^"'`\\n]+)\\1\\s*\\)""")


def bare(path):
    text = open(path, encoding="utf-8").read()
    found = [match.group(2) for match in executor.STATIC.finditer(text)] + [match.group(2) for match in DYNAMIC.finditer(text)]
    return [specifier for specifier in found if not specifier.startswith((".", "node:", "bun:")) and specifier != "bun"]


if __name__ == "__main__":
    request = json.load(sys.stdin)
'''),
    (ORACLE, '''    print(json.dumps({"keyed": keyed, "overlay": overlay, "violations": sorted(violations)}))''', '''    leaves = {os.path.normpath(leaf) for leaf in request.get("leaves", [])}
    external = sorted([path, specifier] for path in held - leaves if not path.endswith(".json") for specifier in bare(path))
    print(json.dumps({"keyed": keyed, "overlay": overlay, "violations": sorted(violations), "bare": external}))'''),
    (TESTS, ''' test(`scope ${row.id}: every cheap dynamic edge of the executor is a declared root''', ''' test(`scope ${row.id}: no walked file names a bare package specifier, so no module the runtime trace opens can hide from the static walk (a workspace name resolves through an installed link the walk cannot follow)`,()=>{
  const leaves=new Set(row.scope.leaves.map(path=>normalize(join(checkout,path)))),walked=[...compilerClosure(row.scope.roots,row.scope.leaves)].filter(path=>!leaves.has(path)&&!path.endsWith(".json")),isBare=(specifier:string)=>!specifier.startsWith(".")&&!/^(?:node:|bun:)/u.test(specifier)&&specifier!=="bun";
  const actual=walked.flatMap(path=>[...staticSpecifiers(path),...dynamicSpecifiers(path)].filter(isBare).map(specifier=>relative(checkout,path)+" -> "+specifier)).sort();
  const run=spawnSync(python,["-I",schedulingOracleFile],{input:JSON.stringify({entry,roots:row.scope.roots.map(path=>join(checkout,path)),leaves:row.scope.leaves.map(path=>join(checkout,path)),prefixes:[],ports:[]}),encoding:"utf8",maxBuffer:1<<26});
  expect(run.status,run.stderr).toBe(0);
  expect((JSON.parse(run.stdout) as {bare:string[][]}).bare.map(([path,specifier])=>relative(checkout,path!)+" -> "+specifier).sort()).toEqual(actual);
  expect(actual).toEqual([]);
 });

 test(`scope ${row.id}: every cheap dynamic edge of the executor is a declared root'''),
]
