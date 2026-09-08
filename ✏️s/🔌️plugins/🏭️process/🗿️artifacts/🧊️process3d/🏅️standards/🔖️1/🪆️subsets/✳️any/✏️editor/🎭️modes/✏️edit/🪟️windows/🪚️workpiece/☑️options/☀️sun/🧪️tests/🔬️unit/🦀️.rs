
use super::*;

#[semio_framework_async_macros::async_test]
async fn sun_off_yields_an_unpressed_toggle_child() {
    let sun = WorldSunConfig { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() };
    let group = measure(&sun);
    assert!(matches!(&group, WindowMeasure::Group { children, .. } if children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed))));
}

#[semio_framework_async_macros::async_test]
async fn sun_on_yields_a_pressed_toggle_child() {
    let sun = WorldSunConfig { enabled: true, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() };
    let group = measure(&sun);
    assert!(matches!(&group, WindowMeasure::Group { children, .. } if children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed))));
}
