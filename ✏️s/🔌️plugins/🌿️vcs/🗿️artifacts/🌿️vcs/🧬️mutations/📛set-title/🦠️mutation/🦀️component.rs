use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, title: &str) {
    projection.title = title.to_string();
}
