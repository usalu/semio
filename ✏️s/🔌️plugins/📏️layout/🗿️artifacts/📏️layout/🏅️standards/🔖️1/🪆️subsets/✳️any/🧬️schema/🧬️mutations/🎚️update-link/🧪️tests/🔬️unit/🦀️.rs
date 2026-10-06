use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_link_raises_resolution_and_switches_the_print_profile() {
    let mut base = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    base.print_target = Some("print".into());
    let link = &mut base.links[0];
    link.state = None;
    link.hash = "sha256:abc".into();
    link.dpi = 72;
    link.color_profile = Some("RGB".into());
    let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let labels = crate::editor::layout::terminology::layout_labels(&view);
    let before = crate::editor::layout::panels::preflight::run_layout_preflight(&base, labels);
    assert!(before.iter().any(|issue| issue.code == "asset.low_resolution"));
    assert!(before.iter().any(|issue| issue.code == "asset.rgb_in_print"));
    let mutation = LayoutMutation::UpdateLink(UpdateLink { id: "link-missing".into(), width: 100, height: 100, dpi: 300, color_profile: Some("CMYK".into()) });
    let next = mutation.diff(&base).diff().apply(&base).expect("link applies");
    assert_eq!(next.links[0].dpi, 300);
    assert_eq!(next.links[0].color_profile.as_deref(), Some("CMYK"));
    let after = crate::editor::layout::panels::preflight::run_layout_preflight(&next, labels);
    assert!(after.iter().all(|issue| issue.code != "asset.low_resolution"));
    assert!(after.iter().all(|issue| issue.code != "asset.rgb_in_print"));
    let restored = mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(restored.links[0].dpi, 72);
    assert_eq!(restored.links[0].color_profile.as_deref(), Some("RGB"));
}
