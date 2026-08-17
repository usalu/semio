import re, os
glue_path = "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs"
text = open(glue_path, encoding="utf-8").read()
missing = [p for p in re.findall(r'#\[path\s*=\s*"([^"]+)"\]', text)
           if p != "." and not os.path.isfile(os.path.normpath(os.path.join(os.path.dirname(glue_path), p)))]
print(len(missing), missing)
