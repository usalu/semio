/// 📌️ A dependency pin: the exact version (`=X.Y.Z`) of the plugin a manifest depends on. One tree is one catalog, and a
/// trusted catalog admits only exact pins inside its own closure (`trustedBootstrapDescriptorClaims`,
/// `🌎️hub/📦️packages/🦀️rust/📜️script.ts`), so a manifest carries no range grammar: a declaration pins the version its own
/// tree builds, through [`tree_pin!`](crate::tree_pin).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionPin(pub Version);

impl VersionPin {
    /// 🔢️ Parses the one wire form, `=X.Y.Z`.
    pub fn parse(input: &str) -> Result<Self, VersionPinParseError> {
        let trimmed = input.trim();
        let Some(rest) = trimmed.strip_prefix('=') else { return Err(VersionPinParseError::NotExact(trimmed.to_string())) };
        Ok(Self(Version::parse(rest)?))
    }

    /// 📌️ Pins the version a crate of this tree is compiled at — [`tree_pin!`](crate::tree_pin) passes the declaring crate's
    /// own `CARGO_PKG_VERSION`, and every plugin and extension crate is a member of one workspace built at one version.
    pub fn of_tree(crate_version: &str) -> Self {
        Self(Version::parse(crate_version).unwrap_or_else(|error| panic!("compiled crate version {crate_version:?} is not a strict major.minor.patch triple: {error}")))
    }

    /// ✅️ Whether `version` is exactly the pinned version.
    pub fn matches(&self, version: &Version) -> bool {
        &self.0 == version
    }

    /// ✅️ Parses `raw` and compares; an unparsable target version never matches.
    pub fn matches_raw(&self, raw: &str) -> bool {
        Version::parse(raw).is_ok_and(|version| self.matches(&version))
    }
}

impl std::fmt::Display for VersionPin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "={}", self.0)
    }
}

/// 🚧️ Failure parsing a `VersionPin`: anything but `=X.Y.Z` (a range such as `*`, `^`, `~` or `>=` included).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionPinParseError {
    NotExact(String),
    Version(VersionParseError),
}

impl std::fmt::Display for VersionPinParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExact(input) => write!(formatter, "a dependency pins an exact version `=X.Y.Z`, got {input:?}"),
            Self::Version(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for VersionPinParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Version(error) => Some(error),
            Self::NotExact(_) => None,
        }
    }
}

impl From<VersionParseError> for VersionPinParseError {
    fn from(error: VersionParseError) -> Self {
        Self::Version(error)
    }
}

impl ToValue for VersionPin {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.to_string())
    }
}
impl FromValue for VersionPin {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => VersionPin::parse(&s).map_err(|e| ValueError::new(e.to_string())),
            other => Err(ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

impl Serialize for VersionPin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for VersionPin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        VersionPin::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 📌️ The exact pin of the tree the invoking crate is compiled from: `semio_framework::tree_pin!()` expands in the
/// declaring crate, so `CARGO_PKG_VERSION` is that crate's own workspace version.
#[macro_export]
macro_rules! tree_pin {
    () => {
        $crate::VersionPin::of_tree(env!("CARGO_PKG_VERSION"))
    };
}

