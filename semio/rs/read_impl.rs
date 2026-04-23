// Live `KitGraph` read execution. Included in `pub mod read` after `read_module.rs`.

use crate::attribute::AttributeStore;
use crate::attribute::AttributeStoreRef;
use crate::typ::TypeStore;
use crate::tag::TagStore;
use crate::tag::TagStoreRef;
use crate::{error::Result, error::SemioError};

// --- helpers ----------------------------------------------------------------

fn lp(s: &'static str) -> SemioError {
    SemioError::LockPoisoned(s)
}

fn nf(kind: &'static str, id: &Id) -> SemioError {
    SemioError::NotFound { kind, id: id.clone() }
}

fn kit_family(g: &KitGraph, id: &Id) -> Option<crate::family::FamilyStoreRef> {
    g.families.iter().find(|f| f.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}

fn kit_location(g: &KitGraph, id: &Id) -> Option<crate::location::LocationStoreRef> {
    g.locations.iter().find(|l| l.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}

fn kit_find_author(g: &KitGraph, id: &Id) -> Option<crate::author::AuthorStoreRef> {
    g.authors.iter().find(|a| a.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}
fn kit_find_concept(g: &KitGraph, id: &Id) -> Option<crate::concept::ConceptStoreRef> {
    g.concepts.iter().find(|c| c.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}
fn kit_find_tag(g: &KitGraph, id: &Id) -> Option<crate::tag::TagStoreRef> {
    g.tags.iter().find(|t| t.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}
fn kit_find_quality(g: &KitGraph, id: &Id) -> Option<crate::quality::QualityStoreRef> {
    g.qualities.iter().find(|q| q.read().map(|q| q.id == *id).unwrap_or(false)).cloned()
}
fn kit_find_prop(g: &KitGraph, id: &Id) -> Option<crate::prop::PropStoreRef> {
    g.props.iter().find(|p| p.read().map(|p| p.id == *id).unwrap_or(false)).cloned()
}
fn kit_find_attr(g: &KitGraph, id: &Id) -> Option<AttributeStoreRef> {
    g.attributes.iter().find(|a| a.read().map(|a| a.id == *id).unwrap_or(false)).cloned()
}

fn kit_type(g: &KitGraph, id: &Id) -> Option<TypeStoreRef> {
    g.types.iter().find(|t| t.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}
fn kit_design(g: &KitGraph, id: &Id) -> Option<DesignStoreRef> {
    g.designs.iter().find(|d| d.read().map(|r| r.id == *id).unwrap_or(false)).cloned()
}
fn type_all_ports(t: &TypeStore) -> Vec<PortFullDto> {
    let mut out = Vec::new();
    for fw in &t.families {
        let Some(f) = fw.upgrade() else { continue };
        let Ok(fr) = f.read() else { continue };
        for p in &fr.ports {
            if let Ok(pr) = p.read() {
                out.push(pr.to_full_dto());
            }
        }
    }
    out
}

// --- small entity executes ---------------------------------------------------

impl ReadStatCommand {
    pub fn execute(&self, s: &crate::stat::StatStoreRef) -> Result<ReadStatCommandOutput> {
        let s = s.read().map_err(|_| lp("stat"))?;
        Ok(match self {
            ReadStatCommand::ReadStatFullCommand => ReadStatCommandOutput::ReadStatFullCommand { stat: s.to_full_dto() },
            ReadStatCommand::ReadStatShallowCommand => ReadStatCommandOutput::ReadStatShallowCommand { stat: s.to_shallow_dto() },
            ReadStatCommand::ReadStatMetadataCommand => ReadStatCommandOutput::ReadStatMetadataCommand { metadata: s.to_metadata_dto() },
            ReadStatCommand::ReadStatIdCommand => ReadStatCommandOutput::ReadStatIdCommand { id: s.to_id_dto() },
            ReadStatCommand::ReadStatKeyCommand => ReadStatCommandOutput::ReadStatKeyCommand { key: s.key.clone() },
            ReadStatCommand::ReadStatValueCommand => ReadStatCommandOutput::ReadStatValueCommand { value: s.value.clone() },
            ReadStatCommand::ReadStatUnitCommand => ReadStatCommandOutput::ReadStatUnitCommand { unit: s.unit.clone() },
            ReadStatCommand::ReadStatDescriptionCommand => ReadStatCommandOutput::ReadStatDescriptionCommand { description: s.description.clone() },
        })
    }
}

impl ReadBenchmarkCommand {
    pub fn execute(&self, b: &crate::benchmark::BenchmarkStoreRef) -> Result<ReadBenchmarkCommandOutput> {
        let b = b.read().map_err(|_| lp("benchmark"))?;
        Ok(match self {
            ReadBenchmarkCommand::ReadBenchmarkFullCommand => ReadBenchmarkCommandOutput::ReadBenchmarkFullCommand { benchmark: b.to_full_dto() },
            ReadBenchmarkCommand::ReadBenchmarkShallowCommand => ReadBenchmarkCommandOutput::ReadBenchmarkShallowCommand { benchmark: b.to_shallow_dto() },
            ReadBenchmarkCommand::ReadBenchmarkMetadataCommand => ReadBenchmarkCommandOutput::ReadBenchmarkMetadataCommand { metadata: b.to_metadata_dto() },
            ReadBenchmarkCommand::ReadBenchmarkIdCommand => ReadBenchmarkCommandOutput::ReadBenchmarkIdCommand { id: b.to_id_dto() },
            ReadBenchmarkCommand::ReadBenchmarkNameCommand => ReadBenchmarkCommandOutput::ReadBenchmarkNameCommand { name: b.name.clone() },
            ReadBenchmarkCommand::ReadBenchmarkMinCommand => ReadBenchmarkCommandOutput::ReadBenchmarkMinCommand { min: b.min },
            ReadBenchmarkCommand::ReadBenchmarkMaxCommand => ReadBenchmarkCommandOutput::ReadBenchmarkMaxCommand { max: b.max },
            ReadBenchmarkCommand::ReadBenchmarkMinExcludedCommand => ReadBenchmarkCommandOutput::ReadBenchmarkMinExcludedCommand { min_excluded: b.min_excluded },
            ReadBenchmarkCommand::ReadBenchmarkMaxExcludedCommand => ReadBenchmarkCommandOutput::ReadBenchmarkMaxExcludedCommand { max_excluded: b.max_excluded },
        })
    }
}

impl ReadAttributeCommand {
    pub fn execute(&self, a: &AttributeStore) -> Result<ReadAttributeCommandOutput> {
        Ok(match self {
            ReadAttributeCommand::ReadAttributeFullCommand => ReadAttributeCommandOutput::ReadAttributeFullCommand { attribute: a.to_full_dto() },
            ReadAttributeCommand::ReadAttributeShallowCommand => ReadAttributeCommandOutput::ReadAttributeShallowCommand { attribute: a.to_shallow_dto() },
            ReadAttributeCommand::ReadAttributeMetadataCommand => ReadAttributeCommandOutput::ReadAttributeMetadataCommand { metadata: a.to_metadata_dto() },
            ReadAttributeCommand::ReadAttributeIdCommand => ReadAttributeCommandOutput::ReadAttributeIdCommand { id: a.to_id_dto() },
            ReadAttributeCommand::ReadAttributeKeyCommand => ReadAttributeCommandOutput::ReadAttributeKeyCommand { key: a.key.clone() },
            ReadAttributeCommand::ReadAttributeValueCommand => ReadAttributeCommandOutput::ReadAttributeValueCommand { value: a.value.clone() },
            ReadAttributeCommand::ReadAttributeDefinitionCommand => ReadAttributeCommandOutput::ReadAttributeDefinitionCommand { definition: a.definition.clone() },
        })
    }
    pub fn execute_ref(&self, a: &AttributeStoreRef) -> Result<ReadAttributeCommandOutput> {
        let a = a.read().map_err(|_| lp("attribute"))?;
        self.execute(&*a)
    }
}

impl ReadPropCommand {
    pub fn execute(&self, p: &crate::prop::PropStoreRef) -> Result<ReadPropCommandOutput> {
        let p = p.read().map_err(|_| lp("prop"))?;
        Ok(match self {
            ReadPropCommand::ReadPropFullCommand => ReadPropCommandOutput::ReadPropFullCommand { prop: p.to_full_dto() },
            ReadPropCommand::ReadPropShallowCommand => ReadPropCommandOutput::ReadPropShallowCommand { prop: p.to_shallow_dto() },
            ReadPropCommand::ReadPropIdCommand => ReadPropCommandOutput::ReadPropIdCommand { id: p.to_id_dto() },
            ReadPropCommand::ReadPropKeyCommand => ReadPropCommandOutput::ReadPropKeyCommand { key: p.key.clone() },
            ReadPropCommand::ReadPropValueCommand => ReadPropCommandOutput::ReadPropValueCommand { value: p.value.clone() },
            ReadPropCommand::ReadPropUnitCommand => ReadPropCommandOutput::ReadPropUnitCommand { unit: p.unit.clone() },
            ReadPropCommand::ReadPropQualityIdCommand => ReadPropCommandOutput::ReadPropQualityIdCommand { quality: p.to_full_dto().quality },
        })
    }
}

impl ReadTagCommand {
    pub fn execute(&self, t: &TagStore) -> Result<ReadTagCommandOutput> {
        Ok(match self {
            ReadTagCommand::ReadTagFullCommand => ReadTagCommandOutput::ReadTagFullCommand { tag: t.to_full_dto() },
            ReadTagCommand::ReadTagShallowCommand => ReadTagCommandOutput::ReadTagShallowCommand { tag: t.to_shallow_dto() },
            ReadTagCommand::ReadTagMetadataCommand => ReadTagCommandOutput::ReadTagMetadataCommand { metadata: t.to_metadata_dto() },
            ReadTagCommand::ReadTagIdCommand => ReadTagCommandOutput::ReadTagIdCommand { id: t.to_id_dto() },
            ReadTagCommand::ReadTagNameCommand => ReadTagCommandOutput::ReadTagNameCommand { name: t.name.clone() },
            ReadTagCommand::ReadTagOrderCommand => ReadTagCommandOutput::ReadTagOrderCommand { order: t.order },
        })
    }
    pub fn execute_ref(&self, t: &TagStoreRef) -> Result<ReadTagCommandOutput> {
        let t = t.read().map_err(|_| lp("tag"))?;
        self.execute(&*t)
    }
}

// ---- placeholder: remaining impls filled in read_impl2.rs include --------
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/read_impl2.rs"));
