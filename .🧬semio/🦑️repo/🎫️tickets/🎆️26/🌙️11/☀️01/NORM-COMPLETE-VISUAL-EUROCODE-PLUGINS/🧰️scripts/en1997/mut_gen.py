import sys, os, json
sys.path.insert(0, ".")
from spec import *
from rows import *

SEM = {"insert": ("insert", "Inserted"), "remove": ("remove", "Removed"), "update": ("update", "Updated")}
LAB = {"insert": ("Insert {en}", "{de} einfügen"), "remove": ("Remove {en}", "{de} entfernen"), "update": ("Update {en}", "{de} aktualisieren")}

def w(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)

def rs_payload(r, v):
    st, m, k = struct(r, v), mod(r, v), kind(r, v)
    other_from = {"insert": [("remove", )], "remove": [("insert", )], "update": []}[v]
    froms = "".join(f" + From<crate::{mod(r, o[0])}::{struct(r, o[0])}>" for o in other_from)
    if v == "insert":
        fields = f"    pub index: usize,\n    pub {r.item}: semio_s_artifact_norm_en1997_model::{r.struct},\n"
    elif v == "remove":
        fields = "    pub index: usize,\n"
    else:
        fields = f"    pub {r.item}: semio_s_artifact_norm_en1997_model::{r.struct},\n"
    en = LAB[v][0].format(en=r.en); de = LAB[v][1].format(de=r.de)
    if v != "remove":
        pass
    label = f'semio_framework_ui_locale::LocalizedLabel::native("{en}", "{de}")'
    target = ""
    if v == "update":
        target = f"    fn target(&self) -> Vec<String> {{\n        vec![self.{r.item}.id.clone()]\n    }}\n"
        label = f'semio_framework_ui_locale::LocalizedLabel::native(&format!("Update {r.en} {{}}", self.{r.item}.id), &format!("{r.de} {{}} aktualisieren", self.{r.item}.id))'
    verb, rec = SEM[v]
    return f'''//! 🧹 `{k}` payload.

use semio_s_artifact_norm_en1997_diff::En1997Diff;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {st} {{
{fields}}}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for {st} where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::{m}::{st}>{froms} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{
        verb: "{verb}", entity: "{r.slug}", kind: "{k}", record: "{rec}{pascal(r.slug.replace('-', '_'))}",
    }};

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {{
        {label}
    }}
{target}}}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for {st} where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::{m}::{st}>{froms} {{
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {{
        crate::{m}::diff::diff(self, base)
    }}
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {{
    Ok({{
        crate::{m}::inverse::inverse(self, base)?
    
    }})
}}
}}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
'''

def rs_diff(r, v):
    st, m = struct(r, v), mod(r, v)
    c, it = r.coll, r.item
    rows = f"{r.pn}Rows"
    if v == "insert":
        body = f'''    if base.{c}.iter().any(|existing| existing.id == payload.{it}.id) {{
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{r.struct} id {{}} already exists.", payload.{it}.id), [payload.{it}.id.clone()]);
    }}
    if payload.index > base.{c}.len() {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {{}} is past the end ({{}} rows).", payload.index, base.{c}.len()), Vec::<String>::new());
    }}
    protocol::MutationOutcome::new(En1997Diff {{ {c}: Some({rows}::insertion(payload.index, payload.{it}.clone())), ..Default::default() }})
'''
    elif v == "remove":
        body = f'''    if payload.index >= base.{c}.len() {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{r.slug} #{{}} missing", payload.index), vec![payload.index.to_string()]);
    }}
    protocol::MutationOutcome::new(En1997Diff {{ {c}: Some({rows}::removal(&base.{c}, payload.index)), ..Default::default() }})
'''
    else:
        body = f'''    let Some(index) = base.{c}.iter().position(|row| row.id == payload.{it}.id) else {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{r.slug} {{}} missing", payload.{it}.id), vec![payload.{it}.id.clone()]);
    }};
    if base.{c}[index] == payload.{it} {{
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value.");
    }}
    let mut replacement = {rows}::removal(&base.{c}, index);
    replacement.absorb({rows}::insertion(index, payload.{it}.clone()));
    protocol::MutationOutcome::new(En1997Diff {{ {c}: Some(replacement), ..Default::default() }})
'''
    return f'''use crate::{m}::{st};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{{En1997Diff, {rows}}};

pub fn diff(payload: &{st}, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {{
{body}}}
'''

