import re, sys, os
R = "/Users/ueli/Documents/semio"
L = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
NEW = 'bun --cwd "../../../../../../.." repo run @semio-tech/repo-lib:'
tests = """🧪️tests/🎚️tool-configuration-ownership/🟦️.ts
🧪️tests/📈️reference-coordinate-progress/🟦️.ts
🧪️tests/📱️app-verification-source-ownership/🟦️.ts
🧪️tests/🧑‍💻os-dev-composition-ownership/🟦️.ts
🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts
🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts
🧪️tests/🧱️root-inference-law-source/🟦️.ts
🧪️tests/🧱️root-schema-field-source/🟦️.ts
🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts
🧪️tests/🧱️root-taxonomy-workflow-source/🟦️.ts
🧪️tests/🧱️wasm-package-wrappers/🟦️.ts
🧪️tests/🧱️workspace-publication-source/🟦️.ts
🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts
🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts
🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts
🧪️tests/↪️rust-divergence-callback/🟦️.ts""".split("\n")
fixtures = """🧪️tests/🧫️fixtures/↪️rust-divergence-callback/🔣️.json
🧪️tests/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json
🧪️tests/🧫️fixtures/🚚️readme-move-source-authority/🔣️.json
🧪️tests/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json""".split("\n")
OLD = "nx run @semio-tech/repo-lib:"
for rel in tests:
    p = L + rel; s = open(p, encoding="utf8").read(); out = []
    for line in s.split("\n"):
        if OLD in line:
            def sub(m):
                return m.group(0)
            # decide quoting per occurrence: backtick template if the occurrence sits inside a template literal
            parts = line.split(OLD); res = parts[0]
            for nxt in parts[1:]:
                before = res
                inside_tpl = before.count("`") % 2 == 1
                inside_dq = (not inside_tpl) and before.count('"') % 2 == 1
                inside_json_escaped = False
                if inside_tpl: res += NEW + nxt
                elif inside_dq: res += NEW.replace('"', '\\"') + nxt
                else: res += NEW + nxt
            line = res
        out.append(line)
    n = "\n".join(out)
    if n != s: open(p, "w", encoding="utf8").write(n); print("ts", rel[-60:])
for rel in fixtures:
    p = L + rel; s = open(p, encoding="utf8").read()
    n = s.replace('"packageCommand": "' + OLD, '"packageCommand": "' + NEW.replace('"', '\\"'))
    if n != s: open(p, "w", encoding="utf8").write(n); print("json", rel[-60:])
