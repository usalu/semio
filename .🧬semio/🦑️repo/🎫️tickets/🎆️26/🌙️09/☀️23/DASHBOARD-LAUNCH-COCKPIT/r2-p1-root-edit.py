p='📜️script.ts'
s=open(p,encoding='utf8',newline='').read()
a=s.index('function ensureFrameworkOsPlaygroundCatalog()')
b=s.index('//#region 🔖️NativeOsScript')
block=s[a:b]
assert 'resolvePlaygroundDevApp' in block and 'runFrameworkOsPlaygroundDev' in block and block.count('function ')>=3
s=s[:a]+s[b:]

i=s.index('    if (segments[0] === "s") {\n      runFrameworkOsPlaygroundDev')
j=s.index('  private async parseStorybookSegments')
new_run=r'''    if (segments[0] === "mcp") {
      this.runMcp(segments.slice(1));
      return;
    }
    console.error(`[dev] unknown route ${JSON.stringify(segments.join(" "))}: expected storybook, storybook-static or mcp. Playgrounds are dashboard commands: \`bun run dashboard run playground:<variant> --param renderer=react\` (list them with \`bun run dashboard commands playground\`).`);
    process.exit(1);
  }

'''
s=s[:i]+new_run+s[j:]
for name in ['frameworkOsPlaygroundDevEnv, ','loadFrameworkOsPlaygroundSelections, ','resolveFrameworkOsPlaygroundPlugin, ']:
    assert s.count(name)==1,name
    s=s.replace(name,'')
open(p,'w',encoding='utf8',newline='').write(s)
