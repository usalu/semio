import sys,os
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/"
def sub(path,pairs):
    s=open(R+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60])
        s=s.replace(a,b)
    t=R+path+".h39";open(t,"w").write(s);os.replace(t,R+path)
sub('⚙️execution/🟦️.ts',[
('for(const unit of plan.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld){throw Error("Inactive Cargo unit changed under physical custody",{cause:describeCacheRecencyChangeV1(root,unit,row)});}}',
 'const retained:CacheUnit[]=[],skipped:{path:string;cause:unknown}[]=[];for(const unit of plan.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld)skipped.push({path:unit.path,cause:describeCacheRecencyChangeV1(root,unit,row)});else retained.push(unit);}if(skipped.length)await writeFile(join(output,"skipped.json"),JSON.stringify({version:1,note:{en:"Units used or changed after planning are active and were kept; the remaining inactive units were retired.",de:"Nach der Planung genutzte oder geänderte Units sind aktiv und blieben erhalten; die übrigen inaktiven Units wurden entfernt."},skipped}));'),
('let bytes=0,count=0;for(const unit of plan.units){await witnesses.check()','let bytes=0,count=0;for(const unit of retained){await witnesses.check()'),
('/** ♻️ Retires only inactive owned compiler units after complete process, descriptor and physical-lock custody. */','/** ♻️ Retires only inactive owned compiler units after complete process, descriptor and physical-lock custody; units used or changed since planning are active and kept (EN) / seit der Planung genutzte oder geänderte Units sind aktiv und bleiben (DE), never a failure. */')])
sub('🪟️conditional/🟦️.ts',[
(' for(const unit of options.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld)throw Error("Conditional Cargo unit changed before kernel custody");}',
 ' options=Object.freeze({...options,units:Object.freeze(options.units.filter(unit=>{const row=current.get(unit.path);return row!==undefined&&row.bytes===unit.bytes&&row.recencyMs===unit.recencyMs&&!row.lockHeld;}))});')])
sub('📜️history/🧪️tests/📦️compact/🟦️.ts',[
('import {readFileSync,existsSync','import {readdirSync,readFileSync,existsSync'),
('.then(()=>undefined,(error:unknown)=>error);expect(refusal).toBeInstanceOf(Error);expect((refusal as Error).message).toBe("Inactive Cargo unit changed under physical custody");diagnostic=(refusal as Error).cause as Record<string,unknown>;',
 '.then(result=>result,(error:unknown)=>error);expect(refusal).not.toBeInstanceOf(Error);expect(refusal).toMatchObject({units:0,logicalBytes:0,plannedUnits:1});const turn=readdirSync(join(parent,"r")).find(name=>name.startsWith("turn-"));if(!turn)throw Error("Actual retirement turn required");const skipped=JSON.parse(readFileSync(join(parent,"r",turn,"skipped.json"),"utf8")) as {note:{en:string;de:string};skipped:{path:string;cause:Record<string,unknown>}[]};expect(skipped.note.en.length).toBeGreaterThan(0);expect(skipped.note.de.length).toBeGreaterThan(0);expect(skipped.skipped).toHaveLength(1);diagnostic=skipped.skipped[0]!.cause;')])
sub('📜️history/🧪️tests/📏️recensus/🟦️.ts',[
('actual compact custody emits exact recensus refusal without deleting changed data','actual compact custody keeps a unit changed after planning and records the exact recensus diagnostic without failing'),
('checkpoint)),/Inactive Cargo unit changed under physical custody/u);','checkpoint)));')])
print("applied")
