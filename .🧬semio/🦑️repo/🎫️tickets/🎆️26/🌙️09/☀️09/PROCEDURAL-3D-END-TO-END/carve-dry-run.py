import json,os,sys,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import importlib.util
spec=importlib.util.spec_from_file_location("pc",os.path.join(os.path.dirname(os.path.abspath(__file__)),"plan-carve.py"));pc=importlib.util.module_from_spec(spec);spec.loader.exec_module(pc)
R="/Users/ueli/Documents/semio"
SPECS={
"semio-s-artifact-stdio-pdf-model":dict(suffix="model",groups={"core":["object_model","document_metadata","navigation"],"content":["content_operators","colour_and_graphics_state","xobjects"],"font":["font_model"],"annotations":["annotations_and_forms"],"page":["page_model"]},
  note="no facade and no re-exports: the root crate keeps only sqlite, fonts metrics/truetype, io owners, examples::demo, canonical, octets (what genuinely belongs there); every consumer is repointed to the owning crate; carve.owned of the root drops the moved modules and each new crate gets its own",
  consumers="types"),
"semio-s-artifact-stdio-dwg-snapshot-objects":dict(suffix="objects",groups={g:[g] for g in ["metadata","xrecord","tables","entities","associativity","styles","blocks","constraints"]},
  note="the root `objects` module (the object body union and layouts) is what genuinely belongs in the root crate; it depends on every family crate and re-exports nothing; every consumer path `snapshot::objects::<family>::` is repointed to the family crate",
  consumers="paths"),
"semio-s-artifact-stdio-gltf-model-snapshot":dict(suffix="snapshot",groups={"document":["document_and_decoded_i","accessors_and_buffers"],"scene":["scene_graph","cameras"],"materials":["materials_and_textures"],"animation":["skins_and_animations"],"encoding":["encoding","owned_pack","source_form_and_json","attribute_map_and_defa","semantic_cache_codec"]},
  note="group names for modules that start with the listed prefix; verify with dependsOn, regroup on a cycle"),
"semio-s-artifact-raster-raster-editor":dict(suffix="editor",groups={"media":["base64_ingress","media_export","reserved_import"],"app":["app","app_assembly"],"retained":["retained_commands","original_work","store_preparation","original_genesis","original_ephemeral"]},
  note="the editor crate keeps wasm, args_bridge, demo_session, resource_policy and re-exports; consumers (8 dependents) keep their paths"),
"semio-s-artifact-raster-raster-editor-session":dict(suffix="session",groups={"config":["config"],"presence":["presence","selection","layer_selection"],"scene":["scene_helpers","terminology","preparation","initialization"]},
  note="config alone is 47 KB"),
"semio-s-artifact-stdio-txt-tests":dict(suffix="tests",groups={},note="app-fleet separation: editor and viewer sources mounted by mirror move to `-editor` and `-viewer` crates; this crate keeps TxtApps nested as XEditorApps / XViewerApps (see split-hub-apps.py for the nesting and From impls); `pub use semio_s_artifact_stdio_txt_tests::TxtApps` in the artifact root is unchanged"),
"semio-s-artifact-stdio-binary-io-registry":dict(suffix="io-registry",groups={},note="same as txt-tests for BinaryApps; 113 lock files"),
}
def consumers(pkg,s,p,owner):
    import subprocess,collections
    kind=s.get("consumers")
    if not kind: return []
    ident=pkg.replace("-","_")
    pat=["snapshot::objects"] if kind=="paths" else ["pdf_model","snapshot::model"]
    files=[f for f in subprocess.run(["grep","-rIl","--include=*.rs"]+sum([["-e",x] for x in pat],[])+[".","--exclude-dir=node_modules","--exclude-dir=.git","--exclude-dir=target","--exclude-dir=.cache","--exclude-dir=🎫️tickets"],capture_output=True,text=True).stdout.split("\n") if f]
    mine={os.path.normpath(m["file"]) for m in p["modules"]}
    files=[f[2:] for f in files if os.path.normpath(f[2:]) not in mine]
    out=["","## Consumers to repoint (every one, directly to the owning crate)"]
    if kind=="paths":
        fam=collections.Counter()
        for f in files:
            txt=open(f,encoding="utf8").read()
            hit=sorted(set(re.findall(r"snapshot::objects::([a-z_]+)::",txt)))
            if hit: out.append(f"- `{f}`: "+", ".join(f"{h} -> {pkg}-{h}" for h in hit if h in s["groups"]))
    else:
        rootsrc=open(p["entry"].rsplit("/",3)[0]+"/🦀️.rs",encoding="utf8").read() if os.path.exists(p["entry"].rsplit("/",3)[0]+"/🦀️.rs") else ""
        tmap={}
        for mod,names in re.findall(r"pub use (\w+)::\{([^}]*)\}",rootsrc,re.S)+[(m,n) for m,n in re.findall(r"pub use (\w+)::(\w+);",rootsrc)]:
            for n in re.findall(r"\w+",names):
                g=owner.get(mod)
                if g and n[0].isupper(): tmap[n]=g
        for f in files:
            txt=open(f,encoding="utf8").read();hit=sorted({n for n in tmap if re.search(r"\b%s\b"%n,txt)})
            if hit: out.append(f"- `{f}`: "+", ".join(sorted({f'{pkg}-{tmap[n]}' for n in hit}))+f" ({len(hit)} types)")
    return out
