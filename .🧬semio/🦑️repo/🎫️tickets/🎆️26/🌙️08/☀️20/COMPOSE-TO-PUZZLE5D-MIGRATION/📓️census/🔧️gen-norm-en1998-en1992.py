#!/usr/bin/env python3
"""🔧️ Authoring aid for the norm en1992/en1998 mutation fixtures (ticket 26/08/20).

Emits the six committed files per test case. Every value in the tables below was transcribed by
hand from the artifact's own `📸️snapshot/🦀️component.rs` (Default impl) and each leaf's
`🔺️diff/🦀️component.rs` (which field it writes, which guards it runs). Nothing here is inferred
from a directory name.
"""
import json
import os
import re

ROOT = "/Users/ueli/Documents/semio"


def camel(snake: str) -> str:
    parts = snake.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])


def kebab(snake: str) -> str:
    return snake.replace("_", "-")


def num_kebab(text: str) -> str:
    return text.replace(".", "-").rstrip("-")


# ─────────────────────────────────────────────────────────────────────────────
# en1992 — 35 leaves. (field, kind, default-json, new-json, rust-new-literal)
# kinds: f (f64), b (bool), e (enum, rust literal given), s (String), u (u8)
EN1992_FIELDS = [
    ("annex", "e", "De", "En", "crate::document::AnnexChoice::En", "en"),
    ("m_ed_knm", "f", 120.0, 187.5, "187.5", None),
    ("v_ed_kn", "f", 80.0, 96.5, "96.5", None),
    ("f_ck", "f", 30.0, 45.0, "45.0", None),
    ("b_mm", "f", 300.0, 375.0, "375.0", None),
    ("d_mm", "f", 450.0, 512.5, "512.5", None),
    ("a_s_mm2", "f", 1200.0, 1608.5, "1608.5", None),
    ("f_yk", "f", 500.0, 550.0, "550.0", None),
    ("rho_l", "f", 0.01, 0.015625, "0.015625", None),
    ("n_ed_kn", "f", 0.0, 62.5, "62.5", None),
    ("p_kn", "f", 0.0, 45.5, "45.5", None),
    ("a_c_mm2", "f", 135000.0, 168750.0, "168750.0", None),
    ("use_fem", "b", False, True, "true", None),
    ("span_m", "f", 6.0, 7.5, "7.5", None),
    ("udl_kn_m", "f", 20.0, 26.25, "26.25", None),
    ("fire_rating", "e", "R60", "R120", "crate::artifacts::en1992::part_1_2::FireRating::R120", "r120"),
    ("provided_axis_distance_mm", "f", 30.0, 42.5, "42.5", None),
    ("bridge_sigma_c_mpa", "f", 12.0, 15.75, "15.75", None),
    ("bridge_delta_sigma_s_mpa", "f", 100.0, 132.5, "132.5", None),
    ("tightness_class", "e", "Tc1", "Tc2", "crate::artifacts::en1992::part_3::TightnessClass::Tc2", "tc2"),
    ("hd_over_h", "f", 10.0, 12.5, "12.5", None),
    ("liquid_sigma_s_mpa", "f", 200.0, 235.5, "235.5", None),
    ("liquid_rho_p_eff", "f", 0.01, 0.0078125, "0.0078125", None),
    ("liquid_f_ct_eff_mpa", "f", 2.9, 3.25, "3.25", None),
    ("liquid_e_s_mpa", "f", 200000.0, 205000.0, "205000.0", None),
    ("liquid_s_r_max_mm", "f", 250.0, 312.5, "312.5", None),
    ("anchor_h_ef_mm", "f", 80.0, 105.0, "105.0", None),
    ("anchor_cracked", "b", False, True, "true", None),
    ("anchor_f_uk_mpa", "f", 800.0, 900.0, "900.0", None),
    ("anchor_f_yk_mpa", "f", 640.0, 720.0, "720.0", None),
    ("anchor_a_s_mm2", "f", 84.3, 157.0, "157.0", None),
    ("anchor_d_mm", "f", 12.0, 16.0, "16.0", None),
    ("anchor_c1_mm", "f", 100.0, 137.5, "137.5", None),
    ("anchor_n_ed_kn", "f", 10.0, 22.5, "22.5", None),
    ("anchor_v_ed_kn", "f", 5.0, 11.25, "11.25", None),
]

