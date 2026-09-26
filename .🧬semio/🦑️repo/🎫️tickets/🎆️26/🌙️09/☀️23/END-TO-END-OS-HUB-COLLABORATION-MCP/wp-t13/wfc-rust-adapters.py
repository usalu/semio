#!/usr/bin/env python3
"""🦀️ Writes the SUBJECT-half Rust adapter of the four remaining wfc mutate cases from one template (F10)."""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts")
CASES = [
    ("🔲️grid2d", "🔲️mutate-grid2d-1", "wfc grid2d 1", "fourteen", "semio_s_artifact_wfc_grid2d", "grid2d", False),
    ("🖼️bitmap", "🧩️mutate-bitmap-1", "wfc bitmap 1", "ten", "semio_s_artifact_wfc_bitmap", "bitmap", True),
    ("🧊️3d", "🧩️mutate-wfc3d-1", "wfc3d 1", "fifteen", "semio_s_artifact_wfc_3d", "wfc3d", True),
    ("🧱️grid3d", "🧩️mutate-wfc-grid3d-1", "wfc grid3d 1", "fourteen", "semio_s_artifact_wfc_grid3d", "grid3d", False),
]
write = "--write" in sys.argv


def render(title, count, crate, prefix, round_trip):
    imports = f"use {crate}::standards::v1::subsets::any::schema::mutations::{{{prefix}_mutation_report_json{', ' + prefix + '_snapshot_json_round_trip' if round_trip else ''}}};"
    law_import = "    use semio_s_plugin_stdio_test_oracle::law::{self, vector::{self, Leaves}};" if round_trip else "    use semio_s_plugin_stdio_test_oracle::law::vector::{self, Leaves};"
    host_import = "    use semio_repo_test_host::{parse_json, Context, Outcome};" if round_trip else "    use semio_repo_test_host::{Context, Outcome};"
    rt_doc = " The `identity-round-trip` scenario decodes the committed\n//! before-snapshot through the production JSON codec and re-encodes it; the document must survive unchanged." if round_trip else ""
    rt_fn = f'''

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {{
        let uri = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| "the round-trip scenario names no committed snapshot".to_string())?;
        let text = String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{{uri}}: {{error}}"))?;
        let reencoded = {prefix}_snapshot_json_round_trip(&text)?;
        let reparsed = parse_json(&reencoded)?;
        law::round_trip_preserves(&reparsed, &parse_json(&text)?)?;
        Ok(Outcome::with_raw(reencoded.into_bytes(), reparsed))
    }}''' if round_trip else ""
    rt_reg = '.subject("identity-round-trip", subject::identity_round_trip)' if round_trip else ""
    return f'''//! 🦀️ {title} exhaustive mutation case — Rust adapter, the SUBJECT half.
//!
//! The oracle half is `🐍️.py` beside this file, an independent Python second implementation of the same {count}
//! kinds. This adapter replays each committed quintet the scenario's doc string addresses through this subset's
//! production codec and `Mutation` implementation (`{prefix}_mutation_report_json`) and asserts, in role, the laws of
//! `law::vector`: the applied snapshot is the committed after-snapshot, the produced delta is the committed `🔺️diff`,
//! the diagnostics are the committed `🎯️outcome`'s, the vector moves the document, and the mutation's own inverse
//! restores the before-snapshot. The parity phase then compares the snapshot it answers with the reference's.{rt_doc}
//!
//! @see ../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🧬️vector/🦀️.rs

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {{
{host_import}
{law_import}
    {imports}

    fn report(leaves: &Leaves) -> Result<String, String> {{
        {prefix}_mutation_report_json(&leaves.before, &leaves.mutation, &leaves.after)
    }}

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {{
        let leaves = Leaves::read(ctx)?;
        let applied = vector::mutate(ctx.row()?, &report(&leaves)?, &leaves.vector(true))?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }}

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {{
        let leaves = Leaves::read(ctx)?;
        let restored = vector::inverse(ctx.row()?, &report(&leaves)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }}{rt_fn}
}}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by Scenario Outline base id; each handler reads its kind from the row and its vector from the
/// row's doc string. The subject half is `sut`-gated so the oracle-only build never links the subset crate.
pub fn adapter() -> Adapter {{
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse){rt_reg};
    built
}}
//#endregion 🔖️Registration
'''


for artifact, case, title, count, crate, prefix, round_trip in CASES:
    path = ROOT / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests" / case / "🦀️.rs"
    text = render(title, count, crate, prefix, round_trip)
    if path.exists() and "pub fn adapter()" in path.read_text(encoding="utf-8"):
        print(f"problem: {path} already an adapter")
        sys.exit(1)
    print(("write " if write else "dry-run ") + str(path))
    if write:
        path.write_text(text, encoding="utf-8")
