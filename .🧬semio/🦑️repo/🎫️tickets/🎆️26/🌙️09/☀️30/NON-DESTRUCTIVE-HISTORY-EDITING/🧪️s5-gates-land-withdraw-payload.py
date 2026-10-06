"""🎁️ Lets a withdraw-only leaf wrap its payload (design §22.20; found by S5-TEXT-STDIO on gltf `default-scene/unbind`, `📓️s4-gates-report.md` § S5.9).

`#[derive(MutationLeaf)]` refused `mutation_leaf(payload = <Variant>)` on a leaf whose descriptor says `"editable": false`. Now the
payload arm is split: `input_value` and `from_input_value` are always emitted for a wrapped leaf (feature rows and fixtures decode
through the payload variant), `input_schema` and `with_input_value` of that arm only while the leaf is editable; a withdraw-only
leaf gets `input_schema() -> None` and the refusing `with_input_value` as before. `mutation_leaf(input_schema = …)` beside
`editable: false` stays a compile error. Two files, anchors counted, nothing written unless every anchor holds.

    python3 🧪️s5-gates-land-withdraw-payload.py --root <repository root> [--apply | --restore]

`--apply` first copies both files to `🗑️generated/s5-gates/exec/pre-payload/` beside this script; `--restore` copies them back.
"""
import os
import shutil
import sys

DERIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"
SOURCE = f"{DERIVE}/🦀️.rs"
TEST = f"{DERIVE}/🧪️tests/🔬️mutation-leaf-json/🦀️.rs"
BACKUP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "s5-gates", "exec", "pre-payload")

