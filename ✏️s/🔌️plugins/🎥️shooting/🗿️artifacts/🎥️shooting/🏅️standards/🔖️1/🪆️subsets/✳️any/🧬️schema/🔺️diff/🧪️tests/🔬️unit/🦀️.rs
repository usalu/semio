use super::*;
use crate::ShootingSceneLighting;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};

fn asset(id: &str) -> ShootingAsset {
    ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None }
}

fn snapshot(ids: &[&str]) -> ShootingSnapshot {
    ShootingSnapshot { assets: ids.iter().map(|id| asset(id)).collect(), ..ShootingSnapshot::default() }
}

fn ids(snapshot: &ShootingSnapshot) -> Vec<&str> {
    snapshot.assets.iter().map(|asset| asset.id.as_str()).collect()
}

fn rename(id: &str, name: &str) -> ShootingDiff {
    ShootingDiff::asset_patches([(id.to_string(), ShootingAssetPatch { name: Some(name.into()), ..Default::default() })])
}

fn recolor(id: &str, url: &str) -> ShootingDiff {
    ShootingDiff::asset_patches([(id.to_string(), ShootingAssetPatch { url: Some(url.into()), ..Default::default() })])
}

/// ⚖️ LAW: an empty diff is a no-operation on the snapshot.
#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::empty_shooting_snapshot();
    let diff = ShootingDiff::default();
    assert_eq!(protocol::apply_diff(&diff, &base).expect("valid mutation diff"), base);
    assert!(DiffAlgebra::<ShootingSnapshot>::is_empty(&diff));
}

/// 🧩 Structural edits run in order: an insert lands at its index, a move lands at its clamped destination, a remove drops the row.
#[semio_framework_async_macros::async_test]
async fn edits_run_in_order() {
    let base = snapshot(&["a", "b", "c"]);
    let mut diff = ShootingDiff::asset_edit(ShootingEdit::Add { index: 1, item: asset("x") });
    diff.absorb(ShootingDiff::asset_edit(ShootingEdit::Move { id: "c".into(), from: 3, to: 0 }));
    diff.absorb(ShootingDiff::asset_edit(ShootingEdit::Remove { id: "b".into(), index: 3 }));
    assert_eq!(ids(&protocol::apply_diff(&diff, &base).expect("diff applies")), vec!["c", "a", "x"]);
}

/// 🚫️ Malformed edits are rejected with a typed error: a duplicate insert, an out-of-range insert, a missing move or remove, a repeated patch.
#[semio_framework_async_macros::async_test]
async fn malformed_edits_are_rejected() {
    let base = snapshot(&["a", "b"]);
    for diff in [
        ShootingDiff::asset_edit(ShootingEdit::Add { index: 0, item: asset("a") }),
        ShootingDiff::asset_edit(ShootingEdit::Add { index: 3, item: asset("x") }),
        ShootingDiff::asset_edit(ShootingEdit::Move { id: "ghost".into(), from: 0, to: 0 }),
        ShootingDiff::asset_edit(ShootingEdit::Remove { id: "ghost".into(), index: 0 }),
        ShootingDiff::asset_edit(ShootingEdit::Remove { id: "a".into(), index: 1 }),
        ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 1, to: 0 }),
        ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 0, to: 5 }),
        ShootingDiff { assets: Some(ShootingAssetsDelta { edits: Vec::new(), patched: vec![ShootingPatchEntry { id: "a".into(), patch: ShootingAssetPatch::default() }, ShootingPatchEntry { id: "a".into(), patch: ShootingAssetPatch::default() }] }), ..Default::default() },
        rename("ghost", "x"),
    ] {
        assert!(protocol::apply_diff(&diff, &base).is_err(), "{diff:?} must be rejected");
    }
}

