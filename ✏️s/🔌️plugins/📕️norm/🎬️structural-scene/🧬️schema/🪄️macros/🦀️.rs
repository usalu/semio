//! 🪄️ Declaration macros: every scene type carries the same value codec derives and camelCase wire casing.

macro_rules! records {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

macro_rules! variants {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(tag = "kind", content = "data", rename_all = "camelCase", rename_all_fields = "camelCase")]
        $item
    )*};
}

macro_rules! choices {
    ($($item:item)*) => {$(
        #[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}
