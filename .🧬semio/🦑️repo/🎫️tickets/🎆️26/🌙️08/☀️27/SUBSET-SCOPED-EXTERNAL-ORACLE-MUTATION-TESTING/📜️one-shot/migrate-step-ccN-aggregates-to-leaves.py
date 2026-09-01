import io,json,os,re,sys

BASE="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets"
OWNER="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️{cc}/🧬️schema/🧬️mutations"

LEAF_SPECS={
 "set-snapshot":       dict(dir="📋set-snapshot", emoji="📋", ty="SetSnapshot", variant="SetSnapshot",
                            verb="set", entity="snapshot", record="SetSnapshot", display="Set Snapshot",
                            fields=[("snapshot","StepSnapshot")]),
 "set-file-schema":    dict(dir="🏷set-file-schema", emoji="🏷", ty="SetFileSchema", variant="SetFileSchema",
                            verb="set", entity="file-schema", record="SetFileSchema", display="Set File Schema",
                            fields=[("schemas","Vec<String>")]),
 "set-product-identity":dict(dir="🪪set-product-identity", emoji="🪪", ty="SetProductIdentity", variant="SetProductIdentity",
                            verb="set", entity="product-identity", record="SetProductIdentity", display="Set Product Identity",
                            fields=[("identity","Option<ProductIdentity>")]),
 "set-shape-representation":dict(dir="🪜set-shape-representation", emoji="🪜", ty="SetShapeRepresentation", variant="SetShapeRepresentation",
                            verb="set", entity="shape-representation", record="SetShapeRepresentation", display="Set Shape Representation",
                            fields=[("id","u64"),("representation","Option<ShapeRepresentationRow>")]),
 # ⬇️ `demote` is NOT an approved verb; `change` is. The KIND keeps its name — only SEMANTICS.verb is
 # checked against the table, which is the whole point of correction 2.
 "demote-shape-representation":dict(dir="⬇️demote-shape-representation", emoji="⬇️", ty="DemoteShapeRepresentation", variant="DemoteShapeRepresentation",
                            verb="change", entity="shape-representation", record="DemotedShapeRepresentation", display="Demote Shape Representation",
                            fields=[("id","u64")]),
}
MOD={k:v["ty"] for k,v in LEAF_SPECS.items()}
def modname(kind): return kind.replace("-","_")

def brace_match(s, open_idx):
    depth=0
    for i in range(open_idx,len(s)):
        if s[i]=="{": depth+=1
        elif s[i]=="}":
            depth-=1
            if depth==0: return i
    raise ValueError("unbalanced")

def newtype_rewrite(s, agg, kinds):
    """`Agg::Variant { .. }` -> `Agg::Variant(mod::Ty { .. })`, patterns and constructions alike."""
    for kind in kinds:
        spec=LEAF_SPECS[kind]; var=spec["variant"]; mod=modname(kind); ty=spec["ty"]
        out=[]; i=0
        needle=f"{agg}::{var} {{"
        while True:
            j=s.find(needle,i)
            if j==-1:
                out.append(s[i:]); break
            k=brace_match(s, j+len(needle)-1)
            inner=s[j+len(needle)-1:k+1]
            out.append(s[i:j]); out.append(f"{agg}::{var}({mod}::{ty} {inner})")
            i=k+1
        s="".join(out)
        # `Agg::Variant { .. }` with a rest-pattern collapses to a binding-free newtype pattern
        s=s.replace(f"{agg}::{var}({mod}::{ty} {{ .. }})", f"{agg}::{var}(_)")
    return s

