
use super::*;
use crate::editor::flow::terminology::flow_play_labels;
use crate::schema::FLOW_DEFAULT_PROXIMITY_DISTANCE;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_brackets_the_default_distance() {
    let config = FlowConfig::default();
    match measure(&config, flow_play_labels(&config)) {
        WindowMeasure::Slider { value, min, max, .. } => {
            assert_eq!(value, FLOW_DEFAULT_PROXIMITY_DISTANCE);
            assert!(min <= value && value <= max, "default {value} must sit inside {min}..={max}");
        }
        other => panic!("proximity measure must be a slider, got {other:?}"),
    }
}
