"""🔏️ Records the checksums of the two blocks cut from the value-derive crate root (drift refusal for apply.py)."""
import hashlib, json, os, sys
ROOT = sys.argv[1] if len(sys.argv) > 1 else "/Users/ueli/Documents/semio"
text = open(os.path.join(ROOT, "🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs"), encoding="utf-8").read()
def block(a, b):
    return text[text.index(a):text.index(b)]
head = block("#![feature(proc_macro_tracked_path, proc_macro_tracked_env)]\n", "use semio_framework_value_derive_expansion as component;\n")
entry = block("/// 📚️ Compiles a package-relative source JSON asset into an owned intrinsic value.\n", "/// 🗃️ Implements `value::ToValue` for a `#[value(...)]`-annotated struct or enum.\n")
json.dump({"head": hashlib.sha256(head.encode()).hexdigest(), "entry": hashlib.sha256(entry.encode()).hexdigest()}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "anchors.json"), "w"), indent=1)
print(len(head), len(entry))
