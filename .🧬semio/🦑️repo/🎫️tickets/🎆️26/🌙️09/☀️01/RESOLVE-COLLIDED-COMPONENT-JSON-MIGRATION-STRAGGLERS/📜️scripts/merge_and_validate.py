import json, os, sys

SEMIO = "/Users/ueli/Documents/semio"
os.chdir(SEMIO)

DRY_RUN = "--apply" not in sys.argv

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/component_dirs.txt") as f:
    dirs = [l.strip()[2:] for l in f if l.strip()]

FROZEN = {
    "🧫️fixtures/🧪️remaining-package-purity-authority",
    "🧫️fixtures/🧪️cad-draw-path-projection",
}

APPROVED_VERBS = {"add","append","apply","bind","change","clear","commit","connect","create","delete",
"disconnect","drag","duplicate","edit","extract","finish","fix","flatten","group","inline","insert",
"merge","move","remove","rename","reorder","replace","resize","restore","rotate","scale","seal","set",
"split","start","switch","toggle","unbind","unflatten","ungroup","update"}

VALID_INVERTIBILITY = {"self", "explicit-mutation", "plan", "non-invertible"}
VALID_DIFF_PARTICIPATION = {"detect", "apply-only", "plan", "none"}
VALID_OUTCOME_CLASSES = {"applied", "info", "warning", "error", "fatal"}
VALID_COMPOSITION = {"atomic", "composite"}
VALID_SURFACES = {"rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"}
SURFACE_ORDER = ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]
MUTATION_LEAF_DESCRIPTOR_KEYS = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"]

def kebab_of_pascal(s):
    out = []
    for i, c in enumerate(s):
        if c.isupper() and i > 0:
            out.append('-')
        out.append(c.lower())
    return ''.join(out)

def surfaces_from_fs(d):
    entries = os.listdir(d)
    present = set()
    for e in entries:
        p = os.path.join(d, e)
        if e.startswith("🦀️") and e.endswith(".rs"): present.add("rust")
        if e.startswith("🟦️") and e.endswith(".ts"): present.add("typescript")
        if e.startswith("🔗️") and e.endswith(".graphql"): present.add("graphql")
        if e.startswith("🛰️") and e.endswith(".proto"): present.add("protobuf")
        if e.startswith("🔣️") and e.endswith(".schema.json"): present.add("json-schema")
        if e == "📝️text" and os.path.isdir(p): present.add("text")
        if e == "💾️binary" and os.path.isdir(p): present.add("binary")
    return present

results = []
skipped_frozen = []
errors = []
verb_warnings = []

