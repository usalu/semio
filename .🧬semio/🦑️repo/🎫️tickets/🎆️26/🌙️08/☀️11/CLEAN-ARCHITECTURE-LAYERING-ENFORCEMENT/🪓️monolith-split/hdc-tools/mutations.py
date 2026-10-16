"""Seeded-sensitivity runs of the stub-layer oracle: each mutation is applied to the current sandbox only."""
import subprocess, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
D = "🌎️hub/📇️directory/"
O = os.path.join(os.path.dirname(os.path.abspath(__file__)), "hdc_oracle.py")
MUT = [
    ("M1 get_role impl renamed (missing item)", "sqlite", D + "🪶️sqlite/🏘️spaces/🦀️.rs", "async fn get_role(", "async fn get_role_x("),
    ("M2 signature drift of list_spaces", "sqlite", D + "🪶️sqlite/🏘️spaces/🦀️.rs", "async fn list_spaces(&self, limit: i64", "async fn list_spaces(&self, limit: i32"),
    ("M3 SpaceDirectory import dropped from the backend root", "sqlite", D + "🪶️sqlite/🦀️.rs", "SpaceDirectory, ", ""),
    ("M4 dispatch head_seq renamed", "dispatch", D + "🔀️dispatch/🦀️.rs", "async fn head_seq(", "async fn head_seq_x("),
    ("M5 foreign method inside the DocumentDirectory impl", "neo4j", D + "🌐️neo4j/🏘️spaces/🦀️.rs", "impl DocumentDirectory for Neo4jDirectory {", "impl DocumentDirectory for Neo4jDirectory {\n    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> { Ok(None) }"),
    ("M6 postgres rebuild moved out of ProjectionRebuild (renamed)", "postgres", D + "🐘️postgres/📽️projections/🦀️.rs", "async fn rebuild_projections_controlled(", "async fn rebuild_projections_controlled_x("),
    ("M7 PrincipalAuthority bound dropped from wire", "directory", D + "🔤️wire/🦀️.rs", "capped_socket_session_binding<D: PrincipalAuthority>", "capped_socket_session_binding<D: UserDirectory>"),
    ("M8 backend-corpus bound too small", "directory", D + "🧪️tests/🔮️backend-corpus/🦀️.rs", "assert_share_scope_corpus_v1<D: EventLog + ShareTokenDirectory + ?Sized>", "assert_share_scope_corpus_v1<D: ShareTokenDirectory + ?Sized>"),
]
for label, target, rel, old, new in MUT:
    print("###", label, flush=True)
    r = subprocess.run([sys.executable, O, target, "lib", "--mutate=%s@@%s@@%s" % (rel, old, new)], capture_output=True, text=True)
    lines = [l[:300] for l in r.stdout.split("\n") if l.startswith("   +current") or l.startswith("== ") or l.startswith("RESULT") or "mutated" in l]
    print("\n".join(lines[:14]) if lines else r.stdout[-600:] + r.stderr[-600:], flush=True)
print("FINISHED")
