import re, os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
import rgl
V = rgl.ROOT + "/🧰️framework/🔨️modules/🌱️value"
def facade_map():
    m = {}
    for f in [V + "/📦️packages/🦀️rust/🦀️.rs", V + "/🦀️.rs"]:
        t = rgl.read(f)
        t = re.sub(r'//[^\n]*', '', t)
        for mo in re.finditer(r'pub use\s+([A-Za-z_][A-Za-z0-9_:]*?)::\{([^}]*)\}\s*;', t):
            base = mo.group(1)
            if not base.startswith("semio_") and not base.startswith("serde"): continue
            for item in re.split(r',', mo.group(2)):
                item = item.strip()
                if not item: continue
                parts = re.split(r'\s+as\s+', item)
                name = parts[-1].strip()
                m[name] = f"{base}::{item}"
        for mo in re.finditer(r'pub use\s+([A-Za-z_][A-Za-z0-9_:]*?)::([A-Za-z_][A-Za-z0-9_]*)(?:\s+as\s+([A-Za-z_][A-Za-z0-9_]*))?\s*;', t):
            base = mo.group(1)
            if not base.startswith("semio_") and not base.startswith("serde"): continue
            name = mo.group(3) or mo.group(2)
            m[name] = f"{base}::{mo.group(2)}" + (f" as {mo.group(3)}" if mo.group(3) else "")
        for mo in re.finditer(r'pub use\s+(semio_[A-Za-z0-9_]+|serde|serde_json)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)\s*;', t):
            m[mo.group(2)] = f"{mo.group(1)} as {mo.group(2)}"
        for mo in re.finditer(r'pub use\s+(serde|serde_json)\s*;', t):
            m[mo.group(1)] = mo.group(1)
    return m
if __name__ == "__main__":
    m = facade_map()
    print(len(m))
    for k in ["retained_clone","RetainedCloneGrant","RetireOwned","ValueError","retirement","DslValue","NativeEncodeControl","ToValue","native_decoding","shared_issuance","SharedIssuer","serde","artifact_retire_leaf"]:
        print(k, "=>", m.get(k))
