//! 🗺️ The editor's mounted borrowed Map media projection.
use super::{EditorApp, Gis2dPlayApp, GisMapSnapshot, MapFeature, MediaClass, MediaForm, MediaType};
use crate::store;
use semio_framework_pack_json::JsonWriteSource;
use semio_framework_value::ValueError;
use semio_plugin::{ArtifactToolPublicationContract, ArtifactToolPublicationLane, structured_media_export::StructuredMediaSource};
pub(super) use semio_plugin::structured_media_export::{StructuredMediaExport, StructuredMediaExportFactory};

#[path = "🪟️view/🦀️.rs"]
mod view;
#[path = "✅️validation/🦀️.rs"]
mod validation;
use view::MapJsonView;
use validation::MapJsonValidation;

pub(super) struct MapMedia;
impl StructuredMediaSource for MapMedia {
    type App = EditorApp<Gis2dPlayApp>;
    type Projection = MapJsonValidation;
    const TOOL_IDS: &'static [&'static str] = &["export-media:map:out"];
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "export-media:map:out", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
    const MAX_BYTES: usize = store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
    const OVERSIZE: &'static str = "map exceeds the bounded output size";
    const PREPARATION_COPY_BYTES: usize = 128 + 4 * size_of::<usize>() + size_of::<bool>();

    fn descriptor(port: &str) -> Option<(MediaType, &'static str)> {
        (port == "map:out").then_some((MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, "2d.map"))
    }

    fn prepare(_: &str, snapshot: &GisMapSnapshot, state: &mut MapJsonValidation) -> Result<bool, &'static str> {
        state.step(&MapJsonView(snapshot)).map_err(|_| "map data cannot be represented exactly as JSON")
    }

    fn borrow<'a>(_: &str, snapshot: &'a GisMapSnapshot, _: &'a MapJsonValidation) -> Result<impl JsonWriteSource + 'a, ValueError> {
        Ok(MapJsonView(snapshot))
    }
}
