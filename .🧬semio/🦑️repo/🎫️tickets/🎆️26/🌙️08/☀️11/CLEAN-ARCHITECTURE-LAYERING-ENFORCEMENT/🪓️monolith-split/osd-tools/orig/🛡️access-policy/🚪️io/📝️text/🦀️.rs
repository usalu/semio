//! 🚪️ Physical Directory wire admission, canonical emission and bounded receipts.

use super::super::super::access_policy::*;
use std::sync::OnceLock;

/// ⚖️ The one hub authority: the declared policy's decision.
pub fn directory_access_permits(roles: &[DirectoryAccessRoleV1], action: DirectoryAccessActionV1, space_kind: Option<&str>) -> bool {
    DirectoryAccessPolicyV1::declared().permits(roles, action, space_kind)
}

impl DirectoryAccessPolicyV1 {
pub fn parse(source: &str) -> Result<Self, String> {
        let policy: Self = serde_json::from_str(source).map_err(|error| format!("access policy does not parse: {error}"))?;
        if policy.schema != DIRECTORY_ACCESS_POLICY_SCHEMA_V1 || policy.grants.is_empty() || policy.grants.len() > 64 {
            return Err("access policy has an unknown schema or grant count".into());
        }
        for grant in &policy.grants {
            if grant.roles.is_empty() || grant.actions.is_empty() || grant.roles.iter().collect::<std::collections::BTreeSet<_>>().len() != grant.roles.len() || grant.actions.iter().collect::<std::collections::BTreeSet<_>>().len() != grant.actions.len() || grant.space_kinds.as_ref().is_some_and(|kinds| kinds.is_empty() || kinds.iter().collect::<std::collections::BTreeSet<_>>().len() != kinds.len() || kinds.iter().any(|kind| !matches!(kind.as_str(), "atelier" | "studio" | "archive"))) {
                return Err("access policy grant is empty or names an unknown space kind".into());
            }
        }
        Ok(policy)
    }

pub fn declared() -> &'static Self {
        static DECLARED: OnceLock<DirectoryAccessPolicyV1> = OnceLock::new();
        DECLARED.get_or_init(|| Self::parse(include_str!("../../🔣️.json")).expect("the directory's declared access policy is valid"))
    }
}

