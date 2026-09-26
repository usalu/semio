#!/usr/bin/env python3
"""📌️ S17 landing set: W2's exact-pin narrowing (`VersionReq` → `VersionPin`, `=X.Y.Z` only) across the SDK, the 26
extensions, demonstrator, stdio, hub, kernel `.sxt` mirror, the TS kernel twin, the registry TS and their laws/vectors.
Extension own versions (and flow extension manifest versions, procedural's flow-extension roster) follow the tree version
(`env!("CARGO_PKG_VERSION")`), which fixes brep 0.3.0 / math 0.2.0 drift.

Anchored, all-or-nothing: every op must match exactly `count` times on the current tree or nothing is written.
usage: python3 s17-version-pin.py [--write]
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUG = "✏️s/🔌️plugins"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
KERNEL_TS = "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts"
TREE_PIN = "semio_framework::tree_pin!()"
TREE_VERSION = 'env!("CARGO_PKG_VERSION")'

OPS = []


def rep(path, old, new, count=1):
    OPS.append(("rep", path, old, new, count))


def block(path, start, end, new):
    OPS.append(("block", path, start, end, new))


# ---------------------------------------------------------------- manifest (semio-framework)
rep(MANIFEST, "/// 🚧️ Failure parsing a `Version` (`major.minor.patch`, all-numeric segments) or a `VersionReq`.", "/// 🚧️ Failure parsing a `Version` (`major.minor.patch`, all-numeric segments) or a `VersionPin`.")
block(
    MANIFEST,
    "/// 🔢️ A dependency version requirement — the frozen grammar `=X.Y.Z` / `^X.Y.Z` / `~X.Y.Z` /",
    """impl<'de> Deserialize<'de> for VersionReq {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        VersionReq::parse(&raw).map_err(serde::de::Error::custom)
    }
}
""",
    """/// 📌️ A dependency pin: the exact version (`=X.Y.Z`) of the plugin a manifest depends on. One tree is one catalog, and a
/// trusted catalog admits only exact pins inside its own closure (`trustedBootstrapDescriptorClaims`,
/// `🌎️hub/📦️packages/🦀️rust/📜️script.ts`), so a manifest carries no range grammar: a declaration pins the version its
/// own tree builds, through [`tree_pin!`](crate::tree_pin).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionPin(pub Version);

impl VersionPin {
    /// 🔢️ Parses the one wire form, `=X.Y.Z`; every range (`*`, `^`, `~`, `>=`, a bare triple) is refused.
    // 🚫️async: E1 transitive — `FromValue::from_value` (this file, below) calls this synchronously; pure string
    // parsing, no I/O (R9).
    pub fn parse(input: &str) -> Result<Self, VersionPinParseError> {
        let trimmed = input.trim();
        let Some(rest) = trimmed.strip_prefix('=') else { return Err(VersionPinParseError::NotExact(trimmed.to_string())) };
        Ok(Self(Version::parse(rest)?))
    }

    /// 📌️ Pins the version a crate of this tree is compiled at. [`tree_pin!`](crate::tree_pin) evaluates it in a
    /// `const` over the declaring crate's own `CARGO_PKG_VERSION`, so a version that is not a strict
    /// `major.minor.patch` triple is a compile error in that crate, never a guest trap.
    pub const fn of_tree(crate_version: &str) -> Self {
        let bytes = crate_version.as_bytes();
        let mut segments = [0u64; 3];
        let mut segment = 0;
        let mut digits = 0;
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'.' {
                if digits == 0 || segment == 2 {
                    panic!("the compiled crate version is not a strict major.minor.patch triple");
                }
                segment += 1;
                digits = 0;
            } else if byte.is_ascii_digit() {
                segments[segment] = segments[segment] * 10 + (byte - b'0') as u64;
                digits += 1;
            } else {
                panic!("the compiled crate version is not a strict major.minor.patch triple");
            }
            index += 1;
        }
        if segment != 2 || digits == 0 {
            panic!("the compiled crate version is not a strict major.minor.patch triple");
        }
        Self(Version { major: segments[0], minor: segments[1], patch: segments[2] })
    }

    /// ✅️ Whether `version` is exactly the pinned version.
    // 🚫️async: E1 transitive — pure comparison consumed by `matches_raw`, itself required sync (R9).
    pub fn matches(&self, version: &Version) -> bool {
        &self.0 == version
    }

    /// ✅️ Convenience for the dependency graph: parses `raw` and compares; an unparsable target version never matches.
    // 🚫️async: E1 transitive — dependency-graph validation calls this synchronously via `!`; pure parse-and-compare,
    // no I/O (R9).
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

// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only
// manifest type above/below (`PluginDependency.version`) — hand-written, same reasoning as the
// `ToValue`/`FromValue` pair above.
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
/// declaring crate, so `CARGO_PKG_VERSION` is that crate's own workspace version, checked at compile time.
#[macro_export]
macro_rules! tree_pin {
    () => {{
        const TREE_PIN: $crate::VersionPin = $crate::VersionPin::of_tree(env!("CARGO_PKG_VERSION"));
        TREE_PIN
    }};
}
""",
)
rep(MANIFEST, """/// 🔗️ One direct plugin dependency: the depended-on plugin id plus the version requirement it must
/// satisfy — see `resolve_load_order`/`validate_dependency_graph`.""", """/// 🔗️ One direct plugin dependency: the depended-on plugin id plus the exact version it pins — see
/// `resolve_load_order`/`validate_dependency_graph`.""")
rep(MANIFEST, "    pub version: VersionReq,\n}\n\nimpl PluginDependency {\n    pub fn new(plugin_id: impl Into<String>, version: VersionReq) -> Self {", "    pub version: VersionPin,\n}\n\nimpl PluginDependency {\n    pub fn new(plugin_id: impl Into<String>, version: VersionPin) -> Self {")