def write_leaves(cc, kinds, agg):
    root=os.path.join(BASE, f"✳️{cc}", "🧬️schema", "🧬️mutations")
    owner=OWNER.format(cc=cc)
    for kind in kinds:
        spec=LEAF_SPECS[kind]; d=os.path.join(root,spec["dir"]); os.makedirs(d,exist_ok=True)
        fields="\n".join(f"    pub {n}: {t}," for n,t in spec["fields"])
        uses=["use crate::artifacts::step::StepSnapshot;",
              f"use crate::artifacts::step::standards::v_ap214::subsets::{cc}::schema::mutations::{{{agg}}};",
              "use serde::{Deserialize, Serialize};"]
        if kind=="set-snapshot":
            uses.insert(1,"use protocol::command::DiffAlgebra;")
            uses.insert(1,"use crate::artifacts::step::schema::diff::StepDiff;")
            diff="        protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &self.snapshot))"
            inv=f"        vec![{agg}::SetSnapshot(SetSnapshot {{ snapshot: base.clone() }})]"
            label='format!("Set the whole {cc} snapshot")'.replace("{cc}",cc.upper())
            target="Vec::new()"
        else:
            uses.insert(1,f"use crate::artifacts::step::standards::v_ap214::subsets::{cc}::schema::mutations::{{class_diff, class_inverse}};")
            uses.insert(1,"use crate::artifacts::step::standards::v_ap214::engine::ladder::ClassEdit;")
            if kind=="set-file-schema":
                edit="ClassEdit::FileSchema { schemas: self.schemas.clone() }"
                label='format!("Set FILE_SCHEMA to [{}]", self.schemas.join(", "))'; target="self.schemas.clone()"
            elif kind=="set-product-identity":
                uses.insert(1,"use crate::artifacts::step::standards::v_ap214::engine::ladder::ProductIdentity;")
                edit="ClassEdit::ProductIdentity { identity: self.identity.clone() }"
                label='format!("Set the PRODUCT identity chain")'; target="Vec::new()"
            elif kind=="set-shape-representation":
                uses.insert(1,"use crate::artifacts::step::standards::v_ap214::engine::ladder::ShapeRepresentationRow;")
                edit="ClassEdit::Representation { id: self.id, row: self.representation.clone() }"
                label='format!("Set shape representation #{}", self.id)'; target="vec![self.id.to_string()]"
            else:
                edit="ClassEdit::Demotion { id: self.id }"
                label='format!("Demote shape representation #{} onto this class\'s ceiling", self.id)'; target="vec![self.id.to_string()]"
            diff=f"        class_diff(base, &{edit})"
            inv=f"        class_inverse(base, &{edit})"
        body=f'''//! {spec["emoji"]}️ `{kind}` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

''' + "\n".join(uses) + f'''

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct {spec["ty"]} {{
{fields}
}}

impl protocol::MutationKind<StepSnapshot, {agg}> for {spec["ty"]} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{spec["verb"]}", entity: "{spec["entity"]}", kind: "{kind}", record: "{spec["record"]}" }};

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<{agg} as protocol::Mutation<StepSnapshot>>::Diff> {{
{diff}
    }}
    fn inverse(&self, base: &StepSnapshot) -> Vec<{agg}> {{
{inv}
    }}
    fn label(&self) -> String {{
        {label}
    }}
    fn target(&self) -> Vec<String> {{
        {target}
    }}
}}
//#endregion 🔖️Payload
'''
        io.open(os.path.join(d,"🦀️.rs"),"w",encoding="utf8").write(body)
        desc={"schemaVersion":1,"owner":f"{owner}/{spec['dir']}","semanticKind":kind,"displayName":spec["display"],
              "emoji":spec["emoji"],"aggregateVariant":spec["variant"],"payloadSchema":"🔣️payload.schema.json",
              "textOpcode":None,"binaryTag":None,"invertibility":"explicit-mutation","diffParticipation":"detect",
              "outcomeClasses":["applied"],"composition":"atomic","requiredLanguageSurfaces":["rust","json-schema"]}
        io.open(os.path.join(d,"🔣️.json"),"w",encoding="utf8").write(json.dumps(desc,ensure_ascii=False,indent=2)+"\n")
    return root

