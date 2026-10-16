# orphan repairs of batch b1: trait impls and retire macros whose type or trait now lives in a carved crate move into that crate
import ns
S = ns.S
MOVES = [
    dict(src=S + '/♻️retirement/🦀️.rs', match=r'artifact_retire_struct!\s*\(\s*super::(CursorRevisionRecord|EditDigestChains|CursorRevisionAccumulator)\b', dest='os-store-ledger', count=3),
    dict(src=S + '/♻️retirement/🦀️.rs', match=r'impl<S: Send \+ \'static> RetireOwned for super::ArtifactChild<S>', dest='os-store-child', count=1),
    dict(src=S + '/🦀️.rs', match=r'artifact_retire_struct!\s*\(\s*ArtifactEnvelopeMetadata\b', dest='os-store-read', count=1),
    dict(src=S + '/🦀️.rs', match=r'impl<P[^>]*> FoldProjection<P, Mutation> for Arc<P>', dest='os-store-ledger', count=1),
    dict(src=S + '/🦀️.rs', match=r'ReplayEdits<Mutation> for (HashMap<String, Edit<Mutation>>|ArtifactHistoryLedger<Edit<Mutation>>)', dest='os-store-replay', count=2),
]
