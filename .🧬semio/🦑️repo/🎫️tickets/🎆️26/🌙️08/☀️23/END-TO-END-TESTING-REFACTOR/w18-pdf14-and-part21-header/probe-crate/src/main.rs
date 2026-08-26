//! 🔬 Ticket-local probe: runs BOTH halves of every scenario of `mutate-pdf-1-4`,
//! `mutate-pdf-1-4-a` and `mutate-pdf-1-4-x` against the real thesis and prints the first
//! divergence per row — the same comparison the runner makes, without the 40-minute build.
use semio_repo_test_host::{parse_json, Json};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::a::schema::mutations::{apply_a_conformance_mutation, stamp_conformance as a_stamp, PdfA1Mutation};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::io::{decode_pdf, encode_pdf};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::x::schema::mutations::{apply_x_conformance_mutation, stamp_conformance as x_stamp, PdfX1Mutation};
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::a as a_oracle;
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any as any_oracle;
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::x as x_oracle;
use semio_s_plugin_stdio_test_oracle::law::{divergence_within, feature_rows};

const ROOT: &str = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf";
const FIXTURE: &str = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";
const FREEDOM: &[&str] = &["objectNumber", "xrefOffset", "producer", "creationDate", "modificationDate", "documentId", "fileSize", "byteLength", "generation", "streamFilter", "streamLength"];

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

fn rows(case: &str) -> Vec<(String, Json)> {
    let text = std::fs::read_to_string(format!("{ROOT}/🧪️tests/{case}/component.feature")).expect("feature");
    feature_rows(&text)
}

fn tally(label: &str, results: &[(String, Result<(), String>)]) {
    let ok = results.iter().filter(|(_, r)| r.is_ok()).count();
    println!("\n=== {label}: {ok}/{} ===", results.len());
    for (id, r) in results {
        match r {
            Ok(()) => println!("  ✅ {id}"),
            Err(why) => println!("  ❌ {id}: {why}"),
        }
    }
}

fn compare(subject: &Json, oracle: &Json) -> Result<(), String> {
    match divergence_within(subject, oracle, FREEDOM, 0.0001) {
        None => Ok(()),
        Some(first) => Err(first),
    }
}

