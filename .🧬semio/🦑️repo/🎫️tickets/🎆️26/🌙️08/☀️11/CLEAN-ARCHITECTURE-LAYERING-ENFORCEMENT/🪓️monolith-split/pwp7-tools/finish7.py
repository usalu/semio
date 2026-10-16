import sys; sys.path.insert(0,'.')
from plan_all7 import *
from feat7 import *
PARENT='🧵️instance/🦀️.rs'
REMOVE_MODS=['commands_lease','commands_dispatch','commands_ingress','commands_group','typed_ops_tool_machine']
def patch_parent(text, mounts, exports, extra_exports):
    for mod in REMOVE_MODS:
        pat=re.compile(r'(?:#\[cfg[^\n]*\]\n)?#\[path = "[^"]*"\]\nmod '+mod+r';\n\n?')
        assert len(pat.findall(text))==1,(mod,len(pat.findall(text)))
        text=pat.sub('',text)
    # new mounts after `mod extensions;`
    anchor='#[path = "🧩️extensions/🦀️.rs"]\nmod extensions;\n'
    assert text.count(anchor)==1
    blocks=[]
    for name,path,cfg in mounts:
        rel=path[len('🧵️instance/'):] if path.startswith('🧵️instance/') else '../'+path
        blocks.append('#[path = "%s"]\nmod %s;\n'%(rel,name))
    text=text.replace(anchor,anchor+'\n'.join(blocks))
    old='pub(crate) use extensions::InstanceExtension;'
    assert text.count(old)==1
    text=text.replace(old,'pub(crate) use extensions::{ExtensionCtx, ExtensionRef, InstanceExtension};')
    lines=[]
    for mod,names in exports.items(): lines.append('pub(crate) use %s::{%s};'%(mod,', '.join(names)))
    for mod,names in extra_exports: lines.append('pub(crate) use %s::{%s};'%(mod,', '.join(names)))
    end='//#endregion 📥️StagedInstanceMounts'
    assert text.count(end)==1
    text=text.replace(end,'\n'.join(lines)+'\n'+end)
    return text
def add_line(text, line):
    M=text.split('\n'); last=0
    for i,l in enumerate(M):
        if l.startswith('use ') or l.startswith('#[cfg(test)] use '): last=i
        if l.startswith('impl') or l.startswith('///') or l.startswith('pub(crate) struct'): break
    M.insert(last+1,line)
    return '\n'.join(M)
def used_in_header(text,name):
    hdr=text.split('\nimpl',1)[0]
    return re.search(r'(?<![\w])'+name+r'(?![\w])',hdr) is not None
def add_imports(text, names):
    need=[n for n in names if re.search(r'(?<![\w])'+n+r'(?![\w])',text) and not used_in_header(text,n)]
    if not need: return text
    line='use crate::artifact_instance::{%s};\n'%', '.join(need)
    # after the last top-level `use` statement of the header
    M=text.split('\n')
    last=0
    for i,l in enumerate(M):
        if l.startswith('use ') or l.startswith('#[cfg(test)] use '): last=i
        if l.startswith('pub(crate) struct') or l.startswith('impl') or l.startswith('///'): break
    M.insert(last+1,line.rstrip('\n'))
    return '\n'.join(M)
BUNDLE_NAMES=['ExtensionCtx','ExtensionRef','MediaTransfers','RenderCtx','RenderRef','TimeTravelCtx','TimeTravelRef','ToolRunCtx','ToolRunRef','InstanceExtension','DispatchCtx','TypedCtx','CommandCtx','LadderCtx','LadderRef']
def pre_text(rel):
    p=SNAP_DIR+'/'+rel
    return open(p if os.path.exists(p) else PLUG+rel,encoding='utf-8').read()
SNAP_DIR=GEN+'/pwp7-snap'
def finish():
    W=build(); field_sets(W)
    writes,deletes,mounts,rex=plan(W)
    # feature turn files
    tt='⏪️time-travel/🧵️instance/🦀️.rs'; tr='⏯️tool-run/🧵️instance/🦀️.rs'
    writes[tt]=add_imports(patch_tt(writes[tt]),BUNDLE_NAMES)
    writes[tr]=add_imports(patch_tr(writes[tr]),BUNDLE_NAMES)
    writes[tt]=add_line(writes[tt],'use crate::tool_jobs::ToolRunLedger;')
    writes[tr]=add_line(writes[tr],'use crate::tool_jobs::TimeTravelLedger;')
    # extension module
    ex_rel='🧵️instance/🧩️extensions/🦀️.rs'
    old=pre_text(ex_rel)
    tests_mod=old[old.index('#[cfg(test)]'):]
    writes[ex_rel]=extension_file(W,next_emoji(ex_rel),tests_mod)
    # parent
    ptxt=pre_text(PARENT)
    extra=[('render_history_view',['history_command'])]
    writes[PARENT]=patch_parent(ptxt,mounts,rex,extra)
    tier1_patches(writes)
    pubrel='🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🦀️.rs'
    writes[pubrel]=writes[pubrel].rstrip('\n')+'\n'+WINDOW_WRAPPER
    brel='🧵️instance/📥️staging/🏗️ctx-builders/🦀️.rs'
    bt=writes[brel].rstrip('\n')+'\n\n'+delegations(W)+'\n'
    bt=bt.replace('use std::marker::PhantomData;\n','use std::marker::PhantomData;\nuse crate::tool_jobs::time_travel::TimeTravelActionOutcome;\nuse crate::tool_jobs::tool_run::ToolRunActionOutcome;\n',1)
    writes[brel]=bt
    return W,writes,deletes

