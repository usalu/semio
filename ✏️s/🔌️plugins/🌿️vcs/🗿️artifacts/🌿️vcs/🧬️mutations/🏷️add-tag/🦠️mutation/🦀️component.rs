use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, tag: &str) {
    if !projection.tags.contains(&tag.to_string()) { projection.tags.push(tag.to_string()); }
}
