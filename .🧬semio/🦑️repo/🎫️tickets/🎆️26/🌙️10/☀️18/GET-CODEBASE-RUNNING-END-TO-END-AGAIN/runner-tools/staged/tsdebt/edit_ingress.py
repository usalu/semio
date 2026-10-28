"""✏️ tsdebt part 2: the raw-cargo lease fixtures pin the toolchain's own rustc and no wrapper, so the repository's compiler ingress (`.cargo/config.toml` rustc + rustc-wrapper) never refuses them when their scratch lies inside the checkout (`python3 -I edit_ingress.py`, idempotent)."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
EDITS = {
 "f05.ts": [('RUSTUP_TOOLCHAIN:"",CARGO_TARGET_DIR:join(work,target)', 'RUSTUP_TOOLCHAIN:"",RUSTC:join(dirname(cargo!),"rustc"),RUSTC_WRAPPER:"",CARGO_TARGET_DIR:join(work,target)'),
            ('RUSTUP_TOOLCHAIN:"",CARGO_TARGET_DIR:work+"/t-"+variant', 'RUSTUP_TOOLCHAIN:"",RUSTC:cargo.slice(0,-5)+"rustc",RUSTC_WRAPPER:"",CARGO_TARGET_DIR:work+"/t-"+variant')],
 "f06.ts": [('RUSTUP_TOOLCHAIN:"",CHURN_LOG:log', 'RUSTUP_TOOLCHAIN:"",RUSTC:join(dirname(cargo!),"rustc"),RUSTC_WRAPPER:"",CHURN_LOG:log')],
}
for name, pairs in EDITS.items():
    path = os.path.join(HERE, "files", name); text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if text.count(new) == 1: continue
        assert text.count(old) == 1, (name, old); text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
