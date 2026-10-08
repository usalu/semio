p='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts'
s=open(p,encoding='utf8',newline='').read()
i=s.index('  if (["@semio-tech/framework-renderer-wgpu:dev", "@semio-tech/framework-renderer-wgpu:serve"].includes(target)) {')
j=s.index('  const nativeRuntime = target?.match(')
assert s[i:j].count('\n  }\n')==1
s=s[:i]+s[j:]
i=s.index('  if (["@semio-tech/framework-renderer-wgpu:native", "@semio-tech/framework-renderer-wgpu:native-release"].includes(target)) {')
j=s.index('  if (["@semio-tech/framework-renderer-wgpu:native-build"].includes(target)')
assert s[i:j].count('\n  }\n')==1 and 'Select one native variant' in s[i:j]
s=s[:i]+s[j:]
f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
