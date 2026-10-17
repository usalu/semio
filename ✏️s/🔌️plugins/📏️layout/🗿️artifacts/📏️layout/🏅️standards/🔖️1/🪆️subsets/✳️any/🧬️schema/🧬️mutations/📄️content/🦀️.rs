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
#[path = "../↕️change-page-height/🦀️.rs"]
pub mod change_page_height;
#[path = "../🖨️change-print-target/🦀️.rs"]
pub mod change_print_target;
#[path = "../🖇️create-link/🦀️.rs"]
pub mod create_link;
#[path = "../🌱create-page/🦀️.rs"]
pub mod create_page;
#[path = "../✂️delete-link/🦀️.rs"]
pub mod delete_link;
#[path = "../🗑️delete-page/🦀️.rs"]
pub mod delete_page;
#[path = "../✏️rename-layout/🦀️.rs"]
pub mod rename_layout;
#[path = "../🔀reorder-pages/🦀️.rs"]
pub mod reorder_pages;
#[path = "../📎set-page-overrides/🦀️.rs"]
pub mod set_page_overrides;
#[path = "../✒️set-story-runs/🦀️.rs"]
pub mod set_story_runs;
#[path = "../🏛️update-page-columns/🦀️.rs"]
pub mod update_page_columns;
#[path = "../📝️update-paragraph-style/🦀️.rs"]
pub mod update_paragraph_style;
#[path = "../📓️update-spread/🦀️.rs"]
pub mod update_spread;
//#endregion 🪢️TaxonomyMounts

pub use semio_s_artifact_layout_layout_mutations_content_2::{change_link_path, change_page_width, create_character_style, create_story, delete_character_style, delete_story, edit_story, rename_page, set_page_guides, set_page_parent, update_character_style, update_link, update_page_margins, update_parent_page};
