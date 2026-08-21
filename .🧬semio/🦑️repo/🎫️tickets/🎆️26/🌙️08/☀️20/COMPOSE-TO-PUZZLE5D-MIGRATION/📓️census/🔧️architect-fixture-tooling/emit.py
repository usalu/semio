import json, os, re
R=json.load(open('fixtures.json'))
MROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

def snake(s): return s.replace('-','_')
def jdump(v): return json.dumps(v, ensure_ascii=False, indent=2)+"\n"
def esc(s): return s.replace('\\','\\\\').replace('"','\\"')

def what(d, r):
    p=r['plan']; i=r['info']; slug=p['entity']
    if p['cat']=='regular':
        coll=p['coll']
        if p['verb']=='create':
            return f"appends the payload row to `program.{coll}` as `added = [row]`, after a duplicate-id guard"
        if p['verb']=='delete':
            return f"drops `{i['rid']}` from `program.{coll}` as `removed = [\"{i['rid']}\"]`, after a target-missing guard"
        if p['verb']=='rename':
            return f"patches only `name` on `{i['rid']}` in `program.{coll}` — every other patch field stays `null`"
        return (f"patches `{i['rid']}` in `program.{coll}` with the FULL `{p['patch_type']}` that `Patchable::diff_patch` "
                f"snapshots off the payload row (this fixture moves `{p['changed_field']}`)")
    if p['cat']=='edge':
        if d=='🔗🧲connect-adjacency':
            return ("checks both endpoint elements exist, normalizes the pair, forces `normalized = true`, and — because no "
                    "edge yet joins `element-a`/`element-b` — takes the `added = [normalized edge]` branch")
        if d=='✂️🧲disconnect-adjacency':
            return "drops `adjacency-a` from `program.adjacencies` as `removed = [\"adjacency-a\"]`"
        if d=='🔗🧵connect-trace':
            return "finds no trace with this id and therefore takes the `added = [trace]` branch (endpoints are free-form, unchecked)"
        return "drops `trace-a` from `program.traces` as `removed = [\"trace-a\"]`"
    if p['cat']=='scalar':
        f=r['info']['field']
        if p['verb']=='rename':
            return (f"clones `base.{f}`, writes the single `{r['info']['key']}` field, and emits it as the whole "
                    f"`{f}` facet — the diff carries the complete replacement value, not a patch")
        return f"emits the payload value as the whole `{f}` facet verbatim"
    # child
    if r['applied']:
        return (f"reads the live `{r['info']['slot']}` rows off the working-scene cache, appends the payload row, and re-mints a "
                f"content-addressed `table` child handle whose `childId` is a hash of the row list's JSON")
    return (f"reads the live `{r['info']['slot']}` rows off the working-scene cache — which a fresh test process has never "
            f"populated — finds no `{r['info']['rid']}`, and rejects with `mutation.target-missing`")

def inverse_sentence(d, r):
    p=r['plan']
    if p['cat']=='regular':
        v={'create':'delete','delete':'create','rename':'rename','replace':'replace'}[p['verb']]
        return f"{v}-{p['entity']} (this leaf's recorded inverse) did not restore the before-snapshot"
    if d=='🔗🧲connect-adjacency': return "disconnect-adjacency (the inverse of a pair-creating connect) did not restore the before-snapshot"
    if d=='✂️🧲disconnect-adjacency': return "connect-adjacency (the inverse of a disconnect) did not restore the removed edge"
    if d=='🔗🧵connect-trace': return "disconnect-trace (the inverse of an id-creating connect) did not restore the before-snapshot"
    if d=='✂️🧵disconnect-trace': return "connect-trace (the inverse of a disconnect) did not restore the removed trace"
    if p['cat']=='scalar':
        return f"{p['kind']} back to the captured prior value did not restore the before-snapshot"
    return f"delete-{p['entity']} (this leaf's recorded inverse) did not restore the before-snapshot"

