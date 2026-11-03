//! 🧩️ Layout preference schema: how the user arranged the visual windows per device class; a persisted, local-only preference whose twins live in [`schema`](self).

macro_rules! layout_choices {
    ($($item:item)*) => {$(
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

macro_rules! layout_records {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

layout_choices! {
    /// 🪟️ How far the mobile bottom sheet with the properties is pulled up.
    pub enum SheetState {
        #[default]
        Collapsed,
        Half,
        Full,
    }
}

layout_records! {
    /// 🖥️ Desktop dock: column shares left, centre and right, the share of the first window in the left and right column, and the windows the user hid (the viewport cannot be hidden).
    pub struct DockPrefs {
        pub left: f64,
        pub centre: f64,
        pub right: f64,
        pub left_split: f64,
        pub right_split: f64,
        pub hidden: Vec<String>,
    }

    /// 📱️ Tablet: the viewport share and the ordered tabs of the side pane with the active one.
    pub struct TabletPrefs {
        pub viewport: f64,
        pub side_tabs: Vec<String>,
        pub side_active: String,
    }

    /// 📱️ Mobile: ordered tab bar with the active tab, and the bottom sheet that shows the properties over the model.
    pub struct MobilePrefs {
        pub tabs: Vec<String>,
        pub active: String,
        pub sheet: SheetState,
        pub sheet_height: f64,
    }

    /// 🧩️ Layout preferences of all three device classes; one record is kept so a user who switches device finds each arrangement as left.
    pub struct LayoutPrefs {
        pub dock: DockPrefs,
        pub tablet: TabletPrefs,
        pub mobile: MobilePrefs,
    }

    /// 📡️ What the host tells the windows about the device: window width in CSS pixels and whether touch is the primary input; the device class and layout follow from it.
    pub struct DeviceSignal {
        pub width_px: u32,
        pub touch: bool,
    }
}
