import os,re,subprocess,sys
REPO="/Users/ueli/Documents/semio"
IFC=REPO+"/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
X3=IFC+"/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
C=REPO+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1101-tools/ifc4x3"
FW=REPO+"/🧰️framework"
SEM=X3+"/🧬️schema"
CRATES={
 "express":dict(name="semio-s-artifact-stdio-ifc4x3-express",pkg=SEM+"/⚖️validation/🌳️express",root=SEM+"/⚖️validation/🌳️express/🦀️.rs",desc="IFC4x3 ADD2 canonical EXPRESS program, declaration metadata and owner indices as immutable statics.",deps=["json-pack","declarations","add2-catalog"]),
 "schema":dict(name="semio-s-artifact-stdio-ifc4x3-schema",pkg=SEM+"/📚️catalog/🧭️schema",root=SEM+"/📚️catalog/🦀️.rs",desc="IFC4x3 ADD2 retained catalog access, structural domain checks and borrowed attribute queries.",deps=["express","declarations","add2-catalog","domain","model"]),
 "evaluator":dict(name="semio-s-artifact-stdio-ifc4x3-evaluator",pkg=SEM+"/⚖️validation/🧮️evaluator",root=SEM+"/⚖️validation/🧮️evaluator/🦀️.rs",desc="Controlled native EXPRESS evaluator and structural population validation.",deps=["express","schema","model"],dev=["step","schema"]),
 "population":dict(name="semio-s-artifact-stdio-ifc4x3-population",pkg=SEM+"/⚖️validation/👥️population",root=SEM+"/⚖️validation/👥️population/🦀️.rs",desc="Copy-on-write population overlays, receivers and admissions of the retained EXPRESS machine.",deps=["express","model"],dev=[]),
 "machine":dict(name="semio-s-artifact-stdio-ifc4x3-machine",pkg=SEM+"/⚖️validation/⚙️machine",root=SEM+"/⚖️validation/⚙️machine/🦀️.rs",desc="Retained resumable EXPRESS machine core with a delegated IFC-domain frame executor.",deps=["express","population","model"],dev=[]),
 "rules":dict(name="semio-s-artifact-stdio-ifc4x3-rules",pkg=SEM+"/⚖️validation/📚️rules",root=SEM+"/⚖️validation/📚️rules/🦀️.rs",desc="IFC-domain frame kinds of the retained EXPRESS machine and the local-rule candidate adapter.",deps=["machine","express","details","model"],dev=["step","schema"]),
 "templates":dict(name="semio-s-artifact-stdio-ifc4x3-templates",pkg=SEM+"/🏷️templates",root=SEM+"/🏷️templates/🦀️.rs",desc="Released IFC4x3 property and quantity templates with entity applicability.",deps=["express","json-pack"],dev=[]),
 "mvd-catalog":dict(name="semio-s-artifact-stdio-ifc4x3-mvd-catalog",pkg=SEM+"/👁️mvd/📖️catalog",root=SEM+"/👁️mvd/📖️catalog/🦀️.rs",desc="Immutable IFC4x3 ADD2 view-scope rows decoded from the committed resource.",deps=["json-pack"],dev=[]),
 "mvd":dict(name="semio-s-artifact-stdio-ifc4x3-mvd",pkg=SEM+"/👁️mvd",root=SEM+"/👁️mvd/🦀️.rs",desc="Published IFC4x3 ADD2 view scope, header declarations and concept preparation.",deps=["express","schema","mvd-catalog","model"],dev=["step","schema"]),
 "step":dict(name="semio-s-artifact-stdio-ifc4x3-step",pkg=X3+"/🚪️io/🧾️step",root=X3+"/🚪️io/step.rs",desc="IFC4x3 ADD2 STEP boundary and registered native graph composition.",deps=["model"],dev=[]),
 "exchange":dict(name="semio-s-artifact-stdio-ifc4x3-exchange",pkg=X3+"/🚪️io/🌉️exchange",root=X3+"/🚪️io/native_exchange.rs",desc="Native IFC4x3 and glTF artifact composition with bounded atomic geometry projection.",deps=["step","express","model"],dev=[]),
 "fem":dict(name="semio-s-artifact-stdio-ifc4x3-fem",pkg=X3+"/🚪️io/🏗️fem",root=X3+"/🚪️io/🏗️fem/🦀️.rs",desc="Native IFC4x3 structural and BIM domain exchange.",deps=["exchange","step","express","schema","model"],dev=[]),
}
WS={"json-pack":"semio-s-artifact-stdio-ifc-json-pack","declarations":"semio-s-artifact-stdio-ifc-declarations","add2-catalog":"semio-s-artifact-stdio-ifc4x3-add2-catalog","domain":"semio-s-artifact-stdio-ifc-domain","model":"semio-s-artifact-stdio-ifc-4-model","details":"semio-s-artifact-stdio-ifc-details"}
for k,v in CRATES.items(): WS[k]=v["name"]
FWPATH={
 "semio_framework_diagnostic":"🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
 "semio_framework_dsl":"🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust",
 "semio_framework_ui_locale":"🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust",
 "semio_framework_schema_registry":"🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",
 "semio_framework_2d_compute":"🧰️framework/🔨️modules/◻️2d/🧮️compute/📦️packages/🦀️rust",
 "semio_framework_value":"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
 "semio_framework_deflate":"🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust",
 "semio_framework_async":"🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
 "semio_framework_action_bus":"🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust",
 "semio_framework_pack_json":"🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
 "semio_framework_trace":"🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
 "semio_framework_value_native_owned":"🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust",
 "semio_framework_ui_contract":"🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
 "semio_framework_plugin_decl_staged_form":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/staged-form/📦️packages/🦀️rust",
 "semio_s_artifact_bim_model_snapshot":"✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust",
 "semio_s_artifact_bim_model_schema":"✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust",
 "semio_s_artifact_fem_3d_model":"✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust",
}
WSEXT={"semio_framework_artifact_reference":"semio-framework-artifact-reference","pack":"pack","semio_framework_os_kernel":"semio-framework-os-kernel","protocol":"semio-framework-os-kernel","semio_framework_plugin":"semio-framework-plugin","semio_framework_job":"semio-framework-job","semio_framework_value_derive":"semio-framework-value-derive","value_derive":"semio-framework-value-derive","semio_s_artifact_stdio_binary":"semio-s-artifact-stdio-binary","semio_s_artifact_stdio_contract":"semio-s-artifact-stdio-contract","semio_s_artifact_stdio_txt":"semio-s-artifact-stdio-txt","semio_s_artifact_stdio_gltf_model":"semio-s-artifact-stdio-gltf-model","semio_framework_async_macros":"semio-framework-async-macros","semio_s_artifact_stdio_ifc_4_mutations":"semio-s-artifact-stdio-ifc-4-mutations"}
def pkgdir(c): return c["pkg"]+"/📦️packages/🦀️rust"
def rel(frm,to): return os.path.relpath(to,frm)
def closure(c,mode):
    out=subprocess.run(["bun",C+"/files.ts",pkgdir(c),mode],capture_output=True,text=True)
    if out.returncode: print(out.stderr[-600:]); sys.exit(1)
    return [l for l in out.stdout.split("\n") if l]
