use super::*;
use crate::mutations::En1998Mutation;
use protocol::{Mutation as _};

#[semio_framework_async_macros::async_test]
async fn change_site_s_ap_r_diff_updates_plateau_acceleration() {
    let base = En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let mut site = base.site.clone();
    site.s_ap_r = 2.0;
    let mutation = En1998Mutation::ChangeSiteSApR(crate::mutations::change_site_s_ap_r::ChangeSiteSApR { new_s_ap_r: 2.0 });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.site = site;
    assert_eq!(protocol::apply_diff_cold(outcome.diff(), &base).expect("valid mutation diff"), expected);
}
