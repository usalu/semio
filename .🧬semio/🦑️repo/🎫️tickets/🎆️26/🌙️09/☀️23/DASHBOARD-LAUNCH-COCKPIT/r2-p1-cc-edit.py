import re
p='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts'
s=open(p,encoding='utf8',newline='').read()
def cut(s,start,end,expect):
    i=s.index(start); j=s.index(end,i)
    blk=s[i:j]
    assert expect in blk,(start,expect)
    return s[:i]+s[j:]
# selection block
s=cut(s,'    const selectionApi = await import("../../../🎮️playground/🧭️selection/🟦️.ts");','    await testCommandInputs(root, ticketOutput(root, []));','rmSync(selectionFixture')
# mcp rewrites via workspace:dev
a=[l for l in s.split('\n') if 'resolveInvocation(["run", "workspace:dev", "--", "mcp"' in l]
assert len(a)==2
for l in a: s=s.replace(l+'\n','')
# storybook rewrite
l=[l for l in s.split('\n') if 'resolveNxInvocation(["run", "workspace:dev", "--", "storybook", "ui"])' in l]
assert len(l)==1; s=s.replace(l[0]+'\n','')
# playground alias loop
i=s.index('    assert.equal(resolveNxInvocation(["run", "workspace:dev", "--", "s"]).args[1]')
j=s.index('    const osDevProject = JSON.parse(')
assert 'the bare dev alias must honour' in s[i:j]
s=s[:i]+'''    for (const retired of [["run", "workspace:dev", "--", "s"], ["run", "workspace:dev", "--", "mcp", "http", "os"], ["run", "@semio-tech/framework-os-dev:dev"]]) assert.deepEqual(resolveNxInvocation(retired).args, retired, `${retired.join(" ")}: the bootstrap is no playground or MCP resolver; the dashboard registry resolves playgrounds`);
'''+s[j:]
i=s.index('    assert.ok(!(osDevProject.targets.dev?.options?.command')
j=s.index('    assert.equal(root.length > 0, true);')
s=s[:i]+'''    assert.equal(osDevProject.targets.dev, undefined, "the framework-os-dev dev alias is dissolved: playgrounds are the generated dev-<variant>-<renderer>-<profile> targets");
    assert.ok(!invocation.getText(nxSource).includes("PlaygroundSelections") && !invocation.getText(nxSource).includes("PlaygroundCatalog"), "the bootstrap resolves no playground variant");
'''+s[j:]
f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
