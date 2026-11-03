//! ⚖️ Partial and combination factors on actions: the one registry-backed source of γ_G,sup, γ_G,inf, γ_Q, γ_P, ξ, ψ0, ψ1, ψ2 and K_FI (EN 1990 Annex A1, A2 and B) that every Eurocode artifact reads. See [`super`].

use crate::{ndp_number, ndp_number_opt, AnnexChoice, DesignSituation, LimitState};

fn registry(annex: AnnexChoice, id: &str) -> f64 {
    ndp_number(id, annex)
}

//#region 🔖️ReliabilityClass
/// 🛡️ Reliability class RC1 to RC3 of EN 1990 Annex B (Tables B1 to B3); K_FI and the target β are read from the NDP registry, never stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
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

    /// 🔢️ K_FI of Table B3 for the annex, the registry row `ndp.en1990.kfi.<n>`.
    pub fn k_fi(self, annex: AnnexChoice) -> f64 {
        registry(annex, &format!("ndp.en1990.kfi.{}", self.number()))
    }
}
//#endregion 🔖️ReliabilityClass

//#region 🔖️PartialFactorSets
/// 🏗️ Annex A1 (buildings) or Annex A2 (bridges) selected by the structure kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Building,
    Bridge,
}

impl Domain {
    pub fn of(structure_kind: &str) -> Option<Self> {
        match structure_kind {
            "building" => Some(Self::Building),
            "road_bridge" | "footbridge" | "rail_bridge" => Some(Self::Bridge),
            _ => None,
        }
    }

    pub fn tag(self) -> &'static str {
        match self {
            Self::Building => "A1",
            Self::Bridge => "A2",
        }
    }
}

/// ⚖️ The limit-state family a partial-factor set belongs to (Tables A1.2(A-C) and A2.4(A-C)); `GeoC` is Set C (GEO-3, global stability, DA3), `Geo2` the GEO-2 combination of DIN 1054 / EN 1997 DA2* that applies the factors of Set B to the actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactorSet {
    EquA,
    StrB,
    GeoC,
    Geo2,
}

impl FactorSet {
    pub fn tag(self) -> &'static str {
        match self {
            Self::EquA => "equ",
            Self::StrB => "str",
            Self::GeoC => "geo",
            Self::Geo2 => "geo2",
        }
    }

    pub fn label(self, domain: Domain) -> &'static str {
        match (self, domain) {
            (Self::EquA, Domain::Building) => "A1.2(A)",
            (Self::StrB, Domain::Building) => "A1.2(B)",
            (Self::GeoC, Domain::Building) => "A1.2(C)",
            (Self::Geo2, Domain::Building) => "A1.2(B) GEO-2",
            (Self::EquA, Domain::Bridge) => "A2.4(A)",
            (Self::StrB, Domain::Bridge) => "A2.4(B)",
            (Self::GeoC, Domain::Bridge) => "A2.4(C)",
            (Self::Geo2, Domain::Bridge) => "A2.4(B) GEO-2",
        }
    }
}

/// 🔢️ γ_G,sup, γ_G,inf of one set.
pub fn gamma_g(annex: AnnexChoice, domain: Domain, set: FactorSet) -> (f64, f64) {
    let tag = format!("{}.{}", set.tag(), domain.tag());
    (registry(annex, &format!("ndp.en1990.gG_sup.{tag}")), registry(annex, &format!("ndp.en1990.gG_inf.{tag}")))
}

/// 🚦️ The γ_Q family of a variable action category: bridges split road, rail and other variable actions.
pub fn traffic_family(category: &str) -> &'static str {
    match category {
        c if c.starts_with("road_") || c.starts_with("footbridge_") => "road",
        c if c.starts_with("rail_") => "rail",
        _ => "other",
    }
}

/// 🔢️ γ_Q of one variable action category in one set.
pub fn gamma_q(annex: AnnexChoice, domain: Domain, set: FactorSet, category: &str) -> f64 {
    match domain {
        Domain::Building => registry(annex, &format!("ndp.en1990.gQ.{}.A1", set.tag())),
        Domain::Bridge => registry(annex, &format!("ndp.en1990.gQ.{}.{}.A2", traffic_family(category), set.tag())),
    }
}

/// 🔢️ ξ of Eq. 6.10b.
pub fn xi(annex: AnnexChoice, domain: Domain) -> f64 {
    registry(annex, &format!("ndp.en1990.xi.str.{}", domain.tag()))
}

