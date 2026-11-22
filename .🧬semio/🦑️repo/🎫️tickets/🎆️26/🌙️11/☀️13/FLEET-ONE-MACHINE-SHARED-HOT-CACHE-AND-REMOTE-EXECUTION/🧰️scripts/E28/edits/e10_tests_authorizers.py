TL="⚡️caching/🚀️bootstrap/🛠️tools/"
def plan(p,L):
    t=L+TL+"🔏️authorizers/🧪️tests/🟦️.ts"
    p.sub(t,'node.expression.getText(tree)==="files.set"&&node.arguments[0]?.text===".semio-authorizer-compiler.json"','node.expression.getText(tree)==="authorizerFiles.set"&&node.arguments[0]?.text===".semio-authorizer-compiler.json"')
    p.sub(t,'.nxToolingInstallIdentityV1,environment={platform:"neutral",architecture:"neutral",libc:""},values:string[]=[];','.nxAuthorizerBundleIdentityV1,dependencies="d".repeat(64),values:string[]=[];')
    p.sub(t,'new Function("files","compiler","manifest","Buffer","root",code)(files,compiler,{packageManager:"bun@1.4.2"},Buffer,root);','new Function("authorizerFiles","compiler","manifest","Buffer","root",code)(files,compiler,{packageManager:"bun@1.4.2"},Buffer,root);')
    p.sub(t,'const digest=identity(environment,files),independent=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v4\\0").update("7:neutral7:neutral0:");','const digest=identity(dependencies,files),independent=new Bun.CryptoHasher("sha256").update("nx-authorizer-bundle-v1\\0").update("64:"+dependencies);')
    p.sub(t,'test("authorizer publication identity binds the original physical root"','test("authorizer bundle identity binds the original physical root"')
    p.sub(t,'generation=path.join(area,"f".repeat(64)),sdk=path.join(generation,"node_modules/nx")','generation=path.join(area,"nx","f".repeat(64)),bundleRoot=path.join(area,"nx-authorizers","e".repeat(64)),sdk=path.join(generation,"node_modules/nx")')
    p.sub(t,'await writeFile(path.join(generation,"semio-authorizers.mjs"),"neutral immutable body");','await mkdir(bundleRoot,{recursive:true});await writeFile(path.join(bundleRoot,"semio-authorizers.mjs"),"neutral immutable body");')
    p.sub(t,',authorizers:{version:1,mode:"release",entry:"semio-authorizers.mjs",sourceSha256:hash(bytes),outputs:[{path:"semio-authorizers.mjs",sha256:hash("neutral immutable body")}]}};await writeFile(path.join(generation,".semio-authorizer-source.json"),bytes);await writeFile(path.join(generation,".semio-nx-tooling.json"),JSON.stringify(marker));let accepted=false;','},bundle={version:1,digest:"e".repeat(64),deps:"f".repeat(64),authorizers:{version:1,mode:"release",entry:"semio-authorizers.mjs",sourceSha256:hash(bytes),outputs:[{path:"semio-authorizers.mjs",sha256:hash("neutral immutable body")}]}};await writeFile(path.join(bundleRoot,".semio-authorizer-source.json"),bytes);await writeFile(path.join(generation,".semio-nx-tooling.json"),JSON.stringify(marker));await writeFile(path.join(bundleRoot,".semio-nx-authorizers.json"),JSON.stringify(bundle));process.env.SEMIO_NX_AUTHORIZER_ROOT=bundleRoot;let accepted=false;')
    p.sub(t,'  await writeFile(path.join(generation,".semio-authorizer-source.json"),acceptedManifest);','  await writeFile(path.join(bundleRoot,".semio-authorizer-source.json"),acceptedManifest);')
    p.sub(t,'''   await writeFile(path.join(generation,output),payload);
   await writeFile(path.join(generation,".semio-nx-tooling.json"),JSON.stringify({version:1,digest:"f".repeat(64),patches:{"nx@23.2.0":{version:1,files:{"dist/src/utils/handle-import.js":hash(image)}}},authorizers}));''','''   await writeFile(path.join(bundleRoot,output),payload);
   await writeFile(path.join(generation,".semio-nx-tooling.json"),JSON.stringify({version:1,digest:"f".repeat(64),patches:{"nx@23.2.0":{version:1,files:{"dist/src/utils/handle-import.js":hash(image)}}}}));await writeFile(path.join(bundleRoot,".semio-nx-authorizers.json"),JSON.stringify({version:1,digest:"e".repeat(64),deps:"f".repeat(64),authorizers}));process.env.SEMIO_NX_AUTHORIZER_ROOT=bundleRoot;''')
    p.sub(t,'expect(nxAuthorizerReceiptHoldsV1(generation,authorizers),String(bytes)).toBe(accepted);','expect(nxAuthorizerReceiptHoldsV1(bundleRoot,authorizers),String(bytes)).toBe(accepted);')
    p.sub(t,' }finally{db.close();await rm(area,{recursive:true,force:true});}expect(fs.readFileSync(file,"utf8")).toBe(patch);',' }finally{delete process.env.SEMIO_NX_AUTHORIZER_ROOT;db.close();await rm(area,{recursive:true,force:true});}expect(fs.readFileSync(file,"utf8")).toBe(patch);')
    s=L+TL+"🔏️authorizers/🧪️tests/📥️scope/🟦️.ts"
    p.sub(s,'node.expression.getText(source)==="files.set"','node.expression.getText(source)==="authorizerFiles.set"')
    p.sub(s,'new Function("files","compiler","manifest","Buffer","root",code)(files,compiler,{packageManager:"bun@1.3.9"},Buffer,process.cwd());','new Function("authorizerFiles","compiler","manifest","Buffer","root",code)(files,compiler,{packageManager:"bun@1.3.9"},Buffer,process.cwd());')
    c=L+TL+"📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🔣️.json"
    p.sub(c,'''      "bun.lock",
      ".semio-authorizer-source.json",
      ".semio-authorizer-compiler.json",
      ".semio-sdk-material.json",''','''      "bun.lock",
      ".semio-sdk-material.json",''')
    p.sub(c,'''      ".semio-nx-tooling.json",
      "node_modules/nx/package.json"''','''      ".semio-authorizer-source.json",
      ".semio-authorizer-compiler.json",
      ".semio-nx-tooling.json",
      "node_modules/nx/package.json"''')
