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
async fn every_utility_has_an_english_and_a_german_label_and_the_hotkeys_are_a_letter_or_shift_and_a_letter() {
    for row in UTILITIES {
        let (english, german) = ((row.label)(&BimLabels::NATIVE_EN), (row.label)(&BimLabels::NATIVE_DE));
        assert!(!english.as_str().is_empty() && !german.as_str().is_empty(), "{}", row.id);
        assert_eq!(row.keys.is_some(), row.arm.is_some(), "{}: a hotkey and its arming command come together", row.id);
        assert!(row.keys.is_none_or(|keys| keys.strip_prefix("shift+").unwrap_or(keys).len() == 1 && keys.strip_prefix("shift+").unwrap_or(keys).chars().all(|ch| ch.is_ascii_lowercase())), "{}", row.id);
    }
    let move_rotate_slab_walls_and_split = ["move", "rotate", "slab-walls", "split-wall"].map(|id| UTILITIES.iter().find(|row| row.id == id).and_then(|row| row.keys));
    assert_eq!(move_rotate_slab_walls_and_split, [Some("e"), Some("q"), Some("shift+s"), Some("shift+w")], "the tools the audit found without a key have one");
    assert!(draws("wall") && !draws(DEFAULT_UTILITY));
}
