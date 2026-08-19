//! 🧬️ Din18599 snapshot schema — artifact-lane fields only.

use crate::artifacts::din18599::{Din18599ClimateChild, MonthlyClimate, UseClass};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot

/// 📸️ Persisted Din18599 document snapshot. Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
/// round 2 (`norm→C:table` on `din18599.climate`): the inline `MonthlyClimate` (two twelve-month
/// arrays) is replaced by a fixed composed `s.stdio.semio.table` CHILD slot — see
/// `🗿️artifacts/📙️din18599/🦀️component.rs`'s `🔖️Composition` region for the converters/
/// working-scene cache. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s slot-table emission;
/// never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
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
    #[child(kind = "s.stdio.semio.table")]
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

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `➗️mathematical`'s/en1990's own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened
/// via `to_uri()`), never the child's own content.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
async fn enc_child(c: &Din18599ClimateChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
async fn dec_child(s: &str) -> Result<Din18599ClimateChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
async fn enc_use_class(u: UseClass) -> &'static str {
    match u {
        UseClass::Residential => "residential",
        UseClass::Office => "office",
        UseClass::School => "school",
    }
}
async fn dec_use_class(s: &str) -> Result<UseClass, String> {
    match s {
        "residential" => Ok(UseClass::Residential),
        "office" => Ok(UseClass::Office),
        "school" => Ok(UseClass::School),
        other => Err(format!("bad use class {other:?}")),
    }
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
async fn print_din18599_snapshot_body(s: &Din18599Snapshot) -> String {
    format!(
        "useClass={}\nheatedAreaM2={}\noccupants={}\nhT={}\nhV={}\nclimate={}\ninternalGainsWM2={}\nsolarGainsKwh={}\nsystemLossesKwh={}\nrenewableKwh={}\nannualLimitKwh={}\nenergyCarrier={}\nreferenceQPKwh={}",
        enc_use_class(s.use_class),
        s.heated_area_m2,
        s.occupants,
        s.h_t,
        s.h_v,
        enc_child(&s.climate),
        s.internal_gains_w_m2,
        s.solar_gains_kwh,
        s.system_losses_kwh,
        s.renewable_kwh,
        s.annual_limit_kwh,
        enc_str(&s.energy_carrier),
        s.reference_q_p_kwh,
    )
}
async fn parse_din18599_snapshot_body(body: &str) -> Result<Din18599Snapshot, String> {
    let mut use_class = None;
    let mut heated_area_m2 = None;
    let mut occupants = None;
    let mut h_t = None;
    let mut h_v = None;
    let mut climate = None;
    let mut internal_gains_w_m2 = None;
    let mut solar_gains_kwh = None;
    let mut system_losses_kwh = None;
    let mut renewable_kwh = None;
    let mut annual_limit_kwh = None;
    let mut energy_carrier = None;
    let mut reference_q_p_kwh = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("useClass=") {
            use_class = Some(dec_use_class(rest)?);
        } else if let Some(rest) = line.strip_prefix("heatedAreaM2=") {
            heated_area_m2 = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("occupants=") {
            occupants = Some(rest.parse::<u32>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("hT=") {
            h_t = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("hV=") {
            h_v = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("climate=") {
            climate = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("internalGainsWM2=") {
            internal_gains_w_m2 = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("solarGainsKwh=") {
            solar_gains_kwh = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("systemLossesKwh=") {
            system_losses_kwh = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("renewableKwh=") {
            renewable_kwh = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("annualLimitKwh=") {
            annual_limit_kwh = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("energyCarrier=") {
            energy_carrier = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("referenceQPKwh=") {
            reference_q_p_kwh = Some(rest.parse::<f64>().map_err(|e| e.to_string())?);
        } else {
            return Err(format!("din18599 snapshot: unknown line {line:?}"));
        }
    }
    Ok(Din18599Snapshot {
        use_class: use_class.ok_or_else(|| "din18599 snapshot: missing useClass line".to_string())?,
        heated_area_m2: heated_area_m2.ok_or_else(|| "din18599 snapshot: missing heatedAreaM2 line".to_string())?,
        occupants: occupants.ok_or_else(|| "din18599 snapshot: missing occupants line".to_string())?,
        h_t: h_t.ok_or_else(|| "din18599 snapshot: missing hT line".to_string())?,
        h_v: h_v.ok_or_else(|| "din18599 snapshot: missing hV line".to_string())?,
        climate: climate.ok_or_else(|| "din18599 snapshot: missing climate line".to_string())?,
        internal_gains_w_m2: internal_gains_w_m2.ok_or_else(|| "din18599 snapshot: missing internalGainsWM2 line".to_string())?,
        solar_gains_kwh: solar_gains_kwh.ok_or_else(|| "din18599 snapshot: missing solarGainsKwh line".to_string())?,
        system_losses_kwh: system_losses_kwh.ok_or_else(|| "din18599 snapshot: missing systemLossesKwh line".to_string())?,
        renewable_kwh: renewable_kwh.ok_or_else(|| "din18599 snapshot: missing renewableKwh line".to_string())?,
        annual_limit_kwh: annual_limit_kwh.ok_or_else(|| "din18599 snapshot: missing annualLimitKwh line".to_string())?,
        energy_carrier: energy_carrier.ok_or_else(|| "din18599 snapshot: missing energyCarrier line".to_string())?,
        reference_q_p_kwh: reference_q_p_kwh.ok_or_else(|| "din18599 snapshot: missing referenceQPKwh line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
async fn write_child(out: &mut Vec<u8>, c: &Din18599ClimateChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child(reader: &mut store::ByteReader<'_>) -> Result<Din18599ClimateChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

async fn encode_din18599_snapshot_binary(s: &Din18599Snapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, enc_use_class(s.use_class));
    out.extend_from_slice(&s.heated_area_m2.to_le_bytes());
    out.extend_from_slice(&s.occupants.to_le_bytes());
    out.extend_from_slice(&s.h_t.to_le_bytes());
    out.extend_from_slice(&s.h_v.to_le_bytes());
    write_child(&mut out, &s.climate);
    out.extend_from_slice(&s.internal_gains_w_m2.to_le_bytes());
    out.extend_from_slice(&s.solar_gains_kwh.to_le_bytes());
    out.extend_from_slice(&s.system_losses_kwh.to_le_bytes());
    out.extend_from_slice(&s.renewable_kwh.to_le_bytes());
    out.extend_from_slice(&s.annual_limit_kwh.to_le_bytes());
    write_str_lp(&mut out, &s.energy_carrier);
    out.extend_from_slice(&s.reference_q_p_kwh.to_le_bytes());
    out
}
async fn decode_din18599_snapshot_binary(bytes: &[u8]) -> Result<Din18599Snapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    async fn read_f64(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
        Ok(f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "short f64".to_string())?))
    }
    async fn read_u32(reader: &mut store::ByteReader<'_>) -> Result<u32, String> {
        Ok(u32::from_le_bytes(reader.read_bytes(4).map_err(|e| e.to_string())?.try_into().map_err(|_| "short u32".to_string())?))
    }
    let use_class = dec_use_class(&read_str_lp(&mut reader)?)?;
    let heated_area_m2 = read_f64(&mut reader)?;
    let occupants = read_u32(&mut reader)?;
    let h_t = read_f64(&mut reader)?;
    let h_v = read_f64(&mut reader)?;
    let climate = read_child(&mut reader)?;
    let internal_gains_w_m2 = read_f64(&mut reader)?;
    let solar_gains_kwh = read_f64(&mut reader)?;
    let system_losses_kwh = read_f64(&mut reader)?;
    let renewable_kwh = read_f64(&mut reader)?;
    let annual_limit_kwh = read_f64(&mut reader)?;
    let energy_carrier = read_str_lp(&mut reader)?;
    let reference_q_p_kwh = read_f64(&mut reader)?;
    Ok(Din18599Snapshot { use_class, heated_area_m2, occupants, h_t, h_v, climate, internal_gains_w_m2, solar_gains_kwh, system_losses_kwh, renewable_kwh, annual_limit_kwh, energy_carrier, reference_q_p_kwh })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack, real hex/bracket text + fixed-width/LEB128 binary
/// primitives — same upgrade `➗️mathematical`/en1990 made once their snapshot gained a real
/// `ArtifactChild<S>` slot (the old `impl_norm_artifact_record!`/`dsl::DslRecord`-derive-driven path
/// cannot express a composed child slot, which has no `dsl::DslField` impl reachable from this
/// crate). The other fourteen norm families are unaffected — they have no composed child slot and
/// keep `impl_norm_artifact_record!` unchanged.
impl store::ArtifactDsl for Din18599Snapshot {
    const EXTENSION: &'static str = "din18599";
    async fn envelope_id() -> &'static str {
        "norm.din18599"
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_din18599_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_din18599_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Din18599Snapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_din18599_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_din18599_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din18599Snapshot {
    async fn default() -> Self {
        let climate = crate::artifacts::din18599::din18599_climate_child_from_data(&MonthlyClimate {
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