EN1998_FIELDS = [
    ("seismic_zone", "u", 2, 4, "4", None),
    ("ground_type", "s", "b", "c", '"c"', None),
    ("importance_class", "s", "cc2", "cc3", '"cc3"', None),
    ("structural_system", "s", "moment_frame_dch", "wall_dcm", '"wall_dcm"', None),
    ("t1_s", "f", 0.3, 0.75, "0.75", None),
    ("mass_t", "f", 500.0, 812.5, "812.5", None),
    ("v_rd_kn", "f", 800.0, 925.0, "925.0", None),
    ("drift_mm", "f", 20.0, 33.5, "33.5", None),
    ("height_m", "f", 12.0, 18.75, "18.75", None),
    ("multiple_resisting_systems", "b", True, False, "false", None),
    ("annex", "s", "de", "en", '"en"', None),
    ("en_a_gr", "f", 0.15, 0.25, "0.25", None),
    ("en_ground_type", "s", "b", "e", '"e"', None),
    ("en_spectrum_type", "s", "type1", "type2", '"type2"', None),
    ("period_ratio", "f", 2.0, 3.5, "3.5", None),
    ("bridge_v_rd_kn", "f", 600.0, 725.0, "725.0", None),
    ("bearing_d_ed_mm", "f", 120.0, 165.5, "165.5", None),
    ("bearing_d_rd_mm", "f", 250.0, 312.5, "312.5", None),
    ("retrofit_knowledge_level", "s", "kl2", "kl3", '"kl3"', None),
    ("retrofit_limit_state", "s", "significant_damage", "near_collapse", '"near_collapse"', None),
    ("retrofit_e_d_kn", "f", 250.0, 337.5, "337.5", None),
    ("retrofit_r_k_kn", "f", 400.0, 512.5, "512.5", None),
    ("retrofit_gamma_el", "f", 1.0, 1.25, "1.25", None),
    ("silo_height_m", "f", 10.0, 14.5, "14.5", None),
    ("silo_radius_m", "f", 5.0, 6.25, "6.25", None),
    ("silo_n_rd_kn", "f", 500.0, 640.0, "640.0", None),
    ("silo_v_ed_kn", "f", 180.0, 225.5, "225.5", None),
    ("silo_v_rd_kn", "f", 300.0, 412.5, "412.5", None),
    ("silo_q_nominal", "f", 2.0, 2.75, "2.75", None),
    ("tank_height_m", "f", 8.0, 11.5, "11.5", None),
    ("tank_radius_m", "f", 4.0, 5.75, "5.75", None),
    ("tank_mass_t", "f", 300.0, 425.0, "425.0", None),
    ("tank_v_rd_kn", "f", 400.0, 537.5, "537.5", None),
    ("tower_m_ed_knm", "f", 1200.0, 1562.5, "1562.5", None),
    ("tower_m_rd_knm", "f", 2500.0, 2812.5, "2812.5", None),
    ("tower_is_chimney", "b", True, False, "false", None),
    ("tower_q_nominal", "f", 2.5, 3.25, "3.25", None),
    ("tower_mass_t", "f", 80.0, 112.5, "112.5", None),
    ("foundation_area_m2", "f", 100.0, 144.0, "144.0", None),
    ("foundation_p_rd_kpa", "f", 500.0, 625.0, "625.0", None),
    ("foundation_h_ed_kn", "f", 150.0, 212.5, "212.5", None),
    ("foundation_h_rd_kn", "f", 400.0, 475.0, "475.0", None),
    ("k_foundation", "f", 500000.0, 640000.0, "640000.0", None),
    ("k_soil", "f", 200000.0, 262500.0, "262500.0", None),
    ("wall_height_m", "f", 4.0, 5.5, "5.5", None),
    ("wall_phi_deg", "f", 30.0, 37.5, "37.5", None),
    ("wall_soil_gamma_kn_m3", "f", 18.0, 20.5, "20.5", None),
    ("wall_r", "f", 1.5, 2.25, "2.25", None),
    ("wall_h_rd_kn", "f", 150.0, 187.5, "187.5", None),
]

