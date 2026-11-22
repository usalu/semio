import sys,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
G=L+"⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/"
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    before=hashlib.sha256(s.encode()).hexdigest()[:8]
    for old,new in pairs:
        assert s.count(old)==1,(path[-40:],old[:60],s.count(old))
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
    print(path[-60:],before,"->",hashlib.sha256(s.encode()).hexdigest()[:8])
which=sys.argv[1]
if which=="bun":
    p=L+"🗂️workspaces/🟦️bun/🟨️.cjs"
    edit(p,[
 ('  return selection.kind === "closure" ? yield* closurePlan(owner, scopes, selection.roots) : scopes;\n',
  '  owner.scopeDirectories = scopes.map(scope => scope.directory);\n  return selection.kind === "closure" ? yield* closurePlan(owner, scopes, selection.roots) : scopes;\n'),
 ('/** 🧬️ Narrows the declared scopes to the root scope,',
  '/** 🧭️ Names the deepest declared scope directory that contains the path (the path itself or a descendant), the root scope when none does; the one ownership rule of closures and of the manifests a walk retained. */\nfunction deepestScopeDirectory(directories, path) {\n  const depth = directory => directory === "." ? 0 : directory.split("/").length;\n  return directories.reduce((best, directory) => (directory === "." || path === directory || path.startsWith(directory + "/")) && depth(directory) > depth(best) ? directory : best, ".");\n}\n/** 🧬️ Narrows the declared scopes to the root scope,'),
 (', depth = directory => directory === "." ? 0 : directory.split("/").length;\n  if (!byDirectory.has(".")',';\n  if (!byDirectory.has(".")'),
 ('  const ownerOf = path => scopes.reduce((best, scope) => (scope.directory === "." || path === scope.directory || path.startsWith(scope.directory + "/")) && depth(scope.directory) > depth(best) ? scope.directory : best, ".");\n',
  '  const ownerOf = path => deepestScopeDirectory(scopes.map(scope => scope.directory), path);\n'),
 ('  return Object.freeze({ root: owner.root, paths, texts: Object.freeze(texts) });',
  '  const directories = owner.scopeDirectories ?? [], textOwners = Object.freeze(Object.fromEntries(texts.map(({ path }) => [path, deepestScopeDirectory(directories, path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : ".")])));\n  return Object.freeze({ root: owner.root, paths, texts: Object.freeze(texts), textOwners });'),
 ('/** 🔭️ Exposes the exact path states, directory listing projections and manifest digests one ownership computation retained, so a later process can prove the same topology by stamps. */',
  '/** 🔭️ Exposes the exact path states, directory listing projections and manifest digests one ownership computation retained, so a later process can prove the same topology by stamps; `textOwners` names, per retained manifest, the declared installation scope that owns it. */'),
    ])
    edit(L+"🗂️workspaces/🟦️bun/🟦️.ts",[
 ('texts: readonly Readonly<{ path: string; sha256: string }>[] }>;','texts: readonly Readonly<{ path: string; sha256: string }>[]; textOwners: Readonly<Record<string, string>> }>;')])