fn main() {
    let input = std::fs::read(FIXTURE).expect("fixture");
    let mut out: Vec<(String, Result<(), String>)> = Vec::new();

    // ── ✳️any ────────────────────────────────────────────────────────────────
    let any_pages = |value: &Json| -> Vec<PageDoc> {
        let items = match value.get("params").and_then(|p| p.get("snapshot")).and_then(|s| s.get("pages")) {
            Some(Json::Array(items)) => items.clone(),
            _ => Vec::new(),
        };
        let n = |page: &Json, key: &str, fallback: f64| match page.get(key) {
            Some(Json::Number(v)) => *v,
            _ => fallback,
        };
        items.iter().map(|p| PageDoc { width: n(p, "width", 612.0), height: n(p, "height", 792.0), text: p.str("text") }).collect()
    };
    for (id, params) in rows("mutate-pdf-1-4") {
        let s = spec(&id, params.clone());
        let mutation = match id.as_str() {
            "no-mutation" => PdfMutation::NoMutation,
            _ => PdfMutation::SetSnapshot { snapshot: PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: any_pages(&s) } },
        };
        let base = decode_pdf(&input).expect("decode");
        // mutate
        let mut snap = base.clone();
        apply_pdf_mutation(&mut snap, &mutation);
        let subject = any_oracle::project_pdf_1_4(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = any_oracle::project_pdf_1_4(&any_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply")).expect("project");
        out.push((format!("mutate-{id}"), compare(&subject, &oracle)));
        // inverse
        let mut snap = base.clone();
        apply_pdf_mutation(&mut snap, &mutation);
        let undo = match mutation {
            PdfMutation::NoMutation => PdfMutation::NoMutation,
            PdfMutation::SetSnapshot { .. } => PdfMutation::SetSnapshot { snapshot: base.clone() },
        };
        apply_pdf_mutation(&mut snap, &undo);
        let subject = any_oracle::project_pdf_1_4(&encode_pdf(&snap).expect("encode")).expect("project");
        let mutated = any_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply");
        let oracle = any_oracle::project_pdf_1_4(&any_oracle::oracle_apply_mutation(&mutated, &any_oracle::oracle_inverse_spec(&input, &s).expect("inverse spec")).expect("oracle undo")).expect("project");
        out.push((format!("inverse-{id}"), compare(&subject, &oracle)));
    }
    {
        let snap = decode_pdf(&input).expect("decode");
        let subject = any_oracle::project_pdf_1_4(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = any_oracle::project_pdf_1_4(&any_oracle::oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).expect("oracle")).expect("project");
        out.push(("identity-round-trip".to_string(), compare(&subject, &oracle)));
    }
    tally("mutate-pdf-1-4 (✳️any)", &out);

    // ── ✳️a ──────────────────────────────────────────────────────────────────
    let mut out: Vec<(String, Result<(), String>)> = Vec::new();
    let a_mutation = |id: &str, params: &Json, base: &PdfSnapshot| -> PdfA1Mutation {
        match id {
            "no-mutation" => PdfA1Mutation::NoMutation,
            "set-snapshot" => PdfA1Mutation::SetSnapshot { snapshot: a_stamp(base.clone(), params.str("conformance") == "stamped") },
            "set-page-text" => PdfA1Mutation::SetPageText { text: params.str("text") },
            _ => PdfA1Mutation::ClearPageText,
        }
    };
    for (id, params) in rows("mutate-pdf-1-4-a") {
        let s = spec(&id, params.clone());
        let base = decode_pdf(&input).expect("decode");
        let mut snap = base.clone();
        let m = a_mutation(&id, &params, &base);
        apply_a_conformance_mutation(&mut snap, &m);
        let subject = a_oracle::project_conformance(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = a_oracle::project_conformance(&a_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply")).expect("project");
        out.push((format!("mutate-{id}"), compare(&subject, &oracle)));

        let undo_spec = a_oracle::oracle_inverse_spec(&input, &s).expect("inverse spec");
        let undo_params = undo_spec.get("params").cloned().unwrap_or(Json::Null);
        let mut snap2 = base.clone();
        apply_a_conformance_mutation(&mut snap2, &m);
        let m2 = a_mutation(&undo_spec.str("kind"), &undo_params, &snap2.clone());
        apply_a_conformance_mutation(&mut snap2, &m2);
        let subject = a_oracle::project_conformance(&encode_pdf(&snap2).expect("encode")).expect("project");
        let mutated = a_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply");
        let oracle = a_oracle::project_conformance(&a_oracle::oracle_apply_mutation(&mutated, &undo_spec).expect("oracle undo")).expect("project");
        out.push((format!("inverse-{id}"), compare(&subject, &oracle)));
    }
    {
        let snap = decode_pdf(&input).expect("decode");
        let subject = a_oracle::project_conformance(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = a_oracle::project_conformance(&a_oracle::oracle_round_trip(&input).expect("oracle rt")).expect("project");
        out.push(("identity-round-trip".to_string(), compare(&subject, &oracle)));
    }
    tally("mutate-pdf-1-4-a (✳️a)", &out);

    // ── ✳️x ──────────────────────────────────────────────────────────────────
    let mut out: Vec<(String, Result<(), String>)> = Vec::new();
    let x_mutation = |id: &str, params: &Json, base: &PdfSnapshot| -> PdfX1Mutation {
        let n = |key: &str| match params.get(key) {
            Some(Json::Number(v)) => *v,
            _ => 0.0,
        };
        match id {
            "no-mutation" => PdfX1Mutation::NoMutation,
            "set-snapshot" => PdfX1Mutation::SetSnapshot { snapshot: x_stamp(base.clone(), params.str("conformance") == "stamped") },
            "set-page-size" => PdfX1Mutation::SetPageSize { width: n("width"), height: n("height") },
            _ => PdfX1Mutation::CollapsePageSize,
        }
    };
    for (id, params) in rows("mutate-pdf-1-4-x") {
        let s = spec(&id, params.clone());
        let base = decode_pdf(&input).expect("decode");
        let mut snap = base.clone();
        let m = x_mutation(&id, &params, &base);
        apply_x_conformance_mutation(&mut snap, &m);
        let subject = x_oracle::project_conformance(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = x_oracle::project_conformance(&x_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply")).expect("project");
        out.push((format!("mutate-{id}"), compare(&subject, &oracle)));

        let undo_spec = x_oracle::oracle_inverse_spec(&input, &s).expect("inverse spec");
        let undo_params = undo_spec.get("params").cloned().unwrap_or(Json::Null);
        let mut snap2 = base.clone();
        apply_x_conformance_mutation(&mut snap2, &m);
        let m2 = x_mutation(&undo_spec.str("kind"), &undo_params, &snap2.clone());
        apply_x_conformance_mutation(&mut snap2, &m2);
        let subject = x_oracle::project_conformance(&encode_pdf(&snap2).expect("encode")).expect("project");
        let mutated = x_oracle::oracle_apply_mutation(&input, &s).expect("oracle apply");
        let oracle = x_oracle::project_conformance(&x_oracle::oracle_apply_mutation(&mutated, &undo_spec).expect("oracle undo")).expect("project");
        out.push((format!("inverse-{id}"), compare(&subject, &oracle)));
    }
    {
        let snap = decode_pdf(&input).expect("decode");
        let subject = x_oracle::project_conformance(&encode_pdf(&snap).expect("encode")).expect("project");
        let oracle = x_oracle::project_conformance(&x_oracle::oracle_round_trip(&input).expect("oracle rt")).expect("project");
        out.push(("identity-round-trip".to_string(), compare(&subject, &oracle)));
    }
    tally("mutate-pdf-1-4-x (✳️x)", &out);

    let _ = parse_json("{}");

    // ── 🔤 the literal-string byte fix, on the exact shapes that used to break ────────────────
    {
        let original = PdfSnapshot {
            schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
            pages: vec![
                PageDoc { width: 612.0, height: 792.0, text: "Hello Semio".into() },
                PageDoc { width: 595.276, height: 841.89, text: "Zweite Seite (mit Klammern)".into() },
                PageDoc { width: 200.0, height: 300.0, text: String::new() },
                PageDoc { width: 300.0, height: 400.0, text: "Gr\u{fc}\u{df}e \u{fffd} \u{4e2d}\u{6587} \u{2014} a\\b(c)d".into() },
            ],
        };
        let bytes = encode_pdf(&original).expect("encode");
        let back = decode_pdf(&bytes).expect("decode");
        println!("\n=== literal-string retention ===");
        println!("decode(encode(x)).pages == x.pages : {}", back.pages == original.pages);
        for (i, (a, b)) in original.pages.iter().zip(&back.pages).enumerate() {
            if a != b {
                println!("  page {i} DIFFERS: wrote {:?} read {:?}", a.text, b.text);
            }
        }
    }

    // ── 📐 STEP Part-21 header conformance ───────────────────────────────────
    {
        use semio_s_plugin_stdio::artifacts::step::engine::part21::write_part21;
        use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::any::schema::snapshot::StepSnapshot;
        let text = write_part21(&StepSnapshot::default().to_part21_document());
        let header: String = text.lines().take(6).collect::<Vec<_>>().join("\n");
        println!("\n=== STEP: write_part21(StepSnapshot::default()) header ===\n{header}");
        let bytes = text.clone().into_bytes();
        for (label, projected) in [
            ("cc1", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc1::project_step_ap214_cc1(&bytes)),
            ("cc2", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc2::project_step_ap214_cc2(&bytes)),
            ("cc3", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc3::project_step_ap214_cc3(&bytes)),
            ("cc4", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc4::project_step_ap214_cc4(&bytes)),
            ("cc5", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc5::project_step_ap214_cc5(&bytes)),
            ("cc6", semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc6::project_step_ap214_cc6(&bytes)),
        ] {
            match projected {
                Ok(_) => println!("[step::{label}] ruststep ACCEPTS the header this writer emits"),
                Err(why) => println!("[step::{label}] ruststep REFUSES it: {why}"),
            }
        }
        // 🈳 the same defect, reached from the other direction: a caller that hands the writer a
        // COMPLETELY EMPTY header (what `Ifc2x3Mutation::SetHeader { header: Part21Header { .. } }`
        // can carry) must still come out conformant — the fix belongs at the writer, not at each
        // caller.
        use semio_s_plugin_stdio::artifacts::step::engine::part21::{Part21Document, Part21Header};
        let bare = write_part21(&Part21Document { header: Part21Header { file_description: vec![], file_name: vec![], file_schema: vec![] }, instances: vec![] });
        println!("\n=== STEP: write_part21 of a COMPLETELY EMPTY header ===\n{}", bare.lines().take(6).collect::<Vec<_>>().join("\n"));
        match semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc1::project_step_ap214_cc1(bare.as_bytes()) {
            Ok(_) => println!("[step::empty-header] ruststep ACCEPTS it"),
            Err(why) => println!("[step::empty-header] ruststep REFUSES it: {why}"),
        }
    }

    // 🎬 committed demo assets
    let demo = semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::demo_pdf_snapshot();
    let bytes = encode_pdf(&demo).expect("encode demo");
    println!("\n=== demo ===\npages={} bytes={}", demo.pages.len(), bytes.len());
    let dsl = <PdfSnapshot as store::ArtifactDsl>::print_dsl(&demo);
    let pack = <PdfSnapshot as store::ArtifactPack>::encode_pack(&demo);
    std::fs::write("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio", &dsl).expect("write dsl");
    std::fs::write("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio", &pack).expect("write pack");
    std::fs::write("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/📄️example.pdf", &bytes).expect("write pdf");
    println!("rewrote demo assets: dsl={} pack={} pdf={}", dsl.len(), pack.len(), bytes.len());
    // 🔁 re-read them the way the honesty law does
    let back = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse_dsl");
    println!("parse_dsl(print_dsl(demo)) == demo : {}", back == demo);
    let back = <PdfSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode_pack");
    println!("decode_pack(encode_pack(demo)) == demo : {}", back == demo);
}