def tier1_patches(writes):
    # stores ingress: validate_framework_reserved_commit takes the extension's answer
    rel='🧵️instance/🏪️stores/📥️staging/🚪️ingress/🦀️.rs'
    t=pre_text(rel)
    t=sub1(t,'pub(crate) async fn validate_framework_reserved_commit(&self, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {\n        if let Some(fault) = InstanceExtensions::reserved_action_refusal(self, verb) {',
            'pub(crate) async fn validate_framework_reserved_commit(&self, refusal: Option<Fault>, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {\n        if let Some(fault) = refusal {')
    writes[rel]=t
    # stores typed operations: split at the hook
    rel='🧵️instance/🏪️stores/📥️staging/🧾️typed-operations/🦀️.rs'
    t=pre_text(rel)
    old_start=t.index('    pub(crate) fn advance_mounted_window_config_refresh(')
    M=mask(t); ob=M.index('{',old_start); d=0
    for j in range(ob,len(M)):
        if M[j]=='{': d+=1
        elif M[j]=='}':
            d-=1
            if d==0: break
    body=t[old_start:j+1]
    nb=body.replace('pub(crate) fn advance_mounted_window_config_refresh(','pub(crate) fn advance_window_config_refresh(')
    nb=sub1(nb,'-> Result<(), Fault> {\n        let receipt = mounted.pending_window_config_receipt.ok_or_else(','-> Result<bool, Fault> {\n        mounted.pending_window_config_receipt.ok_or_else(')
    nb=sub1(nb,'.ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;\n        let authority','.ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;\n        let authority')
    nb=sub1(nb,'{ return Ok(()); }','{ return Ok(false); }')
    tail='''        mounted.pending_window_config_receipt = None;
        InstanceExtensions::window_config_published(self);
        let token = mounted.next_token();
        mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)
    }'''
    assert tail in nb, nb[-500:]
    nb=nb.replace(tail,'''        Ok(true)
    }

    /// 🪟️ Queues the retained window-config receipt's result page once the refresh is complete and the extensions have been told.
    pub(crate) fn queue_window_config_receipt(mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<(), Fault> {
        let receipt = mounted.pending_window_config_receipt.take().ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;
        let token = mounted.next_token();
        mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)
    }''')
    t=t[:old_start]+nb+t[j+1:]
    t=sub1(t,'until the displaced snapshot has admitted custody, then queues the retained receipt\'s result page; every unit','until the displaced snapshot has admitted custody; `true` once it is complete, when the use case tells the extensions and queues the receipt\'s result page; every unit')
    writes[rel]=t
    # docs: document_loading_refusal
    rel='🧵️instance/📄️docs/📥️staging/🧷️command-routes/🦀️.rs'
    t=pre_text(rel)
    t=sub1(t,'pub(crate) fn document_loading_refusal(&self, tools: &ToolOperations<A>, verb: &str) -> Option<Fault> {','pub(crate) fn document_loading_refusal(&self, tools: &ToolOperations<A>, verb: &str, admitted_by_extension: bool) -> Option<Fault> {')
    t=sub1(t,'|| InstanceExtensions::admitted_while_loading(verb);','|| admitted_by_extension;')
    writes[rel]=t
WINDOW_WRAPPER='''
impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🪟️ Continues the window-config authority refresh of a published receipt; once it is complete the extensions are told
    /// before the retained receipt's result page is queued.
    pub(crate) fn advance_mounted_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant) -> Result<(), Fault> {
        if self.stores.advance_window_config_refresh(mounted, retained)? {
            X::window_config_published(&mut *self.time_travel, &mut *self.tool_runs);
            InstanceStores::<A>::queue_window_config_receipt(mounted)?;
        }
        Ok(())
    }
}
'''
def delegations(W):
    out=["impl<%s> VcsArtifactApp<A, M> {"%BOUNDS]
    first=True
    for m in W.ms:
        if m['crate'] in ('app','composer-api') and m['name']!='time_travel_ledger': continue
        if m['vis']!='pub' : continue
        if m['name']=='time_travel_ledger':
            sg=m['sig']
            head=m['text'][:sg['body_open']].rstrip()
            out.append(head+' {\n        &self.time_travel\n    }\n'); continue
        sg=m['sig']
        head=m['text'][:sg['body_open']].rstrip()
        names=[]
        for a,b in sg['params'][1:] if sg['recv'] else sg['params']:
            p=m['text'][a:b].strip()
            mm=re.match(r'(?:mut\s+)?(\w+)\s*:',p); assert mm,p
            names.append(mm.group(1))
        snake=SNAKE[m['crate']]
        recv='%s_ref'%snake if m['recv']=='ref' else '%s_ctx'%snake
        awaiting='.await' if m['is_async'] else ''
        out.append('%s {\n        self.%s().%s(%s)%s\n    }\n'%(head,recv,m['name'],', '.join(names),awaiting))
    out.append('}')
    return '\n'.join(out)
