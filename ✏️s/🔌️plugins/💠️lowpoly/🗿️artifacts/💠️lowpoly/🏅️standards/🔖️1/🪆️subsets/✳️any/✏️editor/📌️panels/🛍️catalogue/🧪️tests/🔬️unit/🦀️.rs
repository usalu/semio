extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, render};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_primitives() {
    let mut a = app().await;
    let json = render(&mut a, semio_s_artifact_lowpoly_lowpoly_panels::catalogue::LOWPOLY_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("lowpoly-play-catalogue.box"));
    assert!(json.contains("Cube"));
    assert!(json.contains("Ico Sphere"));
}
