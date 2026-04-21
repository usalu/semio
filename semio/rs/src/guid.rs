use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// Stable identity used at serialization boundaries and as a dictionary key when
/// resolving DTOs into the in-memory graph. Never used for in-graph traversal.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Guid(String);

impl Guid {
    /// Produces a fresh UUIDv7 (monotonic) wrapped as a `Guid`.
    pub fn new_v7() -> Self {
        Self(uuid::Uuid::now_v7().to_string())
    }

    /// Borrow the underlying string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the [`Guid`], returning the inner [`String`].
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for Guid {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Guid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Guid {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<String> for Guid {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Guid {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<Guid> for String {
    fn from(g: Guid) -> Self {
        g.0
    }
}

impl PartialEq<str> for Guid {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Guid {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
