use pack::snapshot::{ArtifactDsl, ArtifactPack};
use std::{env, fs, path::Path, process::ExitCode};

fn canonical<S: ArtifactDsl + ArtifactPack>(source: &str, bytes: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let snapshot = S::parse_dsl(source).map_err(|error| error.to_string())?;
    let text = snapshot.print_dsl();
    if S::decode_pack(bytes).is_ok_and(|decoded| decoded.print_dsl() == text) {
        return Ok(None);
    }
    let encoded = snapshot.encode_pack();
    let decoded = S::decode_pack(&encoded).map_err(|error| error.to_string())?;
    if decoded.print_dsl() != text {
        return Err("canonical Pack encoding changed the DSL snapshot".into());
    }
    Ok(Some(encoded))
}

macro_rules! models {
    ($family:expr, $source:expr, $bytes:expr, $(($flag:literal, $ty:path)),+) => {
        match $family {
            $(#[cfg(feature = $flag)] $flag => canonical::<$ty>($source, $bytes),)+
            _ => Err(format!("no Pack model linked for {}", $family)),
        }
    };
}

fn audit(source: &str, bytes: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let family = source.split_whitespace().nth(1).and_then(|dialect| dialect.split('.').nth(1)).unwrap_or_default();
    models!(family, source, bytes,
        ("en1990", semio_s_artifact_norm_en1990_model::En1990Snapshot),
        ("en1991", semio_s_artifact_norm_en1991_model::En1991Snapshot),
        ("en1992", semio_s_artifact_norm_en1992_model::En1992Snapshot),
        ("en1993", semio_s_artifact_norm_en1993_model::En1993Snapshot),
        ("en1994", semio_s_artifact_norm_en1994_model::En1994Snapshot),
        ("en1995", semio_s_artifact_norm_en1995_model::En1995Snapshot),
        ("en1996", semio_s_artifact_norm_en1996_model::En1996Snapshot),
        ("en1997", semio_s_artifact_norm_en1997_model::En1997Snapshot),
        ("en1998", semio_s_artifact_norm_en1998_model::En1998Snapshot),
        ("en1999", semio_s_artifact_norm_en1999_model::En1999Snapshot)
    )
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let emit = match args.next().as_deref() {
        Some("check") => false,
        Some("emit") => true,
        _ => return ExitCode::FAILURE,
    };
    let paths: Vec<_> = args.collect();
    assert!(paths.len().is_multiple_of(2));
    let mut clean = true;
    for (index, entry) in paths.chunks_exact(2).enumerate() {
        let dsl = Path::new(&entry[0]);
        let pack = Path::new(&entry[1]);
        let text = fs::read_to_string(dsl).expect("curated DSL");
        let bytes = fs::read(pack).expect("curated binary Pack file");
        match audit(&text, &bytes) {
            Ok(None) => println!("[DEBUG] PASS {index} {}", pack.display()),
            Ok(Some(encoded)) if emit => {
                assert_eq!(text, fs::read_to_string(dsl).expect("current DSL"));
                assert_eq!(bytes, fs::read(pack).expect("current Pack"));
                println!("[DEBUG] ENCODED {index} {} {}", hex(&bytes), hex(&encoded));
            }
            Ok(Some(_)) => {
                clean = false;
                println!("[DEBUG] DIFFER {index} {}", pack.display());
            }
            Err(error) => {
                clean = false;
                println!("[DEBUG] FAIL {index} {} {error}", pack.display());
            }
        }
    }
    if clean { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