def idents(files):
    found=set()
    for f in files:
        if not f.endswith(".rs"): continue
        t=open(f,encoding="utf-8").read()
        for m in re.finditer(r"\b(semio_[a-z0-9_]+)\b",t): found.add(m.group(1))
        if re.search(r"\bpack::",t): found.add("pack")
        if re.search(r"\bprotocol::",t): found.add("protocol")
        if re.search(r"\bvalue_derive\b",t): found.add("value_derive")
    return found
def write_min(c):
    d=pkgdir(c); os.makedirs(d,exist_ok=True)
    p=d+"/Cargo.toml"
    open(p,"w",encoding="utf-8").write('[package]\nname = "%s"\n\n[lib]\npath = "%s"\n'%(c["name"],rel(d,c["root"])))
def manifest(key,c):
    d=pkgdir(c)
    ALWAYS={"semio_framework_trace","semio_framework_value_native_owned","semio_framework_ui_contract","semio_format"}
    prod=idents(closure(c,"prod"))-ALWAYS; both=idents(closure(c,"test")); dev_only=(both-prod)-{"semio_format"}
    own={v["name"].replace("-","_") for v in CRATES.values()}|{"semio_s_artifact_stdio_ifc_json_pack","semio_s_artifact_stdio_ifc_declarations","semio_s_artifact_stdio_ifc4x3_add2_catalog","semio_s_artifact_stdio_ifc_domain","semio_s_artifact_stdio_ifc_4_model","semio_s_artifact_stdio_ifc_details"}
    lines=['[package]','workspace = "%s"'%rel(d,IFC),'name = "%s"'%c["name"],'version.workspace = true','edition.workspace = true','rust-version.workspace = true','description = "%s"'%c["desc"],'','[package.metadata.semio]','role = "library"','','[lints]','workspace = true','','[lib]','path = "%s"'%rel(d,c["root"]),'','[dependencies]']
    depnames=[]
    for ws in c["deps"]: depnames.append(("ws",WS[ws]))
    def spec(i):
        if i in WSEXT: return (WSEXT[i],'{ workspace = true }')
        if i in FWPATH: return (i.replace("_","-"),'{ path = "%s" }'%rel(d,REPO+"/"+FWPATH[i]))
        return None
    seen=set(n for _,n in depnames)
    for kind,n in depnames: lines.append('%s = { workspace = true }'%n)
    unknown=[]
    for i in sorted(prod):
        if i in own: continue
        s=spec(i)
        if not s: unknown.append(i); continue
        if s[0] in seen: continue
        seen.add(s[0]); lines.append('%s = %s'%s)
    dev=[]
    for dv in c.get("dev",[]):
        n=WS[dv]
        if n not in seen: dev.append('%s = { workspace = true }'%n)
    for i in sorted(dev_only):
        if i in own: continue
        s=spec(i)
        if not s: unknown.append(i); continue
        if s[0] in seen: continue
        seen.add(s[0]); dev.append('%s = %s'%s)
    feats={"semio_framework_plugin":'"artifact-app-testing"',"semio_framework_os_kernel":'"protocol-laws", "mutation-testing"',"semio_s_artifact_stdio_contract":'"testing"'}
    featured={WSEXT[i]:'%s = { workspace = true, features = [%s] }'%(WSEXT[i],f) for i,f in feats.items() if i in both}
    dev=[x for x in dev if x.split(" = ")[0] not in featured]
    lines+=['','[dev-dependencies]']+dev+list(featured.values())
    lines.append('semio-framework-async-macros = { workspace = true }')
    lines.append('serde_json = { workspace = true }')
    open(d+"/Cargo.toml","w",encoding="utf-8").write("\n".join(lines)+"\n")
    print(key,"unknown",unknown,"prod",sorted(i for i in prod if i not in own),"devonly",sorted(i for i in dev_only if i not in own))
if __name__=="__main__":
    for k,c in CRATES.items(): write_min(c)
    for k,c in CRATES.items():
        if len(sys.argv)>1 and k not in sys.argv[1:]: continue
        manifest(k,c)
