import sys; sys.path.insert(0,'.')
from rw7 import *
import hashlib, json
FIELD_TYPE={
 'app':'A','constructor':'ConstructorState<A>','stores':'InstanceStores<A>','roster':'PeerRoster<A>','ingress':'ActionIngress','interaction':'LocalInteraction<A>',
 'history':'CommandHistory<A>','host_events':'HostEvents','txn':'Transactions<A>','child_composition':'ChildComposition<M>','pending':'PendingReserved<A>',
 'tools':'ToolOperations<A>','media':'MediaTransfers<A>','ingest':'EnvelopeJobs<A, M>','maintenance':'MaintenanceCursors','close':'CloseCursors<A, M>',
 'tool_runs':'ToolRunLedger<A>','time_travel':'TimeTravelLedger<A>','tool_machines':'ToolMachineRuntime<A::Snapshot, A::Mutation, A::Config, A::ConfigMutation>'}
BOUNDS="A: ArtifactApp, M: SpaceMember + MemberFactory + 'static"
WIDEN={'dispatch_framework_reserved_action','admit_framework_reserved_spawn','dispatch_catalogue_example','start_typed_command_operation'}
def post_calls(text, crate):
    """explicit extension answers for the tier-1 methods that used to call the hooks themselves"""
    if crate=='app': return text
    text=text.replace('self.stores.advance_mounted_window_config_refresh(','self.advance_mounted_window_config_refresh(')
    M=mask(text); out=[]; i=0
    rx=re.compile(r'\.\s*(validate_framework_reserved_commit|document_loading_refusal)\s*\(')
    while True:
        m=rx.search(M,i)
        if not m: out.append(text[i:]); break
        out.append(text[i:m.end()])
        op=m.end()-1; cp=match_close(M,op,'(',')')
        args=[text[a:b].strip() for a,b in split_args(M,text,op+1,cp)]
        if m.group(1)=='validate_framework_reserved_commit':
            assert len(args)==2,args
            out.append('X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, %s), %s, %s'%(args[0],args[0],args[1]))
        else:
            assert len(args)==2,args
            out.append('%s, %s, X::admitted_while_loading(%s)'%(args[0],args[1],args[1]))
        out.append(')'); i=cp+1
    return ''.join(out)
class World:
    pass
def build():
    W=World()
    W.ms=methods()
    W.T=Table(W.ms)
    for m in W.ms:
        if m['recv'] is None and m['crate'] not in('app','composer-api'):
            g=[]
            if re.search(r'\bA\b',m['text']): g.append('A')
            if re.search(r'\bM\b',m['text']): g.append('M')
            W.T.free[m['name']]=('::<%s>'%', '.join(g)) if g else ''
    W.flags=[]
    W.conv={}
    for i,m in enumerate(W.ms):
        if m['crate']=='composer-api': continue
        cview=(m['recv']=='ref' and m['crate'] in VIEW)
        m['view']=cview
        txt=m['text'] if m['crate']=='app' else whole_borrows(m['text'])
        m['new']=post_calls(rewrite(txt,dict(crate=m['crate'],view=cview,name=m['name']),W.T,W.flags),m['crate'])
        if m['vis']=='' and m['name'] in WIDEN and m['recv'] is not None:
            n=m['new']; mm=re.search(r'(?m)^(?P<ind>[ \t]*)(?P<kw>(?:async\s+)?fn\s+'+re.escape(m['name'])+r'\b)',n)
            assert mm,m['name']
            m['new']=n[:mm.start('kw')]+'pub(super) '+n[mm.start('kw'):]
        if m['name']=='dispatch_typed' and m['crate']=='dispatch':
            n=m['new']
            for a,b in [("pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut","pub fn dispatch_typed<'b, 'context>(mut self, command: A::Command, meta: &'ctx ActionMeta, identity: &'ctx mut"),("cx:&'a mut semio_framework_job::StepContext<'context>","cx:&'ctx mut semio_framework_job::StepContext<'context>"),("+ 'a + use<'a, 'b, 'context, A, M>","+ 'ctx + use<'ctx, 'b, 'context, A, M, X>")]:
                assert n.count(a)==1,a
                n=n.replace(a,b)
            m['new']=n
        m['id']=i
    return W
def used_bridges(m):
    t=mask(m['new'])
    SNS='render|dispatch|typed|time_travel|tool_run|command|ladder|extension'
    mut=set(re.findall(r'self\.as_(%s)\(\)'%SNS,t))
    view=set(re.findall(r'self\.(%s)_ref\(\)'%SNS,t))
    return mut,view
def direct_fields(m):
    t=mask(m['new'])
    used=set(re.findall(r'(?<![\w.])self\s*\.\s*(\w+)',t))&set(FIELDS)
    return used
def field_sets(W):
    SN2C={v:k for k,v in SNAKE.items()}
    ctxf={}; viewf={}; bridges={}
    ext_c=set(); ext_v=set()
    order=['render','dispatch','typed-ops','tt-turns','tr-turns','commands','ladders']
    byc=collections.defaultdict(list)
    for m in W.ms:
        if m['crate'] in order: byc[m['crate']].append(m)
    # ExtensionCtx needs tt and tr first: compute those two before commands
    for c in order:
        cf=set(); vf=set(); mutb=set(); viewb_ctx=set(); viewb_view=set()
        for m in byc[c]:
            d=direct_fields(m); mb,vb=used_bridges(m)
            cf|=d
            if m['view']: vf|=d
            for s in mb:
                if s=='extension': cf|=ext_c; mutb.add('extension')
                else: cf|=ctxf[SN2C[s]]; mutb.add(s)
            for s in vb:
                if s=='extension': cf|=ext_v; (viewb_view if m['view'] else viewb_ctx).add('extension'); 
                else:
                    cf|=viewf[SN2C[s]]; (viewb_view if m['view'] else viewb_ctx).add(s)
                if m['view']:
                    vf|= (ext_v if s=='extension' else viewf[SN2C[s]])
        cf|=vf
        ctxf[c]=cf; viewf[c]=vf; bridges[c]=dict(mut=mutb,ctx_view=viewb_ctx,view_view=viewb_view)
        if c=='tt-turns' or c=='tr-turns':
            ext_c|=cf; ext_v|=vf
    W.ctxf,W.viewf,W.bridges,W.ext_c,W.ext_v=ctxf,viewf,bridges,ext_c,ext_v
    return W
if __name__=='__main__':
    W=build(); print(len(W.flags)); [print(f) for f in W.flags[:30]]
    field_sets(W)
    for c in ['render','dispatch','typed-ops','tt-turns','tr-turns','commands','ladders']:
        print(c,'CTX',sorted(W.ctxf[c],key=FIELDS.index)); print('   VIEW',sorted(W.viewf[c],key=FIELDS.index)); print('   bridges',{k:sorted(v) for k,v in W.bridges[c].items()})
    print('EXT',sorted(W.ext_c,key=FIELDS.index)); print('EXTV',sorted(W.ext_v,key=FIELDS.index))
