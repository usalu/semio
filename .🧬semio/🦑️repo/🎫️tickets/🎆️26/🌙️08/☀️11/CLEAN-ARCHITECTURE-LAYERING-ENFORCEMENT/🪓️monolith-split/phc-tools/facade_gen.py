#!/usr/bin/env python3
"""🪟️ Facade of semio-framework-plugin-host: explicit `pub use` items for the API the repository's consumers name, resolved against the carved crates."""
import os, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, plan_host
from carve import REPO

CONSUMERS = os.path.join(os.path.dirname(os.path.abspath(__file__)), '../🗑️generated/phc-consumers.json')

# names exposed by the facade's own component module (session, metrics, activation, package assets)
OWN = {'activation', 'package_assets', 'ArtifactSession', 'SessionLanePack', 'RuntimeMetricsPublisher', 'decode_dispatch_report', 'decode_merge_report', 'decode_conflicts'}
EXTERNAL = {'PackageHash': 'semio_framework_actor', 'PackageId': 'semio_framework_actor', 'original_actor_context': 'semio_framework_plugin_guest_contract'}
SHARD_MODULE = 'shard'
STALE = {'Budget'}
# extra names the bin and the integration test of the facade package use
EXTRA_TOP = ['GuestActorOpening', 'GuestInstance', 'CompiledHandle', 'PackageRef', 'GuestRuntime', 'GuestRuntimes']


def public_names(model, ident):
    root = model.roots[ident]
    names = set(n for n, its in root.items.items() if any(it.vis.startswith('pub') for it in its))
    names |= {u.binding for u in root.uses if not u.glob and u.vis.startswith('pub')}
    return names


def consumer_heads():
    d = json.load(open(CONSUMERS, encoding='utf-8'))
    heads = collections.OrderedDict()
    for full in sorted(d['full']):
        segs = full.split('::')
        heads.setdefault(segs[0], set()).add('::'.join(segs[1:]))
    return heads


def owners(model):
    own = collections.defaultdict(list)
    for ident in plan_host.CRATES:
        if ident == plan_host.FACADE:
            continue
        for n in public_names(model, ident):
            own[n].append(ident)
    return own


def generate(model, shard_members):
    heads = consumer_heads()
    own = owners(model)
    lines = []
    bycrate = collections.defaultdict(list)
    problems = []
    names = sorted((set(heads) | set(EXTRA_TOP)) - {SHARD_MODULE} - STALE)
    for n in names:
        if n in OWN:
            continue
        if n in EXTERNAL:
            bycrate[EXTERNAL[n]].append(n)
            continue
        cands = own.get(n, [])
        if len(cands) != 1:
            problems.append((n, cands))
            continue
        bycrate[cands[0]].append(n)
    return bycrate, problems


SHARD_ITEMS = ['OriginalShardIdentity', 'OriginalShardIdentityIssuer', 'ShardFrame', 'ShardIdentityPolicy', 'ShardOutcome', 'grant', 'native_identity_issuer', 'to_actor_turn_result']
EXECUTOR_ITEMS = ['OutcomeSink', 'ShardExecutor']

HEADER = '''//! 🖥️ Plugin host facade: the sandboxed Wasmtime component plugin host assembled from its producer crates, with the session, metrics, activation and package-asset glue of the host itself.
'''


def crate_ident_path(ident):
    return ident


def generate_text(C, current):
    """Full text of the facade lib root: kept header lines of the original root, then the explicit re-exports."""
    bycrate, problems = generate(C.new, None)
    assert not problems, problems
    out = [HEADER.rstrip('\n')]
    keep = []
    for ln in current.split('\n'):
        if ln.startswith('extern crate ') or ln.startswith('#![') or ln.startswith('//! '):
            if ln.startswith('//! '):
                continue
            keep.append(ln)
    out += keep
    out.append('')
    out.append('#[path = "../../🦀️.rs"]')
    out.append('mod component;')
    out.append('pub use component::{ArtifactSession, RuntimeMetricsPublisher, SessionLanePack, decode_conflicts, decode_dispatch_report, decode_merge_report};')
    out.append('#[cfg(not(target_arch = "wasm32"))]')
    out.append('pub use component::{activation, package_assets};')
    for c in sorted(bycrate, key=lambda c: (c.startswith('semio_framework_actor') is False, c)):
        names = sorted(bycrate[c])
        out.append(f'pub use {c}::{names[0]};' if len(names) == 1 else f'pub use {c}::{{{", ".join(names)}}};')
    out.append('')
    out.append('pub mod shard {')
    out.append(f'    pub use semio_framework_plugin_host_shard::{{{", ".join(sorted(SHARD_ITEMS))}}};')
    out.append('    pub mod executor {')
    out.append(f'        pub use semio_framework_plugin_host_shard_executor::{{{", ".join(sorted(EXECUTOR_ITEMS))}}};')
    out.append('    }')
    out.append('}')
    out.append('')
    out.append('/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the one world\'s contract-parity')
    out.append('/// test (effect ↔ host-async import parity, plus the collapsed shape itself) — mounted here rather than inside `🦀️.rs` (other packets are live')
    out.append('/// in that file). The test uses a narrow owned WIT source inspector and adds no external parser to')
    out.append('/// the test graph.')
    out.append('#[cfg(test)]')
    out.append('#[path = "../../🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs"]')
    out.append('mod schema_parity;')
    return '\n'.join(out) + '\n'
