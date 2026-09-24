import json, re
S = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/"
survey = json.load(open(".tmp-ticket/wp-t4/gltf/survey.json"))
plan = json.load(open(".tmp-ticket/wp-t4/gltf/plan.json"))
CASES = {
    "🎬️scene": ("🎬️mutate-gltf-2-0-scene", "scene", "document/scenes, document/nodes and every node binding"),
    "💿️buffer": ("💿️mutate-gltf-2-0-buffer", "buffer", "document/buffers and document/bufferViews"),
    "🕸️mesh": ("🕸️mutate-gltf-2-0-mesh", "mesh", "document/meshes, their primitives and morph targets, and document/accessors"),
}
def camel(name): 
    parts = name.split("_"); return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])
def expr(kind, module, field, ty):
    key = camel(field)
    simple = {"usize": f'num(params, "{key}")?', "u32": f'num_u32(params, "{key}")?', "u64": f'num(params, "{key}")? as u64', "Vec<usize>": f'order(params, "{key}")?', "String": f'text(params, "{key}")?', "Option<String>": f'optional_text(params, "{key}")?', "Vec<f64>": f'floats(params, "{key}")?', "Vec<String>": f'texts(params, "{key}")?', "Vec<u8>": f'bytes(params, "{key}")?'}
    if ty in simple: return simple[ty]
    if ty == "GltfDataPresence": return f"match presence(params)? {{ Some(value) => {module}::GltfDataPresence::Present {{ value }}, None => {module}::GltfDataPresence::Absent }}"
    if ty == "GltfNodeTransform": return f"transform::<{module}::GltfNodeTransform>(params, |matrix| {module}::GltfNodeTransform::Matrix {{ matrix }}, |translation, rotation, scale| {module}::GltfNodeTransform::Trs {{ translation, rotation, scale }})?"
    if ty == "GltfComponentType": return f'GltfComponentType::from_code(num(params, "{key}")? as u64)?'
    if ty == "GltfAccessorType": return f'text(params, "{key}")?.parse::<GltfAccessorType>()?'
    raise Exception((kind, field, ty))
def arm(kind):
    v = survey[kind]; module = v["module"]; (struct, body) = v["payloads"][0]
    fields = re.findall(r"pub (\w+):\s*([^,\n]+)", body)
    inits = ", ".join(f"{f}: {expr(kind, module, f, t.strip())}" for f, t in fields)
    payload = f"{module}::{struct} {{ {inits} }}" if fields else f"{module}::{struct} {{}}"
    return f'            "{kind}" => {module}::apply(&{payload}, before).map_err(|error| error.detail),'
def cell(value): return json.dumps(value, ensure_ascii=False, separators=(", ", ": "))
for subset, (case, short, surface) in CASES.items():
    kinds = [k for k, v in plan.items() if v["subset"] == subset]
    kinds.sort()
    modules = sorted({survey[k]["module"] for k in kinds})
    used_types = " ".join(p[1] for k in kinds for p in survey[k]["payloads"])
    io_imports = ["parse_gltf_document", "serialize_gltf_document"] + (["GltfAccessorType", "GltfComponentType"] if "GltfComponentType" in used_types else [])
    helpers = open(".tmp-ticket/wp-t4/gltf/subject-helpers.rs").read()
    if "GltfNodeTransform" not in used_types:
        helpers = helpers.split("    //#region 🔖️Transform")[0] + helpers.split("    //#endregion 🔖️Transform\n")[1]
    if "GltfDataPresence" not in used_types:
        helpers = helpers.split("    //#region 🔖️Presence")[0] + helpers.split("    //#endregion 🔖️Presence\n")[1]
    for name, marker in [("floats", "Vec<f64>"), ("texts", "Vec<String>"), ("bytes", "Vec<u8>"), ("optional_text", "Option<String>"), ("num_u32", "u32"), ("order", "Vec<usize>"), ("text", "String")]:
        if marker not in used_types:
            helpers = re.sub(r"    /// [^\n]*\n    fn " + name + r"\(.*?\n    }\n", "", helpers, count=1, flags=re.S)
    kind_list = ",\n    ".join(f'"{k}"' for k in kinds)
    rust = open(".tmp-ticket/wp-t4/gltf/adapter-template.rs").read()
    rust = rust.replace("@SUBSET@", subset).replace("@SHORT@", short).replace("@COUNT@", str(len(kinds))).replace("@SURFACE@", surface)
    rust = rust.replace("@KINDS@", kind_list).replace("@IO@", ", ".join(io_imports)).replace("@MODULES@", ", ".join(modules))
    rust = rust.replace("@SNAPSHOT@", "{GltfJson, GltfSnapshot}" if "GltfDataPresence" in used_types else "GltfSnapshot")
    rust = rust.replace("@HELPERS@", helpers.rstrip("\n")).replace("@ARMS@", "\n".join(arm(k) for k in kinds))
    open(f"{S}{subset}/🧪️tests/{case}/🦀️.rs", "w").write(rust)
    rows_m = "\n".join(f"      | {k} | {plan[k]['fixture']} | {cell(plan[k]['params'])} |" for k in kinds)
    rows_i = "\n".join(f"      | {k} | {plan[k]['fixture']} | {cell(plan[k]['params'])} | {cell(plan[k]['inverse'])} | {cell(plan[k]['restore'])} |" for k in kinds)
    feature = open(".tmp-ticket/wp-t4/gltf/feature-template.feature").read()
    feature = feature.replace("@SUBSET@", subset).replace("@SHORT@", short).replace("@COUNT@", str(len(kinds))).replace("@SURFACE@", surface).replace("@ROWS_MUTATE@", rows_m).replace("@ROWS_INVERSE@", rows_i)
    open(f"{S}{subset}/🧪️tests/{case}/🥒️.feature", "w").write(feature)
    print(subset, len(kinds))
