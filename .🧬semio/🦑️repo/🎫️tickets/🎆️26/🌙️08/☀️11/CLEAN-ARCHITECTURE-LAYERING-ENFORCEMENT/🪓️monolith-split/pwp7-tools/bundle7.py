import sys; sys.path.insert(0,'.')
from gen7 import *
SN2C={v:k for k,v in SNAKE.items()}
CTXNAME=dict(CTX); VIEWNAME=dict(VIEW)
CTXNAME['extension']='ExtensionCtx'; VIEWNAME['extension']='ExtensionRef'
def fset(W,c,view):
    if c=='extension': return W.ext_v if view else W.ext_c
    return (W.viewf if view else W.ctxf)[c]
def sorted_fields(s): return sorted(s,key=FIELDS.index)
def struct_text(name, lt, fields, view, doc):
    lines=['%s'%doc,'pub struct %s<%s, %s, X> {'%(name,lt,BOUNDS)]
    for f in sorted_fields(fields):
        if f=='app': lines.append("    pub app: &%s%s A,"%("'v" if view else "'ctx", '' if view else ' mut'))
        elif f=='mounted_policy': lines.append('    pub mounted_policy: crate::MountedOwnerPolicyV1,')
        else: lines.append("    pub %s: &%s%s %s,"%(f,"'v" if view else "'ctx",'' if view else ' mut',FIELD_TYPE[f]))
    lines.append('    pub extension: PhantomData<fn() -> X>,')
    lines.append('}')
    return '\n'.join(lines)
def build_expr(target_name, target_fields, src_is_view, to_view):
    """struct literal building target (view or ctx) from self (ctx or view)"""
    parts=[]
    for f in sorted_fields(target_fields):
        if f=='mounted_policy': parts.append('mounted_policy: self.mounted_policy')
        elif to_view:
            parts.append('%s: self.%s'%(f,f) if src_is_view else '%s: &*self.%s'%(f,f))
        else:
            assert not src_is_view
            parts.append('%s: &mut *self.%s'%(f,f))
    parts.append('extension: PhantomData')
    return '%s { %s }'%(target_name,', '.join(parts))
def bundle_file(W,c,emoji):
    name=CTX[c]; vname=VIEW.get(c)
    out=['//! %s️ Borrowed bundle of the `%s` use cases: `%s`%s (WP7 of the instance family).'%(emoji,{'tt-turns':'time-travel-turns','tr-turns':'tool-run-turns'}.get(c,'instance-'+c),name,(' and its shared twin `%s`'%vname) if vname else ''),
         '//!','//! 🧺️ The fields carry the names of the composer fields they borrow, so a method body written for the composer reads the same; `X` is the registered `InstanceExtension` list.',
         'use super::*;','use std::marker::PhantomData;','']
    out.append(struct_text(name,"'ctx",W.ctxf[c],False,"/// 🧺️ The exclusive borrows the `%s` use cases need; built by the composer (`VcsArtifactApp::%s_ctx`) or reborrowed by a higher bundle."%(c,SNAKE[c])))
    out.append('')
    br=W.bridges[c]
    impl=["impl<'ctx, %s, X> %s<'ctx, A, M, X> {"%(BOUNDS,name)]
    if vname:
        impl.append("    /// 🔍️ The shared twin over the same borrows, for the `&self` members.")
        impl.append("    pub fn shared(&self) -> %s<'_, A, M, X> {"%vname)
        impl.append("        %s"%build_expr(vname,W.viewf[c],False,True))
        impl.append("    }")
        impl.append('')
    for s in sorted(br['mut']):
        tgt=CTXNAME['extension' if s=='extension' else SN2C[s]]
        tf=fset(W,'extension' if s=='extension' else SN2C[s],False)
        impl.append("    /// ⬇️ Reborrows the `%s` bundle this bundle bridges to."%s)
        impl.append("    pub fn as_%s(&mut self) -> %s<'_, A, M, X> {"%(s,tgt))
        impl.append("        %s"%build_expr(tgt,tf,False,False))
        impl.append("    }")
        impl.append('')
    for s in sorted(br['ctx_view']):
        tgt=VIEWNAME['extension' if s=='extension' else SN2C[s]]
        tf=fset(W,'extension' if s=='extension' else SN2C[s],True)
        impl.append("    /// ⬇️ Shares the `%s` view of the same borrows."%s)
        impl.append("    pub fn %s_ref(&self) -> %s<'_, A, M, X> {"%(s,tgt))
        impl.append("        %s"%build_expr(tgt,tf,False,True))
        impl.append("    }")
        impl.append('')
    if c in ('tt-turns','tr-turns'):
        impl.append("    /// 🔌️ The bundle of this extension, reborrowed from the extension seam's bundle.")
        impl.append("    pub fn of_extension<'e>(ext: &'e mut ExtensionCtx<'_, A, M, X>) -> %s<'e, A, M, X> {"%name)
        parts=[]
        for f in sorted_fields(W.ctxf[c]):
            parts.append('mounted_policy: ext.mounted_policy' if f=='mounted_policy' else '%s: &mut *ext.%s'%(f,f))
        parts.append('extension: PhantomData')
        impl.append("        %s { %s }"%(name,', '.join(parts)))
        impl.append("    }")
        impl.append('')
    while impl and impl[-1]=='': impl.pop()
    impl.append('}')
    out+=impl
    if vname:
        out.append('')
        out.append(struct_text(vname,"'v",W.viewf[c],True,"/// 🔍️ The shared borrows the `%s` `&self` use cases need (`Copy`: every field is a shared reference)."%c))
        out.append('')
        out.append("impl<'v, %s, X> Clone for %s<'v, A, M, X> {\n    fn clone(&self) -> Self {\n        *self\n    }\n}\n"%(BOUNDS,vname))
        out.append("impl<'v, %s, X> Copy for %s<'v, A, M, X> {}\n"%(BOUNDS,vname))
        vi=["impl<'v, %s, X> %s<'v, A, M, X> {"%(BOUNDS,vname)]
        first=True
        for s in sorted(br['view_view']):
            tgt=VIEWNAME['extension' if s=='extension' else SN2C[s]]
            tf=fset(W,'extension' if s=='extension' else SN2C[s],True)
            vi.append("    /// ⬇️ Shares the `%s` view of the same borrows."%s)
            vi.append("    pub fn %s_ref(&self) -> %s<'_, A, M, X> {"%(s,tgt))
            vi.append("        %s"%build_expr(tgt,tf,True,True))
            vi.append("    }")
            vi.append('')
        if c in ('tt-turns','tr-turns'):
            vi.append("    /// 🔌️ The shared bundle of this extension, shared from the extension seam's view.")
            vi.append("    pub fn of_extension_ref<'e>(ext: &'e ExtensionRef<'_, A, M, X>) -> %s<'e, A, M, X> {"%vname)
            parts=['mounted_policy: ext.mounted_policy' if f=='mounted_policy' else '%s: ext.%s'%(f,f) for f in sorted_fields(W.viewf[c])]
            parts.append('extension: PhantomData')
            vi.append("        %s { %s }"%(vname,', '.join(parts)))
            vi.append("    }")
            vi.append('')
        while vi and vi[-1]=='': vi.pop()
        vi.append('}')
        if len(vi)>2: out+=vi
    return '\n'.join(out)+'\n'
