#!/usr/bin/env python3
"""🏗️ Builds the staged batch `ownerbytes` (lane ownerbytes) from the CURRENT live files: rerun it right before the gate lands the batch (`python3 -I build.py [--root DIR]`; default: the live checkout holding this ticket). Edits are exact-string and each must match once, so a live file that already carries one stops the build."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else HERE.split("/.🧬semio/")[0]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CLI = L + "/🏗️build/🚀️bootstrap/⌨️cli"
REV = L + "/🏗️build/📥️inputs/🔏️revision"
STAGED = os.path.join(HERE, "staged", "ownerbytes")
NEW = os.path.join(HERE, "new")


def read(rel):
    with open(os.path.join(ROOT, rel), encoding="utf8") as handle:
        return handle.read()


def edit(rel, pairs):
    text = read(rel)
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"edit anchor must match exactly once ({text.count(old)}x) in {rel[-60:]}: {old[:70]!r}")
        text = text.replace(old, new)
    return text


def source(name):
    with open(os.path.join(NEW, name), encoding="utf8") as handle:
        return handle.read()


FEATURE = """
  Scenario: Growing owner manifests fail a law before they refuse any command
    Given the fixed frontend, maintenance and reset owners and the authored bootstrap byte ceiling
    When an owner manifest outgrows its headroom or the ceiling exceeds the Source revision text ceiling
    Then the owner byte law fails while every recovery route still enters
    And an independent Python and SQLite reading agrees with the law on every corpus case
"""

rows = {
    CLI + "/🧬️schema/🔣️.json": edit(CLI + "/🧬️schema/🔣️.json", [('"maximumOwnerBytes": 65536,', '"maximumOwnerBytes": 262144,\n    "ownerHeadroomPercent": 25,')]),
    CLI + "/🧪️tests/🥒️.feature": read(CLI + "/🧪️tests/🥒️.feature") + FEATURE,
    CLI + "/🧪️tests/📏️owner-bytes/🟦️.ts": source("law.ts"),
    CLI + "/🧪️tests/📏️owner-bytes/🔣️.json": source("corpus.json"),
    CLI + "/🧪️tests/📏️owner-bytes/🧬️schema/🔣️.json": source("corpus.schema.json"),
    CLI + "/🧪️tests/📏️owner-bytes/🐍️oracle/🐍️.py": source("oracle.py"),
    CLI + "/🧪️tests/🟦️.ts": edit(CLI + "/🧪️tests/🟦️.ts", [
        ('.replace(/(await |=>\\s*)import\\(([^)]+)\\)/gu,"$1load($2)").replace(/import\\.meta\\.url/gu,JSON.stringify(new URL("../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url).href)),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText+";return "+name;}', '.replace(/(await |=>\\s*|return )import\\(([^)]+)\\)/gu,"$1load($2)").replace(/import\\.meta\\.url/gu,JSON.stringify(new URL("../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url).href)),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText+";return "+name;}'),
        ('if(path==="./⚙️operations/🔤️options/🟦️.ts")return {normalizeBuildOptionsV1};', 'if(path==="./⚙️operations/🔤️options/🟦️.ts")return {normalizeBuildOptionsV1};if(path==="./🏛️sealed/📥️execution/🟦️.ts")return {executeImplementationV1:outerBindings.executeCapturedBuildSourceV1};'),
    ]),
    REV + "/🧬️schema/🔣️.json": edit(REV + "/🧬️schema/🔣️.json", [
        ('"type": "string",\n          "maxLength": 65536', '"type": "string",\n          "maxLength": 262144'),
        ('"maximumTextBytes": 65536,', '"maximumTextBytes": 262144,'),
        ("whose text is retained (64 KiB): schemas and policy files are read under this ceiling.", "whose text is retained (256 KiB): schemas, policy files and project manifests are read under this ceiling."),
        ("deren Text behalten wird (64 KiB): Schemas und Richtliniendateien werden unter dieser Obergrenze gelesen.", "deren Text behalten wird (256 KiB): Schemas, Richtliniendateien und Projektmanifeste werden unter dieser Obergrenze gelesen."),
    ]),
    REV + "/🧪️tests/🔣️.json": edit(REV + "/🧪️tests/🔣️.json", [
        ('"bytes": 65536,\n      "maximumBytes": 65536,', '"bytes": 262144,\n      "maximumBytes": 262144,'),
        ('"maximumBytes": 65537,', '"maximumBytes": 262145,'),
    ]),
    REV + "/🧪️tests/🟦️.ts": edit(REV + "/🧪️tests/🟦️.ts", [
        ("db.query<{accepted:number},[number,number,number]>", "db.query<{accepted:number},[number,number,number,number]>"),
        ("output:{maximumBytes:98304,maximumLines:16}}),data:unknown=JSON.parse(result.stdout.trim().split(\"\\n\").at(-1)??\"\");expect(result.status,row.name", "output:{maximumBytes:Math.ceil(schema.definitions.Policy.const.maximumTextBytes/3)*4+4096,maximumLines:16}}),data:unknown=JSON.parse(result.stdout.trim().split(\"\\n\").at(-1)??\"\");expect(result.status,row.name"),
        ("(?3=0 or ?2<=65536)) accepted\").get(row.bytes,row.maximumBytes,row.retainText?1:0)", "(?3=0 or ?2<=?4)) accepted\").get(row.bytes,row.maximumBytes,row.retainText?1:0,schema.definitions.Policy.const.maximumTextBytes)"),
    ]),
}


def digest(data):
    return hashlib.sha256(data).hexdigest()


shutil.rmtree(STAGED, ignore_errors=True)
for sub in ("files", "base"):
    os.makedirs(os.path.join(STAGED, sub))
manifest = {}
for index, (rel, text) in enumerate(rows.items()):
    name = "f%03d" % index
    live = os.path.join(ROOT, rel)
    base = open(live, "rb").read() if os.path.exists(live) else None
    data = text.encode("utf8")
    with open(os.path.join(STAGED, "files", name), "wb") as handle:
        handle.write(data)
    if base is not None:
        with open(os.path.join(STAGED, "base", name), "wb") as handle:
            handle.write(base)
    manifest[rel] = {"file": name, "base": digest(base) if base is not None else None, "new": digest(data)}
with open(os.path.join(STAGED, "manifest.json"), "w", encoding="utf8") as handle:
    json.dump(manifest, handle, ensure_ascii=False, indent=1)
shutil.copy(os.path.join(HERE, "stage.py"), os.path.join(STAGED, "stage.py"))
shutil.copy(os.path.join(HERE, "verify.sh"), os.path.join(STAGED, "verify.sh"))
print(f"staged {len(manifest)} rows -> {STAGED}")
