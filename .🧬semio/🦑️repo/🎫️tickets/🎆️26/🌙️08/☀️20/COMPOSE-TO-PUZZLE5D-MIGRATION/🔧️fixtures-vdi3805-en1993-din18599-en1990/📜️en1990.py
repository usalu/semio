"""📘️ EN 1990 — 10 hand-authored mutation fixture cases (6 applied + 4 rejected).

The four rejections are FORCED by this artifact's own composed-child design, not chosen for
convenience: `q_k` is an `s.stdio.semio.table` CHILD slot, and `en1990_qk` reads the live entry list
out of the session-side `EN1990_QK_SCRATCH` working-scene cache — a cache that a snapshot decoded
from committed JSON can never have seeded. Every index-addressed variable-action mutation therefore
sees an EMPTY list and can only answer `mutation.target-missing`. `insert-variable-action` is the one
index mutation that succeeds against an empty list, and its resulting handle is the content address
`en1990_qk_scene_id` mints — computed here with the repo's own pinned toolchain, not guessed.
"""
import copy
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

common = import_module("📜️common")
REPO = common.REPO

ROOT = os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")

# 🏷️ `En1990Mutation` is `#[serde(tag = "mutation", rename_all = "camelCase")]` — INTERNALLY tagged,
# so a payload encodes as `{"mutation":"changeResistance","new_resistance_kn":320.0}`: the camelCased
# VARIANT name under the `mutation` key (serde lowercases only the first character of a variant),
# with the payload struct's own fields flattened alongside. Only `ChangeAnnex` carries
# `#[serde(rename_all = "camelCase")]` of its own, so it alone spells its field `newAnnex`; the other
# nine payload structs keep snake_case field names.
QK_TARGET = {"artifactId": "en1990-qk", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"}}

# 🔗 `en1990_qk_scene_id(&[])` — the content address of the EMPTY variable-action list. Computed with
# the repo's pinned toolchain via `std::collections::hash_map::DefaultHasher` over
# `serde_json::to_string(&Vec::<En1990QkEntry>::new())` == "[]".
QK_EMPTY = "en1990-qk-7904dd65836c8ff4"
# 🔗 `en1990_qk_scene_id(&[En1990QkEntry { category: "Q_snow", value: 12.5 }])` — the content address
# of the one-entry list `insert-variable-action` produces, over `[{"category":"Q_snow","value":12.5}]`.
QK_ONE_SNOW = "en1990-qk-69c0017661d2372c"

BEFORE = {
    "gK": 45.0,
    "qK": {"childId": QK_EMPTY, "target": QK_TARGET},
    "resistanceKn": 250.0,
    "consequenceClass": 2,
    "annex": "De",
    "seismicAEdKn": 0.0,
}

DIFF_NULL = {
    "gK": None,
    "qK": None,
    "resistanceKn": None,
    "consequenceClass": None,
    "annex": None,
    "seismicAEdKn": None,
    "selectedCheckIndex": None,
}

APPLIED = {"status": "applied"}

APPLIED_CASES = []
REJECTED_CASES = []


def case(leaf, kind, module, name, summary, mutation, changes, extra_applied, extra_diff):
    after = copy.deepcopy(BEFORE)
    diff = copy.deepcopy(DIFF_NULL)
    for field, value in changes.items():
        assert field in BEFORE and BEFORE[field] != value, field
        after[field] = copy.deepcopy(value)
        diff[field] = copy.deepcopy(value)
    APPLIED_CASES.append(
        dict(leaf=leaf, kind=kind, module=module, case=name, summary=summary, after=after, mutation=mutation, diff=diff, extra_applied=extra_applied, extra_diff=extra_diff)
    )


def rejected(leaf, kind, module, name, summary, mutation, code, level, extra_rejected):
    REJECTED_CASES.append(
        dict(
            leaf=leaf,
            kind=kind,
            module=module,
            case=name,
            summary=summary,
            mutation=mutation,
            outcome={"status": "rejected", "code": code, "messages": [{"level": level, "code": code}]},
            extra_rejected=extra_rejected,
        )
    )


# ── 1. change-annex (lives in the 🐷set-snapshot leaf directory) ──────────────
case(
    "🐷set-snapshot",
    "change-annex",
    "change_annex",
    "switches-the-national-annex-from-de-to-en",
    "EN 1990's national-annex choice flips `De` → `En`, which is what selects the partial factors and "
    "psi values for every combination. The payload struct `ChangeAnnex` still lives in the "
    "`🐷set-snapshot` leaf directory that predates the semantic-vocabulary rename — the directory "
    "name is stale, the mutation is not.",
    {"mutation": "changeAnnex", "newAnnex": "En"},
    {"annex": "En"},
    '''    assert_eq!(snapshot.annex, crate::document::AnnexChoice::En, "change-annex/switches-the-national-annex-from-de-to-en: the annex must resolve to EN");
    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "change-annex/switches-the-national-annex-from-de-to-en: the composed q_k child handle must be identical, not re-minted");''',
    '''    assert_eq!(raised_diff.annex, Some(crate::document::AnnexChoice::En), "change-annex/switches-the-national-annex-from-de-to-en: the diff must publish annex = En");
    assert!(raised_diff.q_k.is_none(), "change-annex/switches-the-national-annex-from-de-to-en: only the variable-action mutations may write the q_k child slot");''',
)

# ── 2. change-permanent-action ────────────────────────────────────────────────
case(
    "🐐change-permanent-action",
    "change-permanent-action",
    "change_permanent_action",
    "raises-the-permanent-action-to-62-5-kn",
    "The characteristic permanent action G_k rises 45 → 62.5 kN. The builder's finiteness guard "
    "passes, the no-op guard does not fire, and a single-scalar diff is published — the resistance "
    "it will be checked against is a separate mutation.",
    {"mutation": "changePermanentAction", "new_g_k": 62.5},
    {"gK": 62.5},
    '''    assert_eq!(snapshot.g_k, 62.5, "change-permanent-action/raises-the-permanent-action-to-62-5-kn: G_k must be 62.5 kN");
    assert_eq!(snapshot.resistance_kn, before().resistance_kn, "change-permanent-action/raises-the-permanent-action-to-62-5-kn: the design resistance must not move with the action");''',
    '''    assert_eq!(raised_diff.g_k, Some(62.5), "change-permanent-action/raises-the-permanent-action-to-62-5-kn: the diff must publish gK = 62.5");
    assert!(raised_diff.seismic_a_ed_kn.is_none(), "change-permanent-action/raises-the-permanent-action-to-62-5-kn: the seismic accidental action must stay null");''',
)

# ── 3. change-resistance ──────────────────────────────────────────────────────
case(
    "🐘change-resistance",
    "change-resistance",
    "change_resistance",
    "raises-the-design-resistance-to-320-kn",
    "The design resistance R_d rises 250 → 320 kN. This is the right-hand side of every "
    "E_d <= R_d verification; none of the action inputs on the left-hand side is republished.",
    {"mutation": "changeResistance", "new_resistance_kn": 320.0},
    {"resistanceKn": 320.0},
    '''    assert_eq!(snapshot.resistance_kn, 320.0, "change-resistance/raises-the-design-resistance-to-320-kn: R_d must be 320 kN");
    assert_eq!(snapshot.g_k, before().g_k, "change-resistance/raises-the-design-resistance-to-320-kn: the permanent action must not move with the resistance");''',
    '''    assert_eq!(raised_diff.resistance_kn, Some(320.0), "change-resistance/raises-the-design-resistance-to-320-kn: the diff must publish resistanceKn = 320");
    assert!(raised_diff.g_k.is_none(), "change-resistance/raises-the-design-resistance-to-320-kn: gK must stay null");''',
)

# ── 4. change-consequence-class ───────────────────────────────────────────────
case(
    "🐑change-consequence-class",
    "change-consequence-class",
    "change_consequence_class",
    "escalates-the-building-from-cc2-to-cc3",
    "The consequence class goes CC2 → CC3, raising K_FI. This builder is the only one in EN 1990 with "
    "a RANGE invariant rather than a finiteness one — `!(1..=3).contains(&new)` is `mutation.invariant` "
    "(fatal); 3 is inside the range, so the change is published.",
    {"mutation": "changeConsequenceClass", "new_consequence_class": 3},
    {"consequenceClass": 3},
    '''    assert_eq!(snapshot.consequence_class, 3, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the consequence class must be CC3");
    assert_eq!(before().consequence_class, 2, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the committed before-snapshot must start at CC2");''',
    '''    assert_eq!(raised_diff.consequence_class, Some(3u8), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the diff must publish consequenceClass as the u8 3");
    assert!(raised_diff.annex.is_none(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the national annex choice must stay null");''',
)

# ── 5. change-seismic-action ──────────────────────────────────────────────────
case(
    "🦄change-seismic-action",
    "change-seismic-action",
    "change_seismic_action",
    "enables-the-seismic-situation-with-an-85-kn-a-ed",
    "The seismic accidental action A_Ed goes 0 → 85 kN. Zero is this field's documented "
    "`seismic situation disabled` sentinel, so this case is what turns Eq. 6.12b on; it also pins "
    "that a 0.0 -> 85.0 move clears the no-op equality guard.",
    {"mutation": "changeSeismicAction", "new_seismic_a_ed_kn": 85.0},
    {"seismicAEdKn": 85.0},
    '''    assert_eq!(snapshot.seismic_a_ed_kn, 85.0, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: A_Ed must be 85 kN");
    assert_eq!(before().seismic_a_ed_kn, 0.0, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the committed before-snapshot must start with the seismic situation disabled");''',
    '''    assert_eq!(raised_diff.seismic_a_ed_kn, Some(85.0), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the diff must publish seismicAEdKn = 85");
    assert!(raised_diff.consequence_class.is_none(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the consequence class must stay null");''',
)

# ── 6. insert-variable-action ─────────────────────────────────────────────────
case(
    "🐴insert-variable-action",
    "insert-variable-action",
    "insert_variable_action",
    "seeds-the-first-variable-action-q-snow-at-12-5-kn",
    "The one index-addressed variable-action mutation that succeeds against a freshly decoded "
    "snapshot: `en1990_qk` reads an EMPTY list out of the unseeded working-scene cache, index 0 "
    "clamps to 0 (so no `mutation.clamped` warning fires), and the resulting one-entry list is "
    "re-minted as a CONTENT-ADDRESSED child handle — `en1990-qk-69c0017661d2372c` is "
    "`DefaultHasher` over the JSON text of a one-entry list whose category is Q_snow and whose value is 12.5. The diff therefore touches the "
    "`q_k` child slot and nothing else; the twelve-and-a-half kilonewtons themselves never appear in "
    "the diff, only their address does.",
    {"mutation": "insertVariableAction", "index": 0, "category": "Q_snow", "value": 12.5},
    {"qK": {"childId": QK_ONE_SNOW, "target": QK_TARGET}},
    '''    assert_eq!(snapshot.q_k.child_id, "en1990-qk-69c0017661d2372c", "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the q_k handle must be the content address of the one-entry list");
    assert_ne!(snapshot.q_k.child_id, before().q_k.child_id, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: inserting must re-mint the handle, never reuse the empty-list address");
    assert_eq!(snapshot.q_k.target, before().q_k.target, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: only the content address moves — the child slot still targets the same table artifact");
    assert_eq!(crate::artifacts::en1990::en1990_qk(&snapshot).len(), 1, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the working-scene cache seeded by the diff builder must read back exactly one entry");''',
    '''    assert!(raised_diff.q_k.is_some(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the diff must publish the q_k child slot");
    assert_eq!(raised_diff.q_k.as_ref().map(|child| child.child_id.as_str()), Some("en1990-qk-69c0017661d2372c"), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the published handle must be the one-entry content address");
    assert!(raised_diff.g_k.is_none() && raised_diff.resistance_kn.is_none(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: adding a variable action must not restate the permanent action or the resistance");''',
)

# ── 7. remove-variable-action (REJECTED) ──────────────────────────────────────
rejected(
    "🐎remove-variable-action",
    "remove-variable-action",
    "remove_variable_action",
    "refuses-to-remove-action-0-from-an-unseeded-child-slot",
    "`remove-variable-action` addresses the `q_k` table BY INDEX, and `q_k` is a composed "
    "`s.stdio.semio.table` child slot whose live entries live only in the session-side "
    "`EN1990_QK_SCRATCH` working-scene cache. A snapshot decoded from committed JSON can never have "
    "seeded that cache, so `en1990_qk` fails soft to an EMPTY list and `0 >= 0` trips the "
    "`mutation.target-missing` guard. That is not an accident of this fixture — it is the documented "
    "cache-miss behaviour of the composed-child design, and it is exactly what this case pins.",
    {"mutation": "removeVariableAction", "index": 0},
    "mutation.target-missing",
    "error",
    '''    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: a refused removal must not re-mint the q_k handle");
    assert!(crate::artifacts::en1990::en1990_qk(&before()).is_empty(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the unseeded working-scene cache must read back an empty entry list — the reason index 0 is missing");
    assert!(<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation(), &before()).is_empty(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: removing an absent index has nothing to undo");''',
)

# ── 8. reorder-variable-actions (REJECTED) ────────────────────────────────────
rejected(
    "🐗reorder-variable-actions",
    "reorder-variable-actions",
    "reorder_variable_actions",
    "refuses-to-move-action-0-to-slot-1-in-an-empty-list",
    "`reorder-variable-actions` checks `from` against the live `q_k` length BEFORE it clamps `to`, so "
    "against the empty list an unseeded working-scene cache reads back, `from = 0` is already out of "
    "range and answers `mutation.target-missing`. The `to` clamp and the `mutation.no-op` "
    "already-in-place guard are never reached.",
    {"mutation": "reorderVariableActions", "from": 0, "to": 1},
    "mutation.target-missing",
    "error",
    '''    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "reorder-variable-actions/refuses-to-move-action-0-to-slot-1-in-an-empty-list: a refused reorder must not re-mint the q_k handle");
    assert!(<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation(), &before()).is_empty(), "reorder-variable-actions/refuses-to-move-action-0-to-slot-1-in-an-empty-list: an empty list has no reorder to undo");''',
)

# ── 9. change-variable-action-category (REJECTED) ─────────────────────────────
rejected(
    "🐮change-variable-action-category",
    "change-variable-action-category",
    "change_variable_action_category",
    "refuses-to-recategorise-a-missing-action-0",
    "`change-variable-action-category` looks the entry up with `q_k.get_mut(index)` before it "
    "compares categories, so against the empty list an unseeded working-scene cache reads back, the "
    "lookup misses and the builder answers `mutation.target-missing` with the index as its target "
    "address. The `mutation.no-op` same-category guard is never reached.",
    {"mutation": "changeVariableActionCategory", "index": 0, "new_category": "Q_wind"},
    "mutation.target-missing",
    "error",
    '''    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "change-variable-action-category/refuses-to-recategorise-a-missing-action-0: a refused recategorisation must not re-mint the q_k handle");
    assert!(<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation(), &before()).is_empty(), "change-variable-action-category/refuses-to-recategorise-a-missing-action-0: there is no previous category to restore");''',
)

# ── 10. change-variable-action-value (REJECTED) ───────────────────────────────
rejected(
    "🦌change-variable-action-value",
    "change-variable-action-value",
    "change_variable_action_value",
    "refuses-to-revalue-a-missing-action-0",
    "`change-variable-action-value` runs its finiteness invariant FIRST (18.75 kN is finite, so that "
    "guard passes), then looks the entry up by index. Against the empty list an unseeded "
    "working-scene cache reads back, the lookup misses and the builder answers "
    "`mutation.target-missing` — an `error`, not the `fatal` its invariant guard would have raised.",
    {"mutation": "changeVariableActionValue", "index": 0, "new_value": 18.75},
    "mutation.target-missing",
    "error",
    '''    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "change-variable-action-value/refuses-to-revalue-a-missing-action-0: a refused revaluation must not re-mint the q_k handle");
    assert!(<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation(), &before()).is_empty(), "change-variable-action-value/refuses-to-revalue-a-missing-action-0: there is no previous value to restore");''',
)

# ── emit ──────────────────────────────────────────────────────────────────────
assert len(APPLIED_CASES) == 6, len(APPLIED_CASES)
assert len(REJECTED_CASES) == 4, len(REJECTED_CASES)

for entry in APPLIED_CASES:
    rust = common.test_source(
        artifact="en1990",
        snapshot_ty="En1990Snapshot",
        diff_ty="En1990Diff",
        mutation_ty="En1990Mutation",
        kind=entry["kind"],
        case=entry["case"],
        summary=entry["summary"],
        extra_applied=entry["extra_applied"],
        extra_diff=entry["extra_diff"],
    )
    common.emit_case(ROOT, entry["leaf"], entry["case"], BEFORE, entry["after"], entry["mutation"], entry["diff"], APPLIED, rust)

for entry in REJECTED_CASES:
    rust = common.rejected_test_source(
        artifact="en1990",
        snapshot_ty="En1990Snapshot",
        diff_ty="En1990Diff",
        mutation_ty="En1990Mutation",
        kind=entry["kind"],
        case=entry["case"],
        summary=entry["summary"],
        extra_rejected=entry["extra_rejected"],
    )
    common.emit_rejected_case(ROOT, entry["leaf"], entry["case"], BEFORE, entry["mutation"], entry["outcome"], rust)

lines = []
for entry in APPLIED_CASES + REJECTED_CASES:
    lines.append('    #[path = "{}/🧪️tests/{}/🦀️component.rs"]'.format(entry["leaf"], entry["case"]))
    lines.append("    mod tests_{}_{};".format(entry["module"], entry["case"].replace("-", "_")))
print("\n".join(lines))
