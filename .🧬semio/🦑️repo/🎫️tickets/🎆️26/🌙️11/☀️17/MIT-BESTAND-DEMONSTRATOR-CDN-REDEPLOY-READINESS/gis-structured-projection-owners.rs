mod map {
    use super::*;

    impl StructuredMediaSource for MapMedia {
        type App = EditorApp<Gis2dPlayApp>;
        type Projection = MapJsonValidation;
        const TOOL_IDS: &'static [&'static str] = &["export-media:map:out"];
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
            ArtifactToolPublicationContract { tool_id: "export-media:map:out", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ];
        const MAX_BYTES: usize = store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
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
}

mod terrain {
    use super::*;

    enum TerrainJsonView<'a> {
        Elevation(ElevationJsonView<'a>),
        Scene(TerrainSceneView<'a>),
    }

    impl JsonWriteSource for TerrainJsonView<'_> {
        fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
            match self {
                Self::Elevation(view) => view.node_at_path(path),
                Self::Scene(view) => view.node_at_path(path),
            }
        }

        fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
            match self {
                Self::Elevation(view) => view.object_key_at_path(path, index),
                Self::Scene(view) => view.object_key_at_path(path, index),
            }
        }
    }

    impl StructuredMediaSource for TerrainMedia {
        type App = EditorApp<Gis3dPlayApp>;
        type Projection = TerrainSceneRange;
        const TOOL_IDS: &'static [&'static str] = TOOLS;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
            ArtifactToolPublicationContract { tool_id: TOOLS[0], lanes: &[ArtifactToolPublicationLane::HostOnly] },
            ArtifactToolPublicationContract { tool_id: TOOLS[1], lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ];
        const MAX_BYTES: usize = 1_048_576;
        const PREPARATION_COPY_BYTES: usize = 33 * size_of::<i16>() + size_of::<TerrainSceneRange>();

        fn descriptor(port: &str) -> Option<(MediaType, &'static str)> {
            match port {
                "elevation:out" => Some((GIS_ELEVATION_MEDIA_TYPE, GIS_ELEVATION_GRID_SCHEMA)),
                "scene:out" => Some((MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, "mesh.document")),
                _ => None,
            }
        }

        fn prepare(port: &str, snapshot: &GisTerrainSnapshot, state: &mut TerrainSceneRange) -> Result<bool, &'static str> {
            let Some(source) = snapshot.elevation_source.as_ref() else {
                return Err(if port == "elevation:out" { "no authored elevation source" } else { "This terrain has no elevation source" });
            };
            let [west, south, east, north] = source.bounds;
            if !((-180.0..east).contains(&west) && east <= 180.0 && (-90.0..north).contains(&south) && north <= 90.0) {
                return Err(if port == "elevation:out" { "no authored elevation source" } else { "The authored terrain elevation grid is invalid" });
            }
            if port == "scene:out" && !(0.0..=f64::from(f32::MAX) / f64::from(i16::MIN.unsigned_abs())).contains(&snapshot.exaggeration) {
                return Err("Terrain elevation cannot be projected in the defined world coordinates");
            }
            if port == "scene:out" && state.geometry.is_none() {
                let origin = terrain_grid_origin(source.bounds);
                state.geometry = Some(TerrainGridMeshProjection::from_bounds(source.bounds, [origin.lon, origin.lat], snapshot.exaggeration, 33));
                return Ok(false);
            }
            state.step(&source.rows).map_err(|_| if port == "elevation:out" { "no authored elevation source" } else { "The authored terrain elevation grid is invalid" })
        }

        fn borrow<'a>(port: &str, snapshot: &'a GisTerrainSnapshot, range: &'a TerrainSceneRange) -> Result<impl JsonWriteSource + 'a, ValueError> {
            let source = snapshot.elevation_source.as_ref()
                .ok_or_else(|| ValueError::invariant("the prepared terrain projection has no elevation source"))?;
            if port == "elevation:out" {
                return Ok(TerrainJsonView::Elevation(ElevationJsonView(source)));
            }
            let projection = range.geometry.ok_or_else(|| ValueError::invariant("the prepared terrain mesh has no geometry"))?;
            Ok(TerrainJsonView::Scene(TerrainSceneView { rows: &source.rows, projection, range: *range }))
        }
    }
}