def extension_structs(W):
    out=[]
    out.append(struct_text('ExtensionCtx',"'ctx",W.ext_c,False,"/// 🔌️ The exclusive borrows an extension's context hooks (`dispatch_action`, `drive_turn`, `render_body`, `after_app_render`) run over; the union of what the time-travel and tool-run bundles borrow."))
    out.append('')
    out.append(struct_text('ExtensionRef',"'v",W.ext_v,True,"/// 🔍️ The shared borrows an extension's `has_pending_work` reads."))
    out.append('')
    out.append("impl<'v, %s, X> Clone for ExtensionRef<'v, A, M, X> {\n    fn clone(&self) -> Self {\n        *self\n    }\n}\n"%BOUNDS)
    out.append("impl<'v, %s, X> Copy for ExtensionRef<'v, A, M, X> {}"%BOUNDS)
    return '\n'.join(out)
def builders_text(W,emoji):
    out=['//! %s️ The composer\'s bundle builders: one exclusive builder per use-case crate and one shared builder per crate that has `&self` members (WP7 of the instance family).'%emoji,
         '//!','//! 🏗️ Each builder borrows the composer fields its bundle names; the extension list is fixed here to [`InstanceExtensions`].',
         'use super::*;','use std::marker::PhantomData;','',
         'impl<%s> VcsArtifactApp<A, M> {'%BOUNDS]
    def lit(tgt,fields,to_view):
        parts=[]
        for f in sorted_fields(fields):
            if f=='mounted_policy': parts.append('mounted_policy: self.mounted_policy')
            elif to_view: parts.append('%s: &self.%s'%(f,f))
            else: parts.append('%s: &mut self.%s'%(f,f))
        parts.append('extension: PhantomData')
        return '%s { %s }'%(tgt,', '.join(parts))
    for c in ['render','dispatch','typed-ops','tt-turns','tr-turns','commands','ladders']:
        out.append("    #[doc(hidden)]")
        out.append("    pub fn %s_ctx(&mut self) -> %s<'_, A, M, InstanceExtensions> {"%(SNAKE[c],CTX[c]))
        out.append("        %s"%lit(CTX[c],W.ctxf[c],False)); out.append("    }"); out.append('')
        if c in VIEW:
            out.append("    #[doc(hidden)]")
            out.append("    pub fn %s_ref(&self) -> %s<'_, A, M, InstanceExtensions> {"%(SNAKE[c],VIEW[c]))
            out.append("        %s"%lit(VIEW[c],W.viewf[c],True)); out.append("    }"); out.append('')
    out.append("    pub(crate) fn extension_ctx(&mut self) -> ExtensionCtx<'_, A, M, InstanceExtensions> {")
    out.append("        %s"%lit('ExtensionCtx',W.ext_c,False)); out.append("    }"); out.append('')
    out.append("    pub(crate) fn extension_ref(&self) -> ExtensionRef<'_, A, M, InstanceExtensions> {")
    out.append("        %s"%lit('ExtensionRef',W.ext_v,True)); out.append("    }")
    out.append('}')
    return '\n'.join(out)+'\n'
if __name__=='__main__':
    W=build(); field_sets(W)
    print(bundle_file(W,'tr-turns','🐠'))
    print(extension_structs(W)[:1500])
