//! 🪪️ Semantic identities for procedural catalogue entries sharing a widget family.

//#region 🔖️Identity
pub fn item_key(kind: &str, neuron_kind: Option<&str>, format: Option<&str>, action: Option<&str>) -> String {
    let variant = match kind {
        "neuron" => neuron_kind,
        "outputExport" => format,
        "outputAction" => action,
        _ => None,
    };
    match variant {
        Some(variant) => format!("procedural-play-catalogue.{kind}.{variant}"),
        None => format!("procedural-play-catalogue.{kind}"),
    }
}
//#endregion 🔖️Identity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