/// 🔢️ γ_P,sup (effect adds) or γ_P,inf (effect relieves) for prestress in persistent and transient situations.
pub fn gamma_p(annex: AnnexChoice, relieving: bool) -> f64 {
    registry(annex, if relieving { "ndp.en1990.gP_inf" } else { "ndp.en1990.gP_sup" })
}
//#endregion 🔖️PartialFactorSets

//#region 🔖️PsiFactors
/// 📊️ ψ factors for one action category (EN 1990 Tables A1.1, A2.1-A2.3 / DIN EN 1990/NA Table NA.A.1.1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PsiRow {
    pub psi_0: f64,
    pub psi_1: f64,
    pub psi_2: f64,
}

impl PsiRow {
    /// 🧯️ The most unfavourable row (1, 1, 1) for an action without a registered category.
    pub const UNITY: Self = Self { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 };

    pub fn as_tuple(self) -> (f64, f64, f64) {
        (self.psi_0, self.psi_1, self.psi_2)
    }
}

/// 🔤️ The canonical registry key of an imposed-load category such as `B` or `B2` (EN 1991-1-1, NA-DE: `A1..E1`) for `office`; the NA-DE categories `T` (stairs) and `Z` (access) follow the adjoining floor category A.
pub fn canonical_category(key: &str) -> &str {
    let bytes = key.as_bytes();
    let coded = matches!(bytes.len(), 1 | 2) && (bytes.len() == 1 || bytes[1].is_ascii_digit());
    if !coded {
        return key;
    }
    match bytes[0] {
        b'A' | b'T' | b'Z' => "residential",
        b'B' => "office",
        b'C' => "congregation",
        b'D' => "retail",
        b'E' => "storage",
        b'F' => "traffic_light",
        b'G' => "traffic_heavy",
        b'H' => "roof",
        _ => key,
    }
}

/// 🔎️ The ψ row of the registry key `key` (`ndp.en1990.psi.<key>.<0|1|2>`) for the annex; `None` when a row is not registered.
pub fn psi_row_of_key(annex: AnnexChoice, key: &str) -> Option<PsiRow> {
    let at = |index: u8| ndp_number_opt(&format!("ndp.en1990.psi.{key}.{index}"), annex);
    Some(PsiRow { psi_0: at(0)?, psi_1: at(1)?, psi_2: at(2)? })
}

/// 🌨️ Snow switches to the above-1000 m row for both annexes; every other category is unchanged.
pub fn resolve_psi_category(annex: AnnexChoice, category: &str, altitude_m: f64) -> String {
    match category {
        "snow" | "snow_high" if altitude_m > registry(annex, "ndp.en1990.snow_altitude_m") => "snow_high".into(),
        "snow" | "snow_high" => "snow".into(),
        other => other.into(),
    }
}
//#endregion 🔖️PsiFactors

//#region 🔖️ActionFactors
/// ⚖️ The partial and combination factors of one design context, read from the registry: the annex, the reliability class (K_FI, EN 1990 Annex B), the design situation, the limit state and the domain and factor set (Tables A1.2 and A2.4). Persistent and transient ultimate situations carry γ_G, γ_Q and ξ; accidental, seismic and serviceability combinations carry 1.0 on permanent actions and ψ on variable ones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionFactors {
    annex: AnnexChoice,
    reliability_class: ReliabilityClass,
    situation: DesignSituation,
    limit_state: LimitState,
    domain: Domain,
    set: FactorSet,
    k_fi: f64,
}

impl ActionFactors {
    /// 🧮️ Buildings, Set B (STR), ultimate limit state of `situation`, K_FI of `reliability_class` (Table B3).
    pub fn new(annex: AnnexChoice, reliability_class: ReliabilityClass, situation: DesignSituation) -> Self {
        Self { annex, reliability_class, situation, limit_state: LimitState::Uls, domain: Domain::Building, set: FactorSet::StrB, k_fi: reliability_class.k_fi(annex) }
    }

    /// 🏗️ The factors of Annex A1 or A2.
    pub fn domain(mut self, domain: Domain) -> Self {
        self.domain = domain;
        self
    }

    /// ⚖️ The factor set (A, B, C, GEO-2).
    pub fn set(mut self, set: FactorSet) -> Self {
        self.set = set;
        self
    }

