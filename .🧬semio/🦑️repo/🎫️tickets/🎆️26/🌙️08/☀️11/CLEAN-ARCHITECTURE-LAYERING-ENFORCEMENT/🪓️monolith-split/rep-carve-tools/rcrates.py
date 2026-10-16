"""📇 Registry of the carved replication crates (node = taxonomy folder, package = `<node>/📦️packages/🦀️rust`, root = lib root file)."""
from rlib import *

#  key, cargo suffix, node under R, "single" (lib root is <node>/🦀️.rs) or "glue" (lib root is <package>/🦀️.rs), level, description
CRATES = [
    ("ids", "ids", "🆔️ids", "single", 0, "Replication identifiers: mutation, actor, artifact, schema ids, payload hash and the hybrid logical timestamp."),
    ("codec", "codec", "⚙️codec", "glue", 0, "Pack-level byte codec primitives under the replication contract: byte spans and ranges, chunk and codec ids, history read limits, owned operation bytes and the OpBinary and OpText trait family."),
    ("interaction", "interaction", "📡️wire/🕹️interaction", "single", 0, "Interaction vocabulary: hover and selection specs, domain topology, the selection and hover machines and state validation."),
    ("history-index", "history-index", "🔗️causal/🔀️transition/🔁️fold/🗂️index", "single", 0, "Ordered index and set of the history fold with their insertion cursors."),
    ("command-ingress", "command-ingress", "📡️wire/🎮️command/📥️ingress", "single", 0, "Paged command ingress: fixed command pages, command envelopes, batches and the resumable batch driver."),
    ("wire-core", "wire-core", "🧾️wire", "glue", 1, "Wire primitives: protocol limits, record kinds and flags, merge and undo policies, varint and string codecs, scalars, dictionaries and retained wire fields."),
    ("local-interaction", "local-interaction", "📡️wire/🏠️local-interaction", "single", 1, "Local interaction roots, updates and transport pages."),
    ("format", "format", "📐️format", "glue", 2, "The .spr record file format: header, frames, commit chain, writer, recovery, retained verification and slice helpers."),
    ("mutation", "mutation", "🎮️mutation", "single", 2, "The mutation contract: Mutation trait family, leaf descriptors, outcome vocabulary, keyed map algebra and foreign targets."),
    ("presence", "presence", "📡️wire/👥️presence", "single", 2, "Peer presence: window views, typing, tool runs, history edits, the borrowed presence reader, interaction roster and peer overlay."),
    ("edit", "edit", "🎮️mutation/✏️edit", "single", 3, "The recorded Edit: transaction reference, mutation meta and origin, canonical field cursors, transaction admission and the canonical edit seal."),
    ("causal", "causal", "🔗️causal", "glue", 4, "Causal envelopes: MutationEnvelope, ArtifactDiff, InverseMutation, frontiers, the mutation dag and their binary and text codecs."),
    ("conflict", "conflict", "⚔️conflict", "single", 5, "Conflict vocabulary and replay reports."),
    ("history", "history", "🔗️causal/🔀️transition", "single", 5, "History transitions: fold, drafts, checkpoints, alternatives, supersession and the transition codec."),
    ("frames", "frames", "📡️wire", "single", 5, "Lane-tagged client and server frames, artifact bootstrap and the frame codec."),
    ("mutation-messages", "mutation-messages", "🎮️mutation/📨️messages", "glue", 6, "Mutation message ledgers: compose, retire, accumulate, clamp and copy cursors, replay retirement and prepared replay."),
]
BY_KEY = {c[0]: c for c in CRATES}
KEYS = [c[0] for c in CRATES]

def cname(key): return "semio-framework-replication-" + BY_KEY[key][1]
def ident(key): return cname(key).replace("-", "_")
def short_id(key): return "replication-" + BY_KEY[key][1]
def node(key): return R + BY_KEY[key][2]
def pkgdir(key): return node(key) + "/" + PACKAGE
def libroot(key): return (pkgdir(key) if BY_KEY[key][3] == "glue" else node(key)) + "/🦀️.rs"
def description(key): return BY_KEY[key][5]
def project_name(key): return "@semio-tech/" + short_id(key) + "-rs"
def level(key): return BY_KEY[key][4]
