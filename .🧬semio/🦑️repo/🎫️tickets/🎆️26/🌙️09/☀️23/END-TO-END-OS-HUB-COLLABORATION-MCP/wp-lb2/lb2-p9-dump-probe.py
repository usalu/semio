#!/usr/bin/env python3
"""🔬️ LB2 p9 SCRATCH-ONLY probe: instruments the seven new root declarations to dump their exact runtime capability
requirements (`ArtifactDeclarationBuilder::runtime_capability_requirements`, before `try_build`) to `$LB2_CAPABILITY_DUMP`,
then turns a dump into the schema-first `runtime_capabilities` rows payload (`payload/p9/runtime-capabilities.json`) with the
id/descriptor grammar every existing stdio definition follows (checked against all 29 runtime definitions: 0 deviations).

usage: python3 lb2-p9-dump-probe.py instrument <scratch-root>     (refuses the live tree)
       python3 lb2-p9-dump-probe.py rows <dump.tsv> <tree>          (writes the payload from a dump + the tree's definitions)
"""
import json, os, re, sys

LIVE = "/Users/ueli/Documents/semio"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
ROOTS = ["🪟️bmp", "🔊️wav", "🌦️epw", "💾️binary", "🏗️ifc", "🎞️gif", "🧿️semio"]
HERE = os.path.dirname(os.path.abspath(__file__))

DUMP = '''
fn lb2_capability_dump(builder: &semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) {
    let Ok(path) = std::env::var("LB2_CAPABILITY_DUMP") else { return };
    let rows = semio_framework_plugin::resolve_ready(builder.runtime_capability_requirements()).expect("[DEBUG] requirements");
    let mut out = String::new();
    for row in rows {
        let kind = semio_framework_plugin::resolve_ready(row.kind()).as_str().to_string();
        let claims = semio_framework_plugin::resolve_ready(row.claims()).iter().map(|claim| format!("{}={}", claim.namespace().as_str(), claim.value())).collect::<Vec<_>>().join("\\t");
        out.push_str(&format!("{}\\t{}\\t{}\\n", "KIND", kind, claims));
    }
    use std::io::Write;
    std::fs::OpenOptions::new().create(true).append(true).open(path).expect("[DEBUG] dump").write_all(out.as_bytes()).expect("[DEBUG] write");
}
'''


def instrument(tree):
    if os.path.realpath(tree) == os.path.realpath(LIVE):
        sys.exit("refusing the live tree")
    for root in ROOTS:
        path = os.path.join(tree, ART, root, "🦀️.rs")
        text = open(path, encoding="utf-8").read()
        start = text.index("pub fn declaration(")
        end = text.index("\n}\n", start) + 3
        body = text[start:end]
        if "\n        .try_build()\n}\n" in body:
            body = body.replace("    semio_framework_plugin::ArtifactDeclaration::builder(definition)\n", "    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition)\n", 1)
            body = body.replace("\n        .try_build()\n}\n", ";\n    lb2_capability_dump(&builder);\n    builder.try_build()\n}\n")
        else:
            last = re.search(r"\n    ([^\n]*)\.try_build\(\)\n\}\n$", body)
            body = body[: last.start()] + f"\n    let builder = {last.group(1)};\n    lb2_capability_dump(&builder);\n    builder.try_build()\n}}\n"
        kind = "s.stdio." + re.sub(r"^[^a-z]+", "", root)
        open(path, "w", encoding="utf-8").write(text[:start] + body + text[end:] + DUMP.replace('"KIND"', f'"{kind}"'))
        print("instrumented", root)


def slug(value):
    return re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")


def rows(dump, tree):
    wanted = {}
    for line in open(dump, encoding="utf-8"):
        kind, category, *claims = line.rstrip("\n").split("\t")
        wanted.setdefault(kind, set()).add((category, tuple(sorted(tuple(claim.split("=", 1)) for claim in claims))))
    payload = {}
    for root in ROOTS:
        definition = json.load(open(os.path.join(tree, ART, root, "📜️artifact-definition.json"), encoding="utf-8"))
        owner, standard = definition["id"], definition["standards"][0]["id"]
        out = {}
        for category, claims in sorted(wanted.get(owner, ())):
            if category == "grammar":
                continue
            if category == "representation":
                continue
            if category == "composer" and not dict(claims)["dialect"].startswith(owner + "@"):
                continue
            if category == "codec":
                schema = dict(claims)["codec"]
                extension = dict(claims)["codec-extension"].rsplit(":", 1)[1]
                identity = f"{standard}.codec.codec-{slug(schema)}-extension-{slug(extension)}.v1"
            else:
                namespace, value = claims[0]
                identity = f"{owner}.{category}.{namespace}-{slug(value)}.v1"
            ordered = sorted(claims, key=lambda claim: f"{claim[0]}:{claim[1]}")
            out[identity] = {"id": identity, "category": category, "descriptor": f"runtime-capability:{category}:" + "|".join(f"{n}:{v}" for n, v in ordered), "claims": [{"namespace": n, "value": v} for n, v in sorted(claims)]}
        seen = set()
        for representation in definition["representations"]:
            claims = [("mime", mime) for mime in representation["mimes"]] + [("extension", extension) for extension in representation["extensions"]]
            key = frozenset(claims)
            if key in seen:
                continue
            seen.add(key)
            leaf = "-".join([f"mime-{slug(v)}" for n, v in claims if n == "mime"] + [f"extension-{slug(v)}" for n, v in claims if n == "extension"])
            identity = f"{standard}.representation.{leaf}"
            out[identity] = {"id": identity, "category": "representation", "descriptor": "runtime-capability:representation:" + "|".join(f"{n}:{v}" for n, v in claims), "claims": [{"namespace": n, "value": v} for n, v in claims]}
        payload[root] = sorted(out.values(), key=lambda row: row["id"])
        print(root, len(payload[root]), "rows", sorted({row["category"] for row in payload[root]}))
    target = os.path.join(HERE, "payload", "p9", "runtime-capabilities.json")
    os.makedirs(os.path.dirname(target), exist_ok=True)
    open(target, "w", encoding="utf-8").write(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")
    print("wrote", target)


if __name__ == "__main__":
    if sys.argv[1:2] == ["instrument"]:
        instrument(sys.argv[2])
    elif sys.argv[1:2] == ["rows"]:
        rows(sys.argv[2], sys.argv[3])
    else:
        print(__doc__)
        sys.exit(2)