    /// 🎚️ Project override of K_FI.
    pub fn k_fi_override(mut self, k_fi: f64) -> Self {
        self.k_fi = k_fi;
        self
    }

    /// 🚦️ The same context in another design situation.
    pub fn in_situation(mut self, situation: DesignSituation) -> Self {
        self.situation = situation;
        self
    }

    /// 📏️ The same context for the serviceability limit state (all permanent and variable partial factors 1.0, ψ on the accompanying actions).
    pub fn serviceability(mut self) -> Self {
        self.limit_state = LimitState::Sls;
        self
    }

    pub fn annex(&self) -> AnnexChoice {
        self.annex
    }

    pub fn reliability_class(&self) -> ReliabilityClass {
        self.reliability_class
    }

    pub fn situation(&self) -> DesignSituation {
        self.situation
    }

    /// 🔢️ K_FI of the context.
    pub fn k_fi(&self) -> f64 {
        self.k_fi
    }

    /// 🚨️ Whether the context is an accidental or seismic situation (EN 1990 6.4.3.3 and 6.4.3.4).
    pub fn exceptional(&self) -> bool {
        matches!(self.situation, DesignSituation::Accidental | DesignSituation::Seismic)
    }

    fn fundamental(&self) -> bool {
        self.limit_state == LimitState::Uls && !self.exceptional()
    }

    /// 🔢️ γ_G,sup of an unfavourable permanent action times K_FI; 1.0 in exceptional situations and for serviceability.
    pub fn gamma_g_sup(&self) -> f64 {
        if self.fundamental() { self.k_fi * gamma_g(self.annex, self.domain, self.set).0 } else { 1.0 }
    }

    /// 🔢️ γ_G,inf of a favourable permanent action (without K_FI, EN 1990 B3.3); 1.0 in exceptional situations and for serviceability.
    pub fn gamma_g_inf(&self) -> f64 {
        if self.fundamental() { gamma_g(self.annex, self.domain, self.set).1 } else { 1.0 }
    }

    /// 🔢️ ξ of Eq. 6.10b (1.0 where the situation does not use it).
    pub fn xi(&self) -> f64 {
        if self.fundamental() { xi(self.annex, self.domain) } else { 1.0 }
    }

    /// 🔢️ ξ·γ_G,sup of the permanent actions in Eq. 6.10b.
    pub fn gamma_g_sup_610b(&self) -> f64 {
        self.xi() * self.gamma_g_sup()
    }

    /// 🔢️ γ_Q of a variable action of `category` times K_FI; 1.0 in exceptional situations and for serviceability.
    pub fn gamma_q(&self, category: &str) -> f64 {
        if self.fundamental() { self.k_fi * gamma_q(self.annex, self.domain, self.set, category) } else { 1.0 }
    }

    /// 🔢️ γ_Q of a variable action without a bridge traffic category (buildings).
    pub fn gamma_q_building(&self) -> f64 {
        self.gamma_q("")
    }

    /// 🔢️ γ_P of a prestress action in persistent and transient situations, 1.0 elsewhere.
    pub fn gamma_p(&self, relieving: bool) -> f64 {
        if self.fundamental() { gamma_p(self.annex, relieving) } else { 1.0 }
    }

    /// 🔢️ The factor of a permanent action: γ_G,inf when its effect relieves, γ_G,sup when it adds.
    pub fn gamma_g(&self, relieving: bool) -> f64 {
        if relieving { self.gamma_g_inf() } else { self.gamma_g_sup() }
    }

    /// 🔎️ The ψ row of a category (imposed letters `A..H`, `snow`, `wind`, `temperature`, the bridge keys), `None` for an unknown category.
    pub fn psi_known(&self, category: &str) -> Option<PsiRow> {
        psi_row_of_key(self.annex, canonical_category(category))
    }

    /// 🔎️ The ψ row of a category; an unknown category carries the most unfavourable row (1, 1, 1).
    pub fn psi(&self, category: &str) -> PsiRow {
        self.psi_known(category).unwrap_or(PsiRow::UNITY)
    }

    /// 🌨️ The ψ row of snow at an altitude: the above-1000 m row applies above `ndp.en1990.snow_altitude_m`.
    pub fn psi_snow(&self, altitude_m: f64) -> PsiRow {
        self.psi(&resolve_psi_category(self.annex, "snow", altitude_m))
    }
}
//#endregion 🔖️ActionFactors
