//! 🧩️ Current Specific artifact selection depends on captured ownership, without a pilot registry.
pub fn current_fixture_contribution_eligible(directory:&str,role:Option<&str>,declaration_present:bool)->bool{role==Some("artifact")&&(directory.starts_with("✏️s/")||declaration_present)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
