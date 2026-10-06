"""✏️ Lands the withdraw-only leaf marker (design §22.20, `📓️s4-gates-report.md` § S5.1c / § S5.2).

One wave, schema first: the leaf descriptor schema gains the optional boolean `editable` (default true); `#[derive(MutationLeaf)]`
accepts it as the one optional key beside the fourteen required ones and, when it is `false`, emits `input_schema() -> None`
(a compile error beside `mutation_leaf(payload = …)` or `mutation_leaf(input_schema = …)`); the parser fixture gains three
vectors and the parser test one law. Every anchor is a literal of the live sources with an asserted count, so a drifted file
refuses the whole run (exit 2, nothing written). `--check` is the default.

    python3 🧪️s5-gates-land-editable.py --root <repository or scratch root> [--apply]
"""
import json
import os
import sys

DERIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"
SOURCE = f"{DERIVE}/🦀️.rs"
TEST = f"{DERIVE}/🧪️tests/🔬️mutation-leaf-json/🦀️.rs"
VECTORS = f"{DERIVE}/🧫️fixtures/🔣️mutation-leaf-json/🔣️.json"
SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧬️schema/🔣️.json"

SCHEMA_OLD = """        "requiredLanguageSurfaces": {
          "type": "array",
          "minItems": 1,
          "uniqueItems": true,
          "items": {
            "$ref": "#/$defs/MutationLanguageSurface"
          }
        }
      }
    },
    "MutationLawsI64": {"""
SCHEMA_NEW = """        "requiredLanguageSurfaces": {
          "type": "array",
          "minItems": 1,
          "uniqueItems": true,
          "items": {
            "$ref": "#/$defs/MutationLanguageSurface"
          }
        },
        "editable": {
          "description": "Whether the history editor may edit this leaf's inputs. Absent or true: the leaf is editable and shows at least one input. False declares it withdraw-only (design §22.20): #[derive(MutationLeaf)] answers input_schema() == None, no gate judges its inputs, and its history row offers Withdraw only.",
          "type": "boolean",
          "default": true
        }
      }
    },
    "MutationLawsI64": {"""

STRUCT_OLD = "    required_language_surfaces: Vec<MutationLeafLanguageSurface>,\n}\n"
STRUCT_NEW = "    required_language_surfaces: Vec<MutationLeafLanguageSurface>,\n    editable: bool,\n}\n"
KEYS_OLD = '"composition", "requiredLanguageSurfaces"];\n'
KEYS_NEW = KEYS_OLD + "\n/// ✏️ The one optional descriptor key: `false` declares the leaf withdraw-only (design §22.20); absent means editable.\nconst MUTATION_LEAF_EDITABLE_KEY: &str = \"editable\";\n"
CHECK_OLD = '    if object.len() != MUTATION_LEAF_DESCRIPTOR_KEYS.len() || MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str())) { return Err("mutation descriptor must contain exactly the fourteen schema fields".to_string()); }\n'
CHECK_NEW = '    if MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str()) && key != MUTATION_LEAF_EDITABLE_KEY) { return Err("mutation descriptor must contain exactly the fourteen schema fields, and beside them only the optional editable".to_string()); }\n'
PARSE_OLD = "composition, required_language_surfaces })\n}\n"
PARSE_NEW = "composition, required_language_surfaces, editable })\n}\n"
SURFACES_OLD = '    let required_language_surfaces = mutation_leaf_surfaces(object.get("requiredLanguageSurfaces").unwrap())?;\n'
SURFACES_NEW = SURFACES_OLD + '    let editable = match object.get(MUTATION_LEAF_EDITABLE_KEY) { None => true, Some(serde_json::Value::Bool(value)) => *value, Some(_) => return Err("editable must be a boolean".to_string()) };\n'
EXPAND_OLD = "pub fn expand_mutation_leaf(input: TokenStream) -> TokenStream {\n"
EXPAND_NEW = """/// 🚪️ The `input_schema` of a withdraw-only leaf (descriptor `editable: false`, design §22.20): always `None`, so the history editor
/// never opens on it. Such a leaf declares neither `payload` nor `input_schema` — both name an editable payload.
fn mutation_leaf_withdraw_only(descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if descriptor.editable { return Ok(None); }
    if let Some(variant) = &attrs.payload { return Err(syn::Error::new_spanned(variant, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf payload")); }
    if let Some(path) = &attrs.input_schema { return Err(syn::Error::new_spanned(path, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf input_schema")); }
    Ok(Some(quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            ::core::option::Option::None
        }
    }))
}

""" + EXPAND_OLD
INSTANCE_OLD = """    let instance_schema = attrs.input_schema.as_ref().map(|path| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            #path(self)
        }
    });
"""
INSTANCE_NEW = INSTANCE_OLD + "    let withdraw_only = match mutation_leaf_withdraw_only(&descriptor, &attrs) { Ok(tokens) => tokens, Err(error) => return error.to_compile_error().into() };\n"
IMPL_OLD = "            #editable\n            #instance_schema\n"
IMPL_NEW = IMPL_OLD + "            #withdraw_only\n"
TEST_LAW = """#[test]
fn a_withdraw_only_descriptor_answers_no_input_schema() {
    let fixture = fixture();
    let authority = authority(fixture["authorityOwner"].as_str().unwrap());
    let raw = |name: &str| fixture["cases"].as_array().unwrap().iter().find(|vector| vector["name"] == name).unwrap()["raw"].as_str().unwrap().as_bytes().to_vec();
    let editable = parse_mutation_leaf_descriptor(&raw("valid-full"), &authority).unwrap();
    let marked = parse_mutation_leaf_descriptor(&raw("valid-editable-true"), &authority).unwrap();
    let withdrawn = parse_mutation_leaf_descriptor(&raw("valid-withdraw-only"), &authority).unwrap();
    assert!(editable.editable && marked.editable && !withdrawn.editable);
    let contract: syn::Path = syn::parse_str("::protocol").unwrap();
    let plain = MutationLeafAttrs { contract: contract.clone(), payload: None, input_schema: None };
    assert!(mutation_leaf_withdraw_only(&editable, &plain).unwrap().is_none());
    let emitted = mutation_leaf_withdraw_only(&withdrawn, &plain).unwrap().unwrap().to_string();
    assert!(emitted.contains("input_schema") && emitted.contains("None"), "{emitted}");
    let wrapped = MutationLeafAttrs { contract: contract.clone(), payload: Some(syn::parse_str("Apply").unwrap()), input_schema: None };
    let instanced = MutationLeafAttrs { contract, payload: None, input_schema: Some(syn::parse_str("schema_at_path").unwrap()) };
    assert!(mutation_leaf_withdraw_only(&withdrawn, &wrapped).is_err() && mutation_leaf_withdraw_only(&withdrawn, &instanced).is_err());
}
"""


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[land-editable] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def swap(texts, path, old, new):
    """✍️ Replaces the single occurrence of `old` in the planned text of `path`."""
    found = texts[path].count(old)
    if found != 1:
        refuse(f"{path} holds {found} occurrence(s) of {old[:80]!r}, expected 1")
    texts[path] = texts[path].replace(old, new)


