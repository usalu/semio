//! 🦀️ Bitmap mount contract — Rust adapter, the SUBJECT half.
//!
//! Measures what the subset really mounts — the editor and viewer definitions the plugin builder produces, the OS
//! artifact kind, the inference tool id, the mutation roster and the bundled example builders — against the committed
//! language-agnostic statement `shared://🧩️mount-contract/🔣️.json`, and answers with what it measured. The surfaces are
//! gated behind `component-app-assembly`, which this subset's contribution declares as the subject host's feature.
//!
//! @see ../../🔮️oracles/🔣️.json — `subjectFeatures`

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_wfc_bitmap::examples::{flowers_24, rooms_16};
    use semio_s_artifact_wfc_bitmap::schema::snapshot::text::{parse_dsl, print_dsl};
    use semio_s_artifact_wfc_bitmap::{artifact_kind, editor, inferences, mutations, viewer};

    const CONTRACT: &str = "shared://🧩️mount-contract/🔣️.json";

    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }

    fn texts(values: &[String]) -> Json {
        Json::Array(values.iter().map(|value| text(value)).collect())
    }

    fn answer(entries: Vec<(&str, Json)>) -> Outcome {
        let projection = Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        Outcome::with_raw(projection.to_string().into_bytes(), projection)
    }

    fn agree(what: &str, measured: &Json, committed: Json) -> Result<(), String> {
        if *measured == committed {
            Ok(())
        } else {
            Err(format!("{what}: the subset mounts {} but the committed contract states {}", measured.to_string(), committed.to_string()))
        }
    }

    pub fn surface_ids(ctx: &Context) -> Result<Outcome, String> {
        let contract = ctx.fixture_json(CONTRACT)?;
        let measured = [
            ("editorAppId", text(&editor::bitmap::create_bitmap_editor().id)),
            ("viewerAppId", text(&viewer::bitmap::create_bitmap_viewer().id)),
            ("artifactKindId", text(&artifact_kind().id)),
            ("inferenceToolId", text(inferences::BITMAP_INFERENCE_TOOL_ID)),
        ];
        for (key, value) in &measured {
            agree(key, value, text(&contract.str(key)))?;
        }
        Ok(answer(measured.into_iter().collect()))
    }

    pub fn window_kinds(ctx: &Context) -> Result<Outcome, String> {
        let committed = Json::Array(ctx.fixture_json(CONTRACT)?.array("windowKindIds"));
        let editor = texts(&editor::bitmap::create_bitmap_editor().window_kinds.iter().map(|window| window.id.clone()).collect::<Vec<_>>());
        let viewer = texts(&viewer::bitmap::create_bitmap_viewer().window_kinds.iter().map(|window| window.id.clone()).collect::<Vec<_>>());
        agree("editor window kinds", &editor, committed.clone())?;
        agree("viewer window kinds", &viewer, committed)?;
        Ok(answer(vec![("editor", editor), ("viewer", viewer)]))
    }

    pub fn mutation_vocabulary(ctx: &Context) -> Result<Outcome, String> {
        let kinds = texts(&mutations::KINDS.iter().map(|kind| kind.to_string()).collect::<Vec<_>>());
        agree("mutation roster", &kinds, Json::Array(ctx.fixture_json(CONTRACT)?.array("mutationKinds")))?;
        Ok(answer(vec![("mutationKinds", kinds)]))
    }

    pub fn examples(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_json(CONTRACT)?.array("examples");
        let mut rows = Vec::new();
        for (id, snapshot) in [("rooms-16", rooms_16::snapshot()), ("flowers-24", flowers_24::snapshot())] {
            let reparsed = parse_dsl(&print_dsl(&snapshot)).map_err(|error| format!("{id}: the printed DSL does not parse back: {error:?}"))?;
            if reparsed != snapshot {
                return Err(format!("{id}: printing the example to DSL and parsing it back changed it"));
            }
            let row = Json::Object(vec![
                ("id".to_string(), text(id)),
                ("inputWidth".to_string(), Json::Number(f64::from(snapshot.input.width))),
                ("inputHeight".to_string(), Json::Number(f64::from(snapshot.input.height))),
                ("paletteSize".to_string(), Json::Number(snapshot.input.palette.len() as f64)),
            ]);
            let statement = committed.iter().find(|entry| entry.str("id") == id).ok_or_else(|| format!("the committed contract lists no example {id}"))?;
            for key in ["inputWidth", "inputHeight", "paletteSize"] {
                agree(&format!("{id}.{key}"), &row.get(key).cloned().unwrap_or(Json::Null), statement.get(key).cloned().unwrap_or(Json::Null))?;
            }
            rows.push(row);
        }
        if rows.len() != committed.len() {
            return Err(format!("the subset bundles {} examples but the committed contract lists {}", rows.len(), committed.len()));
        }
        Ok(answer(vec![("examples", Json::Array(rows))]))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, one handler per scenario id. The subject half is
/// `sut`-gated so a build without the subset never compiles it.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("surface-ids", subject::surface_ids).subject("window-kinds", subject::window_kinds).subject("mutation-vocabulary", subject::mutation_vocabulary).subject("examples", subject::examples);
    built
}
//#endregion 🔖️Registration
