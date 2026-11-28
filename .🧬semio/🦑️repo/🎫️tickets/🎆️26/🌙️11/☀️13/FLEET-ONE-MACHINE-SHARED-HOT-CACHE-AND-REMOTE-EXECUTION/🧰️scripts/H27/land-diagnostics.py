import sys
root="/Users/ueli/Documents/semio/🧰️framework/"
edits=[
(root+"🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts",
 'else if([...persistentReceivers].some(row=>row.image&&row.stage!=="closed"&&options.directory.startsWith(row.image.root+sep)))throw Error("Nested persistent image requires actual parent scope");',
 'else{const outer=[...persistentReceivers].find(row=>row.image&&row.stage!=="closed"&&options.directory.startsWith(row.image.root+sep));if(outer)throw Error("Nested persistent image requires actual parent scope: "+options.directory+" lies inside "+outer.image!.root+" ("+outer.stage+")");}'),
(root+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts",
 'throw Error("One complete effective Cargo configuration required",{cause:{version:1,status:output.status,signaled:output.signal!==null,stdoutBytes:Buffer.byteLength(output.stdout),stderrBytes:Buffer.byteLength(output.stderr)}});',
 'throw Error("One complete effective Cargo configuration required (status "+output.status+", signal "+output.signal+", "+Buffer.byteLength(output.stdout)+" stdout bytes): "+output.stderr.split("\\n").find(line=>line.trim()!=="")?.slice(0,300),{cause:{version:1,status:output.status,signaled:output.signal!==null,stdoutBytes:Buffer.byteLength(output.stdout),stderrBytes:Buffer.byteLength(output.stderr)}});'),
(root+"🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts",
 'const paths=nativeArtifactCacheProducerLayoutV1(layout.root,policy,candidate.context),bound=record(paths.charge,"Binding")as Binding,observed=',
 'const paths=nativeArtifactCacheProducerLayoutV1(layout.root,policy,candidate.context),bound=(()=>{try{return record(paths.charge,"Binding")as Binding;}catch(error){throw (error as NodeJS.ErrnoException).code==="ENOENT"?new Error("Producer charge record missing for a "+entry!.state+" slot: "+paths.charge,{cause:error}):error;}})(),observed='),
]
data=[]
for p,a,b in edits:
    s=open(p,encoding="utf8").read()
    assert s.count(a)==1,p
    data.append((p,s.replace(a,b)))
for p,s in data: open(p,"w",encoding="utf8").write(s)
print("ok")
