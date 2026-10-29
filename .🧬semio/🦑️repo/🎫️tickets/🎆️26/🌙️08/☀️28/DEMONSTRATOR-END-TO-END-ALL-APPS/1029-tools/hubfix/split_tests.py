import os,sys,shutil,re
B="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/"
def run(d,n,vol):
    root=B+d+"/🏅️standards/🔖️1/🪆️subsets/🌐️any/"
    snap=root+"🧬️schema/📸️snapshot/"
    t=root+"🧪️tests/"
    pk=t+"📦️packages/🦀️rust/"
    os.makedirs(pk,exist_ok=True)
    os.makedirs(t+"loads",exist_ok=True)
    mv=[("native-tests.rs","native-tests.rs"),("identity-index-tests.rs","identity-index-tests.rs"),("identity-index.json","identity-index.json"),("write-pack-tests.rs","write-pack-tests.rs"),("loads/receiver-tests.rs","loads/receiver-tests.rs"),("loads/cases.json","loads/cases.json"),("loads/schema.json","loads/schema.json"),("loads/loads.feature","loads/loads.feature")]
    for a,b in mv:
        if os.path.exists(snap+a): shutil.move(snap+a,t+b)
    # strip mounts
    def strip(f,pat):
        s=open(snap+f).read()
        s2=re.sub(pat,"",s)
        assert s!=s2,(f)
        open(snap+f,'w').write(s2)
    strip("native.rs",r'#\[cfg\(test\)\]\n#\[path = "native-tests.rs"\]\nmod tests;\n?')
    strip("identity-index.rs",r'#\[cfg\(test\)\]\n#\[path = "identity-index-tests.rs"\]\nmod tests;\n?')
    strip("write-pack.rs",r'#\[cfg\(test\)\]\n#\[path = "write-pack-tests.rs"\]\nmod tests;\n?')
    strip("receive-record.rs",r'#\[cfg\(test\)\]\n#\[path = "loads/receiver-tests.rs"\]\nmod load_tests;\n')
    # rewrite crate:: references in moved tests
    M=f"semio_s_artifact_fem_{n}d_model::standards::v1::subsets::any::schema"
    for f in ["native-tests.rs","identity-index-tests.rs","write-pack-tests.rs","loads/receiver-tests.rs"]:
        p=t+f
        s=open(p).read()
        s=s.replace("crate::standards::v1::subsets::any::schema",M)
        open(p,'w').write(s)
    V="volumetric" if vol else "planar"
    model_cargo=open(root+"🧬️schema/📦️packages/🦀️rust/Cargo.toml").read()
    deps=model_cargo[model_cargo.index("[dependencies]"):model_cargo.index("[dev-dependencies]")]
    deps=deps.replace(f"semio-s-artifact-fem-{n}d-model = {{ workspace = true }}\n","")
    deps+=f"semio-s-artifact-fem-{n}d-model = {{ workspace = true }}\nsemio-s-artifact-fem-{n}d-io = {{ workspace = true }}\nsemio-framework-value-registry = {{ workspace = true }}\n" if False else f"semio-s-artifact-fem-{n}d-model = {{ workspace = true }}\nsemio-s-artifact-fem-{n}d-io = {{ workspace = true }}\n"
    cargo=f'''[package]
workspace = "../../../../../../.."
name = "semio-s-artifact-fem-{n}d-snapshot-tests"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "FEM {n}d persistent snapshot laws: native receiving, identity index, Pack publication, load receiving and bundled example loading (test-only crate)"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

'''+deps+"\n[dev-dependencies]\nserde_json = { workspace = true }\n"
    open(pk+"Cargo.toml","w").write(cargo)
    N=f"Fem{n}d"
    lib=f'''//! 🧪️ {N} persistent snapshot laws, kept in a dedicated test crate so the model crate stays inside its source boundary.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
use semio_s_artifact_fem_{n}d_model::standards::v1::subsets::any::schema::snapshot::*;
#[cfg(test)]
use semio_s_artifact_fem_{n}d_model::standards::v1::subsets::any::schema::structure::*;
#[cfg(test)]
use semio_framework_dsl_record::receiving::{{NativeSnapshotDecodeOwner, work}};
#[cfg(test)]
use semio_framework_dsl_record::{{FieldValue, RecordValue}};
#[cfg(test)]
use semio_framework_value::retirement::RetireOwned;
#[cfg(test)]
use semio_framework_value::{{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind}};
#[cfg(test)]
use semio_framework_value::retained_clone::retained_clone_grant_admits;
#[cfg(test)]
use semio_framework_value_native_owned::native_decoding::{{NativeDecodeConstructionRecipient, NativeDecodeRetirementRecipient}};
#[cfg(test)]
use semio_s_fem_material::{V}::FemMaterialValue;
#[cfg(test)]
use semio_s_fem_node::{V}::FemNodeValue;

#[cfg(test)]
#[path = "native-tests.rs"]
mod native_tests;
#[cfg(test)]
#[path = "identity-index-tests.rs"]
mod identity_index_tests;
#[cfg(test)]
#[path = "write-pack-tests.rs"]
mod write_pack_tests;
#[cfg(test)]
#[path = "loads/receiver-tests.rs"]
mod load_tests;
'''
    open(t+"🦀️.rs","w").write(lib)
for a in sys.argv[1:]:
    d,n,vol=a.split(":")
    run(d,n,vol=="1")
