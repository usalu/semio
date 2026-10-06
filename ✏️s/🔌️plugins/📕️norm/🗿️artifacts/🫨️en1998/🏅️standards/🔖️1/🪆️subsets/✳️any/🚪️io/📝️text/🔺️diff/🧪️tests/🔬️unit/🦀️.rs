use super::*;
use crate::mutations::En1998Mutation;
use crate::DeSeismicZone;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn update_site_diff_updates_zone() {
    let base = En1998Snapshot::default();
    let mut site = base.site.clone();
    site.seismic_zone = DeSeismicZone::Zone3;
    let mutation = En1998Mutation::UpdateSite(crate::mutations::update_site::UpdateSite { site: site.clone() });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.site = site;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
