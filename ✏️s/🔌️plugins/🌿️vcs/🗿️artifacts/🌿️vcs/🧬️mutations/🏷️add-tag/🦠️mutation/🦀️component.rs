use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, tag: &str) {
    if !projection.tags.contains(&tag.to_string()) { projection.tags.push(tag.to_string()); }
}
