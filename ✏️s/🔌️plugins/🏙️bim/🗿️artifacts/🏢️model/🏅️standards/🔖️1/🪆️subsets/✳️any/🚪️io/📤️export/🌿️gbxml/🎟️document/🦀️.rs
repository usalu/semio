//! 📄️ Reads the canonical exchange document from caller-owned gbXML indexes.

use super::*;
use super::markup::with_number_text;
use super::original_materials::layers;
use super::original_surfaces::{envelope, surface};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, retained_clone::ordered_map::RetainedOrderedMap};
use semio_os_kernel::InferenceWorkFailure;
use semio_plugin::app::ArtifactOutputChunks;
use std::cmp::Ordering;

#[path = "📇️.rs"]
mod index;
use index::{BuildingAreas, Ranked, UsedOpenings};

#[path = "🌐️.rs"]
mod tree;

fn index_of<T>(map: &RetainedOrderedMap<String, T>, id: &str) -> Option<usize> {
    let (mut first, mut end) = (0, map.len());
    while first < end {
        let mid = first + (end - first) / 2;
        match map.get_index(mid)?.0.as_str().cmp(id) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => first = mid + 1,
            Ordering::Greater => end = mid,
        }
    }
    None
}

fn room_parts<'a>(model: &'a ModelSnapshot, inferred: &'a ModelInference, room: usize) -> Option<(&'a EnvelopeSpace, &'a semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Storey)> {
    let envelope = envelope(model, inferred, room)?;
    Some((envelope, model.storeys.get(&envelope.storey)?))
}

fn curtain_collision(model: &ModelSnapshot, used: &UsedOpenings, id: &str) -> Option<usize> {
    let index = index_of(&model.curtain_wall_types, id.strip_prefix("curtain:")?)?;
    used.curtain(index).then_some(index)
}

fn with_count<R>(value: impl std::fmt::Display, use_value: impl FnOnce(&str) -> R) -> R {
    use std::fmt::Write;
    struct Text { buffer: [u8; 24], len: usize }
    impl Write for Text {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            let end = self.len + text.len();
            self.buffer.get_mut(self.len..end).ok_or(std::fmt::Error)?.copy_from_slice(text.as_bytes());
            self.len = end;
            Ok(())
        }
    }
    let mut text = Text { buffer: [0; 24], len: 0 };
    write!(&mut text, "{value}").expect("an integer fits in 24 decimal bytes");
    use_value(std::str::from_utf8(&text.buffer[..text.len]).expect("decimal is UTF-8"))
}

#[derive(Clone, Copy)]
enum Id {
    Campus, Building, Storey, Space, Zone, Opaque, Curtain, OpeningType, Schematic, Material, Surface, Opening,
}

const IDS: &[Id] = &[Id::Campus, Id::Building, Id::Storey, Id::Space, Id::Zone, Id::Opaque, Id::Curtain, Id::OpeningType, Id::Schematic, Id::Material, Id::Surface, Id::Opening];

impl Id {
    fn prefix(self) -> &'static str {
        match self {
            Self::Campus => "campus-", Self::Building => "building-", Self::Storey => "storey-",
            Self::Space => "space-", Self::Zone => "zone-", Self::Opaque | Self::Schematic => "construction-",
            Self::Curtain => "windowtype-", Self::OpeningType => "", Self::Material => "mat-",
            Self::Surface => "surface-", Self::Opening => "opening-",
        }
    }
}

#[derive(Default)]
struct PlanIndex {
    surfaces: OriginalSurfaces,
    openings: OriginalOpenings,
    zones: OriginalZones,
    constructions: OriginalConstructions,
    materials: OriginalMaterials,
    areas: BuildingAreas,
    used: UsedOpenings,
    storeys: Ranked,
    windows: Ranked,
    stage: usize,
}

/// 📦️ Bounded original gbXML planning, encoding, delivery and retirement.
#[derive(Default)]
pub struct OriginalGbxml {
    plan: PlanIndex,
    names: OriginalNames,
    layers: Ranked,
    pages: GbxmlPages,
    stage: usize,
}

struct View<'a> {
    model: &'a ModelSnapshot,
    inferred: &'a ModelInference,
    plan: &'a PlanIndex,
    names: Option<&'a OriginalNames>,
    layers: Option<&'a Ranked>,
}

