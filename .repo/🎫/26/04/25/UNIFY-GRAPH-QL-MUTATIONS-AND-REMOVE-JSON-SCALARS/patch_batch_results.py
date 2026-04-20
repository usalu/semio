import pathlib
import re

p = pathlib.Path("semio/rs/lib.rs")
text = p.read_text(encoding="utf-8")

c1 = len(re.findall(r"(?m)^[ \t]+conflicts: None,\n(?![ \t]+change_kind)", text))
text = re.sub(
    r"(^[ \t]+)conflicts: None,\n(?![ \t]+change_kind)",
    r"\1conflicts: None,\n\1change_kind: None,\n\1inverse: None,\n",
    text,
    flags=re.MULTILINE,
)
c2 = len(re.findall(r"(?m)^[ \t]+conflicts: Some\(conflicts\),\n(?![ \t]+change_kind)", text))
text = re.sub(
    r"(^[ \t]+)conflicts: Some\(conflicts\),\n(?![ \t]+change_kind)",
    r"\1conflicts: Some(conflicts),\n\1change_kind: None,\n\1inverse: None,\n",
    text,
    flags=re.MULTILINE,
)
p.write_text(text, encoding="utf-8")
print("candidates", c1, c2)
