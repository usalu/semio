//! 📏️ EN 1999 mutation binary unit smoke.

use semio_s_artifact_norm_en1999_mut_inputs::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1999_mutations::En1999Mutation;

#[test]
fn a_mutation_crosses_the_binary_op_codec_unchanged() {
    let mutation = En1999Mutation::ChangeAnnex(ChangeAnnex { new_annex: semio_s_plugin_norm_compliance::AnnexChoice::En });
    let bytes = <En1999Mutation as protocol::OpBinary>::encode_op(&mutation).expect("encode");
    assert_eq!(<En1999Mutation as protocol::OpBinary>::decode_op(&bytes).expect("decode"), mutation);
}
