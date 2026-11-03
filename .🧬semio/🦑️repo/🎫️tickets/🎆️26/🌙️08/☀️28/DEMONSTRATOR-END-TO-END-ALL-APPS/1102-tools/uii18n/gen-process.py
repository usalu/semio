import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mkpatch import patch
f = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️.rs"
patch(f, [('format!("{cursor}/{len} steps · {volume:.4} m³")', 'format!("{cursor}/{len} {} · {volume:.4} m³", labels.steps.as_str())')], str(pathlib.Path(__file__).parent / "patches/process-status-steps-1.patch"))
f2 = "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/🦀️.rs"
patch(f2, [('/// 📝 Renders generation questions as semantic controls with typed change bindings.\n/// 🏷️ The three user-visible words of the generated input form, resolved by the calling app.\npub struct GenerationFormLabels', '/// 🏷️ The three user-visible words of the generated input form, resolved by the calling app.\npub struct GenerationFormLabels'),
 ('}\n\npub fn generation_form(', '}\n\n/// 📝 Renders generation questions as semantic controls with typed change bindings.\npub fn generation_form(')], str(pathlib.Path(__file__).parent / "patches/procedural-docstring-1.patch"))