for d in dirs:
    if d in FROZEN:
        skipped_frozen.append(d)
        continue
    jpath = os.path.join(d, "🔣️.json")
    cpath = os.path.join(d, "🔣️component.json")
    j = json.load(open(jpath, encoding="utf-8"))
    c = json.load(open(cpath, encoding="utf-8"))

    merged = dict(c)  # component.json is the authoritative base

    # requiredLanguageSurfaces: always recompute from filesystem ground truth
    fs_surf = surfaces_from_fs(d)
    merged["requiredLanguageSurfaces"] = [s for s in SURFACE_ORDER if s in fs_surf]

    # payloadSchema: must reference a real file present in the dir. Prefer whichever
    # of the two original values already names a real file (component.json first,
    # since it is the authoritative base); only fall back to a best-guess canonical
    # schema filename when NEITHER original value resolves to a real file.
    schema_files = [e for e in os.listdir(d) if e.startswith("🔣️") and e.endswith(".schema.json")]
    c_val = c.get("payloadSchema")
    j_val = j.get("payloadSchema")
    if c_val in schema_files:
        merged["payloadSchema"] = c_val
    elif j_val in schema_files:
        merged["payloadSchema"] = j_val
    elif "🔣️.schema.json" in schema_files:
        merged["payloadSchema"] = "🔣️.schema.json"
    elif "🔣️payload.schema.json" in schema_files:
        merged["payloadSchema"] = "🔣️payload.schema.json"
    elif len(schema_files) == 1:
        merged["payloadSchema"] = schema_files[0]
    else:
        errors.append((d, f"payloadSchema ambiguous: c={c_val!r} j={j_val!r} candidates={schema_files}"))
        continue

    # --- validate merged result against derive rules ---
    problems = []
    if set(merged.keys()) != set(MUTATION_LEAF_DESCRIPTOR_KEYS):
        problems.append(f"key set mismatch: {sorted(merged.keys())}")
    if merged.get("schemaVersion") != 1:
        problems.append("schemaVersion != 1")
    if merged.get("owner") != d:
        problems.append(f"owner {merged.get('owner')!r} != dir path {d!r}")
    sk = merged.get("semanticKind", "")
    if "-" not in sk:
        problems.append(f"semanticKind {sk!r} has no hyphen")
    if not sk.islower() and sk != sk.lower():
        problems.append(f"semanticKind {sk!r} not lowercase")
    dirname = os.path.basename(d)
    if not dirname.endswith(sk):
        problems.append(f"dirname {dirname!r} does not end with semanticKind {sk!r}")
    av = merged.get("aggregateVariant", "")
    if kebab_of_pascal(av) != sk:
        problems.append(f"kebab(aggregateVariant={av!r})={kebab_of_pascal(av)!r} != semanticKind {sk!r}")
    verb = sk.split("-")[0] if sk else ""
    if verb not in APPROVED_VERBS:
        verb_warnings.append((d, verb, sk))
    to = merged.get("textOpcode")
    if to is not None and (not isinstance(to, str) or "-" not in to and len(to.split("-")) > 1):
        pass  # kebab check loose; textOpcode may be single-word too per observed data
    bt = merged.get("binaryTag")
    if bt is not None and not isinstance(bt, int):
        problems.append(f"binaryTag {bt!r} not int/null")
    if merged.get("invertibility") not in VALID_INVERTIBILITY:
        problems.append(f"invertibility {merged.get('invertibility')!r} invalid")
    if merged.get("diffParticipation") not in VALID_DIFF_PARTICIPATION:
        problems.append(f"diffParticipation {merged.get('diffParticipation')!r} invalid")
    oc = merged.get("outcomeClasses")
    if not isinstance(oc, list) or not oc or any(o not in VALID_OUTCOME_CLASSES for o in oc):
        problems.append(f"outcomeClasses {oc!r} invalid")
    if merged.get("composition") not in VALID_COMPOSITION:
        problems.append(f"composition {merged.get('composition')!r} invalid")
    rls = merged.get("requiredLanguageSurfaces")
    if not isinstance(rls, list) or any(s not in VALID_SURFACES for s in rls):
        problems.append(f"requiredLanguageSurfaces {rls!r} invalid")
    if not isinstance(merged.get("payloadSchema"), str) or not merged["payloadSchema"]:
        problems.append("payloadSchema empty/missing")

    if problems:
        errors.append((d, "; ".join(problems)))
        continue

    results.append((d, merged))

print(f"Frozen skipped: {len(skipped_frozen)}")
print(f"Errors (excluded from apply): {len(errors)}")
for e in errors:
    print("  ERROR:", e)
print(f"Verb warnings (pre-existing, not merge-blocking): {len(verb_warnings)}")
for v in verb_warnings:
    print("  VERB:", v)
print(f"Ready to merge: {len(results)}")

if not DRY_RUN:
    for d, merged in results:
        jpath = os.path.join(d, "🔣️.json")
        cpath = os.path.join(d, "🔣️component.json")
        ordered = {k: merged[k] for k in MUTATION_LEAF_DESCRIPTOR_KEYS}
        text = json.dumps(ordered, ensure_ascii=False, indent=2) + "\n"
        with open(jpath, "w", encoding="utf-8") as f:
            f.write(text)
        os.remove(cpath)
    print(f"APPLIED: wrote {len(results)} merged 🔣️.json files and removed their 🔣️component.json siblings.")
else:
    print("DRY RUN only -- pass --apply to write changes.")

# --- spot check preview (only in dry-run) ---
if DRY_RUN:
    sample_dirs = [
        "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node",
        "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs",
        "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/📝️set-page-text",
        "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app",
    ]
    resultmap = dict(results)
    for sd in sample_dirs:
        print(f"\n=== {sd} ===")
        print(json.dumps(resultmap.get(sd), ensure_ascii=False, indent=2))
