#[path = "➕️insert-page/🦀️.rs"] pub mod insert_page;
pub(crate) use insert_page::Mutation as Alias;
pub enum ProbeMutation { InsertPage(Alias) }
