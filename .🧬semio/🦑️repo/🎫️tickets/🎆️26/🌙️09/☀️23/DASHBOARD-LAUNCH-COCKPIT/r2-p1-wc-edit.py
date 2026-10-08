p='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts'
s=open(p,encoding='utf8',newline='').read()
i=s.index('  test("resolveFrameworkOsPlaygroundPlugin maps CLI segments to OS plugin ids"')
j=s.index('  test("assigns a unique port per dev and test slot"')
blk=s[i:j]
assert blk.count('  test(')==3
s=s[:i]+s[j:]
for n in ['frameworkOsPlaygroundDevEnv, ','resolveFrameworkOsPlaygroundPlugin, ']:
    assert s.count(n)==1,n
    s=s.replace(n,'')
print('catalog uses', s.count('loadFrameworkOsPlaygroundCatalog'))
f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