def run(pkg):
    idx=pc.index();p=pc.plan(pkg,idx);s=SPECS[pkg];mods={m['name']:m for m in p['modules']}
    L=[f"# Dry run: {pkg}",f"manifest `{p['manifest']}`, source {p['totalBytes']} bytes, carve metadata: {p['carveMetadata']}",f"note: {s['note']}",""]
    owner={}
    for g,ms in s['groups'].items():
        for m in ms:
            hit=[n for n in mods if n==m or n.startswith(m)]
            for n in hit: owner[n]=g
    base=pkg.rsplit('-',1)[0] if s['suffix'] and pkg.endswith(s['suffix']) else pkg
    gdeps={}
    for n,g in owner.items():
        for d in mods[n]['dependsOn']:
            dg=owner.get(d,'facade')
            if dg!=g: gdeps.setdefault(g,set()).add(dg)
    cyc=[g for g in gdeps if any(g in gdeps.get(x,()) and x in gdeps[g] for x in gdeps[g])]
    L+=["## New crates"]
    for g,ms in s['groups'].items():
        files=[mods[n] for n in mods if owner.get(n)==g]
        L.append(f"- `{base}-{g}` ({sum(f['bytes'] for f in files)} bytes) depends on groups {sorted(gdeps.get(g,[]))}; files: "+", ".join(f"`{f['file']}`" for f in files))
    L+=["",f"group cycles: {cyc or 'none'}","","## Edits"]
    L+=[f"- root `{p['manifest']}` + `{p['entry']}`: delete the moved `#[path] mod` lines (no re-exports, no aliases); add the new crates only where the root itself uses them"]
    L+=[f"- workspace manifest `{w}`: members and [workspace.dependencies.<new crate>]" for w in p['workspaceManifests']]
    L+=[f"- direct dependent manifests that must name the owning crates they use (no facade): "+", ".join(f"`{d}`" for d in p['directDependents'])]
    L+=consumers(pkg,s,p,owner)
    L+=[f"- `📋️build-owners.json`: one row per new crate, root = its `📦️packages/🦀️rust`"]
    L+=[f"- {len(p['lockFiles'])} Cargo.lock files gain one package block per new crate and the facade's dependency edges: "+", ".join(f"`{l}`" for l in p['lockFiles'])]
    L+=["","## Verify (when released)","`repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p <each new crate> -p "+pkg+" --lib`, memory_pressure normal, at least 40 percent free, one cargo at a time; then re-read `units.jsonl` peaks and drop the carving entry when below 1536 MB."]
    out=f"{os.path.dirname(os.path.abspath(__file__))}/📓️dry-run-{pkg}.md"
    open(out,'w',encoding='utf8').write("\n".join(L)+"\n");print(out,len(p['lockFiles']),cyc)
for k in (sys.argv[1:] or SPECS): run(k)
