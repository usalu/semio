use super::*;

#[semio_framework_async_macros::async_test]
async fn every_utility_is_unique_and_armed_in_at_least_one_window() {
    let mut ids: Vec<&str> = UTILITIES.iter().map(|row| row.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), UTILITIES.len());
    assert!(UTILITIES.iter().all(|row| !row.windows.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn the_windows_accept_their_tools_and_the_schedule_none() {
    assert_eq!(for_window(plan::WINDOW_KIND_ID).len(), UTILITIES.len(), "the plan accepts every tool");
    assert_eq!(for_window(section::WINDOW_KIND_ID).len(), 1, "the section accepts the select tool and its storey height handle");
    assert!(for_window(world::WINDOW_KIND_ID).iter().any(|utility| utility.as_str() == "wall"));
    assert!(for_window("bim-edit-schedule").is_empty());
    assert_eq!(definitions().len(), UTILITIES.len());
    assert!(for_window(section::WINDOW_KIND_ID).iter().all(|utility| utility.as_str() == DEFAULT_UTILITY));
}

#[semio_framework_async_macros::async_test]
async fn every_utility_has_an_english_and_a_german_label_and_the_hotkeys_are_single_letters() {
    for row in UTILITIES {
        assert!(!row.label.0.is_empty() && !row.label.1.is_empty(), "{}", row.id);
        assert_eq!(row.keys.is_some(), row.arm.is_some(), "{}: a hotkey and its arming command come together", row.id);
        assert!(row.keys.is_none_or(|keys| keys.len() == 1 && keys.chars().all(|ch| ch.is_ascii_lowercase())), "{}", row.id);
    }
    assert!(draws("wall") && !draws(DEFAULT_UTILITY));
}
