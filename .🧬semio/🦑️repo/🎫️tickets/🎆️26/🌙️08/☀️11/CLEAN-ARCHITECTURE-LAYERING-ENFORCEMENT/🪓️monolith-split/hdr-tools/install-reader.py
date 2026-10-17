import sys,re
# 📎️ Adds the default schema-leaf reader install after every register_schema_leaf_documents call that lacks one (idempotent).
p=sys.argv[1]; asset=sys.argv[2] if len(sys.argv)>2 else "semio_framework_plugin::reactor::package_asset"
s=open(p).read()
rx=re.compile(r'(    ::semio_framework_schema_registry::register_schema_leaf_documents\(&\[\n(?:.*\n)*?    \]\);\n)(?!    #\[cfg\(target_arch = "wasm32"\)\]\n    semio_framework_schema_registry::install_default_schema_leaf_reader)')
n=0
def rep(m):
    global n; n+=1
    return m.group(1)+f'    #[cfg(target_arch = "wasm32")]\n    semio_framework_schema_registry::install_default_schema_leaf_reader({asset}, "");\n    #[cfg(not(target_arch = "wasm32"))]\n    semio_framework_schema_registry::install_default_schema_leaf_reader({asset}, env!("CARGO_MANIFEST_DIR"));\n'
s2=rx.sub(rep,s)
open(p,"w").write(s2); print("installs added:",n, "register calls:",s2.count("register_schema_leaf_documents(&["), "installs:",s2.count("install_default_schema_leaf_reader("))
