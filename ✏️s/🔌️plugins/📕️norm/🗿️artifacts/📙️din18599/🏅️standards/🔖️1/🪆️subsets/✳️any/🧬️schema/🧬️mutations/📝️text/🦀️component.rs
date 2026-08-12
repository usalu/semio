//! 🔧️ Din18599 artifact — OpText/OpBinary codecs for `Din18599Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro no longer applies now that the whole-document-replace variant is
//! gone).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use crate::artifacts::din18599::schema::mutations::Din18599Mutation;
use crate::artifacts::din18599::schema::mutations::{change_use_class, change_heated_area_m2, change_occupants, change_h_t, change_h_v, change_internal_gains_w_m2, change_solar_gains_kwh, change_system_losses_kwh, change_renewable_kwh, change_annual_limit_kwh, change_energy_carrier, change_reference_q_p_kwh, update_climate};
use crate::artifacts::din18599::MonthlyClimate;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `Din18599Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `Din18599Mutation` itself, and
/// every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Din18599MutationDsl {
    ChangeUseClass {
        new_use_class: UseClass,
    },
    ChangeHeatedAreaM2 {
        new_heated_area_m2: f64,
    },
    ChangeOccupants {
        new_occupants: u32,
    },
    ChangeHT {
        new_h_t: f64,
    },
    ChangeHV {
        new_h_v: f64,
    },
    ChangeInternalGainsWM2 {
        new_internal_gains_w_m2: f64,
    },
    ChangeSolarGainsKwh {
        new_solar_gains_kwh: f64,
    },
    ChangeSystemLossesKwh {
        new_system_losses_kwh: f64,
    },
    ChangeRenewableKwh {
        new_renewable_kwh: f64,
    },
    ChangeAnnualLimitKwh {
        new_annual_limit_kwh: f64,
    },
    ChangeEnergyCarrier {
        new_energy_carrier: String,
    },
    ChangeReferenceQPKwh {
        new_reference_q_p_kwh: f64,
    },
    UpdateClimate {
        #[dsl(block)]
        new_climate: MonthlyClimate,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for Din18599MutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Din18599MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn din18599_mutation_to_dsl(mutation: &Din18599Mutation) -> Din18599MutationDsl {
    match mutation {
        Din18599Mutation::ChangeUseClass(payload) => Din18599MutationDsl::ChangeUseClass { new_use_class: payload.new_use_class.clone() },
        Din18599Mutation::ChangeHeatedAreaM2(payload) => Din18599MutationDsl::ChangeHeatedAreaM2 { new_heated_area_m2: payload.new_heated_area_m2.clone() },
        Din18599Mutation::ChangeOccupants(payload) => Din18599MutationDsl::ChangeOccupants { new_occupants: payload.new_occupants.clone() },
        Din18599Mutation::ChangeHT(payload) => Din18599MutationDsl::ChangeHT { new_h_t: payload.new_h_t.clone() },
        Din18599Mutation::ChangeHV(payload) => Din18599MutationDsl::ChangeHV { new_h_v: payload.new_h_v.clone() },
        Din18599Mutation::ChangeInternalGainsWM2(payload) => Din18599MutationDsl::ChangeInternalGainsWM2 { new_internal_gains_w_m2: payload.new_internal_gains_w_m2.clone() },
        Din18599Mutation::ChangeSolarGainsKwh(payload) => Din18599MutationDsl::ChangeSolarGainsKwh { new_solar_gains_kwh: payload.new_solar_gains_kwh.clone() },
        Din18599Mutation::ChangeSystemLossesKwh(payload) => Din18599MutationDsl::ChangeSystemLossesKwh { new_system_losses_kwh: payload.new_system_losses_kwh.clone() },
        Din18599Mutation::ChangeRenewableKwh(payload) => Din18599MutationDsl::ChangeRenewableKwh { new_renewable_kwh: payload.new_renewable_kwh.clone() },
        Din18599Mutation::ChangeAnnualLimitKwh(payload) => Din18599MutationDsl::ChangeAnnualLimitKwh { new_annual_limit_kwh: payload.new_annual_limit_kwh.clone() },
        Din18599Mutation::ChangeEnergyCarrier(payload) => Din18599MutationDsl::ChangeEnergyCarrier { new_energy_carrier: payload.new_energy_carrier.clone() },
        Din18599Mutation::ChangeReferenceQPKwh(payload) => Din18599MutationDsl::ChangeReferenceQPKwh { new_reference_q_p_kwh: payload.new_reference_q_p_kwh.clone() },
        Din18599Mutation::UpdateClimate(payload) => Din18599MutationDsl::UpdateClimate { new_climate: payload.new_climate.clone() },
    }
}

fn din18599_mutation_from_dsl(mutation: Din18599MutationDsl) -> Din18599Mutation {
    match mutation {
        Din18599MutationDsl::ChangeUseClass { new_use_class } => Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class }),
        Din18599MutationDsl::ChangeHeatedAreaM2 { new_heated_area_m2 } => Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2 }),
        Din18599MutationDsl::ChangeOccupants { new_occupants } => Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants }),
        Din18599MutationDsl::ChangeHT { new_h_t } => Din18599Mutation::ChangeHT(change_h_t::mutation::ChangeHT { new_h_t }),
        Din18599MutationDsl::ChangeHV { new_h_v } => Din18599Mutation::ChangeHV(change_h_v::mutation::ChangeHV { new_h_v }),
        Din18599MutationDsl::ChangeInternalGainsWM2 { new_internal_gains_w_m2 } => Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2 { new_internal_gains_w_m2 }),
        Din18599MutationDsl::ChangeSolarGainsKwh { new_solar_gains_kwh } => Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::mutation::ChangeSolarGainsKwh { new_solar_gains_kwh }),
        Din18599MutationDsl::ChangeSystemLossesKwh { new_system_losses_kwh } => Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::mutation::ChangeSystemLossesKwh { new_system_losses_kwh }),
        Din18599MutationDsl::ChangeRenewableKwh { new_renewable_kwh } => Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::mutation::ChangeRenewableKwh { new_renewable_kwh }),
        Din18599MutationDsl::ChangeAnnualLimitKwh { new_annual_limit_kwh } => Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh { new_annual_limit_kwh }),
        Din18599MutationDsl::ChangeEnergyCarrier { new_energy_carrier } => Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::mutation::ChangeEnergyCarrier { new_energy_carrier }),
        Din18599MutationDsl::ChangeReferenceQPKwh { new_reference_q_p_kwh } => Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh { new_reference_q_p_kwh }),
        Din18599MutationDsl::UpdateClimate { new_climate } => Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate }),
    }
}

impl OpText for Din18599Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(din18599_mutation_from_dsl(<Din18599MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Din18599MutationDsl as OpText>::print_op(&din18599_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Din18599MutationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Din18599Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        din18599_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(din18599_mutation_from_dsl(Din18599MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_text_round_trips_change_use_class() {
        store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office }));
    }

    #[test]
    fn op_text_round_trips_update_climate() {
        store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: MonthlyClimate {
            theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0],
            g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0],
        } }));
    }

    /// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip over
    /// the closed vocabulary, one sample value per field.
    #[test]
    fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    fn every_mutation() -> Vec<Din18599Mutation> {
        vec![
            Din18599Mutation::ChangeUseClass(change_use_class::mutation::ChangeUseClass { new_use_class: crate::artifacts::din18599::UseClass::Office }),
            Din18599Mutation::ChangeHeatedAreaM2(change_heated_area_m2::mutation::ChangeHeatedAreaM2 { new_heated_area_m2: 120.0 }),
            Din18599Mutation::ChangeOccupants(change_occupants::mutation::ChangeOccupants { new_occupants: 5 }),
            Din18599Mutation::ChangeHT(change_h_t::mutation::ChangeHT { new_h_t: 95.0 }),
            Din18599Mutation::ChangeHV(change_h_v::mutation::ChangeHV { new_h_v: 42.0 }),
            Din18599Mutation::ChangeInternalGainsWM2(change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2 { new_internal_gains_w_m2: 4.0 }),
            Din18599Mutation::ChangeSolarGainsKwh(change_solar_gains_kwh::mutation::ChangeSolarGainsKwh { new_solar_gains_kwh: 90.0 }),
            Din18599Mutation::ChangeSystemLossesKwh(change_system_losses_kwh::mutation::ChangeSystemLossesKwh { new_system_losses_kwh: 850.0 }),
            Din18599Mutation::ChangeRenewableKwh(change_renewable_kwh::mutation::ChangeRenewableKwh { new_renewable_kwh: 1600.0 }),
            Din18599Mutation::ChangeAnnualLimitKwh(change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh { new_annual_limit_kwh: 8000.0 }),
            Din18599Mutation::ChangeEnergyCarrier(change_energy_carrier::mutation::ChangeEnergyCarrier { new_energy_carrier: "district_heat".to_string() }),
            Din18599Mutation::ChangeReferenceQPKwh(change_reference_q_p_kwh::mutation::ChangeReferenceQPKwh { new_reference_q_p_kwh: 10500.0 }),
            Din18599Mutation::UpdateClimate(update_climate::mutation::UpdateClimate { new_climate: crate::artifacts::din18599::MonthlyClimate {
                theta_e_c: [-12.0, -9.0, -2.0, 6.0, 15.0, 22.0, 25.0, 24.0, 18.0, 9.0, -1.0, -8.0],
                g_h_w_m2: [25.0, 55.0, 95.0, 135.0, 175.0, 195.0, 205.0, 185.0, 135.0, 85.0, 35.0, 18.0],
            } }),
        ]
    }
}
//#endregion 🧪️Tests
