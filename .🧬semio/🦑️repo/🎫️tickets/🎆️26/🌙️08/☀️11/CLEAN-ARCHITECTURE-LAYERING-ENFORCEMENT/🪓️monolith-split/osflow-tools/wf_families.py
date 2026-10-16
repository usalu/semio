"""📚 Planned file ownership of the workflow producers (relative to the workflow artifact dir)."""
import os
os.environ["OSFAM"]="workflow"
from manifests import FL
MUT=["add-input","add-node","add-parameter","bind-input","bind-output","bind-parameter-field","change-parameter","connect-ports","disconnect-edge","move-node","move-nodes","remove-input","remove-node","remove-parameter","rename-node","unbind-input","unbind-output","unbind-parameter-field","set-node-positions","update-node-ports"]
def mutation_files():
    d=FL+"🧬️schema/🧬️mutations/"
    out=["🧬️schema/🧬️mutations/🦀️.rs"]
    for e in sorted(os.listdir(d)):
        p=d+e+"/🦀️.rs"
        if os.path.exists(p):out.append("🧬️schema/🧬️mutations/"+e+"/🦀️.rs")
    return out
LIB={
 "graph":["🕸️graph/🦀️.rs"],
 "snapshot":["📸️snapshot/🦀️.rs","🚪️io/🦀️.rs","🚪️io/💾️binary/🦀️.rs","🚪️io/💾️binary/📸️snapshot/🦀️.rs","🚪️io/📝️text/🦀️.rs","🚪️io/📝️text/📸️snapshot/🦀️.rs","🚪️io/🪶️sqlite/🦀️.rs","🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs","🚪️io/🪶️sqlite/📸️snapshot/💰️backing/🦀️.rs","🚪️io/🪶️sqlite/📸️snapshot/📏️cells/🦀️.rs"],
 "schema":["🧬️schema/🦀️.rs","🚪️io/💾️binary/🧬️mutations/🦀️.rs","🚪️io/📝️text/🧬️mutations/🦀️.rs"]+mutation_files(),
}
def leaf_tests():
    d=FL+"🧬️schema/🧬️mutations/"
    return ["🧬️schema/🧬️mutations/"+e+"/🧪️tests/🔬️unit/🦀️.rs" for e in sorted(os.listdir(d)) if os.path.exists(d+e+"/🧪️tests/🔬️unit/🦀️.rs")]
LIB["schema-laws"]=["🧬️schema/⚖️laws/🦀️.rs"]
TESTS={"graph":[],"snapshot":[],"schema":[],"schema-laws":["🧪️tests/🔁️workflow/🦀️.rs","🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]+leaf_tests()}