type ResultFor<T> = Result<T, &'static str>;

impl View<'_> {
    fn count(&self, id: Id) -> usize {
        let model = self.model;
        let plan = self.plan;
        match id {
            Id::Campus => model.sites.len(), Id::Building => model.buildings.len(), Id::Storey => model.storeys.len(),
            Id::Space => model.spaces.len(), Id::Zone => plan.zones.len(), Id::Opaque => plan.constructions.opaque_len(),
            Id::Curtain => model.curtain_wall_types.len(), Id::OpeningType => model.openings.len(), Id::Schematic => usize::from(plan.constructions.has_schematic()),
            Id::Material => plan.materials.len(), Id::Surface => plan.surfaces.extent(), Id::Opening => plan.openings.len(),
        }
    }

    fn ordinal(&self, family: Id, index: usize) -> usize {
        IDS[..family as usize].iter().map(|&family| self.count(family)).sum::<usize>() + index
    }

    fn with_base<R>(&self, family: Id, index: usize, read: impl FnOnce(&[GbxmlTextPart<'_>]) -> R) -> Option<R> {
        use GbxmlTextPart::{Name, Text};
        let model = self.model;
        let plan = self.plan;
        let prefix = Text(family.prefix());
        macro_rules! raw_key { ($key:expr) => { read(&[prefix, Name($key)]) }; }
        Some(match family {
            Id::Campus => raw_key!(model.sites.get_index(index)?.0),
            Id::Building => raw_key!(model.buildings.get_index(index)?.0),
            Id::Storey => raw_key!(model.storeys.get_index(index)?.0),
            Id::Space => { room_parts(model, self.inferred, index)?; raw_key!(model.spaces.get_index(index)?.0) }
            Id::Zone => self.with_zone_cad(index, |cad| {
                let mut parts = [Text(""); 4];
                parts[0] = prefix;
                for (i, part) in cad.iter().enumerate() {
                    parts[i + 1] = match part { Text(raw) => Name(raw), _ => *part };
                }
                read(&parts[..cad.len() + 1])
            })?,
            Id::Opaque => {
                let (key, _, value) = self.opaque(index)?;
                with_number_text(value, |value| read(&[prefix, Name(key), Text("-"), Text(value)]))
            }
            Id::Curtain => {
                if !plan.used.curtain(index) { return None; }
                read(&[prefix, Text("curtain_"), Name(model.curtain_wall_types.get_index(index)?.0)])
            }
            Id::OpeningType => {
                let (_, opening) = model.openings.get_index(index)?;
                match &opening.kind {
                    OpeningKind::Window { window_type } if index_of(&model.window_types, window_type).and_then(|kind| plan.used.window(kind)) == Some(index) =>
                        read(&[Text("windowtype-"), Name(window_type)]),
                    OpeningKind::Door { door_type } if index_of(&model.door_types, door_type).and_then(|kind| plan.used.door(kind)) == Some(index) =>
                        read(&[Text("construction-door-"), Name(door_type)]),
                    _ => return None,
                }
            }
            Id::Schematic => raw_key!("unspecified"),
            Id::Material => {
                let layer = self.material(index)?;
                with_count((layer.thickness * 1e6).round() as i64, |thickness| read(&[prefix, Name(&layer.material), Text("-"), Text(thickness)]))
            }
            Id::Surface => raw_key!(&surface(model, self.inferred, plan.surfaces.get(index)?.surface)?.id),
            Id::Opening => {
                let row = plan.openings.row(index)?;
                let surface = surface(model, self.inferred, row.address)?;
                    if row.glazing { read(&[prefix, Name(&surface.id), Text("_glass")]) }
                else { raw_key!(&surface.element) }
            }
        })
    }

    fn with_zone_cad<R>(&self, index: usize, read: impl FnOnce(&[GbxmlTextPart<'_>]) -> R) -> Option<R> {
        use GbxmlTextPart::Text;
        let zone = self.plan.zones.get(self.model, self.inferred, index)?;
        Some(if let Some(variant) = zone.owner_variant {
            with_count(variant, |variant| read(&[Text(zone.owner), Text("#"), Text(variant)]))
        } else {
            read(&[Text(zone.owner.trim_start_matches(':'))])
        })
    }

    fn with_id<R>(&self, kind: Id, index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R> {
        let suffix = self.names?.suffix(self.ordinal(kind, index))?;
        self.with_base(kind, index, |base| {
            if suffix == 1 { return read(GbxmlText::Parts(base)); }
            with_count(suffix, |suffix| {
                use GbxmlTextPart::Text;
                let mut parts = [Text(""); 8];
                parts[..base.len()].copy_from_slice(base);
                parts[base.len()] = Text("-");
                parts[base.len() + 1] = Text(suffix);
                read(GbxmlText::Parts(&parts[..base.len() + 2]))
            })
        })
    }

    fn opaque(&self, index: usize) -> Option<(&str, std::result::Result<&str, &'static str>, f64)> {
        let model = self.model;
        let side = surface(model, self.inferred, self.plan.constructions.source(index)?)?;
        let holder = holder_of(model, side);
        let located = holder.and_then(|holder| layers_of(model, side, holder));
        let key = located.map_or("none", |(id, _)| id);
        let label = match (holder, located) {
            (Some(Holder::Wall(_)), Some((id, _))) => model.wall_types.get(id).map(|row| row.name.as_str()),
            (Some(Holder::Slab(_)), Some((id, _))) => model.slab_types.get(id).map(|row| row.name.as_str()),
            (Some(Holder::Roof(_)), Some((id, _))) => model.roof_types.get(id).map(|row| row.name.as_str()),
            (Some(Holder::Ceiling(_)), Some((id, _))) => model.ceiling_types.get(id).map(|row| row.name.as_str()),
            _ => None,
        };
        Some((key, label.ok_or_else(|| side.kind.name()), round9(side.u_value?)))
    }

    fn material(&self, index: usize) -> Option<&semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Layer> {
        self.plan.materials.layer_at(self.model, self.inferred, &self.plan.constructions, index)
    }
}

impl GbxmlNameSource for View<'_> {
    fn name_source_identity(&self) -> usize { std::ptr::from_ref(self.inferred) as usize }
    fn name_count(&self) -> usize { IDS.iter().map(|&family| self.count(family)).sum() }

    fn with_name<R>(&self, mut index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R> {
        for &kind in IDS {
            let count = self.count(kind);
            if index < count { return self.with_base(kind, index, |parts| read(GbxmlText::Parts(parts))); }
            index -= count;
        }
        None
    }
}

fn compare_text(left: GbxmlText<'_>, right: GbxmlText<'_>) -> Ordering {
    fn chars(text: GbxmlText<'_>) -> impl Iterator<Item = char> + '_ {
        let mut position = 0;
        std::iter::from_fn(move || loop {
            let (character, count) = text.next(position)?;
            position += count;
            if character.is_some() { return character; }
        })
    }
    chars(left).cmp(chars(right))
}

fn window_key<R>(model: &ModelSnapshot, used: &UsedOpenings, index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R> {
    use GbxmlTextPart::Text;
    let count = model.curtain_wall_types.len();
    Some(if index < count {
        if !used.curtain(index) { return None; }
        read(GbxmlText::Parts(&[Text("curtain:"), Text(model.curtain_wall_types.get_index(index)?.0)]))
    } else {
        let rank = index - count;
        used.window(rank)?;
        read(GbxmlText::Text(model.window_types.get_index(rank)?.0))
    })
}

impl PlanIndex {
    fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        let done = match self.stage {
            0 => self.surfaces.advance(model, inferred, cx)?,
            1 => self.openings.advance(model, inferred, &self.surfaces, cx)?,
            2 => self.zones.advance(model, inferred, cx)?,
            3 => self.constructions.advance(model, inferred, &self.surfaces, cx)?,
            4 => self.materials.advance(model, inferred, &self.constructions, cx)?,
            5 => self.areas.advance(model, inferred, cx)?,
            6 => self.used.advance(model, inferred, &self.surfaces, cx)?,
            7 => self.storeys.advance(model.storeys.len(), |a,b| {
                let one = model.storeys.get_index(a)?;
                let two = model.storeys.get_index(b)?;
                Some(one.1.level.cmp(&two.1.level).then_with(|| one.0.cmp(two.0)))
            }, cx)?,
            8 => self.windows.advance(model.curtain_wall_types.len() + model.window_types.len(), |a,b| {
                window_key(model, &self.used, a, |a| window_key(model, &self.used, b, |b| compare_text(a,b))).flatten()
            }, cx)?,
            _ => return Ok(true),
        };
        if done { self.stage += 1; }
        Ok(false)
    }
}

fn original_failure(failure: InferenceWorkFailure) -> OriginalTurnFailure {
    match failure {
        InferenceWorkFailure::Original(failure) => failure,
        other => ValueError::new(ValueRefusalKind::InvariantViolated, format!("{other:?}")).into(),
    }
}

impl OriginalGbxml {
    /// 📏️ Quotes the next physical encoding or sealed-output transfer.
    pub fn encoding_demand(&self) -> Option<RetirementDemand> {
        (self.stage >= 3).then(|| self.pages.work_demand())
    }

    /// 📬️ Accepts the runtime's original output queue.
    pub fn receive(&mut self, chunks: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.pages.receive(chunks, cx)
    }

    /// 🛠️ Performs one paid plan, encoding or publication operation.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML export was cancelled").into()); }
        if inferred.energy_envelopes.is_empty() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, NO_ENVELOPE).into()); }
        match self.stage {
            0 => {
                if self.plan.advance(model, inferred, cx).map_err(original_failure)? { self.stage += 1; }
            }
            1 => {
                let view = View { model, inferred, plan: &self.plan, names: None, layers: None };
                if self.names.advance(&view, cx).map_err(original_failure)? { self.stage += 1; }
            }
            2 => {
                let view = View { model, inferred, plan: &self.plan, names: Some(&self.names), layers: None };
                if self.layers.advance(self.plan.materials.len(), |a,b| {
                    view.with_id(Id::Material,a,|a| view.with_id(Id::Material,b,|b| compare_text(a,b))).flatten()
                }, cx).map_err(original_failure)? { self.stage += 1; }
            }
            _ => {
                let view = View { model, inferred, plan: &self.plan, names: Some(&self.names), layers: Some(&self.layers) };
                return self.pages.advance(&view, cx);
            }
        }
        Ok(false)
    }

    /// ✍️ The localized progress key of the current original export turn.
    pub fn stage(&self) -> &'static str {
        match (self.stage, self.plan.stage) {
            (0, 0) => "bim-export-gbxml-surfaces",
            (0, 1) => "bim-export-gbxml-openings",
            (0, 2) => "bim-export-gbxml-zones",
            (0, 3 | 4) => "bim-export-gbxml-constructions",
            (0, _) => "bim-export-gbxml-structure",
            (1 | 2, _) => "bim-export-gbxml-identifiers",
            _ => "bim-export-encode",
        }
    }

    /// 🎁️ Returns the sealed original queue to the runtime.
    pub fn take(&mut self, cx: &mut StepContext<'_>) -> Result<Option<ArtifactOutputChunks>, OriginalTurnFailure> { self.pages.take(cx) }
}

macro_rules! owners {
    ($self:ident, $($owner:expr),+) => {
        impl OriginalGbxml {
            /// ♻️ Quotes retirement of the current original owner.
            pub fn close_demand(&$self, bytes: usize) -> Result<RetirementDemand, ValueError> {
                $(if !$owner.terminal_is_empty() { return $owner.close_demand(bytes); })+
                $self.pages.close_demand(bytes)
            }

            /// 🔚️ Releases one original owner under the caller's funded closing turn.
            pub fn close_step(&mut $self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
                $(if !$owner.terminal_is_empty() {
                    $owner.close_step(cx).map_err(original_failure)?;
                    return Ok($self.terminal_is_empty());
                })+
                $self.pages.close_step(cx)?;
                Ok($self.terminal_is_empty())
            }

            /// 🧺️ Whether the caller has recovered all retained planning and output owners.
            pub fn terminal_is_empty(&$self) -> bool {
                $($owner.terminal_is_empty() &&)+ $self.pages.terminal_is_empty()
            }
        }
    }
}

owners!(self, self.layers, self.names, self.plan.windows, self.plan.storeys, self.plan.used, self.plan.areas,
    self.plan.materials, self.plan.constructions, self.plan.zones, self.plan.openings, self.plan.surfaces);