MANIFEST_TEST = "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️plugin-dependency/🦀️.rs"
rep(MANIFEST_TEST, "//! lane W0-C: `Version`/`VersionReq` parse+match matrix, dependency-graph toposort/cycle/", "//! lane W0-C: `Version`/`VersionPin` parse+match matrix, dependency-graph toposort/cycle/")
block(
    MANIFEST_TEST,
    "//#region 🔖️VersionAndVersionReq\n",
    "//#endregion 🔖️VersionAndVersionReq\n",
    """//#region 🔖️VersionAndVersionPin
fn pin(major: u64, minor: u64, patch: u64) -> VersionPin {
    VersionPin(Version::new(major, minor, patch))
}

#[semio_framework_async_macros::async_test]
async fn version_parses_valid_triples_and_rejects_malformed_input() {
    assert_eq!(Version::parse("1.2.3").unwrap(), Version::new(1, 2, 3));
    assert_eq!(Version::parse("0.0.0").unwrap(), Version::new(0, 0, 0));
    assert!(matches!(Version::parse("1.2").unwrap_err(), VersionParseError::Malformed(_)));
    assert!(matches!(Version::parse("1.2.3.4").unwrap_err(), VersionParseError::Malformed(_)));
    assert!(matches!(Version::parse("1.x.3").unwrap_err(), VersionParseError::NonNumeric(_, seg) if seg == "x"));
    assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
}

#[semio_framework_async_macros::async_test]
async fn version_ord_matches_semver_precedence() {
    assert!(Version::new(1, 0, 0) < Version::new(1, 0, 1));
    assert!(Version::new(1, 0, 0) < Version::new(1, 1, 0));
    assert!(Version::new(1, 0, 0) < Version::new(2, 0, 0));
    assert!(Version::new(1, 9, 9) < Version::new(2, 0, 0));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_parses_only_the_exact_form_and_refuses_every_range() {
    assert_eq!(VersionPin::parse("=1.2.3").unwrap(), pin(1, 2, 3));
    assert_eq!(VersionPin::parse(" =0.1.0 ").unwrap(), pin(0, 1, 0));
    for range in ["*", "^1.2.3", "~1.2.3", ">=1.2.3", "1.2.3", ""] {
        assert!(matches!(VersionPin::parse(range).unwrap_err(), VersionPinParseError::NotExact(_)), "{range:?} must be refused");
    }
    assert!(matches!(VersionPin::parse("=1.x.3").unwrap_err(), VersionPinParseError::Version(_)));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_display_round_trips_through_parse() {
    for raw in ["=0.1.0", "=1.2.3"] {
        let parsed = VersionPin::parse(raw).unwrap();
        assert_eq!(parsed.to_string(), raw);
        assert_eq!(VersionPin::parse(&parsed.to_string()).unwrap(), parsed);
    }
}

#[semio_framework_async_macros::async_test]
async fn version_pin_matches_only_its_exact_version() {
    let exact = pin(1, 2, 3);
    assert!(exact.matches(&Version::new(1, 2, 3)));
    for other in [Version::new(1, 2, 4), Version::new(1, 2, 2), Version::new(1, 3, 0), Version::new(2, 0, 0), Version::new(0, 0, 0)] {
        assert!(!exact.matches(&other), "{exact} must not match {other}");
    }
    assert!(exact.matches_raw("1.2.3"));
    assert!(!exact.matches_raw("1.2"));
    assert!(!exact.matches_raw("not-a-version"));
}

#[semio_framework_async_macros::async_test]
async fn version_pin_of_tree_accepts_only_a_strict_triple() {
    assert_eq!(VersionPin::of_tree("0.1.0"), pin(0, 1, 0));
    assert_eq!(VersionPin::of_tree("12.30.4"), pin(12, 30, 4));
    for malformed in ["0.1", "0.1.0.1", "0.1.0-alpha", "", ".1.0", "0..0"] {
        assert!(std::panic::catch_unwind(|| VersionPin::of_tree(malformed)).is_err(), "{malformed:?} must not pin");
    }
    assert_eq!(crate::tree_pin!(), VersionPin::parse(concat!("=", env!("CARGO_PKG_VERSION"))).unwrap());
}

#[semio_framework_async_macros::async_test]
async fn plugin_dependency_serde_round_trips_as_an_exact_pin_string() {
    let dependency = PluginDependency::new("cad", pin(1, 0, 0));
    let json = serde_json::to_value(&dependency).unwrap();
    assert_eq!(json, serde_json::json!({ "pluginId": "cad", "version": "=1.0.0" }));
    let round_tripped: PluginDependency = serde_json::from_value(json).unwrap();
    assert_eq!(round_tripped, dependency);
    assert_eq!(<PluginDependency as FromValue>::from_value(dependency.to_value()).unwrap(), dependency);
}

#[semio_framework_async_macros::async_test]
async fn a_range_dependency_is_refused_on_every_decode_path() {
    for range in ["*", "^0.1.0", "~0.1.0", ">=0.1.0", "0.1.0"] {
        assert!(serde_json::from_value::<PluginDependency>(serde_json::json!({ "pluginId": "cad", "version": range })).is_err(), "serde must refuse {range:?}");
        let value = DslValue::object([("pluginId".to_string(), DslValue::String("cad".into())), ("version".to_string(), DslValue::String(range.into()))]);
        assert!(<PluginDependency as FromValue>::from_value(value).is_err(), "the descriptor codec must refuse {range:?}");
    }
}
//#endregion 🔖️VersionAndVersionPin
""",
)
rep(MANIFEST_TEST, "VersionReq::Any", "pin(1, 0, 0)", 13)
rep(MANIFEST_TEST, 'vec![PluginDependency::new("b", VersionReq::parse("^2.0.0").unwrap())]', 'vec![PluginDependency::new("b", pin(2, 0, 0))]')
rep(MANIFEST_TEST, 'required: "^2.0.0".into()', 'required: "=2.0.0".into()')
rep(MANIFEST_TEST, 'dependencies: vec![PluginDependency::new("cad", VersionReq::parse("^1.0.0").unwrap())],', 'dependencies: vec![PluginDependency::new("cad", pin(1, 0, 0))],')