def main():
    arguments = sys.argv[1:]
    root = (arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else "").rstrip("/")
    apply = "--apply" in arguments
    if root == "" or not os.path.isdir(os.path.join(root, DERIVE)):
        refuse("--root must name a directory that holds the dsl derive")
    texts = {}
    for path in (SOURCE, TEST, VECTORS, SCHEMA):
        if not os.path.isfile(os.path.join(root, path)):
            refuse(f"{path} is missing under {root}")
        texts[path] = open(os.path.join(root, path), encoding="utf-8").read()
    if "MUTATION_LEAF_EDITABLE_KEY" in texts[SOURCE] or '"editable"' in texts[SCHEMA]:
        refuse("the editable marker already stands in the derive or the descriptor schema")
    swap(texts, SCHEMA, SCHEMA_OLD, SCHEMA_NEW)
    for old, new in ((STRUCT_OLD, STRUCT_NEW), (KEYS_OLD, KEYS_NEW), (CHECK_OLD, CHECK_NEW), (SURFACES_OLD, SURFACES_NEW), (PARSE_OLD, PARSE_NEW), (EXPAND_OLD, EXPAND_NEW), (INSTANCE_OLD, INSTANCE_NEW), (IMPL_OLD, IMPL_NEW)):
        swap(texts, SOURCE, old, new)
    if not texts[TEST].endswith("}\n"):
        refuse(f"{TEST} does not end with a closed item")
    texts[TEST] += TEST_LAW
    fixture = json.loads(texts[VECTORS])
    full = next(case["raw"] for case in fixture["cases"] if case["name"] == "valid-full")
    if not full.endswith("}") or any(case["name"].endswith(("withdraw-only", "editable-true", "editable-type")) for case in fixture["cases"]):
        refuse("the parser fixture has no plain valid-full vector, or already holds the editable vectors")
    added = [
        {"name": "valid-withdraw-only", "raw": full[:-1] + ',"editable":false}', "schemaAccepted": True, "parserAccepted": True, "diagnostic": "accepted"},
        {"name": "valid-editable-true", "raw": full[:-1] + ',"editable":true}', "schemaAccepted": True, "parserAccepted": True, "diagnostic": "accepted"},
        {"name": "wrong-editable-type", "raw": full[:-1] + ',"editable":"no"}', "schemaAccepted": False, "parserAccepted": False, "diagnostic": "editable"},
    ]
    closing = "\n    }\n  ]\n}\n"
    if not texts[VECTORS].endswith(closing):
        refuse("the parser fixture does not end with its last case in the expected layout")
    rendered = ["\n".join("    " + line for line in json.dumps(case, ensure_ascii=False, indent=2).split("\n")) for case in added]
    texts[VECTORS] = texts[VECTORS][: -len(closing)] + "\n    },\n" + ",\n".join(rendered) + "\n  ]\n}\n"
    if [case["name"] for case in json.loads(texts[VECTORS])["cases"]][-3:] != [case["name"] for case in added]:
        refuse("the parser fixture did not take the three vectors at its end")
    json.loads(texts[SCHEMA])
    if apply:
        for path, text in texts.items():
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    print(f"[land-editable] schema property, 8 derive hunks, 1 parser law, 3 vectors in 4 file(s) under {root}: {'WRITTEN' if apply else 'planned (check only; pass --apply)'}")


main()
