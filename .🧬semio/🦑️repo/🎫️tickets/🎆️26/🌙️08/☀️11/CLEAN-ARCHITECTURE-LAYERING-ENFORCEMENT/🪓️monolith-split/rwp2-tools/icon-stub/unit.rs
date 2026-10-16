//! 🔁️ The bounded icon re-rasterisation a post-boot density change drives — ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k, closing packet W1g's gap 1 ("icon atlas does not
//! re-rasterise on a post-boot scale change"). The glyph atlas already re-rasterises lazily per
//! glyph; this is the icon half, paid across frames instead of in one resize step.

use super::*;
use semio_framework_canvas::{install_runtime_assets, RuntimeAsset, RuntimeAssets, RuntimeAssetsInstallError};
use std::sync::OnceLock;

/// 🔌️ Installs the repository's own runtime assets once per test process, the way a host does at boot; the atlas has no catalog source before that.
fn install_repository_pack() {
    static ASSETS: OnceLock<RuntimeAssets<'static>> = OnceLock::new();
    let assets = *ASSETS.get_or_init(|| {
        let directory = RuntimeAsset::IconPack.repository_directory();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|ancestor| ancestor.join(directory).is_dir()).expect("a repository above the crate manifest");
        RuntimeAssets::from_root(root).expect("runtime assets must be readable from the repository")
    });
    match install_runtime_assets(assets) {
        Ok(()) | Err(RuntimeAssetsInstallError::AlreadyInstalled) => {}
        Err(failure) => panic!("{failure}"),
    }
}

#[test]
fn a_rebuild_quantises_its_cell_to_the_same_ramp_the_one_shot_build_uses() {
    assert_eq!(IconAtlasRebuild::new(1.0).cell_size(), ICON_SIZE);
    assert_eq!(IconAtlasRebuild::new(2.0).cell_size(), ICON_SIZE * 2);
    assert_eq!(IconAtlasRebuild::new(3.0).cell_size(), ICON_SIZE * 3);
    assert_eq!(IconAtlasRebuild::new(4.0).cell_size(), ICON_SIZE * 3, "the fixed 2048 UV space with 16 columns caps the raster scale at 3×");
    assert_eq!(IconAtlasRebuild::new(1.4).cell_size(), ICON_SIZE, "a fractional factor rounds, so 1.4× keeps the 1× cell");
    assert_eq!(IconAtlasRebuild::new(f32::NAN).cell_size(), ICON_SIZE, "a non-finite factor degrades to 1× rather than panicking");
}

#[test]
fn a_rebuild_reports_its_own_scale_and_starts_with_nothing_rasterised() {
    install_repository_pack();
    let rebuild = IconAtlasRebuild::new(2.0);
    assert_eq!(rebuild.scale_factor(), 2.0);
    assert_eq!(rebuild.progress(), (0, icon_atlas_source_count()));
    assert!(icon_atlas_source_count() > ICON_RASTER_STEP_BUDGET, "the budget must actually split the work, or the job is pointless");
}

#[test]
fn each_step_spends_at_most_its_budget_and_the_last_one_answers_the_packed_atlas() {
    install_repository_pack();
    let total = icon_atlas_source_count();
    let mut rebuild = IconAtlasRebuild::new(1.0);
    let mut steps = 0;
    let atlas = loop {
        steps += 1;
        assert!(steps <= total + 1, "a rebuild must terminate in at most one step per source");
        match rebuild.step(ICON_RASTER_STEP_BUDGET) {
            IconAtlasRebuildStep::Rasterized(spent) => {
                assert!(spent <= ICON_RASTER_STEP_BUDGET, "a step never overspends its budget");
                assert!(rebuild.progress().0 <= total);
            }
            IconAtlasRebuildStep::Complete(atlas) => break atlas,
        }
    };
    assert!(steps > 1, "the whole atlas must not land in a single step — that is the interactive deadline this job exists to protect");
    assert_eq!(rebuild.progress(), (total, total));
    assert_eq!(atlas.width, ATLAS_COLS * ICON_SIZE);
    assert!(atlas.height >= ICON_SIZE);
    assert!(atlas.icon_uv("semio-logo").is_some(), "the logo is the source the one-shot build appends last");
}

#[test]
fn a_zero_budget_still_makes_progress_so_a_rebuild_can_never_stall() {
    install_repository_pack();
    let mut rebuild = IconAtlasRebuild::new(1.0);
    assert!(matches!(rebuild.step(0), IconAtlasRebuildStep::Rasterized(1)));
    assert_eq!(rebuild.progress().0, 1);
}

#[test]
fn a_rebuild_driven_to_completion_is_byte_identical_to_the_one_shot_build() {
    install_repository_pack();
    let one_shot = build_icon_atlas_scaled(1.0);
    let mut rebuild = IconAtlasRebuild::new(1.0);
    let stepped = loop {
        if let IconAtlasRebuildStep::Complete(atlas) = rebuild.step(7) {
            break atlas;
        }
    };
    assert_eq!((stepped.width, stepped.height), (one_shot.width, one_shot.height));
    assert_eq!(stepped.pixels, one_shot.pixels, "the stepped and one-shot paths are the SAME rasterisation, so a density change cannot drift from a boot");
}

#[test]
fn a_catalog_icon_is_rasterised_into_the_atlas_from_the_installed_pack() {
    install_repository_pack();
    let pack = icon_pack().expect("the installed icon pack");
    assert_eq!(icon_atlas_source_count(), pack.plain_len() + 1, "the atlas sources are the pack's catalog icons plus the logo");
    assert!(pack.plain("plus").is_some(), "the catalog icon the shell paints for :plus: is in the pack");
    let atlas = build_icon_atlas_scaled(1.0);
    let [u0, v0, u1, v1] = atlas.icon_uv("plus").expect("the catalog icon plus is in the atlas");
    let texture = ICON_ATLAS_TEXTURE_SIZE as f32;
    let (x0, y0, x1, y1) = ((u0 * texture).round() as u32, (v0 * texture).round() as u32, (u1 * texture).round() as u32, (v1 * texture).round() as u32);
    assert_eq!((x1 - x0, y1 - y0), (ICON_SIZE, ICON_SIZE));
    let visible = (y0..y1).flat_map(|y| (x0..x1).map(move |x| (x, y))).filter(|&(x, y)| atlas.pixels[((y * atlas.width + x) * 4 + 3) as usize] > 0).count();
    assert!(visible > 0, "the plus cell holds rasterised, non-transparent pixels");
    for id in pack.plain_ids().into_iter().take(8) {
        assert!(atlas.icon_uv(id).is_some(), "{id} is in the atlas");
    }
    assert!(atlas.icon_uv("semio-logo").is_some());
}
