p='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts'
s=open(p,encoding='utf8',newline='').read()
imp='import type { PlaygroundSelection as PlaygroundVariant } from "./🎮️playground/🧭️selection/🟦️.ts";\n'
assert s.count(imp)==1
s=s.replace(imp,'')
i=s.index('/** 🔌️ Resolves the default dev port for a given catalog variant and renderer. */')
j=s.index('//#endregion 🖥️FrameworkOsPlaygroundDev')
blk=s[i:j]
assert blk.count('export function ')==3 and 'devToolingEnv' in blk
s=s[:i].rstrip('\n')+'\n'+s[j:]
ex='export { loadFrameworkOsPlaygroundSelections } from "./🎮️playground/🧭️selection/🟦️.ts";\n'
assert s.count(ex)==1
s=s.replace(ex,'')
f=open(p,'r+',encoding='utf8',newline=''); f.write(s); f.truncate(); f.close()
