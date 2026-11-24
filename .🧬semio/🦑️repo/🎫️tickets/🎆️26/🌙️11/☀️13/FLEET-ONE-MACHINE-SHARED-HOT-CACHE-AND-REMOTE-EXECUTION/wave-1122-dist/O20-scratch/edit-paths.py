import sys
p=sys.argv[1]
s=open(p,encoding='utf8').read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:60],s.count(old))
    s=s.replace(old,new)
sub('const MINIMUM_SHIPPED_DEPTH=3;','const MINIMUM_SHIPPED_DEPTH=3,REACHED_FILES=policy.limits.maximumInputFiles;')
sub('reachedFilesV1(sourceReal,undefined,undefined,manifest)','reachedFilesV1(sourceReal,undefined,REACHED_FILES,manifest)',3)
sub('includedSourcesV1(sourceReal,undefined,undefined,manifest)','includedSourcesV1(sourceReal,undefined,REACHED_FILES,manifest)')
sub('||workspace===shipped||remappedNameV1','||remappedNameV1')
sub('`--remap-path-prefix=${shipped}=${relative(workspace,shipped).split(sep).join("/")}`','`--remap-path-prefix=${shipped}=${shipped===workspace?".":relative(workspace,shipped).split(sep).join("/")}`')
sub('''/** 🔗️ The files that the root source reaches beyond itself (see `reachedFilesV1`); the empty list beyond the bound. */''','''const DERIVES=/(?<![A-Za-z0-9_])derive\\s*\\(([^()]*)\\)/gu,WORKSPACE_DERIVES=/(?<![A-Za-z0-9_])(?:MutationLeaf|Mutations)(?![A-Za-z0-9_])/u;
/** 🔖️ The first of `files` that applies the mutation derive (`#[derive(MutationLeaf)]`, `#[derive(Mutations)]`, also inside a `cfg_attr`): the derive reads the workspace authority (the workspace markers, the generated authority projection and schema index, the leaf descriptors and schema documents) by workspace-relative locators and bakes a token of the REAL workspace path into the unit (`MutationSourceProvenance`), so it expands only in the original worktree: on another machine it answers `E0463` (the incomplete-inputs sentinel) after the whole upload, queue and run, or would give other bytes. `undefined` when no file applies it. */
export function workspaceDeriveFileV1(files:Iterable<string>,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):string|undefined{
 for(const file of files){
  const text=file.endsWith(".rs")?read(file):undefined;
  if(text===undefined)continue;
  for(const match of text.matchAll(DERIVES))if(WORKSPACE_DERIVES.test(match[1]!))return file;
 }
 return undefined;
}
/** 🔗️ The files that the root source reaches beyond itself (see `reachedFilesV1`); the empty list beyond the bound. */''')
sub(''' const shipped=reached.reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest),anchored=within(cwd,shipped);''',''' const deriving=workspaceDeriveFileV1(reached);
 if(deriving!==undefined)return decline("not-portable","the mutation derive in "+relative(cwd,deriving).split(sep).join("/")+" reads the workspace authority and bakes the workspace path into the unit: it expands only in the original worktree");
 const shipped=reached.reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest),anchored=within(cwd,shipped);''')
open(p,'w',encoding='utf8').write(s)
