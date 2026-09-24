//! 🧬️ Din18599 snapshot schema — artifact-lane fields only.

use crate::{Din18599ClimateChild, MonthlyClimate, UseClass};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted Din18599 document snapshot. Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
/// round 2 (`norm→C:table` on `din18599.climate`): the inline `MonthlyClimate` (two twelve-month
/// arrays) is replaced by a fixed composed `s.stdio.semio`/`table` CHILD slot — see
/// `🗿️artifacts/⚡️din18599/🦀️.rs`'s `🔖️Composition` region for the converters/
/// working-scene cache. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s slot-table emission;
/// never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(extension = "din18599")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Snapshot {
    #[state(artifact)]
    pub use_class: UseClass,
    #[state(artifact)]
    pub heated_area_m2: f64,
    #[state(artifact)]
    pub occupants: u32,
    #[state(artifact)]
    pub h_t: f64,
    #[state(artifact)]
    pub h_v: f64,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle"))]
    pub climate: Din18599ClimateChild,
    #[state(artifact)]
    pub internal_gains_w_m2: f64,
    #[state(artifact)]
    pub solar_gains_kwh: f64,
    #[state(artifact)]
    pub system_losses_kwh: f64,
    #[state(artifact)]
    pub renewable_kwh: f64,
    #[state(artifact)]
    pub annual_limit_kwh: f64,
    #[state(artifact)]
    pub energy_carrier: String,
    #[state(artifact)]
    pub reference_q_p_kwh: f64,
}

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` over the one derived record spec, composed child included.
impl store::ArtifactDsl for Din18599Snapshot {
    const EXTENSION: &'static str = "din18599";
    fn envelope_id() -> &'static str {
        "norm.din18599"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Din18599Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din18599Snapshot {
    fn default() -> Self {
        let climate = crate::din18599_climate_child_from_data(&MonthlyClimate {
            theta_e_c: [-14.0, -11.186533479473212, -3.4999999999999964, 7.000000000000001, 17.5, 25.186533479473212, 28.0, 25.186533479473212, 17.5, 7.000000000000001, -3.4999999999999964, -11.186533479473212],
            g_h_w_m2: [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0],
        });
        Self {
            use_class: UseClass::Residential,
            heated_area_m2: 100.0,
            occupants: 4,
            h_t: 92.12124613902822,
            h_v: 40.800000000000004,
            climate,
            internal_gains_w_m2: 3.5,
            solar_gains_kwh: 84.0,
            system_losses_kwh: 800.0,
            renewable_kwh: 1500.0,
            annual_limit_kwh: 7500.0,
            energy_carrier: "natural_gas".into(),
            reference_q_p_kwh: 10000.0,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`Din18599Snapshot`] — the surface
/// `../../../../../🧪️tests/⚡️mutate-din18599-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_snapshot_json(snapshot: &Din18599Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_din18599_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`Din18599Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_snapshot_json(text: &str) -> Result<Din18599Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Din18599Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_dsl(text: &str) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din18599Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_dsl(snapshot: &Din18599Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Din18599Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_pack(bytes: &[u8]) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Din18599Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_pack(snapshot: &Din18599Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge

