"""🚪️ Makes a withdraw-only leaf refuse every edited payload (design §22.20; gap found by S5-TEXT-STDIO, `📓️s4-gates-report.md` § S5.8).

`#[derive(MutationLeaf)]` on a leaf whose descriptor says `"editable": false` emitted only `input_schema() -> None`; the leaf kept the
trait default `with_input_value` (rebuild from the payload), so the law `mutation_payload_round_trip_failures` reported "declares
no input schema yet rebuilds from its payload" for every fixture and demo operation of such a leaf. The derive now also emits
`with_input_value(&self, _) -> Err(InvalidValue, "<Leaf> is withdraw-only")`; `from_input_value` stays (feature rows and fixtures
still decode the leaf from its payload). Two files, anchors counted, nothing written unless every anchor holds.

    python3 🧪️s5-gates-land-withdraw-refusal.py --root <repository root> [--apply | --restore]

`--apply` first copies both files to `🗑️generated/s5-gates/exec/pre-refusal/` beside this script; `--restore` copies them back.
"""
import os
import shutil
import sys

DERIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"
SOURCE = f"{DERIVE}/🦀️.rs"
TEST = f"{DERIVE}/🧪️tests/🔬️mutation-leaf-json/🦀️.rs"
BACKUP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "s5-gates", "exec", "pre-refusal")

EDITS = [
    (
        SOURCE,
        """/// 🚪️ The `input_schema` of a withdraw-only leaf (descriptor `editable: false`, design §22.20): always `None`, so the history editor
/// never opens on it. Such a leaf declares neither `payload` nor `input_schema` — both name an editable payload.
fn mutation_leaf_withdraw_only(descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
""",
        """/// 🚪️ The editing surface of a withdraw-only leaf (descriptor `editable: false`, design §22.20): `input_schema` is always `None`, so
/// the history editor never opens on it, and `with_input_value` refuses every edited payload — the editable-payload law
/// (`mutation_payload_round_trip_failures`) holds an inert leaf to that. `from_input_value` stays the trait's: feature rows and
/// fixtures still decode the leaf from its payload. Such a leaf declares neither `payload` nor `input_schema` — both name an
/// editable payload.
fn mutation_leaf_withdraw_only(name: &syn::Ident, descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
""",
    ),
    (
        SOURCE,
        """    Ok(Some(quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            ::core::option::Option::None
        }
    }))
}
""",
        """    Ok(Some(quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            ::core::option::Option::None
        }
        fn with_input_value(&self, _value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, ::std::format!("{} is withdraw-only", ::core::stringify!(#name))))
        }
    }))
}
""",
    ),
    (
        SOURCE,
        "    let withdraw_only = match mutation_leaf_withdraw_only(&descriptor, &attrs) {",
        "    let withdraw_only = match mutation_leaf_withdraw_only(&input.ident, &descriptor, &attrs) {",
    ),
    (
        TEST,
        """    assert!(mutation_leaf_withdraw_only(&editable, &plain).unwrap().is_none());
    let emitted = mutation_leaf_withdraw_only(&withdrawn, &plain).unwrap().unwrap().to_string();
    assert!(emitted.contains("input_schema") && emitted.contains("None"), "{emitted}");
""",
        """    let name: syn::Ident = syn::parse_str("SetSnapshot").unwrap();
    assert!(mutation_leaf_withdraw_only(&name, &editable, &plain).unwrap().is_none());
    let emitted = mutation_leaf_withdraw_only(&name, &withdrawn, &plain).unwrap().unwrap().to_string();
    assert!(emitted.contains("input_schema") && emitted.contains("None"), "{emitted}");
    assert!(emitted.contains("with_input_value") && emitted.contains("InvalidValue") && emitted.contains("SetSnapshot") && emitted.contains("is withdraw-only"), "a withdraw-only leaf refuses every edited payload: {emitted}");
    assert!(!emitted.contains("from_input_value"), "a withdraw-only leaf still decodes from its payload: {emitted}");
""",
    ),
    (
        TEST,
        "    assert!(mutation_leaf_withdraw_only(&withdrawn, &wrapped).is_err() && mutation_leaf_withdraw_only(&withdrawn, &instanced).is_err());",
        "    assert!(mutation_leaf_withdraw_only(&name, &withdrawn, &wrapped).is_err() && mutation_leaf_withdraw_only(&name, &withdrawn, &instanced).is_err());",
    ),
]


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[land-withdraw-refusal] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def main():
    arguments = sys.argv[1:]
    root = (arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else "").rstrip("/")
    if root == "" or not os.path.isfile(os.path.join(root, SOURCE)) or not os.path.isfile(os.path.join(root, TEST)):
        refuse("--root must name a directory that holds the dsl derive and its mutation-leaf-json test")
    if "--restore" in arguments:
        for path in (SOURCE, TEST):
            if not os.path.isfile(os.path.join(BACKUP, path)):
                refuse(f"no pre-wave copy of {path} under {BACKUP}")
        for path in (SOURCE, TEST):
            shutil.copyfile(os.path.join(BACKUP, path), os.path.join(root, path))
        print(f"[land-withdraw-refusal] 2 file(s) restored under {root}")
        return
    texts = {path: open(os.path.join(root, path), encoding="utf-8").read() for path in (SOURCE, TEST)}
    if "is withdraw-only" in texts[SOURCE]:
        refuse("the withdraw-only refusal already stands in the derive")
    for path, old, new in EDITS:
        found = texts[path].count(old)
        if found != 1:
            refuse(f"{path} holds {found} occurrence(s) of {old[:80]!r}, expected 1")
        texts[path] = texts[path].replace(old, new)
    if "--apply" in arguments:
        for path in (SOURCE, TEST):
            os.makedirs(os.path.dirname(os.path.join(BACKUP, path)), exist_ok=True)
            shutil.copy2(os.path.join(root, path), os.path.join(BACKUP, path))
        for path, text in texts.items():
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    print(f"[land-withdraw-refusal] {len(EDITS)} hunk(s) in 2 file(s) under {root}: {'WRITTEN' if '--apply' in arguments else 'planned (check only; pass --apply)'}")


main()
