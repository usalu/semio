"""Task 1 layer A: reliability class as a schema enum, K_FI no longer stored (model, root schema struct, diff, mutation payload)."""
from lib import *
M = "semio_s_artifact_norm_en1990_model::ReliabilityClass"

ENUM = '''/// 🛡️ Reliability class RC1 to RC3 of EN 1990 Annex B (Tables B1 to B3); K_FI and the target β are read from the NDP registry, never stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ReliabilityClass {
    #[dsl(key = "rc1")]
    RC1,
    #[dsl(key = "rc2")]
    RC2,
    #[dsl(key = "rc3")]
    RC3,
}

impl ReliabilityClass {
    pub const ALL: [Self; 3] = [Self::RC1, Self::RC2, Self::RC3];

    /// 🔢️ The class number 1 to 3 that keys the registry rows `ndp.en1990.kfi.<n>`, `beta1.<n>` and `beta50.<n>`.
    pub fn number(self) -> u8 {
        match self {
            Self::RC1 => 1,
            Self::RC2 => 2,
            Self::RC3 => 3,
        }
    }

    /// 🔢️ The class of a number 1 to 3, `None` outside.
    pub fn from_number(number: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|class| class.number() == number)
    }
}

'''

def snapshot(t):
    t = rep(t, "    #[state(artifact)]\n    pub reliability_class: u8,\n", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n", guard="pub reliability_class: ReliabilityClass")
    t = rep(t, "    #[state(artifact)]\n    pub k_fi_declared: f64,\n", "", guard="/// 🛡️ Reliability class")
    t = rep(t, "            reliability_class: 2,\n", "            reliability_class: ReliabilityClass::RC2,\n", guard="ReliabilityClass::RC2")
    t = rep(t, "            k_fi_declared: 1.0,\n", "", guard="/// 🛡️ Reliability class")
    t = rep(t, "/// 🏛 Seismic importance class", ENUM + "/// 🏛 Seismic importance class", guard="/// 🛡️ Reliability class")
    return t
edit(SNAP + "/🦀️.rs", snapshot)

def schema(t):
    t = rep(t, "    pub reliability_class: u8,\n", "    pub reliability_class: %s,\n" % M, guard=M)
    t = rep(t, "    #[state(artifact)]\n    pub k_fi_declared: f64,\n", "", guard="@@")
    t = rep(t, "            k_fi_declared: self.k_fi_declared,\n", "", guard="@@")
    t = rep(t, "            k_fi_declared: snapshot.k_fi_declared,\n", "", guard="@@")
    return t
def schema_safe(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "    pub reliability_class: u8,\n", "    pub reliability_class: %s,\n" % M)
    t = rep(t, "    #[state(artifact)]\n    pub k_fi_declared: f64,\n", "")
    t = rep(t, "            k_fi_declared: self.k_fi_declared,\n", "")
    t = rep(t, "            k_fi_declared: snapshot.k_fi_declared,\n", "")
    return t
edit(SCH + "/🦀️.rs", schema_safe)

def diff(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "    pub reliability_class: Option<u8>,\n", "    pub reliability_class: Option<%s>,\n" % M)
    t = rep(t, "    #[state(artifact)]\n    pub k_fi_declared: Option<f64>,\n", "")
    t = rep(t, "        if other.k_fi_declared.is_some() {\n            self.k_fi_declared = other.k_fi_declared;\n        }\n", "")
    t = rep(t, "        if let Some(value) = self.k_fi_declared {\n            next.k_fi_declared = value;\n        }\n", "")
    t = rep(t, "            && self.k_fi_declared.is_none()\n", "")
    t = rep(t, "            k_fi_declared: self.k_fi_declared.map(|_| base.k_fi_declared),\n", "")
    return t
edit(DIFF + "/🦀️.rs", diff)

RC = MUT + "/🎯change-reliability-class"
edit(RC + "/🦀️.rs", lambda t: rep(t, "    pub new_reliability_class: u8,\n", "    pub new_reliability_class: %s,\n" % M, guard=M))