# ---------------------------------------------------------------- plugin SDK
BUILDER = f"{SDK}/🏗️builder/🦀️.rs"
rep(BUILDER, "    pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionReq) -> Self {", "    pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionPin) -> Self {")
rep(BUILDER, "    /// 🔗️ Declares a direct plugin dependency this plugin requires to load — contract freeze §3/§4.\n", "    /// 🔗️ Declares a direct plugin dependency this plugin requires to load, pinned exactly (`semio_framework::tree_pin!()`\n    /// for a plugin of the same tree) — contract freeze §3/§4.\n")
SDK_ROOT = f"{SDK}/🦀️.rs"
rep(SDK_ROOT, "        /// 🔗️ Declares a direct plugin dependency — contract freeze §3/§4: the FIRST dependency\n        /// declared (in call order) must be the same plugin `.extends(...)` names.\n        pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionReq) -> Self {", "        /// 🔗️ Declares a direct plugin dependency, pinned exactly (`semio_framework::tree_pin!()`) — contract freeze §3/§4:\n        /// the FIRST dependency declared (in call order) must be the same plugin `.extends(...)` names.\n        pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionPin) -> Self {")
rep(f"{SDK}/🏗️builder/🧪️tests/🔬️plugin-builder-dependency/🦀️.rs", '.depends_on("builder-test-dep-target-ok", semio_framework::VersionReq::Any)', f'.depends_on("builder-test-dep-target-ok", {TREE_PIN})')
rep(f"{SDK}/🧪️tests/🔬️plugin-runtime-extension-bundle-dependency/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN, 4)
rep(f"{SDK}/🖥️host/🧪️tests/🔬️app-router/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN)
rep(f"{SDK}/🖥️host/🧪️tests/🔬️artifact-mutation-router/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN, 2)
rep(f"{SDK}/🖥️host/🧪️tests/🔬️host-transaction-coordinator/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN)
rep(f"{SDK}/🖥️host/🧪️tests/🔬️opening-resolver/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN)
rep(f"{SDK}/🧪️tests/🔬️app-artifact-contribution/🦀️.rs", "semio_framework::VersionReq::Any", TREE_PIN, 2)
GRAPH = f"{SDK}/🖥️host/🧪️tests/🔬️plugin-graph/🦀️.rs"
rep(GRAPH, "semio_framework::VersionReq::parse(req).unwrap()", "semio_framework::VersionPin::parse(req).unwrap()")
rep(GRAPH, '("cad", "^1.0.0")', '("cad", "=1.0.0")')
rep(GRAPH, '("base", "^1.0.0")', '("base", "=1.0.0")', 3)
rep(GRAPH, '("missing", "*")', '("missing", "=1.0.0")')
rep(GRAPH, '("base", "^2.0.0")', '("base", "=2.0.0")')
rep(GRAPH, '("a", "*")', '("a", "=1.0.0")')
rep(GRAPH, '("b", "*")', '("b", "=1.0.0")')
rep(GRAPH, """async fn hot_reload_is_rejected_when_it_would_break_a_live_dependents_version_requirement() {""", """async fn hot_reload_is_rejected_when_it_would_break_a_live_dependents_pin() {""")
rep(GRAPH, """    graph.prepare_hot_reload(&manifest("base", "1.1.0", &[]).await).await.expect("a caret-compatible bump must still validate");""", """    let bump = graph.prepare_hot_reload(&manifest("base", "1.1.0", &[]).await).await.unwrap_err();
    assert!(matches!(bump, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })), "an exact pin admits no bump at all");
    graph.prepare_hot_reload(&manifest("base", "1.0.0", &[]).await).await.expect("reloading the pinned version must still validate");""")

# ---------------------------------------------------------------- kernel `.sxt` mirror
SXT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️.rs"
rep(SXT, """/// `version` is the plain `VersionReq` display string (`=X.Y.Z`/`^X.Y.Z`/`~X.Y.Z`/`>=X.Y.Z`/`*`,
/// contract freeze §3) — round-trips losslessly through `semio_framework::VersionReq::parse` at
/// any call site that does depend on that crate (e.g. the guest `ExtensionManifest`).""", """/// `version` is the exact pin's display string `=X.Y.Z` — the only form `semio_framework::VersionPin::parse`
/// accepts at any call site that does depend on that crate (e.g. the guest `ExtensionManifest`); `from_json`
/// refuses every range (`*`, `^`, `~`, `>=`, a bare triple) the same way.""")
rep(SXT, """            version: value.get("version").and_then(Value::as_str).map(str::to_owned).ok_or_else(|| "missing field version".to_string())?,
        })
    }
}
""", """            version: value.get("version").and_then(Value::as_str).filter(|version| is_exact_pin(version)).map(str::to_owned).ok_or_else(|| "field version must be an exact pin `=X.Y.Z`".to_string())?,
        })
    }
}

/// 📌️ Whether `raw` is an exact dependency pin `=X.Y.Z` (three all-numeric segments), the `.sxt` twin of
/// `semio_framework::VersionPin::parse`.
fn is_exact_pin(raw: &str) -> bool {
    raw.trim().strip_prefix('=').is_some_and(|version| {
        let segments: Vec<&str> = version.split('.').collect();
        segments.len() == 3 && segments.iter().all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()))
    })
}
""")
SXT_TEST = "🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🧪️tests/🔬️unit/🦀️.rs"
rep(SXT_TEST, 'dependencies: vec![PackagePluginDependency { plugin_id: "flow".into(), version: "^1.0.0".into() }],', 'dependencies: vec![PackagePluginDependency { plugin_id: "flow".into(), version: "=1.0.0".into() }],')
rep(SXT_TEST, """    let dependency = PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() };
    let json = dependency.to_json();
    assert_eq!(json, object([("pluginId".to_string(), Value::from("cad")), ("version".to_string(), Value::from("^1.0.0"))]));
    let round_tripped = PackagePluginDependency::from_json(&json).unwrap();
    assert_eq!(round_tripped, dependency);
}
""", """    let dependency = PackagePluginDependency { plugin_id: "cad".into(), version: "=1.0.0".into() };
    let json = dependency.to_json();
    assert_eq!(json, object([("pluginId".to_string(), Value::from("cad")), ("version".to_string(), Value::from("=1.0.0"))]));
    let round_tripped = PackagePluginDependency::from_json(&json).unwrap();
    assert_eq!(round_tripped, dependency);
}

#[semio_framework_async_macros::async_test]
async fn package_plugin_dependency_refuses_every_range() {
    use crate::os_pack::json::{object, Value};
    for range in ["*", "^1.0.0", "~1.0.0", ">=1.0.0", "1.0.0", "=1.0", "=1.x.0"] {
        let json = object([("pluginId".to_string(), Value::from("cad")), ("version".to_string(), Value::from(range))]);
        assert!(PackagePluginDependency::from_json(&json).is_err(), "{range:?} must be refused");
    }
}
""")

# ---------------------------------------------------------------- hub
rep("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs", """    let semio_framework::VersionReq::Exact(version) = dependency.version else { return Err(catalog("compiled Stdio fixture dependency is not exact")); };
    let version = version.to_string();""", """    let version = dependency.version.0.to_string();""")

# ---------------------------------------------------------------- plugins: demonstrator, stdio, flow viewer test
DEMO = f"{PLUG}/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs"
rep(DEMO, """/// 📌️ Pins one composed plugin exactly at the version of the tree this bundle is compiled from: a trusted catalog
/// admits only exact dependency pins inside its own closure (`=x.y.z`, `trustedBootstrapDescriptorClaims`), never a range.
fn same_tree_pin() -> Result<VersionReq, PluginAssemblyError> {
    Version::parse(PLUGIN_VERSION).map(VersionReq::Exact).map_err(|error| PluginAssemblyError::new("plugin-assembly.dependency-version", format!("compiled workspace version is not semver: {error}")))
}

""", "")
rep(DEMO, "same_tree_pin()?", TREE_PIN, 6)
rep(f"{PLUG}/🗄️stdio/📇️registry/🦀️.rs", """    let version = semio_framework::Version::parse(env!("CARGO_PKG_VERSION")).map_err(|error| failure(format!("compiled Stdio catalog version is invalid: {error}")))?;
    Ok(semio_framework::PluginDependency::new("stdio", semio_framework::VersionReq::Exact(version)))""", """    Ok(semio_framework::PluginDependency::new("stdio", semio_framework::tree_pin!()))""")
rep(f"{PLUG}/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs", '.depends_on("flow", semio_framework::VersionReq::Any)', f'.depends_on("flow", {TREE_PIN})')

# ---------------------------------------------------------------- 26 extensions: exact pin + tree version
FLOW_EXT = f"{PLUG}/🌊️flow/🧩️extensions"
for ext, bundle_old in [
    ("🏗️bim", 'ExtensionBundle::new("flow-extension-bim", "Bim", "0.1.0")'),
    ("📃️list", 'ExtensionBundle::new("flow-extension-list", EXTENSION_LABEL, "0.1.0")'),
    ("📐️brep", 'ExtensionBundle::new("flow-extension-brep", "Brep", "0.3.0")'),
    ("📖️dictionary", 'ExtensionBundle::new("flow-extension-dictionary", EXTENSION_LABEL, "0.1.0")'),
    ("📝️text", 'ExtensionBundle::new("flow-extension-text", EXTENSION_LABEL, "0.1.0")'),
    ("🔤️primitive", 'ExtensionBundle::new("flow-extension-primitive", EXTENSION_LABEL, "0.1.0")'),
    ("🖍️draw", 'ExtensionBundle::new("flow-extension-draw", "Draw", "0.1.0")'),
    ("🧠️logic", 'ExtensionBundle::new("flow-extension-logic", EXTENSION_LABEL, "0.1.0")'),
    ("🧮️math", 'ExtensionBundle::new("flow-extension-math", EXTENSION_LABEL, "0.2.0")'),
]:
    path = f"{FLOW_EXT}/{ext}/🦀️.rs"
    bundle_new = bundle_old.rsplit(", ", 1)[0] + f", {TREE_VERSION})"
    rep(path, f'{bundle_old}.extends("flow").depends_on("flow", semio_framework::VersionReq::Any)', f'{bundle_new}.extends("flow").depends_on("flow", {TREE_PIN})')
for ext, slug, label, version, count in [("🏗️bim", "bim", "Bim", "0.1.0", 2), ("📃️list", "list", "List", "0.1.0", 1), ("📐️brep", "brep", "Brep", "0.3.0", 1), ("📝️text", "text", "Text", "0.1.0", 1), ("🔤️primitive", "core", "Core", "0.1.0", 1), ("🖍️draw", "draw", "Draw", "0.1.0", 2), ("🧠️logic", "logic", "Logic", "0.1.0", 1)]:
    rep(f"{FLOW_EXT}/{ext}/🦀️.rs", f'build_manifest_json("{slug}", "{label}", "{version}", ', f'build_manifest_json("{slug}", "{label}", {TREE_VERSION}, ', count)
rep(f"{FLOW_EXT}/📖️dictionary/🦀️.rs", '        "dictionary",\n        "Dictionary",\n        "0.1.0",\n', f'        "dictionary",\n        "Dictionary",\n        {TREE_VERSION},\n')
rep(f"{FLOW_EXT}/🧮️math/🦀️.rs", '        "math",\n        "Math",\n        "0.2.0",\n', f'        "math",\n        "Math",\n        {TREE_VERSION},\n')
rep(f"{FLOW_EXT}/🧮️math/🧪️tests/🔬️unit/🦀️.rs", '        "math",\n        "Math",\n        "0.2.0",\n', f'        "math",\n        "Math",\n        {TREE_VERSION},\n')
rep(f"{FLOW_EXT}/📐️brep/🧪️tests/🔬️unit/🦀️.rs", 'build_manifest_json("brep", "Brep", "0.3.0", ', f'build_manifest_json("brep", "Brep", {TREE_VERSION}, ')
rep(f"{FLOW_EXT}/📐️brep/🧪️tests/🔬️unit/🦀️.rs", 'ExtensionBundle::new("flow-extension-brep", "Brep", "0.3.0")', f'ExtensionBundle::new("flow-extension-brep", "Brep", {TREE_VERSION})')
for parent, ext, label in [
    ("🏭️process", "🔩️metal", "Process Metal Machines"),
    ("🏭️process", "🤖️robotic", "Process Robotic Machines"),
    ("🏭️process", "🧱️concrete", "Process Concrete Machines"),
    ("🏭️process", "🪵️wood", "Process Wood Machines"),
    ("📐️cad", "🏛️aec-building-structure", "CAD AEC Building Structure"),
    ("📐️cad", "📐️spatial-shape", "CAD Spatial Shape"),
    ("📐️cad", "🔥️aec-building-energy", "CAD AEC Building Energy"),
    ("🪵️sourcing", "🧱️slabs", "Sourcing Module Slabs"),
    ("🪵️sourcing", "🪟️windows", "Sourcing Module Windows"),
    ("🪵️sourcing", "🪵️beams", "Sourcing Module Beams"),
]:
    pid = parent.split("️", 1)[1]
    rep(f"{PLUG}/{parent}/🧩️extensions/{ext}/🦀️.rs", f'ExtensionBundle::new(EXTENSION_ID, "{label}", "0.1.0").extends("{pid}").depends_on("{pid}", semio_framework::VersionReq::Any)', f'ExtensionBundle::new(EXTENSION_ID, "{label}", {TREE_VERSION}).extends("{pid}").depends_on("{pid}", {TREE_PIN})')
rep(f"{PLUG}/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs", 'ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", "0.1.0").extends("cad").depends_on("cad", semio_framework::VersionReq::parse("^0.1.0").expect("valid version req"))', f'ExtensionBundle::new(EXTENSION_ID, "CAD AEC Building", {TREE_VERSION}).extends("cad").depends_on("cad", {TREE_PIN})')
for ext in ["🎮️control", "📝️text", "📣️effect", "🧠️logic", "🧮️math"]:
    path = f"{PLUG}/📜️imperative/🧩️extensions/{ext}/🦀️.rs"
    rep(path, 'const MODULE_VERSION: &str = "0.1.0";', f"const MODULE_VERSION: &str = {TREE_VERSION};")
    rep(path, '.extends("imperative").depends_on("imperative", semio_framework::VersionReq::Any)', f'.extends("imperative").depends_on("imperative", {TREE_PIN})')
PB = f"{PLUG}/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs"
rep(PB, '        .version("0.1.0")\n        .package_id("semio:playbook-module-procedural")', f'        .version({TREE_VERSION})\n        .package_id("semio:playbook-module-procedural")')
rep(PB, '        .depends_on("playbook", VersionReq::parse("^0.1.0").expect("declared playbook version"))', f'        .depends_on("playbook", {TREE_PIN})')
rep(PB, 'ExtensionBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", "0.1.0").extends("playbook").depends_on("playbook", VersionReq::parse("^0.1.0").expect("declared playbook version"))', f'ExtensionBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", {TREE_VERSION}).extends("playbook").depends_on("playbook", {TREE_PIN})')

# ---------------------------------------------------------------- procedural's flow-extension roster: tree version, no drift
PROC = f"{PLUG}/🌀️procedural/🦀️.rs"
rep(PROC, """/// 🌊️ `(slug, extension id, label, version)` for every flow extension this plugin installs. The
/// slug is the only free variable in a declaration: the contribution id is
/// `s.procedural.flow-extension.<slug>` and the native executable is
/// `semio.s.plugin.flow.extension.<slug>`, so the table states each extension once instead of
/// spelling those three strings out nine times. `🎮️commands` reads the same table to answer
/// `listFlowExtensions`.
pub(crate) const FLOW_EXTENSIONS: [(&str, &str, &str, &str); 9] = [
    ("brep", "brep", "Brep", "0.3.0"),
    ("math", "math", "Math", "0.1.0"),
    ("primitive", "core", "Core", "0.1.0"),
    ("logic", "logic", "Logic", "0.1.0"),
    ("dictionary", "dictionary", "Dictionary", "0.1.0"),
    ("list", "list", "List", "0.1.0"),
    ("text", "text", "Text", "0.1.0"),
    ("draw", "draw", "Draw", "0.1.0"),
    ("bim", "bim", "Bim", "0.1.0"),
];""", """/// 🌊️ `(slug, extension id, label)` for every flow extension this plugin installs. The
/// slug is the only free variable in a declaration: the contribution id is
/// `s.procedural.flow-extension.<slug>` and the native executable is
/// `semio.s.plugin.flow.extension.<slug>`, so the table states each extension once instead of
/// spelling those three strings out nine times. Every extension is a member of this tree, so its
/// version is [`FLOW_EXTENSION_VERSION`]. `🎮️commands` reads the same table to answer
/// `listFlowExtensions`.
pub(crate) const FLOW_EXTENSIONS: [(&str, &str, &str); 9] = [
    ("brep", "brep", "Brep"),
    ("math", "math", "Math"),
    ("primitive", "core", "Core"),
    ("logic", "logic", "Logic"),
    ("dictionary", "dictionary", "Dictionary"),
    ("list", "list", "List"),
    ("text", "text", "Text"),
    ("draw", "draw", "Draw"),
    ("bim", "bim", "Bim"),
];

/// 📌️ The version every flow extension of [`FLOW_EXTENSIONS`] is built at — the tree's own workspace version.
pub(crate) const FLOW_EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");""")
rep(PROC, """        .map(|(slug, extension, label, version)| {
            let native = format!("semio.s.plugin.flow.extension.{slug}");
            FlowExtensionDeclaration::new(flow_extension_declaration_id(slug), FlowExtensionManifest::new(*extension, *label, *version)?, FlowExtensionExecutableIdentity::native(native.clone(), native, "register")?)""", """        .map(|(slug, extension, label)| {
            let native = format!("semio.s.plugin.flow.extension.{slug}");
            FlowExtensionDeclaration::new(flow_extension_declaration_id(slug), FlowExtensionManifest::new(*extension, *label, FLOW_EXTENSION_VERSION)?, FlowExtensionExecutableIdentity::native(native.clone(), native, "register")?)""")
CMDS = f"{PLUG}/🌀️procedural/🎮️commands/🦀️.rs"
DESC = '.describe(LocalizedLabel::native("Lists every flow extension the procedural plugin can load (id, extension, label and version); nothing is changed.", "Listet alle Flow-Erweiterungen auf, die das Prozedural-Plugin laden kann (Id, Erweiterung, Bezeichnung und Version); nichts wird geändert."))'
rep(CMDS, DESC * 3, DESC)
rep(CMDS, """        .map(|(slug, extension, label, version)| {""", """        .map(|(slug, extension, label)| {""")
rep(CMDS, """                ("version".to_string(), DslValue::String((*version).to_string())),""", """                ("version".to_string(), DslValue::String(super::FLOW_EXTENSION_VERSION.to_string())),""")
rep(f"{PLUG}/🌀️procedural/🎮️commands/🧪️tests/🔬️unit/🦀️.rs", """    assert_eq!(ids.len(), crate::FLOW_EXTENSIONS.len(), "every row carries an id");""", """    assert_eq!(ids.len(), crate::FLOW_EXTENSIONS.len(), "every row carries an id");
    assert!(rows.iter().all(|row| row.get("version").and_then(DslValue::as_str) == Some(env!("CARGO_PKG_VERSION"))), "every extension is reported at the tree version");""")

# ---------------------------------------------------------------- TS kernel twin
rep(KERNEL_TS, """/** 🔗️ Widens a {@link PluginCatalogTarget.dependsOn} plugin-id list (the crate's declared runtime
 * dependencies, carried without version info) into `PluginRegistryEntry.dependencies` — each id gets
 * the always-satisfied `*` requirement so {@link resolvePluginLoadOrder}/
 * {@link validatePluginDependencyGraph} can validate presence and detect cycles from the registry's
 * pre-build view, which has no `VersionReq` to read; the real requirement travels on the loaded
 * manifest's own `dependencies` once the plugin's descriptor is available. */
function dependsOnToPluginDependencies(dependsOn: readonly string[] | undefined): readonly PluginDependency[] | undefined {
  return dependsOn?.map((pluginId) => ({ pluginId, version: "*" }));
}""", """/** 🔗️ Widens a {@link PluginCatalogTarget.dependsOn} plugin-id list (the crate's declared runtime
 * dependencies, carried without version info) into `PluginRegistryEntry.dependencies` — each edge
 * carries only its id, so {@link resolvePluginLoadOrder}/{@link validatePluginDependencyGraph}
 * validate presence and detect cycles from the registry's pre-build view, which has no pin to read;
 * the exact pin travels on the loaded manifest's own `dependencies` once the plugin's descriptor is
 * available. */
function dependsOnToPluginDependencies(dependsOn: readonly string[] | undefined): readonly PluginDependency[] | undefined {
  return dependsOn?.map((pluginId) => ({ pluginId }));
}""")
rep(KERNEL_TS, """/** 🔢️ A frozen `major.minor.patch` version requirement string — one of `*`, `=X.Y.Z`, `^X.Y.Z`,
 * `~X.Y.Z`, `>=X.Y.Z` (contract freeze §3). Mirrors Rust `VersionReq`'s `Display`/`Serialize`
 * wire form exactly; parsing/matching stays server-side (Rust `resolve_load_order` et al.) — this
 * type only lets the browser host read/display/round-trip the requirement string. */
export type VersionReq = string;

/** 🔗️ One direct plugin dependency — mirrors Rust `PluginDependency`
 * (`🛂️manifest/🦀️.rs`). */
export type PluginDependency = {
  readonly pluginId: string;
  readonly version: VersionReq;
};""", """/** 📌️ An exact dependency pin string, `=X.Y.Z` — the only form a manifest declares, because a trusted
 * catalog admits only exact pins inside its closure. Mirrors Rust `VersionPin`'s `Display`/`Serialize`
 * wire form exactly. */
export type VersionPin = string;

/** 🔗️ One direct plugin dependency — mirrors Rust `PluginDependency` (`🛂️manifest/🦀️.rs`). A
 * registry-derived edge carries no `version` (the pre-build view has none to read); a manifest-derived
 * edge carries its exact pin. */
export type PluginDependency = {
  readonly pluginId: string;
  readonly version?: VersionPin;
};""")
rep(KERNEL_TS, """   * extension) and mirrored by the builder's `.depends_on(id, VersionReq)`. A Cargo `[dependencies]`""", """   * extension) and mirrored by the builder's `.depends_on(id, VersionPin)`. A Cargo `[dependencies]`""")
rep(KERNEL_TS, """   * `VersionReq` travels with these (the registry's pre-build view has none to derive it from) —
   * `resolvePlaygroundBoot` maps each id to a `"*"` requirement, which is enough for""", """   * pin travels with these (the registry's pre-build view has none to derive it from) —
   * `resolvePlaygroundBoot` maps each id to a versionless edge, which is enough for""")
rep(KERNEL_TS, """readonly required: VersionReq; readonly actual: string }""", """readonly required: VersionPin; readonly actual: string }""")
block(
    KERNEL_TS,
    "function compareVersions(a: ParsedVersion, b: ParsedVersion): number {\n",
    "  return version.major === 0 && version.minor === 0 && version.patch === req.version.patch;\n}\n",
    """/** 🔢️ Parses the one pin form, `=X.Y.Z`. Mirrors Rust `VersionPin::parse`'s accepted syntax exactly: every
 * range (`*`, `^`, `~`, `>=`, a bare triple) is refused. */
function parseVersionPin(raw: VersionPin): ParsedVersion | null {
  const match = /^=(\\d+\\.\\d+\\.\\d+)$/.exec(raw.trim());
  return match ? parseVersion(match[1]) : null;
}

/** ✅️ True when `actual` (a plain `major.minor.patch` string) is exactly the pinned version. An
 * unparseable `actual`/`pin` — a range included — is unsatisfied and never throws, matching the "typed
 * error, never a panic" law the Rust planner's law tests hold `PlanError` to (contract freeze §1 law 4). */
export function versionSatisfies(actual: string, pin: VersionPin): boolean {
  const required = parseVersionPin(pin);
  const version = parseVersion(actual);
  return required !== null && version !== null && version.major === required.major && version.minor === required.minor && version.patch === required.patch;
}
""",
)
rep(KERNEL_TS, """ * once every missing/mismatched edge has already been reported). A node with no `version` skips the
 * version check for edges pointing at it (nothing to compare against) rather than failing closed.
 * Mirrors Rust `validate_dependency_graph`. */""", """ * once every missing/mismatched edge has already been reported). A node with no `version`, or an edge
 * with no pin (the registry's pre-build view), skips the version check (nothing to compare) rather than
 * failing closed. Mirrors Rust `validate_dependency_graph`. */""")
rep(KERNEL_TS, """      if (target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {""", """      if (dependency.version !== undefined && target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {""")

# ---------------------------------------------------------------- registry TS (pre-build edges carry ids only)
REG = f"{SDK}/📇️registry"
rep(f"{REG}/🎮️playground/🧭️session/🟦️.ts", """      dependencies: entry.dependsOn.map((pluginId) => ({ pluginId, version: "*" })),""", """      dependencies: entry.dependsOn.map((pluginId) => ({ pluginId })),""")
rep(f"{REG}/🎮️playground/🧭️session/🟦️.ts", """\\treadonly dependencies: readonly { readonly pluginId: string; readonly version: string }[];""", """\\treadonly dependencies: readonly { readonly pluginId: string }[];""")
rep(f"{REG}/📽️projection/🟦️.ts", """    const expectedDependencies = entry.dependsOn.map((pluginId) => ({ pluginId, version: "*" }));""", """    const expectedDependencies = entry.dependsOn.map((pluginId) => ({ pluginId }));""")
rep(f"{REG}/🌎️hub-source/🟦️.ts", """dependencies: entry.dependencies.map((pluginId) => ({ pluginId, version: "*" })) }));""", """dependencies: entry.dependencies.map((pluginId) => ({ pluginId })) }));""")

# ---------------------------------------------------------------- TS laws and vectors
BACKBONE = "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
rep(BACKBONE, """    it("validates a graph with every dependency present and version-satisfying", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "1.2.3" },
          { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([]);
    });""", """    it("validates a graph with every dependency present and exactly pinned", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "1.2.3" },
          { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "=1.2.3" }] },
          { pluginId: "c", dependencies: [{ pluginId: "a" }] },
        ]),
      ).toEqual([]);
    });""")
rep(BACKBONE, """      expect(validatePluginDependencyGraph([{ pluginId: "b", dependencies: [{ pluginId: "missing", version: "*" }] }])).toEqual([""", """      expect(validatePluginDependencyGraph([{ pluginId: "b", dependencies: [{ pluginId: "missing" }] }])).toEqual([""")
rep(BACKBONE, """          { pluginId: "b", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "^1.0.0", actual: "2.0.0" }]);""", """          { pluginId: "b", dependencies: [{ pluginId: "a", version: "=1.0.0" }] },
        ]),
      ).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "=1.0.0", actual: "2.0.0" }]);""")
rep(BACKBONE, """            { pluginId: "b", version: "*" },
            { pluginId: "c", version: "*" },""", """            { pluginId: "b" },
            { pluginId: "c" },""")
rep(BACKBONE, """        { pluginId: "c", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },""", """        { pluginId: "c", dependencies: [{ pluginId: "a" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a" }] },""")
rep(BACKBONE, """        { pluginId: "a", dependencies: [{ pluginId: "b", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },""", """        { pluginId: "a", dependencies: [{ pluginId: "b" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a" }] },""")
rep(BACKBONE, """    it("versionSatisfies matches the frozen grammar (*, =, ^, ~, >=), including caret's leading-zero tiers", async () => {
      const { versionSatisfies } = await import("@semio-tech/framework");
      expect(versionSatisfies("1.2.3", "*")).toBe(true);
      expect(versionSatisfies("1.2.3", "=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.4", "=1.2.3")).toBe(false);
      expect(versionSatisfies("1.9.0", "^1.2.3")).toBe(true);
      expect(versionSatisfies("2.0.0", "^1.2.3")).toBe(false);
      expect(versionSatisfies("0.2.9", "^0.2.3")).toBe(true);
      expect(versionSatisfies("0.3.0", "^0.2.3")).toBe(false);
      expect(versionSatisfies("0.0.9", "^0.0.3")).toBe(false);
      expect(versionSatisfies("0.0.3", "^0.0.3")).toBe(true);
      expect(versionSatisfies("1.2.9", "~1.2.3")).toBe(true);
      expect(versionSatisfies("1.3.0", "~1.2.3")).toBe(false);
      expect(versionSatisfies("1.2.3", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("9.9.9", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.2", ">=1.2.3")).toBe(false);
    });""", """    it("versionSatisfies admits only the exact pin and refuses every range", async () => {
      const { versionSatisfies } = await import("@semio-tech/framework");
      expect(versionSatisfies("1.2.3", "=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.4", "=1.2.3")).toBe(false);
      expect(versionSatisfies("1.2.2", "=1.2.3")).toBe(false);
      for (const range of ["*", "^1.2.3", "~1.2.3", ">=1.2.3", "1.2.3"]) expect(versionSatisfies("1.2.3", range)).toBe(false);
    });""")
rep(BACKBONE, """        { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a", moduleUrl: "a.js" },
        { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing", version: "*" }] },""", """        { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a" }] },
        { pluginId: "a", moduleUrl: "a.js" },
        { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing" }] },""")
BROADCAST = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts"
rep(BROADCAST, """        dependencies: [{ pluginId: "cad", version: "*" }],""", """        dependencies: [{ pluginId: "cad" }],""")
rep(BROADCAST, """, version: "*" }""", """ }""", 4)
rep("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx", """dependencies: [{ pluginId: "a", version: "*" }]""", """dependencies: [{ pluginId: "a" }]""", 3)

SATISFY = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/✅️satisfy-version-requirements"
block(
    f"{SATISFY}/🥒️.feature",
    "@capability-version-requirement-satisfaction\n",
    "      | 0.0.1   | >=0.0.0     |\n    Then the reference implementation and this repository agree on every pair\n",
    """@capability-version-requirement-satisfaction
@oracle-semver
@comparison-ordered-json-v1
Feature: Decide whether a version satisfies an exact dependency pin
  A manifest dependency is an exact pin `=X.Y.Z` and nothing else: one tree is one catalog, and a trusted
  catalog admits only exact pins inside its closure. `=X.Y.Z` is also a published semver comparator, so
  `semver` is a genuine oracle for it: for every pin, the two must agree on every version.

  Ranges (`*`, `^`, `~`, `>=`, a bare triple) are outside the pin grammar and refused by design — the
  refusal vectors live in `🚫️reject-malformed-version-input`, not here, because `semver` would satisfy
  them and asserting that would be measuring a deliberate divergence, not a bug.

  @id-exact-pins
  @level-fundamental
  @mode-differential
  Scenario: The exact pin across every component
    Given the version and requirement pairs
      | version | requirement |
      | 1.2.3   | =1.2.3      |
      | 1.2.4   | =1.2.3      |
      | 1.2.2   | =1.2.3      |
      | 1.3.3   | =1.2.3      |
      | 2.2.3   | =1.2.3      |
      | 0.1.0   | =0.1.0      |
      | 0.1.1   | =0.1.0      |
      | 0.0.0   | =0.0.0      |
      | 10.20.30 | =10.20.30  |
    Then the reference implementation and this repository agree on every pair
""",
)
rep(f"{SATISFY}/🟦️.ts", """const DIFFERENTIAL = ["exact-and-any", "caret-tiers", "tilde-and-at-least"];""", """const DIFFERENTIAL = ["exact-pins"];""")
REJECT = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🚫️reject-malformed-version-input"
rep(f"{REJECT}/🥒️.feature", """Feature: Reject malformed version input without raising
  Outside the frozen requirement grammar the contract is this repository's own: "unsatisfied, never a
  throw". No third-party matcher can adjudicate that — `semver` throws on some of these inputs and
  coerces others, which is a different, equally valid contract — so this is a recorded no-oracle case
  specified by the vectors below.""", """Feature: Reject malformed version input and every range without raising
  Outside the exact pin grammar `=X.Y.Z` the contract is this repository's own: "unsatisfied, never a
  throw". No third-party matcher can adjudicate that — `semver` throws on some of these inputs, coerces
  others and satisfies every range, which is a different, equally valid contract — so this is a
  recorded no-oracle case specified by the vectors below.""")
rep(f"{REJECT}/🥒️.feature", """      | 1.2.3.4 | ^1.2.3        |
    Then every pair reports unsatisfied without raising""", """      | 1.2.3.4 | ^1.2.3        |
      | 1.2.3   | *             |
      | 1.2.3   | ^1.2.3        |
      | 1.2.3   | ~1.2.3        |
      | 1.2.3   | >=1.2.3       |
      | 1.2.3   | 1.2.3         |
      | 1.2.3   | =1.2          |
    Then every pair reports unsatisfied without raising""")


def apply(write):
    texts, problems = {}, []
    for op in OPS:
        path = op[1]
        if path not in texts:
            full = ROOT / path
            if not full.exists():
                problems.append(f"missing file {path}")
                continue
            texts[path] = full.read_text()
        text = texts[path]
        if op[0] == "rep":
            _, _, old, new, count = op
            found = text.count(old)
            if found != count:
                problems.append(f"{path}: expected {count}× got {found}× for {old[:90]!r}")
                continue
            texts[path] = text.replace(old, new)
        else:
            _, _, start, end, new = op
            begin = text.find(start)
            stop = text.find(end, begin) if begin >= 0 else -1
            if begin < 0 or stop < 0 or text.count(start) != 1:
                problems.append(f"{path}: block anchors not unique/found ({start[:60]!r})")
                continue
            texts[path] = text[:begin] + new + text[stop + len(end):]
    for path, text in texts.items():
        if "VersionReq" in text and not path.endswith(".md"):
            problems.append(f"{path}: VersionReq still present after the set")
    print(f"ops={len(OPS)} files={len(texts)} problems={len(problems)}")
    for problem in problems:
        print("PROBLEM", problem)
    if problems:
        return 1
    if write:
        for path, text in texts.items():
            (ROOT / path).write_text(text)
        print("WRITTEN")
    else:
        for path in sorted(texts):
            print("OK", path)
    return 0


if __name__ == "__main__":
    sys.exit(apply("--write" in sys.argv))