ARTIFACTS = {
    "en1992": {
        "dir": "📘️en1992",
        "Snapshot": "En1992Snapshot",
        "Diff": "En1992Diff",
        "Mutation": "En1992Mutation",
        "fields": EN1992_FIELDS,
        "family": "EN 1992 concrete-design",
    },
    "en1998": {
        "dir": "📘️en1998",
        "Snapshot": "En1998Snapshot",
        "Diff": "En1998Diff",
        "Mutation": "En1998Mutation",
        "fields": EN1998_FIELDS,
        "family": "EN 1998 seismic-design",
    },
}


def mutations_root(art: str) -> str:
    return os.path.join(
        ROOT,
        "✏️s/🔌️plugins/📕️norm/🗿️artifacts",
        ARTIFACTS[art]["dir"],
        "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
    )


def scan_leaves(art: str):
    """🔎️ Reads every leaf's own mutation/diff source: struct, payload field, written diff field,
    the guards it runs, and its semantic kind."""
    root = mutations_root(art)
    out = {}
    for entry in sorted(os.listdir(root)):
        mut_file = os.path.join(root, entry, "🦠️mutation/🦀️component.rs")
        diff_file = os.path.join(root, entry, "🔺️diff/🦀️component.rs")
        if not os.path.isfile(mut_file):
            continue
        msrc = open(mut_file, encoding="utf-8").read()
        dsrc = open(diff_file, encoding="utf-8").read()
        struct = re.search(r"pub struct ([A-Za-z0-9]+)", msrc).group(1)
        payload = re.search(r"pub (new_[a-z0-9_]+):", msrc).group(1)
        kind = re.search(r'kind: "([^"]+)"', msrc).group(1)
        written = re.search(r"Diff \{ ([a-z0-9_]+):", dsrc).group(1)
        finite = "is_finite" in dsrc
        out[written] = {
            "leafdir": entry,
            "struct": struct,
            "payload": payload,
            "kind": kind,
            "finite": finite,
        }
    return out


def json_dump(value) -> str:
    text = json.dumps(value, indent=2, ensure_ascii=False)
    return text + "\n"


def snapshot_obj(fields, override=None):
    obj = {}
    for name, knd, default, new, _lit, _slug in fields:
        obj[camel(name)] = new if (override == name) else default
    return obj


def diff_obj(fields, target):
    obj = {"artifact": None}
    for name, knd, default, new, _lit, _slug in fields:
        obj[camel(name)] = new if name == target else None
    obj["selectedCheckIndex"] = None
    return obj


def case_name(name, knd, default, new, slug):
    if knd == "b":
        return ("turns-%s-%s" % (kebab(name), "on" if new else "off"), "turns_on" if new else "turns_off")
    if knd in ("s", "e"):
        return ("switches-%s-to-%s" % (kebab(name), (slug or str(new)).replace("_", "-")), "switches")
    return ("raises-%s-to-%s" % (kebab(name), num_kebab(str(new))), "raises")


def value_words(knd, value, slug):
    if knd == "b":
        return "true" if value else "false"
    if knd == "e":
        return str(value)
    if knd == "s":
        return str(value)
    return repr(value) if isinstance(value, float) else str(value)