ARM_OLD = """    let editable = attrs.payload.as_ref().map(|variant| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn input_value(&self) -> ::semio_framework_value::DslValue {
            match self { Self::#variant(payload) => ::semio_framework_value::ToValue::to_value(payload), _ => ::semio_framework_value::ToValue::to_value(self) }
        }
        fn with_input_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            match self {
                Self::#variant(_) => ::semio_framework_value::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
        fn from_input_value(value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::semio_framework_value::FromValue::from_value(value).map(Self::#variant)
        }
    });
"""
ARM_NEW = "    let editable = attrs.payload.as_ref().map(|variant| mutation_leaf_payload_arm(contract, name, variant, descriptor.editable));\n"
DOC_OLD = """/// editable payload.
fn mutation_leaf_withdraw_only(name: &syn::Ident, descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if descriptor.editable { return Ok(None); }
    if let Some(variant) = &attrs.payload { return Err(syn::Error::new_spanned(variant, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf payload")); }
"""
DOC_NEW = """/// editable payload.
/// It may wrap its payload in one variant (`payload`): that arm keeps decoding ([`mutation_leaf_payload_arm`]) and edits nothing.
fn mutation_leaf_withdraw_only(name: &syn::Ident, descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if descriptor.editable { return Ok(None); }
"""
HEAD_OLD = "/// 🚪️ The editing surface of a withdraw-only leaf (descriptor `editable: false`, design §22.20): `input_schema` is always `None`, so\n"
HEAD_NEW = """/// 🎁️ The payload-arm methods of a leaf that wraps its payload in one variant (`mutation_leaf(payload = Variant)`): `input_value` and
/// `from_input_value` always — feature rows and fixtures decode the leaf through that variant — and, while the leaf is `editable`,
/// `input_schema` and `with_input_value`, which edit that variant only. A withdraw-only leaf gets those two from
/// [`mutation_leaf_withdraw_only`] instead.
fn mutation_leaf_payload_arm(contract: &syn::Path, name: &syn::Ident, variant: &syn::Ident, editable: bool) -> proc_macro2::TokenStream {
    let editing = editable.then(|| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn with_input_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            match self {
                Self::#variant(_) => ::semio_framework_value::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
    });
    quote! {
        #editing
        fn input_value(&self) -> ::semio_framework_value::DslValue {
            match self { Self::#variant(payload) => ::semio_framework_value::ToValue::to_value(payload), _ => ::semio_framework_value::ToValue::to_value(self) }
        }
        fn from_input_value(value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::semio_framework_value::FromValue::from_value(value).map(Self::#variant)
        }
    }
}

""" + HEAD_OLD
LAW_OLD = "    assert!(mutation_leaf_withdraw_only(&name, &withdrawn, &wrapped).is_err() && mutation_leaf_withdraw_only(&name, &withdrawn, &instanced).is_err());\n"
LAW_NEW = """    assert!(mutation_leaf_withdraw_only(&name, &withdrawn, &instanced).is_err());
    let wrapping = mutation_leaf_withdraw_only(&name, &withdrawn, &wrapped).unwrap().unwrap().to_string();
    assert_eq!(wrapping, emitted, "a withdraw-only leaf that wraps its payload refuses like any other");
    let variant: syn::Ident = syn::parse_str("Apply").unwrap();
    let contract: syn::Path = syn::parse_str("::protocol").unwrap();
    let editing = mutation_leaf_payload_arm(&contract, &name, &variant, true).to_string();
    let decoding = mutation_leaf_payload_arm(&contract, &name, &variant, false).to_string();
    for method in ["input_schema", "with_input_value", "input_value", "from_input_value"] {
        assert!(editing.contains(&format!("fn {method} (")), "an editable wrapped leaf emits {method}: {editing}");
    }
    assert!(decoding.contains("fn input_value (") && decoding.contains("fn from_input_value ("), "a withdraw-only wrapped leaf still decodes through its payload: {decoding}");
    assert!(!decoding.contains("fn input_schema (") && !decoding.contains("fn with_input_value ("), "its editing methods come from the withdraw-only tokens alone: {decoding}");
"""
INSTANCED_OLD = "    let instanced = MutationLeafAttrs { contract, payload: None, input_schema: Some(syn::parse_str(\"schema_at_path\").unwrap()) };\n"
INSTANCED_NEW = "    let instanced = MutationLeafAttrs { contract: contract.clone(), payload: None, input_schema: Some(syn::parse_str(\"schema_at_path\").unwrap()) };\n"
EDITS = [(SOURCE, ARM_OLD, ARM_NEW), (SOURCE, DOC_OLD, DOC_NEW), (SOURCE, HEAD_OLD, HEAD_NEW), (TEST, INSTANCED_OLD, INSTANCED_NEW), (TEST, LAW_OLD, LAW_NEW)]


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[land-withdraw-payload] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def main():
    arguments = sys.argv[1:]
    root = (arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else "").rstrip("/")
    if root == "" or not os.path.isfile(os.path.join(root, SOURCE)) or not os.path.isfile(os.path.join(root, TEST)):
        refuse("--root must name a directory that holds the dsl derive and its mutation-leaf-json test")
    if "--restore" in arguments:
        if any(not os.path.isfile(os.path.join(BACKUP, path)) for path in (SOURCE, TEST)):
            refuse(f"no complete pre-wave copy under {BACKUP}")
        for path in (SOURCE, TEST):
            shutil.copyfile(os.path.join(BACKUP, path), os.path.join(root, path))
        print(f"[land-withdraw-payload] 2 file(s) restored under {root}")
        return
    texts = {path: open(os.path.join(root, path), encoding="utf-8").read() for path in (SOURCE, TEST)}
    if "fn mutation_leaf_payload_arm" in texts[SOURCE]:
        refuse("the payload arm already stands in the derive")
    for path, old, new in EDITS:
        found = texts[path].count(old)
        if found != 1:
            refuse(f"{path} holds {found} occurrence(s) of {old[:80]!r}, expected 1")
        texts[path] = texts[path].replace(old, new)
    if "--apply" in arguments:
        if os.path.exists(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "coord", "activation.flag")):
            refuse("an activation is running (coord/activation.flag): no Rust save in the closure (rule 68)")
        for path in (SOURCE, TEST):
            os.makedirs(os.path.dirname(os.path.join(BACKUP, path)), exist_ok=True)
            shutil.copy2(os.path.join(root, path), os.path.join(BACKUP, path))
        for path, text in texts.items():
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    print(f"[land-withdraw-payload] {len(EDITS)} hunk(s) in 2 file(s) under {root}: {'WRITTEN' if '--apply' in arguments else 'planned (check only; pass --apply)'}")


main()