/// ➕️ Absorb coalesces same-key rows: add∘remove cancels (and drops the row's patches), patch∘patch merges per field, move∘move keeps the last, move∘remove keeps the remove, remove∘add stays a replace.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_rows_of_one_key() {
    let mut created = ShootingDiff::asset_edit(ShootingEdit::Add { index: 2, item: asset("x") });
    created.absorb(rename("x", "Renamed"));
    created.absorb(ShootingDiff::asset_edit(ShootingEdit::Remove { id: "x".into(), index: 2 }));
    assert_eq!(created.assets, Some(ShootingAssetsDelta::default()), "create∘delete leaves nothing");

    let mut patched = rename("a", "First");
    patched.absorb(recolor("a", "/mesh/second.glb"));
    patched.absorb(rename("a", "Third"));
    assert_eq!(
        patched.assets.expect("assets delta").patched,
        vec![ShootingPatchEntry { id: "a".into(), patch: ShootingAssetPatch { name: Some("Third".into()), url: Some("/mesh/second.glb".into()), ..Default::default() } }],
        "patch∘patch is one patch, the later field wins"
    );

    let mut moved = ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 0, to: 2 });
    moved.absorb(ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 2, to: 1 }));
    assert_eq!(moved.assets.expect("assets delta").edits, vec![ShootingEdit::Move { id: "a".into(), from: 0, to: 1 }], "move∘move is one move from the first source to the last destination");

    let mut returned = ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 0, to: 2 });
    returned.absorb(ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 2, to: 0 }));
    assert!(returned.assets.expect("assets delta").edits.is_empty(), "a move and its mirror vanish");

    let mut dropped = ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 0, to: 2 });
    dropped.absorb(ShootingDiff::asset_edit(ShootingEdit::Remove { id: "a".into(), index: 2 }));
    assert_eq!(dropped.assets.expect("assets delta").edits, vec![ShootingEdit::Remove { id: "a".into(), index: 0 }], "move∘remove removes from the first source");

    let mut replaced = ShootingDiff::asset_edit(ShootingEdit::Remove { id: "a".into(), index: 0 });
    replaced.absorb(ShootingDiff::asset_edit(ShootingEdit::Add { index: 0, item: asset("a") }));
    assert_eq!(replaced.assets.expect("assets delta").edits.len(), 2, "remove∘add stays a replace");
}

/// ➕️ LAW: `absorb(d1, d2)` applies like `d1` then `d2`, over every pair of structural and patch rows this vocabulary builds.
#[semio_framework_async_macros::async_test]
async fn absorb_equals_sequential_application() {
    let base = snapshot(&["a", "b", "c"]);
    let rows = |after: fn(&mut ShootingDiff)| {
        let mut diff = ShootingDiff::default();
        after(&mut diff);
        diff
    };
    let first = [
        rows(|d| d.absorb(rename("a", "Renamed"))),
        rows(|d| d.absorb(ShootingDiff::asset_edit(ShootingEdit::Move { id: "c".into(), from: 2, to: 0 }))),
        rows(|d| d.absorb(ShootingDiff::asset_edit(ShootingEdit::Remove { id: "b".into(), index: 1 }))),
        rows(|d| d.absorb(ShootingDiff::asset_edit(ShootingEdit::Add { index: 1, item: asset("x") }))),
    ];
    for d1 in &first {
        let mid = protocol::apply_diff(d1, &base).expect("first diff applies");
        let seconds = [rename("a", "Again"), recolor("c", "/mesh/c2.glb"), ShootingDiff::asset_edit(ShootingEdit::Move { id: "a".into(), from: 0, to: 1 }), ShootingDiff::asset_edit(ShootingEdit::Remove { id: "a".into(), index: 0 })];
        for d2 in seconds {
            if protocol::apply_diff(&d2, &mid).is_ok() {
                assert_mutation_diff_absorb_law(&base, d1.clone(), d2).await;
            }
        }
    }
}

/// 🔁️ LAW: the negative delta applied after the diff restores the base, for every structural and patch row, scene fields and scalars included.
#[semio_framework_async_macros::async_test]
async fn the_negative_delta_restores_the_base() {
    let mut base = snapshot(&["a", "b", "c"]);
    base.active_asset_id = "a".into();
    let mut diff = ShootingDiff::asset_edit(ShootingEdit::Remove { id: "b".into(), index: 1 });
    diff.absorb(ShootingDiff::asset_edit(ShootingEdit::Add { index: 0, item: asset("x") }));
    diff.absorb(ShootingDiff::asset_edit(ShootingEdit::Move { id: "c".into(), from: 2, to: 0 }));
    diff.absorb(rename("a", "Renamed"));
    diff.absorb(rename("x", "New"));
    diff.absorb(ShootingDiff { scene: Some(ShootingScenePatch { sun_enabled: Some(true), material_roughness: Some(0.5), ..Default::default() }), active_asset_id: Some("c".into()), ..Default::default() });
    assert_diff_algebra_inverse_law::<ShootingSnapshot, ShootingDiff>(&base, &diff).await;
}