HEADER = '''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️component.rs`, which {what}.
//!
//! That leaf's own contract line reads: {doc}
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::artifacts::program::{{ProgramDiff, ProgramMutation, ProgramSnapshot}};
use protocol::{{Mutation, MutationDiff}};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
{diff_const}const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> ProgramSnapshot {{
    serde_json::from_str(BEFORE).expect("{label}: before snapshot decodes")
}}

fn expected_after() -> ProgramSnapshot {{
    serde_json::from_str(AFTER).expect("{label}: after snapshot decodes")
}}

fn mutation() -> ProgramMutation {{
    serde_json::from_str(MUTATION).expect("{label}: mutation decodes")
}}
'''

APPLIED = '''
/// ▶️ {kind} carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn {p}_applies_to_committed_after() {{
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("{label}: {kind} applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "{label}: applied state differs from the committed after-snapshot");
}}

/// ↩️ Applying {kind} and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn {p}_inverse_restores_before() {{
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base);
    undo.reverse();
    let mut state = forward.diff(&base).diff().apply(&base).expect("{label}: forward diff applies");
    for step in &undo {{
        state = step.diff(&state).diff().apply(&state).expect("{label}: inverse step applies");
    }}
    assert_eq!(state, base, "{label}: {inv}");
}}

/// 🔣️ Both committed snapshots and the committed {kind} payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn {p}_committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("{label}: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("{label}: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("{label}: snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} snapshot JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("{label}: {kind} payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("{label}: {kind} payload reparses");
    assert_eq!(reencoded, original, "{label}: committed {kind} payload JSON is not canonical");
}}

/// 🎯️ The declared outcome holds: {kind} applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn {p}_declared_outcome_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("{label}: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "{label}: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "{label}: {kind} raised a diagnostic on a fixture that declares a clean apply");
    assert!(outcome.diff().apply(&base).is_ok(), "{label}: {kind} was rejected by apply on its own before-snapshot");
}}

/// 🔺️ The sparse delta {kind} produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn {p}_produces_committed_diff() {{
    let produced = serde_json::to_value(mutation().diff(&before()).diff()).expect("{label}: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("{label}: committed diff decodes");
    assert_eq!(produced, committed, "{label}: the diff {kind} builds differs from the committed 🔺️diff/🔣️component.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn {p}_committed_diff_is_canonical() {{
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("{label}: committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("{label}: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("{label}: committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what {kind} does, not a summary.
#[semio_framework_async_macros::async_test]
async fn {p}_committed_diff_applies_to_after() {{
    let decoded: ProgramDiff = serde_json::from_str(DIFF).expect("{label}: committed diff decodes");
    let produced = decoded.apply(&before()).expect("{label}: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: the committed diff did not carry before to after");
}}
'''

