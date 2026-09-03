//! 🦀️ gis3d editor-config mutation case — Rust adapter. Covers the 2 kinds
//! `../../✏️editor/🎚️config/🧪️oracle/🔣️.json`'s `gis-gisterrain-1-config` catalog declares: `set-camera`,
//! `set-locale`. No third party implements this repository's own ephemeral editor state and none
//! could adjudicate it (`gis-gisterrain-config-mutation-semantics` no-oracle decision, same file),
//! so this case registers the SUBJECT role only — no `.oracle(...)` handler, matching
//! `os.config.opening`'s own precedent (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/
//! 🧪️tests/mutate-os-config-opening/🥒️.feature`).
//!
//! `gis3d_config_mutation_report_json` (`../../✏️editor/🎚️config/🦀️.rs`) is the whole surface this adapter needs —
//! every field of `Gis3dConfig` is a plain `String`, so this bridge never needs
//! `serde_json::from_str::<Gis3dConfig>` (unreachable from a `sut`-feature adapter crate, which
//! links `semio-s-plugin-gis` as an ordinary dependency, not under `cfg(test)`) — it reaches the
//! REAL, unconditional `Mutation<Gis3dConfig>`/`MutationDiff<Gis3dConfig>` trait chain directly and
//! reports `{base, snapshot, inverseSnapshot}` as a JSON string, parsed here through this host's own
//! dependency-free `parse_json`.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_gis::editor::gis3d::config::gis3d_config_mutation_report_json;

//#region 🔖️Kinds
const KINDS: &[&str] = &["set-camera", "set-locale"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{gis3d_config_mutation_report_json, parse_json, Context, Json, Outcome};

    fn report(ctx: &Context) -> Result<(Json, Json, Json), String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base_camera = spec.str("baseCameraJson");
        let base_locale = spec.str("baseLocale");
        let value = spec.str("value");
        let text = gis3d_config_mutation_report_json(&base_camera, &base_locale, &kind, &value)?;
        let parsed = parse_json(&text)?;
        let base = parsed.get("base").cloned().ok_or("report carries no base")?;
        let snapshot = parsed.get("snapshot").cloned().ok_or("report carries no snapshot")?;
        let inverse_snapshot = parsed.get("inverseSnapshot").cloned().ok_or("report carries no inverseSnapshot")?;
        Ok((base, snapshot, inverse_snapshot))
    }

    /// ✍️ The forward mutation: the resulting record must genuinely differ from the base record —
    /// a mutation whose projection does not move is refused, not reported as a pass.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (base, snapshot, _) = report(ctx)?;
        if snapshot == base {
            return Err("gis3d config mutation produced no observable change".to_string());
        }
        let raw = format!("{snapshot:?}").into_bytes();
        Ok(Outcome::with_raw(raw, snapshot))
    }

    /// ↩️ The inverse law: the kind's own computed inverse (`gis3d_config_mutation_report_json`'s
    /// own `inverseSnapshot`, produced by applying `Mutation::inverse`'s own steps) must restore the
    /// exact pre-mutation record, field for field.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (base, _, inverse_snapshot) = report(ctx)?;
        if inverse_snapshot != base {
            return Err(format!("inverse did not restore the base record: {inverse_snapshot:?} != {base:?}"));
        }
        let raw = format!("{inverse_snapshot:?}").into_bytes();
        Ok(Outcome::with_raw(raw, inverse_snapshot))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. No `.oracle(...)` registration — this
/// vocabulary carries a `noOracleDecisions` entry, not an independent reference implementation, so
/// the runner executes the subject role only, exactly as `os.config.opening`'s own case does.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = KINDS;
    }
    built
}
//#endregion 🔖️Registration
