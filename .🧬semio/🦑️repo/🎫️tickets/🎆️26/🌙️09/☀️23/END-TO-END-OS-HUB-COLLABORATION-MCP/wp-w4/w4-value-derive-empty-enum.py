"""🧩️ W4: the Codex typed-path value-derive set (18:35–18:58) expands `match self { #(#arms),* }` for enums in three new bodies
(`value_key_at_path`, `value_at_path`/`value_shape_at_path`, `edit_value_at_path`). For an EMPTY enum (the plugin SDK's
`NoConfigMutation`/`NoPresenceMutation`/`NoTransientMutation`) that is `match self {}` on `&Empty`, which rustc rejects (E0004: a reference
to an uninhabited type is not empty). The fix emits `match *self {}` when there is no arm — the same form the existing all-unit ToValue
branch already uses. Applied only after the set stayed red with no peer edit for ≥ 30 min (coordinator rule). Idempotent; `--dry-run`.
usage: python3 w4-value-derive-empty-enum.py [--dry-run]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
before = text

EMPTY = "let dispatch = if arms.is_empty() { quote! { match *self {} } } else { quote! { match self { #(#arms),* } } };"

key_old = "        .collect::<syn::Result<Vec<_>>>()?;\n    Ok(quote! { match self { #(#arms),* } })\n}\n"
key_new = "        .collect::<syn::Result<Vec<_>>>()?;\n    " + EMPTY + "\n    Ok(dispatch)\n}\n"
path_old = "    Ok(quote! {\n        if path.is_empty() { #root }\n        match self { #(#arms),* }\n    })\n}\n"
path_new = "    " + EMPTY + "\n    Ok(quote! {\n        if path.is_empty() { #root }\n        #dispatch\n    })\n}\n"
edit_old = "                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(\"cannot remove the enum root\")),\n            };\n        }\n        match self { #(#arms),* }\n    })\n}\n"
edit_new = "                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(\"cannot remove the enum root\")),\n            };\n        }\n        #dispatch\n    })\n}\n"

for old, new in ((key_old, key_new), (path_old, path_new)):
    if new in text:
        continue
    assert text.count(old) == 1, old
    text = text.replace(old, new)

if edit_new not in text:
    assert text.count(edit_old) == 1, "edit body"
    head, tail = text.split(edit_old)
    anchor = head.rindex("fn enum_edit_path_body(")
    last = head.rindex("    Ok(quote! {\n")
    assert last > anchor, "enum_edit_path_body's closing quote! not found"
    head = head[:last] + "    " + EMPTY + "\n" + head[last:]
    text = head + edit_new + tail

if text != before:
    print(("would change " if DRY else "changed ") + PATH)
    if not DRY:
        open(PATH, "w", encoding="utf-8").write(text)
else:
    print("nothing to change")
