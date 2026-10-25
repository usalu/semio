#!/usr/bin/env python3
"""Differential matrix: which manifest mutations make the real pinned Bun refuse `--frozen-lockfile`?
usage: diff-bun.py <scratch-root> [out.json]   (BUN_BIN = pinned bun)"""
import json,os,shutil,subprocess,sys
scratch=sys.argv[1];out=sys.argv[2] if len(sys.argv)>2 else None
B=os.environ.get('BUN_BIN','bun')
def w(root,rel,doc):
    p=os.path.join(root,rel,'package.json');os.makedirs(os.path.dirname(p),exist_ok=True)
    json.dump(doc,open(p,'w'))
BASE={
 '.':{'name':'root','private':True,'workspaces':['pkgs/a','pkgs/b'],'overrides':{}},
 'pkgs/a':{'name':'a','version':'1.0.0','dependencies':{'ms':'2.1.3','b':'workspace:*'},'devDependencies':{'is-number':'7.0.0'}},
 'pkgs/b':{'name':'b','version':'2.0.0','dependencies':{'ms':'2.1.3'}},
}
BASE_WS={
 '.':{'name':'root','private':True,'workspaces':['pkgs/a','pkgs/b']},
 'pkgs/a':{'name':'a','version':'1.0.0','dependencies':{'b':'workspace:*'}},
 'pkgs/b':{'name':'b','version':'2.0.0'},
}
WS_CASES={'baseline','member version bump','workspace dep removed','workspace dep added','member renamed','member dropped from workspaces','member added to workspaces','root name changed','scripts changed (inert)','trustedDependencies added'}
MODE=os.environ.get('BASE_MODE','ext')
def case(name,mut,pre=None):
    return (name,mut,pre)
def edit(path,fn):
    def m(docs):
        fn(docs[path])
    return m
cases=[
 case('baseline',lambda d:None),
 case('member version bump',edit('pkgs/b',lambda x:x.update(version='2.0.9'))),
 case('workspace dep removed',edit('pkgs/a',lambda x:x['dependencies'].pop('b'))),
 case('workspace dep added',edit('pkgs/b',lambda x:x.setdefault('dependencies',{}).update(a='workspace:*'))),
 case('external dep removed (still used elsewhere)',edit('pkgs/a',lambda x:x['dependencies'].pop('ms'))),
 case('external dep removed (last user)',lambda d:(d['pkgs/a']['devDependencies'].pop('is-number'))),
 case('external dep added, already locked, same range',edit('pkgs/b',lambda x:x['dependencies'].update({'is-number':'7.0.0'}))),
 case('external dep range changed, same resolution',edit('pkgs/a',lambda x:x['dependencies'].update({'ms':'^2.1.3'}))),
 case('external dep range changed, other resolution',edit('pkgs/a',lambda x:x['dependencies'].update({'ms':'2.1.2'}))),
 case('external dep added, never locked',edit('pkgs/a',lambda x:x['dependencies'].update({'ansi-regex':'6.0.1'}))),
 case('dep moved dependencies to devDependencies',lambda d:(d['pkgs/a']['devDependencies'].update(ms=d['pkgs/a']['dependencies'].pop('ms')))),
 case('peerDependencies added',edit('pkgs/a',lambda x:x.update(peerDependencies={'ms':'2.1.3'}))),
 case('optionalDependencies added (locked)',edit('pkgs/a',lambda x:x.update(optionalDependencies={'is-number':'7.0.0'}))),
 case('member renamed',edit('pkgs/a',lambda x:x.update(name='a2'))),
 case('member dropped from workspaces',lambda d:d['.'].update(workspaces=['pkgs/a'])),
 case('member added to workspaces',lambda d:(d['.'].update(workspaces=['pkgs/a','pkgs/b','pkgs/c']),d.__setitem__('pkgs/c',{'name':'c','version':'1.0.0'}))),
 case('root dependency added (external, locked)',lambda d:d['.'].update(dependencies={'ms':'2.1.3'})),
 case('overrides added',lambda d:d['.'].update(overrides={'ms':'2.1.3'})),
 case('trustedDependencies added',lambda d:d['.'].update(trustedDependencies=['ms'])),
 case('scripts changed (inert)',edit('pkgs/a',lambda x:x.update(scripts={'x':'y'}))),
 case('overrides removed',lambda d:d['.'].update(overrides={}),lambda d:d['.'].update(overrides={'ms':'2.1.3'})),
 case('overrides value changed',lambda d:d['.'].update(overrides={'ms':'2.1.2'}),lambda d:d['.'].update(overrides={'ms':'2.1.3'})),
 case('overrides unchanged',lambda d:None,lambda d:d['.'].update(overrides={'ms':'2.1.3'})),
 case('resolutions added',lambda d:d['.'].update(resolutions={'ms':'2.1.3'})),
 case('trustedDependencies removed',lambda d:d['.'].update(trustedDependencies=[]),lambda d:d['.'].update(trustedDependencies=['ms'])),
 case('root name changed',lambda d:d['.'].update(name='root2')),
 case('peerDependencies added, never locked',edit('pkgs/a',lambda x:x.update(peerDependencies={'ansi-regex':'6.0.1'}))),
 case('optionalDependencies added, never locked',edit('pkgs/a',lambda x:x.update(optionalDependencies={'ansi-regex':'6.0.1'}))),
 case('devDependencies added, never locked',edit('pkgs/a',lambda x:x['devDependencies'].update({'ansi-regex':'6.0.1'}))),
 case('root devDependency added, never locked',lambda d:d['.'].update(devDependencies={'ansi-regex':'6.0.1'})),
 case('external dep spec to git/tarball-free alias npm:',edit('pkgs/a',lambda x:x['dependencies'].update({'ms':'npm:ms@2.1.3'}))),
]
results=[]
for name,mut,pre in cases:
    if MODE=='ws' and name not in WS_CASES:continue
    root=os.path.join(scratch,'case');shutil.rmtree(root,ignore_errors=True)
    docs=json.loads(json.dumps(BASE_WS if MODE=='ws' else BASE))
    if pre:pre(docs)
    for rel,doc in docs.items():w(root,'' if rel=='.' else rel,doc)
    cache=os.path.join(scratch,'.cache')
    r=subprocess.run([B,'install','--lockfile-only','--ignore-scripts','--cache-dir',cache],cwd=root,capture_output=True,text=True)
    assert r.returncode==0,r.stderr
    lock=open(os.path.join(root,'bun.lock'),encoding='utf8').read()
    mut(docs)
    for rel,doc in docs.items():w(root,'' if rel=='.' else rel,doc)
    verdicts={}
    for flag in ('--dry-run',None):
        cmd=[B,'install','--frozen-lockfile','--ignore-scripts','--cache-dir',cache]+([flag] if flag else [])
        r=subprocess.run(cmd,cwd=root,capture_output=True,text=True)
        verdicts[flag or 'real']=r.returncode
    results.append({'mode':MODE,'case':name,'frozenDryRun':verdicts['--dry-run'],'frozenReal':verdicts['real'],'lock':lock,'manifests':docs})
    print(f"{name:55s} dry={verdicts['--dry-run']} real={verdicts['real']}",flush=True)
if out:json.dump(results,open(out,'w'),ensure_ascii=False,indent=1)
