//! 🍰️ Companion test crate of `semio-s-artifact-layout-layout-mutations-content`: the mutation fixture and unit laws of the content leaves, part two. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod change_data_fields {
    #[path = "../../../🧾change-data-fields/🧪️tests/🧾️attaches/🦀️.rs"]
    pub mod tests_attaches_a_data_fields_payload;
}
#[path = "."]
pub mod change_link_path {
    #[path = "../../../🛤️change-link-path/🧪️tests/🔗️relinks/🦀️.rs"]
    pub mod tests_relinks_link_1_to_a_new_file;
}
#[path = "."]
pub mod change_print_target {
    #[path = "../../../🖨️change-print-target/🧪️tests/🖨️sets/🦀️.rs"]
    pub mod tests_sets_a_cmyk_print_target;
}
#[path = "."]
pub mod create_character_style {
    #[path = "../../../🖋️create-character-style/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod create_link {
    #[path = "../../../🖇️create-link/🧪️tests/🔗️appends-link-3/🦀️.rs"]
    pub mod tests_appends_link_3;
}
#[path = "."]
pub mod create_story {
    #[path = "../../../📖create-story/🧪️tests/📖️appends-story-3/🦀️.rs"]
    pub mod tests_appends_story_3;
}
#[path = "."]
pub mod delete_character_style {
    #[path = "../../../🗑️delete-character-style/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
    #[path = "../../../🗑️delete-character-style/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    pub mod tests_removes_a_middle_row;
}
#[path = "."]
pub mod delete_page {
    #[path = "../../../🗑️delete-page/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    pub mod tests_removes_a_middle_row;
    #[path = "../../../🗑️delete-page/🧪️tests/🚫️removes-page-2/🦀️.rs"]
    pub mod tests_removes_page_2;
}
#[path = "."]
pub mod delete_story {
    #[path = "../../../📕delete-story/🧪️tests/🚫️removes-story-2/🦀️.rs"]
    pub mod tests_removes_story_2;
}
#[path = "."]
pub mod reorder_pages {
    #[path = "../../../🔀reorder-pages/🧪️tests/📍️moves-a-middle-row/🦀️.rs"]
    pub mod tests_moves_a_middle_row;
    #[path = "../../../🔀reorder-pages/🧪️tests/🔀️moves/🦀️.rs"]
    pub mod tests_moves_page_1_behind_page_2;
}
#[path = "."]
pub mod set_page_parent {
    #[path = "../../../🗄️set-page-parent/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_paragraph_style {
    #[path = "../../../📝️update-paragraph-style/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_parent_page {
    #[path = "../../../📖️update-parent-page/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
