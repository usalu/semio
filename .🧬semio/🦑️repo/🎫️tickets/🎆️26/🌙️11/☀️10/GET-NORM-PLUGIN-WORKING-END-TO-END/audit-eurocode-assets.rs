use pack::snapshot::ArtifactDsl;

fn audit<S: ArtifactDsl>(text: &str) -> Result<(), String> {
    let snapshot = S::parse_dsl(text).map_err(|error| error.to_string())?;
    let encoded = snapshot.print_dsl();
    let decoded = S::parse_dsl(&encoded).map_err(|error| format!("printed DSL: {error}"))?;
    if encoded != decoded.print_dsl() {
        return Err("canonical DSL did not round-trip".into());
    }
    Ok(())
}

#[cfg(feature = "en1998")]
fn en1998_reference(path: &str) -> Option<semio_s_artifact_norm_en1998_model::En1998Snapshot> {
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;

    let example = std::path::Path::new(path).parent()?.file_name()?.to_str()?;
    if example.ends_with("seismic-rc-frame-fail") {
        Some(En1998Snapshot::noncompliant_de_office().with_example_joints(true))
    } else if example.ends_with("seismic-rc-frame") {
        Some(En1998Snapshot::compliant_de_office().with_example_joints(false))
    } else if example.ends_with("seismic-multipart-fail") {
        Some(En1998Snapshot::noncompliant_de_multipart().with_example_joints(true))
    } else if example.ends_with("seismic-multipart") {
        Some(En1998Snapshot::compliant_de_multipart().with_example_joints(false))
    } else {
        None
    }
}

#[cfg(feature = "en1998")]
fn audit_en1998_examples(path: &str, text: &str) -> Result<(), String> {
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;

    let Some(expected) = en1998_reference(path) else { return Ok(()) };
    let decoded = En1998Snapshot::parse_dsl(text).map_err(|error| error.to_string())?;
    if decoded != expected {
        eprintln!("[DEBUG] expected EN 1998 constructor output for {path}:\n{}", expected.print_dsl());
        return Err("does not match its code-built example snapshot".into());
    }
    Ok(())
}

macro_rules! audit_families {
    ($family:expr, $text:expr, $(($flag:literal, $model:path)),+) => {
        match $family {
            $(#[cfg(feature = $flag)] $flag => audit::<$model>($text),)+
            _ => Err(format!("no typed model linked for {}", $family)),
        }
    };
}

fn main() -> std::process::ExitCode {
    let mut failures = 0;
    for path in std::env::args().skip(1) {
        let result = std::fs::read_to_string(&path).map_err(|error| error.to_string()).and_then(|text| {
            let family = text.split_whitespace().nth(1).and_then(|dialect| dialect.split('.').nth(1)).unwrap_or_default();
            audit_families!(family, &text,
                ("en1990", semio_s_artifact_norm_en1990_model::En1990Snapshot),
                ("en1991", semio_s_artifact_norm_en1991_model::En1991Snapshot),
                ("en1992", semio_s_artifact_norm_en1992_model::En1992Snapshot),
                ("en1993", semio_s_artifact_norm_en1993_model::En1993Snapshot),
                ("en1994", semio_s_artifact_norm_en1994_model::En1994Snapshot),
                ("en1995", semio_s_artifact_norm_en1995_model::En1995Snapshot),
                ("en1996", semio_s_artifact_norm_en1996_model::En1996Snapshot),
                ("en1997", semio_s_artifact_norm_en1997_model::En1997Snapshot),
                ("en1998", semio_s_artifact_norm_en1998_model::En1998Snapshot),
                ("en1999", semio_s_artifact_norm_en1999_model::En1999Snapshot),
                ("iso16757", semio_s_artifact_norm_iso16757_model::Iso16757Snapshot),
                ("vdi3805", semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot),
                ("din4108", semio_s_artifact_norm_din4108_model::Din4108Snapshot),
                ("din16798", semio_s_artifact_norm_din16798_model::Din16798Snapshot),
                ("din18599", semio_s_artifact_norm_din18599_model::Din18599Snapshot)
            )?;
            #[cfg(feature = "en1998")]
            if family == "en1998" {
                audit_en1998_examples(&path, &text)?;
            }
            Ok(())
        });
        match result {
            Ok(()) => println!("[DEBUG] PASS {path}"),
            Err(error) => {
                failures += 1;
                println!("[DEBUG] FAIL {path}: {error}");
            }
        }
    }
    if failures == 0 { std::process::ExitCode::SUCCESS } else { std::process::ExitCode::FAILURE }
}
