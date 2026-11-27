import os
import re
import sys
import tempfile

BASE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission"
SRC = BASE + "/🟦️.ts"
TEST = BASE + "/🧪️tests/🟦️.ts"


def atomic_write(path, text):
    directory = os.path.dirname(path)
    fd, tmp = tempfile.mkstemp(dir=directory, prefix=".f113-")
    with os.fdopen(fd, "w", encoding="utf-8") as handle:
        handle.write(text)
        handle.flush()
        os.fsync(handle.fileno())
    os.chmod(tmp, os.stat(path).st_mode & 0o777)
    os.replace(tmp, path)


def patch_source(text):
    start = text.index("export function readBuildOwnerRegistryV1(")
    end_marker = " return structuredClone(registry);\n}"
    end = text.index(end_marker, start) + len(end_marker)
    old_fn = text[start:end]
    manifest = re.search(
        r'!\[[^\]]*\]\.some\(name=>owner\.manifest===\(owner\.root==="\."\?name:owner\.root\+"/"\+name\)\)',
        old_fn,
    ).group(0)
    new_fn = (
        "export function readBuildOwnerRegistryV1(value:unknown):BuildOwnerRegistryV1{\n"
        ' if(validateJsonSchemaSubset(registrySchema,value).length)throw Error("Canonical build owner registry required");\n'
        " const registry=value as BuildOwnerRegistryV1,names=new Map<string,string>(),roots=new Map<string,string>(),packages=new Map<string,string>();\n"
        " for(const owner of registry.owners){\n"
        "  const current=ownerLabelV1(owner);\n"
        '  if(names.has(owner.name))throw collisionV1(identityCollisionV1,"name",owner.name,names.get(owner.name)!,current);\n'
        '  if(roots.has(owner.root))throw collisionV1(identityCollisionV1,"root",owner.root,roots.get(owner.root)!,current);\n'
        '  if(!contains(owner.root,owner.manifest)||' + manifest + ')throw collisionV1(identityCollisionV1,"manifest mismatch",owner.manifest,"none (single-owner check)",current);\n'
        "  names.set(owner.name,current);\n"
        "  for(const root of[owner.root,...owner.nativeRoots??[]]){if(roots.has(root))throw collisionV1(nativeCollisionV1,\"native root\",root,roots.get(root)!,current);roots.set(root,current);}\n"
        "  for(const name of owner.packages){if(packages.has(name))throw collisionV1(packageCollisionV1,\"package\",name,packages.get(name)!,current);packages.set(name,current);}\n"
        " }\n"
        " return structuredClone(registry);\n"
        "}"
    )
    text = text[:start] + new_fn + text[end:]

    helpers = (
        'const identityCollisionV1="Canonical build owner identity collision",nativeCollisionV1="Canonical native owner collision",packageCollisionV1="Canonical package owner collision";\n'
        "const ownerLabelV1=(owner:BuildOwnerRegistryV1[\"owners\"][number])=>`${JSON.stringify(owner.name)} (root ${JSON.stringify(owner.root)})`;\n"
        "/** 🧾️ Refusal of a colliding owner identity: the unchanged first line is the prefix, then the field, the colliding value, the earlier owner holding it and the current owner. */\n"
        "const collisionV1=(prefix:string,field:string,value:string,earlier:string,current:string)=>Error(`${prefix}\\nfield: ${field}\\nvalue: ${JSON.stringify(value)}\\nearlier owner: ${earlier}\\ncurrent owner: ${current}`);\n"
    )
    anchor = "/** 📋️ Validates only declarations"
    at = text.index(anchor)
    text = text[:at] + helpers + text[at:]
    return text


TEST_APPEND = """
const collisionMessageV1=(registry:unknown)=>{try{readBuildOwnerRegistryV1(registry);}catch(error){return (error as Error).message;}throw Error("Owner collision was not refused");};
test("a build owner identity collision names the field, the colliding value and both owners",()=>{
 const owner=(name:string,root:string)=>({name,root,manifest:root+"/Cargo.toml",packages:[]});
 const byName=collisionMessageV1({version:1,owners:[owner("app","app"),owner("app","other")],catalogProviders:[]}),byRoot=collisionMessageV1({version:1,owners:[owner("a","shared"),owner("b","shared")],catalogProviders:[]});
 expect(byName.split("\\n")).toEqual(["Canonical build owner identity collision","field: name",'value: "app"','earlier owner: "app" (root "app")','current owner: "app" (root "other")']);
 expect(byRoot.split("\\n")).toEqual(["Canonical build owner identity collision","field: root",'value: "shared"','earlier owner: "a" (root "shared")','current owner: "b" (root "shared")']);
});
test("a native owner collision names the native root and both owners",()=>{
 const owner=(name:string,root:string,nativeRoots:string[])=>({name,root,manifest:root+"/Cargo.toml",packages:[],nativeRoots});
 expect(collisionMessageV1({version:1,owners:[owner("a","a",["native"]),owner("b","b",["native"])],catalogProviders:[]}).split("\\n")).toEqual(["Canonical native owner collision","field: native root",'value: "native"','earlier owner: "a" (root "a")','current owner: "b" (root "b")']);
});
test("a package owner collision names the package and both owners",()=>{
 const owner=(name:string,root:string,packages:string[])=>({name,root,manifest:root+"/Cargo.toml",packages});
 expect(collisionMessageV1({version:1,owners:[owner("a","a",["lib"]),owner("b","b",["lib"])],catalogProviders:[]}).split("\\n")).toEqual(["Canonical package owner collision","field: package",'value: "lib"','earlier owner: "a" (root "a")','current owner: "b" (root "b")']);
});
"""


def patch_test(text):
    if not text.endswith("\n"):
        text += "\n"
    return text + TEST_APPEND


def main():
    src = open(SRC, encoding="utf-8").read()
    test = open(TEST, encoding="utf-8").read()
    if "identityCollisionV1" in src or "collisionMessageV1" in test:
        sys.exit("already applied")
    atomic_write(SRC, patch_source(src))
    atomic_write(TEST, patch_test(test))
    print("applied")


if __name__ == "__main__":
    main()
