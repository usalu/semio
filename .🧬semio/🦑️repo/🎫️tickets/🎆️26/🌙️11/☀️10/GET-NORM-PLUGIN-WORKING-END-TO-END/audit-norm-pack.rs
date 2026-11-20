use pack::snapshot::{ArtifactDsl, ArtifactPack};
use std::{env, fs, path::Path};

fn audit<S: ArtifactDsl + ArtifactPack>(dsl: &Path, pack: &Path, repair: bool) -> Result<(), String> {
    let source = fs::read_to_string(dsl).map_err(|e| e.to_string())?;
    let expected = S::parse_dsl(&source).map_err(|e| e.to_string())?.print_dsl();
    let bytes = fs::read(pack).map_err(|e| e.to_string())?;
    if S::decode_pack(&bytes).is_ok_and(|snapshot| snapshot.print_dsl() == expected) {
        return Ok(());
    }
    if !repair {
        return Err("binary snapshot differs from the authored DSL".into());
    }
    let encoded = S::parse_dsl(&source).map_err(|e| e.to_string())?.encode_pack();
    let decoded = S::decode_pack(&encoded).map_err(|e| e.to_string())?;
    if decoded.print_dsl() != expected || source != fs::read_to_string(dsl).map_err(|e| e.to_string())? || bytes != fs::read(pack).map_err(|e| e.to_string())? {
        return Err("pack round-trip or shared source verification failed".into());
    }
    println!("ENCODED {} {}", encoded.iter().map(|b| format!("{b:02x}")).collect::<String>(), dsl.display());
    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().expect("expected check or repair");
    assert!(mode == "check" || mode == "repair");
    let mut failed = false;
    for arg in args {
        let dsl = Path::new(&arg);
        let binary = dsl.with_file_name("🎒️.pack.semio");
        #[cfg(feature="din4108")]
        type Model = semio_s_artifact_norm_din4108_model::Din4108Snapshot;
        #[cfg(feature="din16798")]
        type Model = semio_s_artifact_norm_din16798_model::Din16798Snapshot;
        match audit::<Model>(dsl,&binary,mode == "repair") {
            Ok(()) => println!("PASS {arg}"),
            Err(e) => { eprintln!("FAIL {arg} {e}"); failed = true; }
        }
    }
    if failed {std::process::exit(1);}
}
