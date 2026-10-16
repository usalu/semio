import re,os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
TOOLS=os.path.dirname(os.path.abspath(__file__))
WS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/"
SRC=WS+"🦀️.rs"
ORIG=TOOLS+"/orig-workspace.rs.txt"
CHILDREN=[ # id, folder, mod, emoji, doc
 ("C01","🧭️plugin-paths-and-descriptor","plugin_paths_and_descriptor","🧭️","Plugin registry rows, repo-root discovery, component paths and committed package descriptors."),
 ("C02","🔬️probe-document","probe_document","🔬️","The generic JSON-valued probe document: snapshot, diff, mutation, store and the codecs it mounts."),
 ("C03","🔖️plugin-binding-and-activation","plugin_binding_and_activation","🔖️","Workspace persistence origin and plugin activation outcome."),
 ("C04","🧊️compiled-components","compiled_components","🧊️","Process-wide compiled runtime, component caches, compile worker and compile flights."),
 ("C05","📨️plugin-artifact-channel","plugin_artifact_channel","📨️","The plugin artifact channel: pending exchanges, budgets and the channel implementation."),
 ("C06","🧩️artifact-app-frames","artifact_app_frames","🧩️","App-command and app-frame helpers plus the `ArtifactChannel` implementation of the plugin channel."),
 ("C07","🔀️routing-and-shell-routed","routing_and_shell_routed","🔀️","Route resolution, component sources, the routing channel, the shell-routed channel and the closed channel set."),
 ("C08","🗂️guest-document-codec","guest_document_codec","🗂️","Guest document codec routes: print, apply, replay, compile and sqlite snapshot bridges."),
 ("C09","🤝️hub-relay","hub_relay","🤝️","Installed artifact kinds, session document pairs and the hub relay with its inbound planning."),
 ("C10","🪪️headless-workspace-state","headless_workspace_state","🪪️","The headless workspace state, plugin artifact binding, drop and provenance helpers."),
 ("C11","🛂️gateway-backend","gateway_backend","🛂️","The headless workspace operations and its `GatewayBackend` implementations."),
 ("C12","💡️inference-facade","inference_facade","💡️","The authenticated hub inference facade and history undo port of the headless workspace."),
]
def load():
    t=open(ORIG,encoding="utf-8").read()
    L=t.split("\n")
    assert L[-1]==""
    L=[l+"\n" for l in L[:-1]]
    return L,lex2(L)
def uniq(L,pred,what):
    r=[i for i,l in enumerate(L) if pred(l)]
    assert len(r)==1,(what,len(r),r)
    return r[0]
def prelude(L,i):
    while i>0:
        p=L[i-1].strip()
        if p.startswith("//") and not p.startswith("//#endregion") and not p.startswith("// #endregion"):i-=1
        elif p.startswith("#["):i-=1
        else:break
    return i
def boundaries(L):
    st=lambda s:uniq(L,lambda l:l.strip()==s,s)
    sw=lambda s:uniq(L,lambda l:l.strip().startswith(s),s)
    def impl_followed(nxt_pred,what):
        r=[i for i in range(len(L)-1) if L[i].rstrip()=="impl HeadlessWorkspace {" and nxt_pred(L[i+1])]
        assert len(r)==1,(what,r);return r[0]
    b={}
    b["C01"]=st("//#region 🔖️PluginPaths")
    b["C02"]=st("//#region 🔖️ProbeDocument")
    b["C03"]=st("//#region 🔖️Binding")
    b["C05a"]=st("//#region 🔖️ArtifactChannel")
    b["C04"]=prelude(L,sw("fn shared_plugin_runtime("))
    b["C05b"]=prelude(L,sw("fn shared_compiled_component("))
    b["C06"]=prelude(L,sw("fn app_command_seq_mut("))
    b["C07a"]=st("//#region 🔖️Routing")
    b["C08"]=st("//#region 🗂️GuestDocumentCodec")
    b["C07b"]=prelude(L,uniq(L,lambda l:re.match(r"(pub(\([^)]*\))? )?enum PluginComponentSource\b",l.strip()) is not None,"PCS"))
    b["P1"]=st("//#endregion 🔖️ArtifactChannel")
    b["C10a"]=st("//#region 🔖️HeadlessWorkspace")
    b["C09"]=prelude(L,uniq(L,lambda l:re.match(r"(pub(\([^)]*\))? )?(struct|enum) InstalledArtifactKind\b",l.strip()) is not None,"IAK"))
    b["C10b"]=prelude(L,sw("pub struct PluginArtifactBinding"))
    b["C11"]=impl_followed(lambda n:n.startswith("    fn new(origin: WorkspaceOrigin"),"C11")
    b["C10c"]=impl_followed(lambda n:"Provenance of content read from" in n,"C10c")
    b["P2"]=st("//#endregion 🔖️HeadlessWorkspace")
    b["C12"]=st("//#region 💡️Inference")
    b["P3"]=uniq(L,lambda l:l.strip()=='#[path = "🧪️tests/🔬️quick/🦀️.rs"]',"quick")-1
    return b
def segments(L):
    b=boundaries(L)
    order=["C01","C02","C03","C05a","C04","C05b","C06","C07a","C08","C07b","P1","C10a","C09","C10b","C11","C10c","P2","C12","P3"]
    pos=[b[k] for k in order]
    assert pos==sorted(pos) and len(set(pos))==len(pos),list(zip(order,pos))
    segs=[("P0",0,b["C01"])]
    for k,s,e in zip(order,pos,pos[1:]+[len(L)]):
        segs.append((k,s,e))
    return segs
def owner_of(k):
    if k.startswith("P"):return "P"
    return k[:3]
