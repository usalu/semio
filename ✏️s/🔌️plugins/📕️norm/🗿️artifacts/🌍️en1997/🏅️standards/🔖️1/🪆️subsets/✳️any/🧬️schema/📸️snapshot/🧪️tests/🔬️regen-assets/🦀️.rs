//! 🧪️ `regen_assets` — moved out of `📸️snapshot/🦀️.rs` into its canonical test implementation.
use semio_framework_test_fixtures::fixture_path;
use semio_s_artifact_norm_en1997_io::binary::snapshot::encode_en1997_pack;
use semio_s_artifact_norm_en1997_io::text::snapshot::{decode_en1997_dsl, encode_en1997_dsl};

#[test]
fn regen_dsl_and_pack_assets_match_the_codecs_or_bless() {
    let assets = fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    semio_s_plugin_norm_compliance::regen::bless_dsl_and_pack_assets(&assets, &|text| {
        let document = decode_en1997_dsl(text)?;
        Ok((encode_en1997_dsl(&document), encode_en1997_pack(&document)))
    });
}