def rs_inverse(r, v):
    st, m = struct(r, v), mod(r, v)
    c, it = r.coll, r.item
    if v == "insert":
        o = "remove"
        return f'''use crate::{m}::{st};
use crate::{mod(r, o)}::{struct(r, o)};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &{st}, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{mod(r, o)}::{struct(r, o)}> {{
    Ok((|| {{
    let at = payload.index.min(base.{c}.len());
    vec![Op::from({struct(r, o)} {{ index: at }})]

    }})())
}}
'''
    if v == "remove":
        o = "insert"
        return f'''use crate::{m}::{st};
use crate::{mod(r, o)}::{struct(r, o)};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &{st}, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{mod(r, o)}::{struct(r, o)}> {{
    Ok((|| {{
    match base.{c}.get(payload.index).cloned() {{
        Some({it}) => vec![Op::from({struct(r, o)} {{ index: payload.index, {it} }})],
        None => Vec::new(),
    }}

    }})())
}}
'''
    return f'''use crate::{m}::{st};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &{st}, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::{m}::{st}> {{
    Ok((|| {{
    match base.{c}.iter().find(|row| row.id == payload.{it}.id).cloned() {{
        Some(prior) => vec![Op::from({st} {{ {it}: prior }})],
        None => Vec::new(),
    }}

    }})())
}}
'''

def manifest(r, v, tag):
    k = kind(r, v)
    d = dirname(r, v)
    return json.dumps({
        "schemaVersion": 1,
        "owner": f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{d}",
        "semanticKind": k,
        "displayName": " ".join(x.title() for x in k.split("-")),
        "emoji": r.emoji[v],
        "aggregateVariant": struct(r, v),
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": tag,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": ["applied", "no-op", "rejected"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }, ensure_ascii=False, indent=2) + "\n"

SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json#/$defs/"
def payload_schema(r, v):
    k, st = kind(r, v), struct(r, v)
    lit = st[0].lower() + st[1:]
    props = {"mutation": {"const": lit}}
    req = ["mutation"]
    if v in ("insert", "remove"):
        props["index"] = {"type": "integer", "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position" if v == "insert" else r.en.capitalize(), "de": "Position" if v == "insert" else r.de}, "description": ({"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."} if v == "insert" else {"en": f"Zero-based position of the {r.en} in its list.", "de": f"Nullbasierte Position von {r.de} in der Liste."}), "step": 1, "precision": 0, "group": "target", "order": 10}}
        req.append("index")
    if v in ("insert", "update"):
        props[camel(r.item)] = {"$ref": SNAP + r.struct, "x-semio-ui": {"role": "value", "label": {"en": r.en.capitalize(), "de": r.de}, "description": ({"en": f"The complete {r.en} record to insert.", "de": f"Der vollständige einzufügende Datensatz ({r.de})."} if v == "insert" else {"en": f"The complete {r.en} record replacing the addressed one by id.", "de": f"Der vollständige Datensatz ({r.de}), der den per ID adressierten ersetzt."}), "group": "record", "order": 20}}
        req.append(camel(r.item))
    return json.dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/en1997/mutation/{k}/schema.json", "title": st, "type": "object", "additionalProperties": False, "required": req, "properties": props}, ensure_ascii=False, indent=2) + "\n"

def payload_ts(r, v):
    k, st = kind(r, v), struct(r, v)
    lit = st[0].lower() + st[1:]
    head = f"/** {r.emoji[v]} `{k}` wire twin: the leaf payload `{st}`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\n"
    if v == "remove":
        return head + f'''import {{ normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired }} from "@semio-tech/norm-js/registry/contract";

export interface {st} {{
  mutation: "{lit}";
  index: number;
}}

export const parse{st}: NormWireReader<{st}> = normWireObject<{st}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})) }});
'''
    if v == "insert":
        return head + f'''import {{ normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired }} from "@semio-tech/norm-js/registry/contract";
import {{ parse{r.struct}, type {r.struct} }} from "../../../📸️snapshot/🟦️.ts";

export interface {st} {{
  mutation: "{lit}";
  index: number;
  {camel(r.item)}: {r.struct};
}}

export const parse{st}: NormWireReader<{st}> = normWireObject<{st}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})), {camel(r.item)}: normWireRequired(parse{r.struct}) }});
'''
    return head + f'''import {{ normWireLiteral, normWireObject, type NormWireReader, normWireRequired }} from "@semio-tech/norm-js/registry/contract";
import {{ parse{r.struct}, type {r.struct} }} from "../../../📸️snapshot/🟦️.ts";

export interface {st} {{
  mutation: "{lit}";
  {camel(r.item)}: {r.struct};
}}

export const parse{st}: NormWireReader<{st}> = normWireObject<{st}>({{ mutation: normWireRequired(normWireLiteral("{lit}")), {camel(r.item)}: normWireRequired(parse{r.struct}) }});
'''

TAG0 = 20
tag = TAG0
for r, v in newkinds():
    d = f"{MUT}/{dirname(r, v)}"
    w(d + "/🦀️.rs", rs_payload(r, v))
    w(d + "/🔺️diff/🦀️.rs", rs_diff(r, v))
    w(d + "/↩️inverse/🦀️.rs", rs_inverse(r, v))
    w(d + "/🔣️.json", manifest(r, v, tag))
    w(d + "/🧬️schema/🔣️.json", payload_schema(r, v))
    w(d + "/🧬️schema/🟦️.ts", payload_ts(r, v))
    tag += 1
print("kinds", tag - TAG0)
