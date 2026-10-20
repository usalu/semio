"""🧪 Consumer repoint model (stage S5) driven by the census. Run with `cwd` = the generated folder."""
from sim import *
import json,collections
R=json.load(open('census.json'))
ABI_NAMES={'ValueError','ValueRefusalKind','DslValue','Number','JsonInteger','json_integer','dsl_value','ToValue','FromValue','DecodedValue','ValueEdit','ValueShape','edit_through_value','IntrinsicRetirement','NativeEncodeControl','NativeEncodeContinuation','native_encoding','NativeDecodeControl','NativeDecodeProgress','bytes','RetireOwned','RetirementDemand','retirement','artifact_retire_struct','artifact_retire_leaf','ErasedSnapshotRetirement','shared_retirement_allocation_bytes','shared_retirement_birth_bytes','admit_shared_retirement','admit_owned_retirement','owned_retirement_birth_bytes','advance_retirement_turn','RetirementTurnError','RetirementReceipt','shared_issuance','CanonicalJsonTree','ControlledValueHasher','DslValueSource','<crate>','<alias:value>','<alias:__value>','value','serde','serde_json','testing'}
RET_NAMES={'retained_clone','RetainedClone','RetainedCloneProgress','RetainedCloneGrant','RetainedCloneStep','RetainedCloneRef','RetainedCloneSource','RetainedOwnedProjection'}
FAC_NAMES={'FactoryPayloadRetirement','ArtifactOwnedValueRetirementFactory','FactoryAuthority','FactoryRetirement','SnapshotRetirementFactory','factory','close_factory_ticket','factory_ticket_demands','factory_arc_birth_bytes','factory_constructor_birth_bytes','FactoryOwnedRetirement','FactoryChildTickets','FactoryChildSlot','FactoryBoxedPublication','FactoryBoxedValue','FactoryRetirementTicket'}
TXT_NAMES={'paged','SharedUtf8','numeric_scratch','scratch_hash','paged_text'}
TYPE_NAMES={'ValueType','ValueKind'}
OTHER={'ordered':'ordered','retirement_queue':'retirement-queue','snapshot_projection':'snapshot-projection','OriginalAliasBatch':'original-aliases'}
DERIVE_GROUPS={'ToValue':'dcodec','FromValue':'dcodec','value_codec':'dcodec','CanonicalJsonTree':'dcanon','RetireOwned':'dretire','FactoryPayloadRetirement':'dretire','RetainedClone':'dretained','RetainedEquality':'dretained','SnapshotProjection':'dretained','NativeReceive':'dretained','owned_json_file':'dres'}
def consumer_plan(x):
    pn,pd=x['prod_names'],x['prod_derives']
    if not pn and not pd: return None
    deps=set();unk=[]
    for n in pn:
        if n in ABI_NAMES: deps.add('A')
        elif n in RET_NAMES: deps.add('RC')
        elif n in FAC_NAMES: deps.add('FA')
        elif n in TXT_NAMES: deps.add('TX')
        elif n in TYPE_NAMES: deps.add('TY')
        elif n in OTHER: deps.add(OTHER[n])
        else: unk.append(n)
    for g in pd:
        deps.add(DERIVE_GROUPS.get(g,'dretained'))
        if g in ('ToValue','FromValue','RetireOwned','CanonicalJsonTree','value_codec'): deps.add('A')
        if g=='RetainedClone': deps.add('RC')
        if g=='FactoryPayloadRetirement': deps.add('FA'); deps.add('A')
    return deps,unk
def S5(E):
    """consumers repointed from FACADE/derive to tiered crates"""
    E=clone(E)
    DV_=N['derive']
    newn={k:'NEW:'+k for k in ['dcodec','dcanon','dretire','dretained','dres','TX','TY']}
    for k in newn.values(): E[k]=set()
    E[newn['TX']]={N['persistent'],N['native-owned'],N['paged-list'],N['ordered']}
    E[newn['TY']]={N['native-owned'],N['intrinsic']}
    # ABI node = intrinsic; RC = retained-clone; FA = factory
    mapn={'A':N['intrinsic'],'RC':N['retained-clone'],'FA':N['factory'],**newn,
          'ordered':N['ordered'],'retirement-queue':N['retirement-queue'],'snapshot-projection':N['snapshot-projection'],'original-aliases':N['original-aliases']}
    unknown=collections.Counter()
    drop=0
    for d,x in R.items():
        E[d].discard(FACADE);E[d].discard(DV_)
        pl=consumer_plan(x)
        if pl is None: drop+=1;continue
        deps,unk=pl
        for u in unk: unknown[u]+=1
        for k in deps: E[d].add(mapn[k])
    # facade & old derive: no dependents now; family keeps as isolated (facade deps irrelevant)
    return E,unknown,drop
if __name__=='__main__':
    E4=S4(S3(S2(S1(base))))
    E5,unk,drop=S5(E4)
    print('unknown names',unk,'dropped consumers',drop)
    keys={'control-contract':N['control-contract'],'allocation-return':N['allocation-return'],'retirement':N['retirement'],'shared-issuance':N['shared-issuance'],'native-encoding':N['native-encoding'],'native-decoding':N['native-decoding'],'intrinsic(ABI)':N['intrinsic'],'clone-source':N['clone-source'],'retained-clone':N['retained-clone'],'paged-list':N['paged-list'],'ordered':N['ordered'],'persistent':N['persistent'],'native-owned':N['native-owned'],'factory':N['factory'],'edit':N['edit'],'registry':N['registry'],'schema':N['schema'],'retained-map':N['retained-map'],'derive-expansion':N['derive-expansion'],'dcodec':'NEW:dcodec','dcanon':'NEW:dcanon','dretire':'NEW:dretire','dretained':'NEW:dretained','dres':'NEW:dres','TX paged/utf8':'NEW:TX','TY type':'NEW:TY','FACADE':FACADE,'old derive':N['derive'],'hash':HASH,'schema-registry':SREG}
    stg=[('S0',base),('S1',S1(base)),('S3',S3(S2(S1(base)))),('S4',E4),('S5',E5)]
    print('%-18s'%'node',*['%7s'%s for s,_ in stg])
    res={}
    for lab,k in keys.items():
        row=[tr(E,[k])[k] for _,E in stg]
        print('%-18s'%lab,*['%7d'%v for v in row])
