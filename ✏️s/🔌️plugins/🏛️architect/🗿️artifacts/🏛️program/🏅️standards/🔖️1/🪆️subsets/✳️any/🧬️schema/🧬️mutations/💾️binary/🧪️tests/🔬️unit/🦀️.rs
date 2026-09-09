use super::*;
use crate::kernel::EntityId;

#[semio_framework_async_macros::async_test]
async fn disconnect_adjacency_round_trips_through_the_binary_codec() {
    let operation = ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::DisconnectAdjacency { id: EntityId("adjacency-1".into()) });
    assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn delete_program_element_round_trips_through_the_binary_codec() {
    let operation = ProgramMutation::DeleteProgramElement(super::super::delete_program_element::DeleteProgramElement { id: EntityId("element-1".into()) });
    assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
}
