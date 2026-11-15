import sys
def apply(t):
    old='''  async settled({rows,ids,stale}){
   const selected=new Uint8Array(rows.length),verified=id=>selected[id]===1,exclude=new Set();for(const id of ids)selected[id]=1;
   for(let id=0;id<rows.length;id++){
    const row=rows[id],own=new Set(),paths=new Set();rowDependenciesV1(row,own,paths);
    if(!own.size)continue;
    if(!verified(id)||stale.has(id))for(const path of own)exclude.add(path);
   }
   return stamps.save(measured,exclude);
  },'''
    new='''  async settled({rows,ids,stale}){
   const selected=new Uint8Array(rows.length),exclude=new Set(),forget=id=>{const own=new Set();rowDependenciesV1(rows[id],own,new Set());for(const path of own)exclude.add(path);};
   for(const id of ids)selected[id]=1;
   for(let id=0;id<rows.length;id++)if(selected[id]!==1)forget(id);
   for(const id of stale)if(selected[id]===1)forget(id);
   return stamps.save(measured,exclude);
  },'''
    assert t.count(old)==1
    t=t.replace(old,new)
    old2='/** 💾️ After the check: keeps the stamps of the directories whose rows were all verified to hold (`verified(id)`) and none stale, so the next pass vouches for them. */'
    assert t.count(old2)==1
    return t.replace(old2,'/** 💾️ After the check: keeps the stamps of the directories whose rows were all verified to hold and none stale, so the next pass vouches for them; only the rows that were not verified and the stale ones are read, never the rows that held. */')
if __name__=="__main__":
    open(sys.argv[2],'w').write(apply(open(sys.argv[1]).read()))
