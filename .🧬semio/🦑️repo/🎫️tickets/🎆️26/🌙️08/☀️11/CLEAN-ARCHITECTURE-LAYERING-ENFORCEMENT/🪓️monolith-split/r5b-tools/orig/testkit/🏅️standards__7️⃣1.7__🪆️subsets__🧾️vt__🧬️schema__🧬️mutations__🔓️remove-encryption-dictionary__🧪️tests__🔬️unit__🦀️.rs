use super::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::applied;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;

#[test]
fn removes_only_a_present_security_handler() {
    let base = support::document_of(vec![support::encryption_dictionary(2, 3)]);
    let mutation = RemoveEncryptionDictionary { version: 2, revision: 3 };
    let next = applied(&base, &PdfVtMutation::RemoveEncryptionDictionary(mutation.clone()));
    assert!(support::encryption_dictionary_with(&next, 2, 3).is_none());
    assert_eq!(<RemoveEncryptionDictionary as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = applied(&support::document(), &PdfVtMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() }));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_a_middle_row() {
    let base = support::with_tail(&applied(&support::document(), &PdfVtMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: 2, revision: 3, placements: Vec::new() })));
    assert_mutation_inverse_sum_law(&PdfVtMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: 2, revision: 3 }), &base).await;
}
