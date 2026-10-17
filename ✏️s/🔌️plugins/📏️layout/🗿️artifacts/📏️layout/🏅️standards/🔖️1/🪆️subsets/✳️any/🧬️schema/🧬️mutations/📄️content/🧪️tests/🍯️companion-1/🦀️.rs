//! 🍯️ Companion test crate of `semio-s-artifact-layout-layout-mutations-content`: the mutation fixture and unit laws of the content leaves, part one. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod change_page_height {
    #[path = "../../../↕️change-page-height/🧪️tests/↕️lengthens/🦀️.rs"]
    pub mod tests_lengthens_page_1;
}
#[path = "."]
pub mod change_page_width {
    #[path = "../../../↔️change-page-width/🧪️tests/↔️widens-page-1/🦀️.rs"]
    pub mod tests_widens_page_1;
}
#[path = "."]
pub mod create_page {
    #[path = "../../../🌱create-page/🧪️tests/➕️appends-page-3/🦀️.rs"]
    pub mod tests_appends_page_3;
}
#[path = "."]
pub mod delete_link {
    #[path = "../../../✂️delete-link/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    pub mod tests_removes_a_middle_row;
    #[path = "../../../✂️delete-link/🧪️tests/🔗️removes-link-2/🦀️.rs"]
    pub mod tests_removes_link_2;
}
#[path = "."]
pub mod delete_story {
    #[path = "../../../📕delete-story/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    pub mod tests_removes_a_middle_row;
}
#[path = "."]
pub mod edit_story {
    #[path = "../../../✍️edit-story/🧪️tests/📝️rewrites-story-1-body/🦀️.rs"]
    pub mod tests_rewrites_story_1_body;
}
#[path = "."]
pub mod rename_layout {
    #[path = "../../../✏️rename-layout/🧪️tests/🏷️renames/🦀️.rs"]
    pub mod tests_renames_the_document;
}
#[path = "."]
pub mod rename_page {
    #[path = "../../../🏷️rename-page/🧪️tests/🏷️renames-page-1/🦀️.rs"]
    pub mod tests_renames_page_1;
}
#[path = "."]
pub mod set_page_guides {
    #[path = "../../../📐️set-page-guides/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod set_page_overrides {
    #[path = "../../../📎set-page-overrides/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod set_story_runs {
    #[path = "../../../✒️set-story-runs/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_character_style {
    #[path = "../../../🎨️update-character-style/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_link {
    #[path = "../../../🎚️update-link/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_page_columns {
    #[path = "../../../🏛️update-page-columns/🧪️tests/🏛️splits/🦀️.rs"]
    pub mod tests_splits_page_1_into_three_columns;
}
#[path = "."]
pub mod update_page_margins {
    #[path = "../../../📐update-page-margins/🧪️tests/📐️sets/🦀️.rs"]
    pub mod tests_sets_asymmetric_margins_on_page_1;
}
#[path = "."]
pub mod update_spread {
    #[path = "../../../📓️update-spread/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
