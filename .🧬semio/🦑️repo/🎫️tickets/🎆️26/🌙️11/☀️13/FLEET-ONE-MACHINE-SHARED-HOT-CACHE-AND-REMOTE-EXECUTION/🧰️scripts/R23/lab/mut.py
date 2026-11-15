import sys,subprocess,shutil,os,re
SC=os.path.dirname(os.path.abspath(__file__));PO="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable"
f=SC+"/ov/"+PO+"/🟦️.ts";orig=open(f,encoding="utf8").read()
muts={
 "backslashes kept after the root":('host.windows?root+rest.replaceAll("/","\\\\"):root+rest','root+rest'),
 "owner not tokenised":('return prefix===undefined?text:prefix+policy.ownerToken;','return text;'),
 "closure keys not moved":('keys.set(key,nextKey);moved.push([nextKey,next]);','keys.set(key,key);moved.push([key,next]);'),
 "root compared with case":('text.slice(0,root.length).toLowerCase()!==root.toLowerCase()','text.slice(0,root.length)!==root'),
 "Windows slash text not mapped":('const moved=text.replace(rootPattern(host),policy.rootToken);','const moved=text;'),
 "raw backslash text accepted":('if(rawPatterns(host).some(pattern=>pattern.test(moved)))throw new UnportableV1("root spelled with backslashes");',''),
 "closure query text not mapped":('index===0?item:mapStrings(item,way.text)','item'),
 "unknown kind left alone":('return{...row,path:way.text(row.path),...typeof row.fact==="string"?{fact:way.text(row.fact)}:{}};','return row;'),
 "extended prefix kept":('const text=slashed(path).replace(/^\\/\\/\\?\\//u,""),root','const text=slashed(path),root'),
}
for name,(a,b) in muts.items():
    if a not in orig:print("ANCHOR MISSING",name);continue
    open(f,"w",encoding="utf8").write(orig.replace(a,b,1))
    res=[]
    for t in ["🧪️tests/🟦️.ts","🧪️tests/🌍️real/🟦️.ts"]:
        r=subprocess.run([SC+"/lab/run-overlay.sh",SC+"/ov",PO+"/"+t],capture_output=True,text=True);txt=r.stdout+r.stderr
        res.append(len(re.findall(r"^\(fail\)",txt,flags=re.M)))
    print(name.ljust(36),"red tests:",res)
open(f,"w",encoding="utf8").write(orig)
