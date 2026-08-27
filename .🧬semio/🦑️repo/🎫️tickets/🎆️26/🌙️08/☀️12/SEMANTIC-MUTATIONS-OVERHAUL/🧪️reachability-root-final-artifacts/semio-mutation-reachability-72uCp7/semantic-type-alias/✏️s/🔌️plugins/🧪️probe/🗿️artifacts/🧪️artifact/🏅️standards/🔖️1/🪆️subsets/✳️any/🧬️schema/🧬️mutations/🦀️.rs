#[path = "➕️insert-page/🦀️.rs"] pub mod insert_page;
pub use insert_page::InsertPageMutation as InsertPageAlias;
pub enum ProbeMutation { InsertPage(InsertPageAlias) }
