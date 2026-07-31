//! ⚙️ Playbook-play app — headless compute (constitutional: engine).

use playbook::PlaybookBlock;

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
pub fn default_block(id: String, kind: &str) -> PlaybookBlock {
    PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}
//#endregion 🔖️DocumentHelpers
