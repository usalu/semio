"""🏷️ Renames norm's own two-segment fault codes to the three-segment `norm.editor.*` grammar and drops framework `mutation.*` rows from norm's notice table."""
import re, subprocess, sys
from pathlib import Path
root = Path(sys.argv[1])
codes = ["unhandled-action", "value-path", "set-field-value", "set-active-example-invalid", "insert-item-value", "set-fields-args", "insert-placement", "edit-unresolved", "apply-remedy-option-type", "apply-remedy-uint", "apply-remedy-int", "apply-remedy-not-applicable", "apply-remedy-missing-remedy", "apply-remedy-missing-option", "apply-remedy-missing-check", "apply-remedy-empty-path", "apply-remedy-remove-path", "entity-delete-empty", "entity-delete-unknown", "remedy-removes-row"]
pattern = re.compile(r'"norm\.(' + "|".join(map(re.escape, codes)) + r')"')
files = subprocess.run(["grep", "-rlE", r'"norm\.(' + "|".join(codes) + r')"', "✏️s", "🌎️hub", "🏢️semio-tech", "🧰️framework"], cwd=root, capture_output=True, text=True).stdout.split()
for name in files:
    path = root / name
    text = path.read_text()
    updated = pattern.sub(lambda m: f'"norm.editor.{m.group(1)}"', text)
    if updated != text:
        path.write_text(updated); print("renamed", name)
table = root / "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🏷️ids-chrome-and-labels/🦀️.rs"
text = table.read_text()
text = "".join(line for line in text.splitlines(keepends=True) if not re.match(r'\s+\("mutation\.[a-z-]+",', line))
text = text.replace("every refusal and warning code the norm editors raise (`mutation.*`, `norm.*`)", "every refusal and warning code the norm editors raise (`norm.editor.*`)")
table.write_text(text)
