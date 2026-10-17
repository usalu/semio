import re,sys,os
E='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
src=open(E+'/🦀️.rs',encoding='utf8').read().split('\n')
def idx(prefix,start=0):
    for i in range(start,len(src)):
        if src[i].startswith(prefix): return i
    raise SystemExit('missing '+prefix)
def region(name):
    s=idx('//#region '+name); e=idx('//#endregion '+name,s); return s,e
cs=idx('/// 👁️✏️ C2 §2.1'); cc=idx('pub const NOTE_PLAY_CONTROLLER_ID')
consts=src[cs:cc+1]
rs,re_=region('🔖️ResetDocument'); reset=src[rs+1:re_]
us,ue=region('🔖️Utilities'); util=src[us+1:ue]
is_,ie=region('🔖️Interaction'); inter=src[is_+1:ie]
ci=idx('/// 🕹️ Per-dispatch scratch'); cj=idx('}',ci)
ctx=src[ci:cj+1]
ti=idx('/// 🌳️ `blocks` domain topology'); topo=src[ti:ie]
icons=src[is_+1:ci]
ks,ke=region('🔖️Commands'); cmds=src[ks+1:ke]
bs,be=region('🔖️ActionBridge'); bridge=src[bs+1:be]
ul=idx('/// 🏷️ Admits Note display text'); ulend=idx('}',ul); uil=src[ul:ulend+1]
def pub(lines):
    out=[]
    for l in lines:
        l=re.sub(r'^fn (note_internal_action|note_utility|note_blocks_topology)\b',r'pub fn \1',l)
        out.append(l)
    return out
sup=['//! 🧰️ Note editor support — the action factory, UI admission, interaction domain constants and topology every taxonomy node shares.','',
'use crate::{NoteBlockNode, NoteSnapshot, NOTE_DOCUMENT_SCHEMA};',
'use semio_framework_plugin::{ActionDefinition, ActionDescriptor, ActionKind, DomainTopology, DslValue, TopologyNode, UtilityCategory, UtilityDefinition};',
'use semio_framework_ui_locale::LocalizedLabel;','']
sup+=consts+['']+pub(reset)+['']+pub(util)+['']+icons+pub(topo)+['']+uil+['']
open(E+'/🧰️support.tmp','w',encoding='utf8').write('\n'.join(sup))
dc=['//! 🎯️ Note dispatch context — the per-dispatch scratch every leaf command handler receives.','','use crate::editor::note::window::NoteCompositeWindowTransient;','']
dc+=[l.replace('crate::editor::note::window::NoteCompositeWindowTransient','NoteCompositeWindowTransient') for l in ctx]+['']
open(E+'/🎯️dispatch-ctx.tmp','w',encoding='utf8').write('\n'.join(dc))
use_cmd=[l for l in src[:idx('//#region 🔖️Constants')] if l.startswith('use crate::editor::note::commands')]
ce=['//! 🎼️ Note command enum — the closed dispatch surface of the editor and the host-action bridge into it.','',
*use_cmd,'use crate::editor::note::dispatch_ctx::NoteDispatchCtx;','use crate::op::NoteMutation;','use crate::NoteSnapshot;','use semio_framework_plugin::{NoConfig, NoConfigMutation};','']
ce+=cmds+['']+[l.replace('mod args_bridge {','pub mod args_bridge {',1) for l in bridge]+['']
open(E+'/🎼️command-enum.tmp','w',encoding='utf8').write('\n'.join(ce))
# root without moved parts
drop=[]
def mark(a,b): drop.append((a,b))
mark(cs,cc+1)
mark(rs,re_+2); mark(us,ue+2); mark(is_,ie+2); mark(ks,ke+2); mark(bs,be+2)
mark(ul-1 if src[ul-1].strip()=='' else ul,ulend+1)
keep=[l for i,l in enumerate(src) if not any(a<=i<b for a,b in drop)]
open(E+'/🦀️.rs.new','w',encoding='utf8').write('\n'.join(keep))
print(len(src),len(keep))
