//! ⚙️ Office OOXML engine — ZIP container of parts (shared).

use crate::artifacts::{mid}::{{schema::snapshot::{Name}Entry, {Name}Artifact, {Name}Diff, {Name}Mutation, {Name}Snapshot, STDIO_{MID}_DOCUMENT_SCHEMA}};

fn to_zip(snap: &{Name}Snapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
        }).collect(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> {Name}Snapshot {
    {Name}Snapshot {
        schema: STDIO_{MID}_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| {Name}Entry {{ name: e.name, data: e.data }}).collect(),
    }
}

pub fn encode_{mid}(snap: &{Name}Snapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::zip::engine::encode_zip(&to_zip(snap), true)
}

pub fn decode_{mid}(data: &[u8]) -> Result<{Name}Snapshot, String> {
  Ok(from_zip(crate::artifacts::zip::engine::decode_zip(data)?))
}

pub fn empty_{mid}_snapshot() -> {Name}Snapshot {{ {Name}Snapshot::default() }}

pub fn register() {{
    crate::artifacts::{mid}::io::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::{mid}::schema::{mid}_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<{Name}Snapshot, {Name}Mutation>(STDIO_{MID}_DOCUMENT_SCHEMA));
}}

pub struct {Name}Engine {{ artifact_state: {Name}Artifact, snapshot_state: {Name}Snapshot }}
impl {Name}Engine {{
    pub fn new(snapshot: {Name}Snapshot) -> Self {{
        Self {{ artifact_state: {Name}Artifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }}
    }}
}}
impl protocol::ArtifactEngine for {Name}Engine {{
    type Artifact = {Name}Artifact; type Snapshot = {Name}Snapshot; type Mutation = {Name}Mutation; type Diff = {Name}Diff;
    fn artifact(&self) -> &Self::Artifact {{ &self.artifact_state }}
    fn snapshot(&self) -> &Self::Snapshot {{ &self.snapshot_state }}
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }}
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }}
}}