REJECTED = '''
/// ▶️ {kind} is rejected here, so the committed after-snapshot is the before-snapshot unchanged.
#[semio_framework_async_macros::async_test]
async fn {p}_leaves_the_before_snapshot_untouched() {{
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = outcome.diff().apply(&base).expect("{label}: the empty rejection diff still applies");
    assert_eq!(applied, expected_after(), "{label}: a rejected {kind} must leave the snapshot exactly as committed");
}}

/// ↩️ A rejected {kind} has nothing to undo: its inverse is empty.
#[semio_framework_async_macros::async_test]
async fn {p}_has_an_empty_inverse() {{
    let base = before();
    assert!(mutation().inverse(&base).is_empty(), "{label}: {inv_reject}");
}}

/// 🔣️ Both committed snapshots and the committed {kind} payload are canonical.
#[semio_framework_async_macros::async_test]
async fn {p}_committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: ProgramSnapshot = serde_json::from_str(text).expect("{label}: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("{label}: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("{label}: snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} snapshot JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("{label}: {kind} payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("{label}: {kind} payload reparses");
    assert_eq!(reencoded, original, "{label}: committed {kind} payload JSON is not canonical");
}}

/// 🎯️ The declared rejection holds, down to the code and the offending path.
#[semio_framework_async_macros::async_test]
async fn {p}_declared_rejection_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("{label}: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "{label}: this fixture declares a rejected outcome");
    let outcome = mutation().diff(&before());
    let raised = outcome.messages().iter().find(|message| message.level >= protocol::diagnostic::Severity::Error).expect("{label}: {kind} must raise an Error-level message here");
    assert_eq!(raised.code.0, "{code}", "{label}: rejection code differs from the committed outcome");
    assert_eq!(raised.target, vec!["{rid}".to_string()], "{label}: rejection path differs from the committed outcome");
}}

/// 🔺️ A rejection never carries a diff: {kind} returns the default ProgramDiff untouched.
#[semio_framework_async_macros::async_test]
async fn {p}_produces_no_diff() {{
    let outcome = mutation().diff(&before());
    assert_eq!(outcome.diff(), &ProgramDiff::default(), "{label}: a rejected {kind} must not build any delta");
}}

/// 🚫️ The case carries the absent-diff marker instead of an invented empty patch (contract D6).
#[semio_framework_async_macros::async_test]
async fn {p}_carries_the_absent_diff_marker() {{
    assert!(ABSENT.is_empty(), "{label}: 🔺️diff/🚫️component.absent must be an empty marker file");
}}

/// 🪞 The committed after-snapshot repeats the before-snapshot byte for byte.
#[semio_framework_async_macros::async_test]
async fn {p}_after_snapshot_repeats_before() {{
    assert_eq!(BEFORE, AFTER, "{label}: a rejected case must commit an after-snapshot identical to its before-snapshot");
    assert_eq!(before(), expected_after(), "{label}: decoded before and after snapshots must be equal");
}}
'''

written=0
wiring=[]
for d, r in sorted(R.items()):
    p=r['plan']; kind=p['kind']; case=r['case']; label=f"{kind}/{case}"
    cdir=os.path.join(MROOT, d, "🧪️tests", case)
    for sub in ["📸️snapshot/⬅️before","📸️snapshot/➡️after","🦠️mutation","🔺️diff","🎯️outcome"]:
        os.makedirs(os.path.join(cdir, sub), exist_ok=True)
    open(os.path.join(cdir,"📸️snapshot/⬅️before/🔣️component.json"),"w").write(jdump(r['before']))
    open(os.path.join(cdir,"📸️snapshot/➡️after/🔣️component.json"),"w").write(jdump(r['after']))
    open(os.path.join(cdir,"🦠️mutation/🔣️component.json"),"w").write(jdump(r['mutation']))
    pref=snake(kind)
    if r['applied']:
        open(os.path.join(cdir,"🔺️diff/🔣️component.json"),"w").write(jdump(r['diff']))
        open(os.path.join(cdir,"🎯️outcome/🔣️component.json"),"w").write(jdump({"status":"applied"}))
        body=HEADER.format(kind=kind, case=case, what=what(d,r), doc=r['doc'], label=label,
                           diff_const='const DIFF: &str = include_str!("🔺️diff/🔣️component.json");\n')
        body+=APPLIED.format(kind=kind, label=label, p=pref, inv=inverse_sentence(d,r))
    else:
        open(os.path.join(cdir,"🔺️diff/🚫️component.absent"),"w").write("")
        rid=r['info']['rid']
        open(os.path.join(cdir,"🎯️outcome/🔣️component.json"),"w").write(jdump({"status":"rejected","code":"mutation.target-missing","path":[rid]}))
        body=HEADER.format(kind=kind, case=case, what=what(d,r), doc=r['doc'], label=label,
                           diff_const='const ABSENT: &str = include_str!("🔺️diff/🚫️component.absent");\n')
        body+=REJECTED.format(kind=kind, label=label, p=pref, code="mutation.target-missing", rid=rid,
                              inv_reject=f"a rejected {kind} must record no inverse step")
    open(os.path.join(cdir,"🦀️component.rs"),"w").write(body)
    wiring.append((d, case, pref))
    written+=1
json.dump(wiring, open('wiring.json','w'), ensure_ascii=False, indent=1)
print('wrote', written, 'cases')
