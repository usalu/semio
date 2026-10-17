//! 🔩️ Layout content mutations: the document, page, spread, story, link and style mutation leaves (part 2).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🛤️change-link-path/🦀️.rs"]
pub mod change_link_path;
#[path = "../↔️change-page-width/🦀️.rs"]
pub mod change_page_width;
#[path = "../🖋️create-character-style/🦀️.rs"]
pub mod create_character_style;
#[path = "../📖create-story/🦀️.rs"]
pub mod create_story;
#[path = "../🗑️delete-character-style/🦀️.rs"]
pub mod delete_character_style;
#[path = "../📕delete-story/🦀️.rs"]
pub mod delete_story;
#[path = "../✍️edit-story/🦀️.rs"]
pub mod edit_story;
#[path = "../🏷️rename-page/🦀️.rs"]
pub mod rename_page;
#[path = "../📐️set-page-guides/🦀️.rs"]
pub mod set_page_guides;
#[path = "../🗄️set-page-parent/🦀️.rs"]
pub mod set_page_parent;
#[path = "../🎨️update-character-style/🦀️.rs"]
pub mod update_character_style;
#[path = "../🎚️update-link/🦀️.rs"]
pub mod update_link;
#[path = "../📐update-page-margins/🦀️.rs"]
pub mod update_page_margins;
#[path = "../📖️update-parent-page/🦀️.rs"]
pub mod update_parent_page;
//#endregion 🪢️TaxonomyMounts
