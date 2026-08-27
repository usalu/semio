//! 🏗️ One-shot converter. Reads a pre-contract flat mutation fixture, decodes it into the
//! artifact's PRODUCTION types, runs production dispatch to obtain the diff and the outcome, and
//! writes the canonical 13-node bundle beside it. Nothing here re-implements a mutation: every
//! `after`, `diff` and `outcome` value comes out of the same code the runtime dispatches to.

use semio_framework_os_kernel as protocol;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// 📄️ The two pre-contract fixture shapes found in the tree.
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Legacy {
    /// `{schemaVersion, semanticKind, before, mutation, after}` — bmp, tiff, jpg, png.
    Flat { before: serde_json::Value, mutation: serde_json::Value, after: serde_json::Value },
    /// `{base, mutation, expected, inverse}` — pdf.
    Concrete { base: serde_json::Value, mutation: serde_json::Value, expected: serde_json::Value },
}

impl Legacy {
    fn parts(&self) -> (&serde_json::Value, &serde_json::Value, &serde_json::Value) {
        match self {
            Legacy::Flat { before, mutation, after } => (before, mutation, after),
            Legacy::Concrete { base, mutation, expected } => (base, mutation, expected),
        }
    }
}

fn write(path: &Path, value: &serde_json::Value) {
    fs::create_dir_all(path.parent().expect("a leaf has a parent")).expect("bundle directory is creatable");
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value).expect("bundle JSON encodes"))).expect("bundle leaf is writable");
}

/// 🧬️ Decomposes one legacy fixture into the canonical bundle for artifact `S`/`M`. The mutation is
/// driven through the `Mutation` trait the runtime itself dispatches on, so nothing here can drift
/// from production behaviour.
fn convert<S, M>(source: &Path, target: &Path) -> Result<String, String>
where
    S: DeserializeOwned + Serialize + Clone + PartialEq,
    M: protocol::Mutation<S>,
    <M as protocol::Mutation<S>>::Diff: Serialize,
{
    let text = fs::read_to_string(source).map_err(|error| format!("{}: {error}", source.display()))?;
    let legacy: Legacy = serde_json::from_str(&text).map_err(|error| format!("{}: unrecognised legacy shape: {error}", source.display()))?;
    let (before_json, mutation_json, after_json) = legacy.parts();
    let before: S = serde_json::from_value(before_json.clone()).map_err(|error| format!("{}: before does not decode: {error}", source.display()))?;
    let declared_after: S = serde_json::from_value(after_json.clone()).map_err(|error| format!("{}: after does not decode: {error}", source.display()))?;
    let mutation: M = serde_json::from_value(mutation_json.clone()).map_err(|error| format!("{}: mutation does not decode: {error}", source.display()))?;

    let mut produced = before.clone();
    let outcome = protocol::Mutation::diff(&mutation, &produced).apply_to(&mut produced);
    if produced != declared_after {
        return Err(format!("{}: production dispatch does not reproduce the committed after-state", source.display()));
    }
    let messages: Vec<(String, String)> = outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect();
    let status = if messages.is_empty() {
        "applied"
    } else if produced == before {
        "no-op"
    } else {
        "applied-with-diagnostics"
    };

    write(&target.join("📸️snapshot").join("⬅️before").join("🔣️component.json"), &serde_json::to_value(&before).expect("before re-encodes"));
    write(&target.join("📸️snapshot").join("➡️after").join("🔣️component.json"), &serde_json::to_value(&produced).expect("after re-encodes"));
    write(&target.join("🦠️mutation").join("🔣️component.json"), mutation_json);
    write(&target.join("🔺️diff").join("🔣️component.json"), &serde_json::to_value(outcome.diff()).expect("diff encodes"));
    let mut declared = serde_json::Map::new();
    declared.insert("status".to_string(), serde_json::Value::String(status.to_string()));
    if !messages.is_empty() {
        declared.insert(
            "messages".to_string(),
            serde_json::Value::Array(messages.iter().map(|(code, level)| serde_json::json!({ "code": code, "level": level })).collect()),
        );
    }
    write(&target.join("🎯️outcome").join("🔣️component.json"), &serde_json::Value::Object(declared));
    Ok(status.to_string())
}

macro_rules! artifact {
    ($name:literal, $root:expr, $snapshot:ty, $mutation:ty) => {
        ($name, $root, (convert::<$snapshot, $mutation>) as fn(&Path, &Path) -> Result<String, String>)
    };
}

use semio_s_plugin_stdio::artifacts as art;

fn main() {
    let repo = PathBuf::from(std::env::args().nth(1).expect("usage: convert <repo-root>"));
    let jobs: Vec<(&str, &str, fn(&Path, &Path) -> Result<String, String>)> = vec![
        artifact!(
            "pdf",
            "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
            art::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot,
            art::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation
        ),
        artifact!(
            "bmp",
            "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
            art::bmp::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot,
            art::bmp::standards::v_v3::subsets::any::schema::mutations::BmpMutation
        ),
        artifact!(
            "tiff",
            "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
            art::tiff::standards::v6_0::subsets::any::schema::snapshot::TiffSnapshot,
            art::tiff::standards::v6_0::subsets::any::schema::mutations::TiffMutation
        ),
        artifact!(
            "jpg",
            "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
            art::jpg::standards::v_jfif_1_01::subsets::any::schema::snapshot::JpgSnapshot,
            art::jpg::standards::v_jfif_1_01::subsets::any::schema::mutations::JpgMutation
        ),
        artifact!(
            "png",
            "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
            art::png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot,
            art::png::standards::v1_2::subsets::any::schema::mutations::PngMutation
        ),
    ];

    let mut failures = 0;
    for (name, root, run) in jobs {
        let mutations_root = repo.join(root);
        let leaves = match fs::read_dir(&mutations_root) {
            Ok(value) => value,
            Err(error) => {
                println!("{name}: {} unreadable: {error}", mutations_root.display());
                failures += 1;
                continue;
            }
        };
        for leaf in leaves.flatten() {
            if !leaf.path().is_dir() {
                continue;
            }
            let tests = leaf.path().join("🧪️tests");
            if !tests.is_dir() {
                continue;
            }
            // 🅰️ Flat shape: the fixture IS `🧪️tests/🔣️component.json`. 🅱️ Concrete shape: one
            // scenario directory holding a single `🔣️component.json`.
            let flat = tests.join("🔣️component.json");
            let targets: Vec<(PathBuf, PathBuf)> = if flat.is_file() {
                vec![(flat.clone(), tests.join("direct-behavior"))]
            } else {
                fs::read_dir(&tests)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter(|entry| entry.path().is_dir())
                    .map(|entry| (entry.path().join("🔣️component.json"), entry.path()))
                    .filter(|(source, _)| source.is_file())
                    .collect()
            };
            for (source, target) in targets {
                match run(&source, &target) {
                    Ok(status) => println!("{name}: {} → {status}", target.strip_prefix(&repo).unwrap_or(&target).display()),
                    Err(error) => {
                        println!("{name}: FAILED {error}");
                        failures += 1;
                    }
                }
            }
        }
    }
    if failures > 0 {
        println!("{failures} failure(s)");
        std::process::exit(1);
    }
}
