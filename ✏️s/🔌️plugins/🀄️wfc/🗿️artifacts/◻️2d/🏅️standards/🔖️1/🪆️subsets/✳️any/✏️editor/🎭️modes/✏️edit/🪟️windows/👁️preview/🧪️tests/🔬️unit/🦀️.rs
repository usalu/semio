//! 🧪️ The `wfc-2d-preview` window — layer projection from an inferred assignment.

use super::{definition, preview_fit, preview_layers_json, render, WFC_2D_PREVIEW_BODY, WFC_2D_PREVIEW_WINDOW};
use crate::editor::wfc2d::transient::{Wfc2dAssignment, Wfc2dTransient};

#[test]
fn window_identity_is_the_declared_preview() {
    assert_eq!(WFC_2D_PREVIEW_WINDOW, "wfc-2d-preview");
    let definition = definition();
    assert_eq!(definition.body_key, WFC_2D_PREVIEW_BODY);
    assert!(definition.actions.is_empty(), "the preview is read-only");
}

/// 🖼️ Every slot contributes a bounds layer, solved or not — an unsolved board still renders.
#[test]
fn every_slot_paints_even_unsolved() {
    let document = crate::examples::hex_ring::document();
    let layers = preview_layers_json(&document, &Wfc2dTransient::default());
    for slot in &document.slots {
        assert!(layers.contains(&format!("\"slot-{}\"", slot.id)), "slot {} is missing from the preview", slot.id);
    }
    assert!(!layers.contains("\"kind\":\"path\""), "an unsolved, unpinned board paints no tile media");
}

/// 🎨 A solved slot additionally paints its tile's vector paths, mapped onto the slot rectangle.
#[test]
fn a_solved_slot_paints_its_tile_media() {
    let document = crate::examples::two_room_corridor::document();
    let transient = Wfc2dTransient { assignments: vec![Wfc2dAssignment { slot_id: "room-a".into(), tile_id: "room".into() }], contradiction: false };
    let layers = preview_layers_json(&document, &transient);
    assert!(layers.contains("tile-room-a-room-0"), "the solved tile's path layer is missing");
    let fit = preview_fit(&document);
    let slot = document.slots.iter().find(|slot| slot.id == "room-a").expect("the boot example declares room-a");
    let expected = format!("\"transform\":[{:.6},0.0,0.0,{:.6},{:.6},{:.6}]", slot.width * fit.scale, slot.height * fit.scale, fit.x(slot.x), fit.y(slot.y));
    assert!(layers.contains(&expected), "tile space must be mapped onto the FITTED slot rectangle, expected {expected}");
}

/// 🔭️ The whole board is centred on the world origin and spans the fit extent, whatever units the
/// example authored — the law that keeps a 0..6-unit board from rendering as a speck at zoom 1.
#[test]
fn the_fit_centres_every_example_on_the_origin() {
    for document in crate::examples::documents() {
        let fit = preview_fit(&document);
        let xs: Vec<f64> = document.slots.iter().flat_map(|slot| [fit.x(slot.x), fit.x(slot.x + slot.width)]).collect();
        let ys: Vec<f64> = document.slots.iter().flat_map(|slot| [fit.y(slot.y), fit.y(slot.y + slot.height)]).collect();
        let (min_x, max_x) = (xs.iter().copied().fold(f64::MAX, f64::min), xs.iter().copied().fold(f64::MIN, f64::max));
        let (min_y, max_y) = (ys.iter().copied().fold(f64::MAX, f64::min), ys.iter().copied().fold(f64::MIN, f64::max));
        assert!((min_x + max_x).abs() < 1e-6 && (min_y + max_y).abs() < 1e-6, "the fitted board is centred on the origin");
        assert!((max_x - min_x).max(max_y - min_y) > 600.0, "the fitted board fills the pane at the default camera");
    }
}

/// 📌️ A pinned slot paints even before a solve — the pin is authored, not inferred.
#[test]
fn a_pinned_slot_paints_without_a_solve() {
    let layers = preview_layers_json(&crate::examples::wall_roof_facade_strip::document(), &Wfc2dTransient::default());
    assert!(layers.contains("tile-bay-1-top-roof-0"));
}

#[test]
fn render_produces_a_surface_for_every_example() {
    for document in crate::examples::documents() {
        render(&document, &Wfc2dTransient::default(), 0.0, 0.0, 1.0).expect("the preview window renders");
    }
}
