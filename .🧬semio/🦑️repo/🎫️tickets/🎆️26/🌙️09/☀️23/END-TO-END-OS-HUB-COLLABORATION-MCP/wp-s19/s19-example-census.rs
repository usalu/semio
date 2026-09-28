//! 🧮️ S19 one-off census (overlay-only `[[test]]` of `semio-s-plugin-norm`, never staged): every norm editor's roster
//! example document against production decode, and every example that ALSO has a code-built constructor (the one the
//! pre-roster `setActiveExample` arms loaded) against that constructor. Mismatches write the constructor's production
//! `print_dsl` + `encode_pack` to `$S19_CENSUS_OUT/<family>/<id>.{dsl,pack}` for the asset regeneration codemod.

use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};
use semio_framework_plugin::ArtifactEditor;

fn out_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("S19_CENSUS_OUT").expect("S19_CENSUS_OUT"))
}

fn roster<E: ArtifactEditor>(family: &str) -> Vec<(String, String)>
where
    E::Snapshot: ArtifactDsl,
{
    let mut rows = Vec::new();
    for example in E::examples() {
        let text = example.document();
        let verdict = match <E::Snapshot as ArtifactDsl>::parse_dsl(&text) {
            Ok(_) => "parses".to_string(),
            Err(error) => format!("PARSE-FAIL {error:?}"),
        };
        println!("[CENSUS] roster family={family} id={} {verdict}", example.id());
        rows.push((example.id().to_string(), text));
    }
    rows
}

fn pair<S: ArtifactDsl + ArtifactPack + PartialEq + std::fmt::Debug>(family: &str, rows: &[(String, String)], id: &str, code: S) {
    let Some((_, text)) = rows.iter().find(|(row, _)| row == id) else {
        println!("[CENSUS] pair family={family} id={id} NOT-IN-ROSTER");
        return;
    };
    let verdict = match S::parse_dsl(text) {
        Ok(decoded) if decoded == code => "EQUAL".to_string(),
        Ok(_) => "DIFFERS".to_string(),
        Err(error) => format!("PARSE-FAIL {error:?}"),
    };
    println!("[CENSUS] pair family={family} id={id} {verdict}");
    if verdict != "EQUAL" {
        let dir = out_dir().join(family);
        std::fs::create_dir_all(&dir).unwrap();
        let printed = code.print_dsl();
        assert_eq!(S::parse_dsl(&printed).as_ref().ok(), Some(&code), "{family}/{id}: production print_dsl must round-trip");
        std::fs::write(dir.join(format!("{id}.dsl")), printed).unwrap();
        std::fs::write(dir.join(format!("{id}.pack")), code.encode_pack()).unwrap();
    }
}

#[test]
fn s19_example_census() {
    use semio_s_artifact_norm_din16798 as din16798;
    use semio_s_artifact_norm_din4108 as din4108;
    use semio_s_artifact_norm_en1991 as en1991;
    use semio_s_artifact_norm_en1993 as en1993;
    use semio_s_artifact_norm_en1996 as en1996;
    use semio_s_artifact_norm_en1997 as en1997;
    use semio_s_artifact_norm_en1998 as en1998;
    use semio_s_artifact_norm_en1999 as en1999;

    let rows = roster::<din4108::editor::din4108::Din4108PlayApp>("din4108");
    pair("din4108", &rows, "demo", din4108::Din4108Snapshot::compliant_etics_dwelling());
    pair("din4108", &rows, "compliant-etics-dwelling", din4108::Din4108Snapshot::compliant_etics_dwelling());
    pair("din4108", &rows, "failing-thin-insulation", din4108::Din4108Snapshot::failing_thin_insulation());

    let rows = roster::<din16798::editor::din16798::Din16798PlayApp>("din16798");
    pair("din16798", &rows, "demo", din16798::Din16798Snapshot::compliant_office());
    pair("din16798", &rows, "compliant-office", din16798::Din16798Snapshot::compliant_office());
    pair("din16798", &rows, "noncompliant-office", din16798::Din16798Snapshot::noncompliant_office());

    let rows = roster::<en1991::editor::en1991::En1991PlayApp>("en1991");
    pair("en1991", &rows, en1991::de_office_compliant::ID, en1991::example_subjects::de_office_compliant());
    pair("en1991", &rows, en1991::multi_fail_noncompliant::ID, en1991::example_subjects::multi_fail_noncompliant());

    let rows = roster::<en1993::editor::en1993::En1993PlayApp>("en1993");
    pair("en1993", &rows, en1993::heb240_compliant::ID, en1993::En1993Snapshot::compliant_heb240_frame());
    pair("en1993", &rows, en1993::high_strength_connection::ID, en1993::En1993Snapshot::noncompliant_overloaded_frame());

    let rows = roster::<en1996::editor::en1996::En1996PlayApp>("en1996");
    pair("en1996", &rows, en1996::loadbearing_wall::ID, en1996::loadbearing_wall::snapshot());
    pair("en1996", &rows, en1996::multi_fail_masonry::ID, en1996::multi_fail_masonry::snapshot());

    let rows = roster::<en1997::editor::en1997::En1997PlayApp>("en1997");
    pair("en1997", &rows, "compliant", en1997::standards::v1::subsets::any::schema::snapshot::compliant_demo());
    pair("en1997", &rows, "noncompliant", en1997::standards::v1::subsets::any::schema::snapshot::noncompliant_demo());

    let rows = roster::<en1998::editor::en1998::En1998PlayApp>("en1998");
    pair("en1998", &rows, en1998::seismic_rc_frame::ID, en1998::seismic_rc_frame::snapshot());
    pair("en1998", &rows, en1998::seismic_rc_frame_fail::ID, en1998::seismic_rc_frame_fail::snapshot());
    pair("en1998", &rows, en1998::seismic_multipart::ID, en1998::seismic_multipart::snapshot());
    pair("en1998", &rows, en1998::seismic_multipart_fail::ID, en1998::seismic_multipart_fail::snapshot());

    let rows = roster::<en1999::editor::en1999::En1999PlayApp>("en1999");
    pair("en1999", &rows, en1999::aluminium_roof_purlin::ID, en1999::aluminium_roof_purlin::snapshot());
    pair("en1999", &rows, en1999::noncompliant_multi_fail::ID, en1999::noncompliant_multi_fail::snapshot());

    roster::<semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp>("din18599");
    roster::<semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp>("en1990");
    roster::<semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp>("en1992");
    roster::<semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp>("en1994");
    roster::<semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp>("en1995");
    roster::<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp>("iso16757");
    roster::<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp>("vdi3805");
}
