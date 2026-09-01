import io,os,re,sys
sys.path.insert(0,"/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad")
from migrate_cc import LEAF_SPECS, modname, newtype_rewrite

def fix(cc, kinds):
    n=cc[-1]; agg=f"StepCc{n}Mutation"
    p=f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧪️tests/mutate-step-ap214-{cc}/🦀️.rs"
    if not os.path.isfile(p): return f"{cc}: no harness"
    s=io.open(p,encoding="utf8").read()
    s=re.sub(r'\n *"no-mutation" => '+agg+r'::NoMutation,','',s)
    s=newtype_rewrite(s, agg, kinds)
    # the harness is an external crate: leaf types need their full path
    base=f"semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::{cc}::schema::mutations"
    for k in kinds:
        spec=LEAF_SPECS[k]
        s=s.replace(f"{agg}::{spec['variant']}({modname(k)}::{spec['ty']} ", f"{agg}::{spec['variant']}({base}::{modname(k)}::{spec['ty']} ")
    io.open(p,"w",encoding="utf8").write(s)
    return f"{cc}: harness updated"

if __name__=="__main__":
    kinds=["set-snapshot","set-file-schema","set-product-identity","set-shape-representation","demote-shape-representation"]
    for cc in sys.argv[1:]:
        print(fix(cc, kinds))
