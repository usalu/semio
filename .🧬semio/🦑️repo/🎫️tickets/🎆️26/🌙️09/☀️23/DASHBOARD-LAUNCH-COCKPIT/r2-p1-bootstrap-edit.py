p='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts'
s=open(p,encoding='utf8',newline='').read()
i=s.index('  if (target === "workspace:dev" || target === "@semio-tech/framework-os-dev:dev") {')
j=s.index('  if (target === "workspace:build" && selected.length) {')
blk=s[i:j]
assert blk.count('\n  }\n')==1 and 'Unknown development renderer' in blk, blk[-200:]
s=s[:i]+s[j:]
k=s.index('/** 🧭️ Loads domain selection helpers only for commands that request them. */')
l=s.index('/** 🛠️ Loads the owner of the current immutable Nx recipe')
s=s[:k]+s[l:]
f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
