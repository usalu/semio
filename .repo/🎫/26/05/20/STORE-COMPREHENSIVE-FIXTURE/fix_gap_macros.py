"""Repair schema_gap_surfaces macros in lib.rs (splice name lists, drop duplicates)."""
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
text = p.read_text(encoding="utf-8")
start = text.index("//#region")
start = text.index("schema_gap_surfaces", start)
start = text.index("pub mod schema_gap_surfaces", start)
end = text.index("//#endregion", start)
end = text.index("schema_gap_surfaces", end)

# Keep module head through __gap_surface_existing_relay_name_idents closing brace.
anchor = "    macro_rules! gap_surface_families {"
i_tail = text.index(anchor, start)
head = text[start:i_tail]

tail = r'''    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_family_name_list {
        (@names) => { __gap_surface_family_name_idents!() };
        () => {
            gap_surface_family_name_list!(@emit $(__gap_surface_family_name_idents!()),*);
        };
        (@emit $($Name:ident),* $(,)?) => {
            gap_surface_families! { $($Name),* }
        };
        (@register $builder:expr) => {{
            gap_surface_family_name_list!(@do_register $builder, $(__gap_surface_family_name_idents!()),*);
        }};
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {
            $crate::register_gap_surface_family_connections!(@expand $builder, $($Name),*)
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@names) => { __gap_surface_existing_relay_name_idents!() };
        () => {
            gap_surface_existing_relay_name_list!(@emit $(__gap_surface_existing_relay_name_idents!()),*);
        };
        (@emit $($Name:ident),* $(,)?) => {
            gap_surface_existing_relays! { $($Name),* }
        };
        (@register $builder:expr) => {{
            gap_surface_existing_relay_name_list!(@do_register $builder, $(__gap_surface_existing_relay_name_idents!()),*);
        }};
        (@do_register $builder:expr, $($Name:ident),* $(,)?) => {
            $crate::register_gap_surface_existing_relay_connections!(@expand $builder, $($Name),*)
        };
    }

    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::gap_surface_family_name_list!();
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        (@expand $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        }};
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )+
            b
        }};
    }

    gap_surface_family_name_list!();

'''

named_anchor = "    gap_surface_family_named!("
i_named = text.index(named_anchor, i_tail)
named_and_rest = text[i_named:end]

# Drop duplicate with_gap / register / apply invocations before named block if any
while "macro_rules! gap_surface_family_name_list" in named_and_rest[:500]:
    i_named = text.index(named_anchor, i_named + 1)
    named_and_rest = text[i_named:end]

relay_tail = r'''
    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relay_name_list!();
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        (@expand $builder:expr, $($Name:ident),* $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )*
            b
        }};
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::schema_gap_surfaces::paste::paste! { [<$Name Connection>] }>(); )+
            b
        }};
    }

    gap_surface_existing_relay_name_list!();

}
'''

# named block ends before with_gap_surface_existing or closing brace
cut = named_and_rest.find("#[macro_export]\n    macro_rules! with_gap_surface_existing_relay_names")
if cut == -1:
    cut = named_and_rest.rfind("gap_surface_family_named!")
    cut = named_and_rest.find("\n\n", cut)
    if cut == -1:
        cut = len(named_and_rest)
named_only = named_and_rest[:cut].rstrip() + "\n\n"

new_mod = head + tail + named_only + relay_tail + text[end:]
p.write_text(new_mod, encoding="utf-8")
print("repaired schema_gap_surfaces", i_tail, "->", end)
