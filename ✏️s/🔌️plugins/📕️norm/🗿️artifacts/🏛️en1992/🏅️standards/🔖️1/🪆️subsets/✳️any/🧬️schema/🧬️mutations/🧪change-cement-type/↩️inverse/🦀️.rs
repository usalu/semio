use crate::mutations::change_cement_type::ChangeCementType;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(_payload: &ChangeCementType, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1992Mutation::ChangeCementType(ChangeCementType { new_cement_type: base.cement_type.clone() })]

    })())
}