elif which=="gen":
    edit(G+"🧬️schema/🔣️.json",[
 ('"walkStamp": "semio.dependency-generation-walk/3"','"walkStamp": "semio.dependency-generation-walk/4"'),
 ('''      "required": [
        "path",
        "sha256",
        "profileSha256"
      ],
      "properties": {
        "path": {
          "type": "string",
          "minLength": 1,
          "maxLength": 2048
        },
        "sha256": {
          "type": "string",
          "pattern": "^[a-f0-9]{64}$"
        },
        "profileSha256": {
          "anyOf": [
            {
              "type": "string",
              "pattern": "^[a-f0-9]{64}$"
            },
            {
              "type": "null"
            }
          ]
        }
      }''','''      "required": [
        "path",
        "owner",
        "sha256",
        "profileSha256"
      ],
      "properties": {
        "path": {
          "type": "string",
          "minLength": 1,
          "maxLength": 2048
        },
        "owner": {
          "$ref": "#/$defs/Scope"
        },
        "sha256": {
          "type": "string",
          "pattern": "^[a-f0-9]{64}$"
        },
        "profileSha256": {
          "anyOf": [
            {
              "type": "string",
              "pattern": "^[a-f0-9]{64}$"
            },
            {
              "type": "null"
            }
          ]
        }
      }'''),
    ])
    edit(G+"🟦️.ts",[
 ('texts:readonly Readonly<{path:string;sha256:string;profileSha256:string|null}>[]}>;','texts:readonly Readonly<{path:string;owner:string;sha256:string;profileSha256:string|null}>[]}>;'),
 ('readonly texts:readonly Readonly<{path:string;sha256:string;profileSha256?:string|null}>[]}','readonly texts:readonly Readonly<{path:string;owner:string;sha256:string;profileSha256?:string|null}>[]}'),
 (' const frontier=evidenceSinkV1(protocol.frontierSuffix),listed=',' const scoped=new Set(input.required),foreign=new Set(membership.texts.filter(row=>!scoped.has(row.owner)).map(row=>row.path)),frontier=evidenceSinkV1(protocol.frontierSuffix),listed='),
 ('  if(!row.list){frontier.push(["s",path,...row.state]);continue;}','  if(!row.list){if(!foreign.has(path))frontier.push(["s",path,...row.state]);continue;}'),
 (' const inputs=input.inputs.map(group=>',' const inputs=input.inputs.filter(group=>scoped.has(group.scope)).map(group=>'),
 ('texts:membership.texts.map(row=>({path:row.path,sha256:row.sha256,profileSha256:row.profileSha256??null}))}','texts:membership.texts.filter(row=>scoped.has(row.owner)).map(row=>({path:row.path,owner:row.owner,sha256:row.sha256,profileSha256:row.profileSha256??null}))}'),
 ('  if(receipt.walkStamp!==protocol.walkStamp)return mismatch("protocol-changed",undefined,receipt.walkStamp);\n',
  '  if(receipt.walkStamp!==protocol.walkStamp)return mismatch("protocol-changed",undefined,receipt.walkStamp);\n  const owned=new Set(receipt.required);if(receipt.inputs.some(group=>!owned.has(group.scope))||receipt.membership.texts.some(row=>!owned.has(row.owner)))return mismatch("receipt-invalid");\n'),
    ])
    edit(G+"💾️storage/🟦️.ts",[
 ('  membership:{...observed,texts:observed.texts.map(row=>({...row,profileSha256:manifestTextProfileV1(root,row)}))},',
  '  membership:{root:observed.root,paths:observed.paths,texts:observed.texts.map(row=>({...row,owner:manifestTextOwnerV1(observed,row.path),profileSha256:manifestTextProfileV1(root,row)}))},'),
 ('/** 🗺️ Maps one completed walk onto the neutral capture input;',
  '/** 🧭️ Reads the declared installation scope the ownership walk recorded for one retained manifest; a walk that recorded none cannot be sealed. */\nconst manifestTextOwnerV1=(observed:BunWorkspaceObservedOwnershipV1,path:string):string=>{const owner=observed.textOwners[path];if(owner===undefined)throw Error("Retained manifest owner required");return owner;};\n/** 🗺️ Maps one completed walk onto the neutral capture input;'),
    ])
if which=="oracle":
    edit(G+"🧪️tests/🐍️.py",[
 ('TIERS = "walk stamp, runtime, installer configuration files, installer inputs, workspace manifests"\n',
  'TIERS = "walk stamp, receipt scoping, runtime, installer configuration files, installer inputs, workspace manifests"\n\n\ndef owner_of(directories, path):\n    depth = lambda directory: 0 if directory == "." else len(directory.split("/"))\n    best = "."\n    for directory in directories:\n        if (directory == "." or path == directory or path.startswith(directory + "/")) and depth(directory) > depth(best):\n            best = directory\n    return best\n\n\ndef text_owner(directories, path):\n    return owner_of(directories, path.rsplit("/", 1)[0] if "/" in path else ".")\n\n\ndef scoped_texts(texts, required):\n    return [text["path"] for text in texts if text["owner"] in set(required)]\n'),
 ('        return {"status": "mismatch", "class": "protocol-changed", "detail": receipt["walkStamp"]}\n',
  '        return {"status": "mismatch", "class": "protocol-changed", "detail": receipt["walkStamp"]}\n    owned = set(receipt["required"])\n    if any(group["scope"] not in owned for group in receipt["inputs"]) or any(text["owner"] not in owned for text in receipt["membership"]["texts"]):\n        return {"status": "mismatch", "class": "receipt-invalid"}\n'),
 ('    elif command == "render":',
  '    elif command == "owners":\n        directories = json.loads(sys.argv[2])\n        print(json.dumps({path: text_owner(directories, path) for path in json.loads(sys.argv[3])}, ensure_ascii=False))\n    elif command == "scoped":\n        print(json.dumps(scoped_texts(json.loads(sys.argv[2]), json.loads(sys.argv[3])), ensure_ascii=False))\n    elif command == "render":'),
 ('verify RECEIPT WORKSPACE STORE RUNTIME_JSON SCHEMA ENVIRONMENT_JSON | render SCHEMA ROWS_JSON (','verify RECEIPT WORKSPACE STORE RUNTIME_JSON SCHEMA ENVIRONMENT_JSON | owners DIRECTORIES_JSON PATHS_JSON | scoped TEXTS_JSON REQUIRED_JSON | render SCHEMA ROWS_JSON ('),
    ])
if which=="gentest":
    edit(G+"🧪️tests/🟦️.ts",[
 ('membership:{...walked,texts:walked.texts.map(row=>({...row,profileSha256:manifestProfileSha256V1(row.path,readFileSync(join(root,row.path)))}))}',
  'membership:{...walked,texts:walked.texts.map(row=>({...row,owner:walked.textOwners[row.path]!,profileSha256:manifestProfileSha256V1(row.path,readFileSync(join(root,row.path)))}))}')])
