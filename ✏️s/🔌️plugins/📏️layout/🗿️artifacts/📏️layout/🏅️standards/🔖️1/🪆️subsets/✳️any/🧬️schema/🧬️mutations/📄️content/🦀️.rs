//! 📄️ Layout content mutations: the document, page, spread, story, link and style mutation leaves.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🧾change-data-fields/🦀️.rs"]
pub mod change_data_fields;
#[path = "../🛤️change-link-path/🦀️.rs"]
pub mod change_link_path;
#[path = "../↕️change-page-height/🦀️.rs"]
pub mod change_page_height;
#[path = "../↔️change-page-width/🦀️.rs"]
pub mod change_page_width;
#[path = "../🖨️change-print-target/🦀️.rs"]
pub mod change_print_target;
#[path = "../🖋️create-character-style/🦀️.rs"]
pub mod create_character_style;
#[path = "../🖇️create-link/🦀️.rs"]
pub mod create_link;
#[path = "../🌱create-page/🦀️.rs"]
pub mod create_page;
#[path = "../📖create-story/🦀️.rs"]
pub mod create_story;
#[path = "../🗑️delete-character-style/🦀️.rs"]
pub mod delete_character_style;
#[path = "../✂️delete-link/🦀️.rs"]
pub mod delete_link;
#[path = "../🗑️delete-page/🦀️.rs"]
pub mod delete_page;
#[path = "../📕delete-story/🦀️.rs"]
pub mod delete_story;
#[path = "../✍️edit-story/🦀️.rs"]
pub mod edit_story;
#[path = "../✏️rename-layout/🦀️.rs"]
pub mod rename_layout;
#[path = "../🏷️rename-page/🦀️.rs"]
pub mod rename_page;
#[path = "../🔀reorder-pages/🦀️.rs"]
pub mod reorder_pages;
#[path = "../📐️set-page-guides/🦀️.rs"]
pub mod set_page_guides;
#[path = "../📎set-page-overrides/🦀️.rs"]
pub mod set_page_overrides;
#[path = "../🗄️set-page-parent/🦀️.rs"]
pub mod set_page_parent;
#[path = "../✒️set-story-runs/🦀️.rs"]
pub mod set_story_runs;
#[path = "../🎨️update-character-style/🦀️.rs"]
pub mod update_character_style;
#[path = "../🎚️update-link/🦀️.rs"]
pub mod update_link;
#[path = "../🏛️update-page-columns/🦀️.rs"]
pub mod update_page_columns;
#[path = "../📐update-page-margins/🦀️.rs"]
pub mod update_page_margins;
#[path = "../📝️update-paragraph-style/🦀️.rs"]
pub mod update_paragraph_style;
#[path = "../📖️update-parent-page/🦀️.rs"]
pub mod update_parent_page;
#[path = "../📓️update-spread/🦀️.rs"]
pub mod update_spread;
//#endregion 🪢️TaxonomyMounts
