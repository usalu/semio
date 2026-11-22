import hashlib
C="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
PUB=C+"💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/"
def ed(p,pairs):
    s=open(p,encoding='utf8').read()
    for a,b in pairs:
        assert s.count(a)==1,(p[-40:],a[:70]);s=s.replace(a,b)
    open(p,'w',encoding='utf8').write(s)
ed(PUB+"🟦️.ts",[('if(generations.names.some(name=>!/^(?:[a-f0-9]{20}|\\.p[a-f0-9]{8})$/u.test(name))||selectedPrefix!==null&&!generations.names.includes(selectedPrefix))throw Error("Exact known current dependency generation roster required");',
 'const unknownNames=generations.names.filter(name=>!/^(?:[a-f0-9]{20}|\\.p[a-f0-9]{8})$/u.test(name));\n if(unknownNames.length>0||selectedPrefix!==null&&!generations.names.includes(selectedPrefix))throw Error("Exact known current dependency generation roster required"+(unknownNames.length>0?": "+JSON.stringify(unknownNames.slice(0,8).map(name=>name.slice(0,64)))+" is not a generation or capsule name":": selected prefix "+selectedPrefix+" is not in the roster"));')])
ed(PUB+"⚙️setup/♻️retirement/🟦️.ts",[(' }catch(error){\n  if(isRepoErrorV1(error)&&error.key==="caching.dependency-store.busy-timeout")',
 ' }catch(error){\n  if(rosterRefusalV1(error)){sayCachingV1("dependency-store.roster-unknown",{detail:String((error as Error).message).replace(/^.*?required:?\\s*/u,"").slice(0,200),retired:retired.length},"warn");return Object.freeze(retired);}\n  if(isRepoErrorV1(error)&&error.key==="caching.dependency-store.busy-timeout")'),
 ('/** 🌀️ Returns the stale binding refusal','/** 🧾️ Whether a refusal says that the dependency store holds a name that is no generation or capsule: retiring obsolete generations is best effort, so it is told and left for a later setup instead of failing the whole stage. */\nexport function rosterRefusalV1(error:unknown):boolean{return error instanceof AggregateError?error.errors.some(rosterRefusalV1):error instanceof Error&&/Exact known current dependency generation roster required/u.test(error.message);}\n\n/** 🌀️ Returns the stale binding refusal')])
ed(C+"📖️texts/🔣️.json",[('    "dependency-store.bindings-moved": {','    "dependency-store.roster-unknown": {\n      "en": "The dependency store holds a name that is no generation: {detail}; obsolete generations are not retired this time ({retired} retired) and the rest is left for a later setup",\n      "de": "Der Abhängigkeitsspeicher enthält einen Namen, der keine Generation ist: {detail}; veraltete Generationen werden diesmal nicht entfernt ({retired} entfernt), der Rest bleibt einem späteren Setup überlassen"\n    },\n    "dependency-store.bindings-moved": {')])
print("ok")
