//! 👁️ Shared analysis display vocabulary for FEM result windows.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, dsl::DslScalar, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum ResultMode {
    #[default]
    #[dsl(key = "static")]
    Static,
    #[dsl(key = "modal")]
    Modal,
    #[dsl(key = "buckling")]
    Buckling,
}

impl TryFrom<&str> for ResultMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "static" => Ok(Self::Static),
            "modal" => Ok(Self::Modal),
            "buckling" => Ok(Self::Buckling),
            _ => Err("unknown FEM result mode"),
        }
    }
}