def rust_source(art, name, knd, default, new, lit, slug, leaf, sibling, case_dir):
    fnpfx = leaf["kind"].replace("-", "_")
    cfg = ARTIFACTS[art]
    snap, diffty, mutty = cfg["Snapshot"], cfg["Diff"], cfg["Mutation"]
    kind = leaf["kind"]
    label = "%s/%s" % (kind, case_dir)
    payload_camel = camel(leaf["payload"])
    field_camel = camel(name)
    dv = value_words(knd, default, None)
    nv = value_words(knd, new, slug)

    if knd == "b":
        applied_assert = 'assert!(%sapplied.%s, "%s: %s must read %s after the change");' % (
            "" if new else "!", name, label, name, nv)
        restored_assert = 'assert!(%srestored.%s, "%s: the inverse must put %s back to %s");' % (
            "" if default else "!", name, label, name, dv)
        after_assert = 'assert!(%sproduced.%s, "%s: the committed diff must leave %s reading %s");' % (
            "" if new else "!", name, label, name, nv)
        some = "Some(%s)" % nv
        diff_field_expr = "outcome.diff().%s" % name
        decoded_field_expr = "decoded.%s" % name
    elif knd == "s":
        applied_assert = 'assert_eq!(applied.%s, %s, "%s: %s must read %s after the change");' % (
            name, lit, label, name, nv)
        restored_assert = 'assert_eq!(restored.%s, "%s", "%s: the inverse must put %s back to %s");' % (
            name, default, label, name, default)
        after_assert = 'assert_eq!(produced.%s, %s, "%s: the committed diff must leave %s reading %s");' % (
            name, lit, label, name, nv)
        some = "Some(%s)" % lit
        diff_field_expr = "outcome.diff().%s.as_deref()" % name
        decoded_field_expr = "decoded.%s.as_deref()" % name
    else:
        applied_assert = 'assert_eq!(applied.%s, %s, "%s: %s must read %s after the change");' % (
            name, lit, label, name, nv)
        restored_assert = 'assert_eq!(restored.%s, %s, "%s: the inverse must put %s back to %s");' % (
            name, value_words(knd, default, None) if knd != "e" else lit.rsplit("::", 1)[0] + "::" + str(default),
            label, name, dv)
        after_assert = 'assert_eq!(produced.%s, %s, "%s: the committed diff must leave %s reading %s");' % (
            name, lit, label, name, nv)
        some = "Some(%s)" % lit
        diff_field_expr = "outcome.diff().%s" % name
        decoded_field_expr = "decoded.%s" % name

    guard_prose = (
        "an `is_finite` `mutation.invariant` guard and a `base.%s == payload.%s` `mutation.no-op` guard"
        % (name, leaf["payload"])
        if leaf["finite"]
        else "a `base.%s == payload.%s` `mutation.no-op` guard (this field is not numeric, so the leaf runs no `is_finite` invariant guard)"
        % (name, leaf["payload"])
    )
    no_message_prose = (
        "must raise neither the leaf's `mutation.invariant` nor its `mutation.no-op` message"
        if leaf["finite"]
        else "must raise no `mutation.no-op` message"
    )
    legacy = ""
    if leaf["leafdir"].endswith("set-snapshot"):
        legacy = (
            "//! ⚠️ This leaf still lives in the pre-migration `%s/` directory: `📦️glue.rs` path-includes\n"
            "//! that exact name, so the directory keeps it while its content is `%s`.\n" % (leaf["leafdir"], leaf["struct"])
        )

    return f'''//! 🧪️ `{kind}` fixture — `{case_dir}`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below was transcribed from this
//! leaf's own `../../🔺️diff/🦀️component.rs`, which writes `{diffty}.{name}` and nothing else,
//! behind {guard_prose}.
{legacy}//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from these files by `fixtures generate` and asserted by the codec matrix, never hand-forged here.

use crate::artifacts::{art}::diff::{diffty};
use crate::artifacts::{art}::mutations::{mutty};
use crate::artifacts::{art}::{snap};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> {snap} {{
    serde_json::from_str(BEFORE).expect("{kind} before-snapshot decodes")
}}
fn expected_after() -> {snap} {{
    serde_json::from_str(AFTER).expect("{kind} after-snapshot decodes")
}}
fn mutation() -> {mutty} {{
    serde_json::from_str(MUTATION).expect("{kind} mutation decodes")
}}

/// ▶️ `{kind}` carries the committed before-snapshot to the committed after-snapshot by moving
/// `{name}` from {dv} to {nv}, leaving every other {cfg["family"]} input alone.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_applies_to_committed_after() {{
    let (applied, messages) = vcs::apply_mutation(&before(), &mutation()).expect("{kind} applies to its committed before-snapshot");
    {applied_assert}
    assert_eq!(applied, expected_after(), "{label}: applied state differs from the committed after-snapshot");
    assert!(messages.is_empty(), "{label}: a real {dv} to {nv} change {no_message_prose}");
}}

/// ↩️ `{kind}` is its own inverse partner: the inverse step restores `{name}` to its pre-change
/// {dv} and nothing else has to be undone.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_inverse_restores_before() {{
    let base = before();
    let (forward, _messages) = vcs::apply_mutation(&base, &mutation()).expect("forward {kind} applies");
    let inverse = <{mutty} as protocol::Mutation<{snap}>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "{label}: a scalar change inverts with exactly one counter-mutation");
    let mut restored = forward;
    for step in &inverse {{
        let (next, _messages) = vcs::apply_mutation(&restored, step).expect("inverse {kind} step applies");
        restored = next;
    }}
    {restored_assert}
    assert_eq!(restored, base, "{label}: the inverse did not restore the committed before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed `{leaf["struct"]}` payload are already canonical:
/// decode then encode is a fixed point, so `{field_camel}` and `{payload_camel}` are spelled exactly
/// the way serde spells them.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_committed_json_is_canonical() {{
    for (side, text) in [("⬅️before", BEFORE), ("➡️after", AFTER)] {{
        let decoded: {snap} = serde_json::from_str(text).expect("{kind} snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("{kind} snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("{kind} snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} snapshot JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("{kind} mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("{kind} mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not the canonical externally-tagged {leaf["struct"]} form carrying {payload_camel}");
}}

/// 🎯️ The declared outcome holds: `{kind}` at {nv} is applied, not rejected, and carries no
/// diagnostic of its own.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_declared_outcome_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("{kind} outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "{label}: this fixture declares an applied outcome");
    let outcome = <{mutty} as protocol::Mutation<{snap}>>::diff(&mutation(), &before());
    assert!(outcome.messages().is_empty(), "{label}: moving {name} from {dv} to {nv} {no_message_prose}");
    assert!(vcs::apply_mutation(&before(), &mutation()).is_ok(), "{label}: an applied outcome must survive the diff-apply seam");
}}

/// 🔺️ The sparse delta `{kind}` produces is exactly the committed diff — the load-bearing
/// assertion: it pins that only `{field_camel}` is written, never the whole-artifact replacement
/// path and never a neighbouring input such as `{camel(sibling)}`.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_produces_committed_diff() {{
    let outcome = <{mutty} as protocol::Mutation<{snap}>>::diff(&mutation(), &before());
    assert_eq!({diff_field_expr}, {some}, "{label}: the diff must set {name} to {nv}");
    assert!(outcome.diff().artifact.is_none(), "{label}: a scalar change must never take the whole-artifact replacement path");
    assert!(outcome.diff().{sibling}.is_none(), "{label}: {kind} must leave {sibling} untouched");
    let produced = serde_json::to_value(outcome.diff()).expect("{kind} produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("{kind} committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️component.json");
}}

/// 🔣️ The committed diff is canonical and decodes back into `{diffty}` with `{field_camel}` set.
/// `selected_check_index` stays unset on purpose: it is an `Option<Option<u32>>` whose `None` and
/// `Some(None)` both encode as JSON `null`, so no fixture can pin the difference — and `{kind}`
/// never writes it anyway.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_committed_diff_is_canonical() {{
    let decoded: {diffty} = serde_json::from_str(DIFF).expect("{kind} committed diff decodes");
    assert_eq!({decoded_field_expr}, {some}, "{label}: the committed diff must carry {name} at {nv}");
    assert!(decoded.selected_check_index.is_none(), "{label}: the committed diff must leave the presence-lane selected_check_index unset");
    let reencoded = serde_json::to_value(&decoded).expect("{kind} committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("{kind} committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed after —
/// the {dv} to {nv} delta is a complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn {fnpfx}_committed_diff_applies_to_after() {{
    let decoded: {diffty} = serde_json::from_str(DIFF).expect("{kind} committed diff decodes");
    let produced = <{diffty} as protocol::MutationDiff<{snap}>>::apply(&decoded, &before()).expect("{kind} committed diff applies to the before-snapshot");
    {after_assert}
    assert_eq!(produced, expected_after(), "{label}: the committed diff did not carry before to after");
}}
'''


