//! 🧪️ Keyed-diff algebra of Vdi3805: map-entry coalescing, nested connection rows, lockstep index rows, the negative diff and the state delta.

use super::Vdi3805Diff;
use crate::mutations::add_curve::AddCurve;
use crate::mutations::add_geometry_connection::AddGeometryConnection;
use crate::mutations::add_product::AddProduct;
use crate::mutations::change_curve_points::ChangeCurvePoints;
use crate::mutations::change_edition_profile::ChangeEditionProfile;
use crate::mutations::remove_curve::RemoveCurve;
use crate::mutations::remove_product::RemoveProduct;
use crate::mutations::Vdi3805Mutation;
use crate::{EditionProfileChoice, Vdi3805Snapshot};
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn diff_of(mutation: &Vdi3805Mutation, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

async fn law(base: &Vdi3805Snapshot, first: &Vdi3805Mutation, second: impl Fn(&Vdi3805Snapshot) -> Vdi3805Mutation) -> Vdi3805Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

fn curve(base: &Vdi3805Snapshot) -> crate::CharacteristicCurve {
    let mut curve = base.curves.values().next().expect("curve").clone();
    curve.id = "curve-created".into();
    curve
}

#[semio_framework_async_macros::async_test]
async fn point_patches_on_one_curve_coalesce_to_the_last_value() {
    let base = Vdi3805Snapshot::default();
    let id = base.curves.keys().next().expect("curve").clone();
    let set = |scale: f64| {
        let mut points = base.curves[&id].points.clone();
        points.iter_mut().for_each(|point| point.y *= scale);
        Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id: id.clone(), new_points: points })
    };
    let sum = law(&base, &set(2.0), |_| set(3.0)).await;
    assert_eq!(sum.curves.expect("curves").modified.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn add_then_remove_cancels_and_remove_then_add_replaces() {
    let base = Vdi3805Snapshot::default();
    let created = curve(&base);
    let sum = law(&base, &Vdi3805Mutation::AddCurve(AddCurve { curve: created.clone() }), |_| Vdi3805Mutation::RemoveCurve(RemoveCurve { id: created.id.clone() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let existing = base.curves.values().next().expect("curve").clone();
    let sum = law(&base, &Vdi3805Mutation::RemoveCurve(RemoveCurve { id: existing.id.clone() }), |_| Vdi3805Mutation::AddCurve(AddCurve { curve: existing.clone() })).await;
    assert_eq!(protocol::apply_diff(&sum, &base).expect("replace"), base);
}

#[semio_framework_async_macros::async_test]
async fn a_connection_replaced_twice_keeps_one_added_connection() {
    let base = Vdi3805Snapshot::default();
    let id = base.geometry.keys().next().expect("geometry").clone();
    let mut connection = base.geometry[&id].connections[0].clone();
    connection.id = "connection-created".into();
    let mut moved = connection.clone();
    moved.medium = "steam".into();
    let sum = law(&base, &Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: id.clone(), connection, index: None }), |_| Vdi3805Mutation::AddGeometryConnection(AddGeometryConnection { id: id.clone(), connection: moved.clone(), index: None })).await;
    let patches = sum.geometry.expect("geometry").modified;
    assert_eq!(patches.len(), 1);
    let rows = patches[0].connections.as_ref().expect("connections");
    assert_eq!(rows.added.len(), 1);
    assert!(rows.removed.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_new_edition_choice_is_added_then_changed_in_place() {
    let base = Vdi3805Snapshot::default();
    let sheet = "sheet-created".to_string();
    let set = |choice: EditionProfileChoice| Vdi3805Mutation::ChangeEditionProfile(ChangeEditionProfile { sheet: sheet.clone(), new_choice: choice });
    let sum = law(&base, &set(EditionProfileChoice::Legacy), |_| set(EditionProfileChoice::Current)).await;
    let rows = sum.edition_profile.expect("edition profile");
    assert_eq!((rows.added.len(), rows.modified.len()), (1, 0));
    assert_eq!(rows.added[0].value, EditionProfileChoice::Current);
}

#[semio_framework_async_macros::async_test]
async fn product_rows_and_index_rows_move_in_lockstep() {
    let base = Vdi3805Snapshot::default();
    let mut product = base.catalog.products[0].clone();
    product.id = "VLV-50-002".into();
    product.identity.article_number = "VLV-50-002".into();
    let mutation = Vdi3805Mutation::AddProduct(AddProduct { product: product.clone(), index: Some(0) });
    let forward = diff_of(&mutation, &base);
    assert!(forward.products.is_some() && forward.index_entries.is_some());
    let after = protocol::apply_diff(&forward, &base).expect("apply");
    assert_eq!(protocol::apply_diff(&forward.inverse(&base), &after).expect("inverse"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let sum = law(&base, &mutation, |_| Vdi3805Mutation::RemoveProduct(RemoveProduct { id: product.identity.article_number.clone() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<Vdi3805Snapshot, Vdi3805Diff>(&base, &after).await;
}
