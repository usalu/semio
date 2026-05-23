from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")
old = """(@expand $builder:expr; $($Name:ident),* $(,)?) => {
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        };"""
new = """(@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};"""
if old not in t:
    if new.split("{{")[0] in t and "}};" in t:
        print("already fixed braces")
    else:
        raise SystemExit("pattern not found")
else:
    t = t.replace(old, new)
    print("fixed braces")

t = t.replace("::paste::paste!", "paste::paste!")
t = t.replace(
    """    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::gap_surface_family_name_list!(@apply_families);
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }""",
    """    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
    }""",
)
t = t.replace(
    """    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }""",
    """    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@expand $builder:expr; $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )*
            b
        }};
    }""",
)
p.write_text(t, encoding="utf-8")

corrupt = [
    "with_gap_surface_family_names!(gap_surface_families)",
    "gap_surface_families! { gap_surface_family_name_list!(@names)",
    "gap_surface_family_name_list!(@names)\n            }",
    "@emit_families",
    "@do_register_bridge",
    "@do_register ",
    "::paste::paste!",
]
for c in corrupt:
    if c in t:
        raise SystemExit(f"corrupt pattern still present: {c!r}")
if "gap_surface_family_name_list!(@apply_families);" not in t:
    raise SystemExit("missing gap_surface_family_name_list!(@apply_families)")
if "(@expand $builder:expr;" not in t:
    raise SystemExit("missing @expand register arm")

print("ok")
