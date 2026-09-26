"""N1 one-off codemod: restores the production mutation codec bridge (decode / apply / inverse) the repository test host reaches, in every norm family whose Wave C rewrite dropped it."""
import re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
FAMILIES = {"⚖️en1990": "en1990", "🏋️en1991": "en1991", "🪨️en1996": "en1996", "🌍️en1997": "en1997", "🫨️en1998": "en1998", "🏛️en1992": "en1992", "🧩️en1994": "en1994"}
def fns(fam):
    T, t = fam.capitalize().replace("En", "En").replace("Din", "Din"), fam
    T = fam[0].upper() + fam[1:]
    return {
        "decode": f"""/// 📥️ Decodes one committed mutation JSON document into [`{T}Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_{t}_mutation_json(text: &str) -> Result<{T}Mutation, String> {{
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}}
""",
        "apply": f"""/// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
pub fn apply_{t}_mutation(base: &{T}Snapshot, mutation: &{T}Mutation) -> Result<({T}Snapshot, Vec<String>), String> {{
    let raised = <{T}Mutation as protocol::Mutation<{T}Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{{:?}}:{{}}", message.level, message.code.0)).collect();
    let applied = <{T}Diff as protocol::MutationDiff<{T}Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{{error:?}}"))?;
    Ok((applied, messages))
}}
""",
        "inverse": f"""/// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
pub fn inverse_{t}_mutation(mutation: &{T}Mutation, base: &{T}Snapshot) -> Vec<{T}Mutation> {{
    <{T}Mutation as protocol::Mutation<{T}Snapshot>>::inverse(mutation, base)
}}
""",
    }
write = "--write" in sys.argv
for directory, fam in FAMILIES.items():
    path = f"{ROOT}/{directory}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    missing = [name for name in ("decode", "apply", "inverse") if not re.search(rf"pub fn {name}_{fam}_mutation", text)]
    if not missing: continue
    block = "".join(fns(fam)[name] for name in missing)
    if "//#endregion 🌉️ExternalCodecBridge" in text:
        text = text.replace("//#endregion 🌉️ExternalCodecBridge", block + "//#endregion 🌉️ExternalCodecBridge", 1)
    else:
        text = text.rstrip("\n") + "\n\n//#region 🌉️ExternalCodecBridge\n" + block + "//#endregion 🌉️ExternalCodecBridge\n"
    print(directory, "adds", missing)
    if write: open(path, "w", encoding="utf-8").write(text)
