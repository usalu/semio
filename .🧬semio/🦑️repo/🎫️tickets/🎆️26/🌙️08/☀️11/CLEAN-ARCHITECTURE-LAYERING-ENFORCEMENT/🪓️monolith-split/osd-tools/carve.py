"""🪚 OSD carve of semio-framework-os-directory: asserted anchor edits (sources), new manifests and project files.
Run once from a tree that equals osd-tools/orig (every edit asserts its anchor count). Writes only inside the directory module."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
M = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
OLD_PKG = M + "/📦️packages/🦀️rust"

def rd(p):
    return open(p, encoding="utf-8").read()

def wr(p, t):
    assert os.path.realpath(p).startswith(os.path.realpath(M) + os.sep), p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(t)

def sub1(path, old, new, count=1):
    t = rd(path)
    assert t.count(old) == count, (path, old[:60], t.count(old))
    wr(path, t.replace(old, new))

def regex_all(path, rx, repl, expect):
    t = rd(path)
    n = len(re.findall(rx, t))
    assert n == expect, (path, rx, n)
    wr(path, re.sub(rx, repl, t))

VAL = r"\bcrate::(DslValue|ToValue|FromValue|ValueError)\b"
VREPL = r"semio_framework_value::\1"

# 1 browser-actor: the leaf lib root
regex_all(M + "/🧬️schema/🌐️browser-actor/🦀️.rs", VAL, VREPL, 8)

# 2 events child
regex_all(M + "/🧬️schema/📖️events-commands-and-admin/🦀️.rs", VAL, VREPL, 19)

# 3 views child: the one cross-crate widening
sub1(M + "/🧬️schema/🪟️views-and-document-open/🦀️.rs", "pub(crate) fn valid_document_open_hash(", "pub fn valid_document_open_hash(")

# 4 schema root
p = M + "/🧬️schema/🦀️.rs"
t = rd(p)
a = t.index('#[path = "🪪️session-authority-v1/🦀️.rs"]')
head, tail = t[:a], t[a:]
new_tail = '''#[path = "🪪️session-authority-v1/🦀️.rs"]
pub mod session_authority;
pub use session_authority::{DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, DirectorySessionAuthorityV1, DirectorySessionKindV1};

#[path = "🌱️space-artifact-creation-v1/🦀️.rs"]
pub mod space_artifact_creation;

#[path = "📇️document-index-v1/🦀️.rs"]
pub mod document_index;
pub use document_index::{DirectoryIndexedDocumentViewV1, DocumentIndexEntryV1};

pub use semio_framework_os_directory_browser_actor::{
    DOCUMENT_BROWSER_ACTOR_INTERFACES, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, DocumentBrowserActorByteLengthV1, DocumentBrowserActorErrorV1, DocumentBrowserActorSourceV1, DocumentExecutionTargetBrowserActorV1, DocumentOpenBrowserActorV1,
};

#[path = "📖️events-commands-and-admin/🦀️.rs"]
mod events_commands_and_admin;
pub use events_commands_and_admin::*;

#[path = "🪟️views-and-document-open/🦀️.rs"]
mod views_and_document_open;
pub use views_and_document_open::*;

#[path = "🛰️execution-target-artifacts-and-stream/🦀️.rs"]
mod execution_target_artifacts_and_stream;
pub use execution_target_artifacts_and_stream::*;

#[path = "../🚪️io/📝️text/🦀️.rs"]
pub mod text;
'''
assert 'mod execution_target_artifacts_and_stream;\npub use execution_target_artifacts_and_stream::*;' in tail
assert tail.rstrip().endswith("pub use execution_target_artifacts_and_stream::*;")
wr(p, head + new_tail)

# 5 text root
p = M + "/🚪️io/📝️text/🦀️.rs"
sub1(p, "use super::super::schema::*;", "use crate::*;")
regex_all(p, VAL, VREPL, 3)
t = rd(p)
cut = t.index('#[path = "📌️document-check-in-v1/🦀️.rs"]')
assert t[cut:].count("#[path") == 2
wr(p, t[:cut].rstrip("\n") + "\n")
sub1(M + "/🚪️io/📝️text/🪪️session-authority-v1/🦀️.rs", "use super::super::super::schema::session_authority::*;", "use crate::session_authority::*;")
sub1(M + "/🚪️io/📝️text/🌱️space-artifact-creation-v1/🦀️.rs", "use super::super::super::schema::space_artifact_creation::*;", "use crate::space_artifact_creation::*;")

# 6 check-in crate
p = M + "/🧬️schema/📌️document-check-in-v1/🦀️.rs"
sub1(p, "use super::{valid_document_open_hash, EditedArtifactFrontierV1, DOCUMENT_OPEN_MAX_SAFE_INTEGER};", "use semio_framework_os_directory_schema::{valid_document_open_hash, EditedArtifactFrontierV1, DOCUMENT_OPEN_MAX_SAFE_INTEGER};")
t = rd(p)
wr(p, t.rstrip("\n") + '\n\n#[path = "../../🚪️io/📝️text/📌️document-check-in-v1/🦀️.rs"]\nmod text;\n')
sub1(M + "/🚪️io/📝️text/📌️document-check-in-v1/🦀️.rs", "use super::super::super::schema::document_check_in::*;", "use super::*;")

# 7 checkpoint pair type
SCH_PAIR = "use super::{ArtifactFrontier, ArtifactHash, CheckpointId, DocumentScope, PublishedArtifactBlob, RebootstrapRequired};"
sub1(M + "/🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs", SCH_PAIR, SCH_PAIR.replace("use super::", "use semio_framework_os_directory_schema::"))

# 8 binary crate
p = M + "/🚪️io/🧱️binary/🦀️.rs"
assert rd(p) == '''#[path = "🪢️checkpoint-pair/🦀️.rs"]
pub mod checkpoint_pair;

#[path = "🔐️descriptor-digest/🦀️.rs"]
pub mod descriptor_digest;

#[path = "🪪️artifact-hash/🦀️.rs"]
pub mod artifact_hash;
''', rd(p)
wr(p, '''//! 🧱️ Directory native binary boundaries: explicit hash text, descriptor digest and the canonical checkpoint pair (value and framing codec).

#[path = "🪢️checkpoint-pair/🦀️.rs"]
pub mod checkpoint_pair;

#[path = "🔐️descriptor-digest/🦀️.rs"]
pub mod descriptor_digest;

#[path = "🪪️artifact-hash/🦀️.rs"]
pub mod artifact_hash;

#[path = "../../🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs"]
pub mod canonical_checkpoint_pair;
pub use canonical_checkpoint_pair::{AdmittedCheckpointSelectionV1, CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1};
''')
sub1(M + "/🚪️io/🧱️binary/🪪️artifact-hash/🦀️.rs", "use crate::schema::{ArtifactHash, ArtifactFrontier, EditedArtifactFrontierV1, valid_document_open_hash};", "use semio_framework_os_directory_schema::{ArtifactHash, ArtifactFrontier, EditedArtifactFrontierV1, valid_document_open_hash};")
sub1(M + "/🚪️io/🧱️binary/🔐️descriptor-digest/🦀️.rs", "use crate::schema::{ArtifactHash, DescriptorDigestError, DocumentDescriptor, validate_document_descriptor_v1};", "use semio_framework_os_directory_schema::{ArtifactHash, DescriptorDigestError, DocumentDescriptor, validate_document_descriptor_v1};")
p = M + "/🚪️io/🧱️binary/🪢️checkpoint-pair/🦀️.rs"
sub1(p, "use crate::schema::{ArtifactFrontier, ArtifactHash, CanonicalCheckpointPairV1, CanonicalCheckpointPairRefusalV1, DocumentScope, PublishedArtifactBlob, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};",
     "use crate::{AdmittedCheckpointSelectionV1, CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1};\nuse semio_framework_os_directory_schema::{ArtifactFrontier, ArtifactHash, DocumentScope, PublishedArtifactBlob, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};")
sub1(p, "expected: &crate::schema::DocumentOpenCheckpointV1) -> Result<crate::schema::AdmittedCheckpointSelectionV1,", "expected: &semio_framework_os_directory_schema::DocumentOpenCheckpointV1) -> Result<AdmittedCheckpointSelectionV1,")
sub1(p, "Ok(crate::schema::AdmittedCheckpointSelectionV1 {", "Ok(AdmittedCheckpointSelectionV1 {")

# 9 access policy crate
wr(M + "/🛡️access-policy/🦀️.rs", '''//! 🛡️ The directory's declared access policy and its one decision authority, one crate because the policy type owns both the schema and the evaluating methods.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

#[path = "🚪️io/📝️text/🦀️.rs"]
mod text;
pub use text::directory_access_permits;
''')
sub1(M + "/🛡️access-policy/🚪️io/📝️text/🦀️.rs", "use super::super::super::access_policy::*;", "use super::schema::*;")

# 10 the old io root is mounted by nobody any more
os.remove(M + "/🚪️io/🦀️.rs")
print("sources ok")
