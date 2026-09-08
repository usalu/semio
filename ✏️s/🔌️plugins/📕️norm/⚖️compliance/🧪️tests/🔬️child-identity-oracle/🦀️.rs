
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identity {
    child_id: String,
    target: Target,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Target {
    artifact_id: String,
    dialect: protocol::io_schema::ArtifactDialect,
}

impl Identity {
    fn observe<T>(child: &store::ArtifactChild<T>) -> Self {
        Self { child_id: child.child_id.clone(), target: Target { artifact_id: child.target.artifact_id.clone(), dialect: child.target.dialect.clone() } }
    }

    fn into_child<T>(self) -> store::ArtifactChild<T> {
        store::ArtifactChild::new(self.child_id, protocol::io_schema::ArtifactRef { artifact_id: self.target.artifact_id, dialect: self.target.dialect })
    }
}

pub fn serialize<T, S: serde::Serializer>(child: &store::ArtifactChild<T>, serializer: S) -> Result<S::Ok, S::Error> {
    Identity::observe(child).serialize(serializer)
}

pub fn deserialize<'de, T, D: serde::Deserializer<'de>>(deserializer: D) -> Result<store::ArtifactChild<T>, D::Error> {
    Identity::deserialize(deserializer).map(Identity::into_child)
}

pub mod optional {
    use super::*;

    pub fn serialize<T, S: serde::Serializer>(child: &Option<store::ArtifactChild<T>>, serializer: S) -> Result<S::Ok, S::Error> {
        child.as_ref().map(Identity::observe).serialize(serializer)
    }

    pub fn deserialize<'de, T, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<store::ArtifactChild<T>>, D::Error> {
        Option::<Identity>::deserialize(deserializer).map(|child| child.map(Identity::into_child))
    }
}