def main():
    summary = []
    for art, cfg in ARTIFACTS.items():
        leaves = scan_leaves(art)
        fields = cfg["fields"]
        names = [f[0] for f in fields]
        missing = [n for n in names if n not in leaves]
        extra = [n for n in leaves if n not in names]
        assert not missing and not extra, (art, missing, extra)
        root = mutations_root(art)
        wiring = []
        for index, (name, knd, default, new, lit, slug) in enumerate(fields):
            leaf = leaves[name]
            sibling = names[(index + 1) % len(names)]
            case_dir, mod_suffix = case_name(name, knd, default, new, slug)
            base = os.path.join(root, leaf["leafdir"], "🧪️tests", case_dir)
            for sub in ("📸️snapshot/⬅️before", "📸️snapshot/➡️after", "🦠️mutation", "🔺️diff", "🎯️outcome"):
                os.makedirs(os.path.join(base, sub), exist_ok=True)
            write(os.path.join(base, "📸️snapshot/⬅️before/🔣️component.json"), json_dump(snapshot_obj(fields)))
            write(os.path.join(base, "📸️snapshot/➡️after/🔣️component.json"), json_dump(snapshot_obj(fields, name)))
            write(os.path.join(base, "🦠️mutation/🔣️component.json"), json_dump({leaf["struct"]: {camel(leaf["payload"]): new}}))
            write(os.path.join(base, "🔺️diff/🔣️component.json"), json_dump(diff_obj(fields, name)))
            write(os.path.join(base, "🎯️outcome/🔣️component.json"), json_dump({"status": "applied"}))
            write(os.path.join(base, "🦀️component.rs"), rust_source(art, name, knd, default, new, lit, slug, leaf, sibling, case_dir))
            wiring.append((leaf["module"] if "module" in leaf else None, leaf["leafdir"], case_dir, mod_suffix, leaf["payload"]))
            summary.append("%s %s %s" % (art, leaf["leafdir"], case_dir))
        write_wiring(art, root, wiring)
    print("\n".join(summary))
    print(len(summary), "cases")


MODULES = {}


def write(path, text):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def write_wiring(art, root, wiring):
    glue = open(os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs"), encoding="utf-8").read()
    dir_to_module = {}
    for match in re.finditer(
        r'pub mod ([a-z0-9_]+) \{\s*\n\s*#\[path = "\.\./\.\./🗿️artifacts/📘️%s/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/([^/]+)/🔺️diff/🦀️component\.rs"\]'
        % art,
        glue,
    ):
        dir_to_module[match.group(2)] = match.group(1)
    lines = []
    for _m, leafdir, case_dir, mod_suffix, _payload in wiring:
        module = dir_to_module[leafdir]
        lines.append('    #[path = "%s/🧪️tests/%s/🦀️component.rs"]' % (leafdir, case_dir))
        lines.append("    mod tests_%s_%s;" % (module, mod_suffix))
    write(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧪️wiring-%s.snippet.rs" % art), "\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
