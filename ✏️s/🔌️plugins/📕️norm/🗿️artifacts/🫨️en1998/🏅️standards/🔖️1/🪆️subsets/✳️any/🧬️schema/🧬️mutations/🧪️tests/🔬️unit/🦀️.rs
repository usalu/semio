use super::*;
use protocol::{Mutation, MutationDiff};

fn every_mutation() -> Vec<En1998Mutation> {
    crate::mutations::text::demo_mutation_cases()
}

#[semio_framework_async_macros::async_test]
async fn every_variant_has_diff_and_inverse() {
    let base = En1998Snapshot::default();
    for mutation in every_mutation() {
        let _ = mutation.diff(&base);
        let _ = mutation.inverse(&base);
    }
}

#[semio_framework_async_macros::async_test]
async fn update_site_applies_seismic_zone_enum() {
    use crate::DeSeismicZone;
    let base = En1998Snapshot::default();
    let mut site = base.site.clone();
    site.seismic_zone = DeSeismicZone::Zone3;
    let mutation = En1998Mutation::UpdateSite(update_site::UpdateSite { site: site.clone() });
    let outcome = mutation.diff(&base);
    let applied = outcome.diff().apply(&base).expect("apply");
    assert_eq!(applied.site.seismic_zone, DeSeismicZone::Zone3);
}
