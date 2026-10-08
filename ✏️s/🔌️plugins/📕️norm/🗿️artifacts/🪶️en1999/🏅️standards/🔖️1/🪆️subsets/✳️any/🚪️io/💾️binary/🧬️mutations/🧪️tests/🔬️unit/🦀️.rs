//! EN 1999 mutation binary unit smoke.

use crate::mutations::change_annex::ChangeAnnex;
use crate::mutations::En1999Mutation;

#[test]
fn a_mutation_crosses_the_binary_op_codec_unchanged() {
    let mutation = En1999Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    let bytes = <En1999Mutation as protocol::OpBinary>::encode_op(&mutation).expect("encode");
    assert_eq!(<En1999Mutation as protocol::OpBinary>::decode_op(&bytes).expect("decode"), mutation);
}