def migrate(cc):
    n=cc[-1]; agg=f"StepCc{n}Mutation"
    root=os.path.join(BASE, f"✳️{cc}", "🧬️schema", "🧬️mutations")
    aggfile=os.path.join(root,"🦀️.rs")
    s=io.open(aggfile,encoding="utf8").read()
    kinds=["set-snapshot","set-file-schema","set-product-identity","set-shape-representation"]
    if f"{agg}::DemoteShapeRepresentation" in s or "DemoteShapeRepresentation {" in s:
        kinds.append("demote-shape-representation")
    write_leaves(cc,kinds,agg)

    # ── enum block -> leaf decls + newtype enum + derive ────────────────────────────────────────
    start=s.index(f"pub enum {agg} {{")
    doc_start=s.rindex("///", 0, start)
    end=brace_match(s, s.index("{", start))
    decls="\n".join(f'#[path = "{LEAF_SPECS[k]["dir"]}/🦀️.rs"]\npub mod {modname(k)};' for k in kinds)
    variants="\n".join(f'    {LEAF_SPECS[k]["variant"]}({modname(k)}::{LEAF_SPECS[k]["ty"]}),' for k in kinds)
    newblock=f'''//#region 🔖️Leaves
{decls}
//#endregion 🔖️Leaves

/// 📐️ Typed conformance-class mutation for `stdio.step` at `ap214/✳️{cc}`.
///
/// ⚠️ `NoMutation` is GONE — `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
/// leaf payload and a unit variant wraps none. Its only role was `inverse()`'s "nothing to undo" arm,
/// now the empty vector. `SetSnapshot` is KEPT: the derive checks `SEMANTICS.verb`, not the kind, and
/// `set` is approved — so this class's whole-document restore survives intact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "s.stdio.step.{cc}")]
pub enum {agg} {{
{variants}
}}'''
    s=s[:doc_start]+newblock+s[end+1:]

    # ── KINDS + kind() ─────────────────────────────────────────────────────────────────────────
    s=re.sub(r'pub const KINDS: &\[&str\] = &\[[^\]]*\];',
             'pub const KINDS: &[&str] = &[' + ", ".join(f'"{k}"' for k in kinds) + '];', s, count=1)
    kstart=s.index("    pub fn kind(&self) -> &'static str {")
    kend=brace_match(s, s.index("{", kstart))
    arms="\n".join(f'            {agg}::{LEAF_SPECS[k]["variant"]}(_) => "{k}",' for k in kinds)
    s=s[:kstart]+"    pub fn kind(&self) -> &'static str {\n        match self {\n"+arms+"\n        }\n    }"+s[kend+1:]

    # ── drop class_edit (each leaf now owns its own edit) ───────────────────────────────────────
    ce=s.index("    fn class_edit(&self) -> Option<ClassEdit> {")
    doc=s.rindex("    /// 🎚️",0,ce)
    s=s[:doc]+s[brace_match(s, s.index("{",ce))+1:]

    # ── replace the hand-written Mutation impl with the shared helpers ──────────────────────────
    mt=s.index("//#region 🔖️MutationTrait")
    mte=s.index("//#endregion 🔖️MutationTrait")+len("//#endregion 🔖️MutationTrait\n")
    rep_arm=""
    if "set-shape-representation" in kinds:
        rep_arm=f'        Some(ClassEdit::Representation {{ id, row }}) => vec![{agg}::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation {{ id, representation: row }})],\n'
    helpers=f'''//#region 🔖️ClassEdit
/// 🎚️ The diff every ladder-axis leaf produces: perform the class-neutral edit, or report the class's
/// own refusal. One implementation, every leaf a caller.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_diff(base: &StepSnapshot, edit: &ClassEdit) -> protocol::MutationOutcome<StepDiff> {{
    match edited(base, edit) {{
        Ok(next) => protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &next)),
        Err(message) => rejected(message),
    }}
}}

/// ↩️ A real per-axis inverse read off the base wherever this class owns a verb for it, and an
/// explicit whole-snapshot restore where it does not.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn class_inverse(base: &StepSnapshot, edit: &ClassEdit) -> Vec<{agg}> {{
    match ladder::invert_class_edit(&base.to_part21_document(), MAX_RUNG, edit) {{
        Some(ClassEdit::FileSchema {{ schemas }}) => vec![{agg}::SetFileSchema(set_file_schema::SetFileSchema {{ schemas }})],
        Some(ClassEdit::ProductIdentity {{ identity }}) => vec![{agg}::SetProductIdentity(set_product_identity::SetProductIdentity {{ identity }})],
{rep_arm}        _ => vec![{agg}::SetSnapshot(set_snapshot::SetSnapshot {{ snapshot: base.clone() }})],
    }}
}}
//#endregion 🔖️ClassEdit

'''
    s=s[:mt]+helpers+s[mte:]

    # ── helpers reachable from the leaves ──────────────────────────────────────────────────────
    s=s.replace("fn rejected(message: String)","pub(crate) fn rejected(message: String)")
    s=s.replace("fn edited(base: &StepSnapshot, edit: &ClassEdit)","pub(crate) fn edited(base: &StepSnapshot, edit: &ClassEdit)")

    # ── every remaining construction/pattern site ──────────────────────────────────────────────
    s=newtype_rewrite(s, agg, kinds)
    # the identity test loses its subject with the variant
    s=re.sub(r"    #\[test\]\n    fn no_mutation_is_the_identity\(\) \{.*?\n    \}\n\n", "", s, flags=re.S)
    s=re.sub(r"\n *"+agg+r"::NoMutation,", "", s)
    s=s.replace(f"vec![{agg}::NoMutation]","Vec::new()")
    io.open(aggfile,"w",encoding="utf8").write(s)
    return kinds

if __name__=="__main__":
    for cc in sys.argv[1:]:
        print(cc, "->", migrate(cc))
